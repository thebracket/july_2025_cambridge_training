# Workshop: Async MUD Server, Part 2

Going back to the `main.rs` file, uncomment the call to `handle_connection` in the `tokio::spawn` block, and let's implement that function. 

```rust
async fn handle_connection(mut socket: tokio::net::TcpStream, addr: std::net::SocketAddr) -> anyhow::Result<()> {
    // Magic number and login always happen first
    check_magic_number(&mut socket).await?;
    let Ok(user) = handle_login(&mut socket).await else {
        tracing::warn!("Closing connection from {} due to failed login", addr);
        return Ok(());
    };
    tracing::info!("User {} connected from {}", user.username, addr);

    // Find a starting room
    let starting_room = world_manager::find_starting_room().await?;
    tracing::info!("User {} starting in room {}", user.username, starting_room);

    // Spawn the player in the world
    let (player_world_tx, player_world_rx) = tokio::sync::mpsc::channel(32);
    world_manager::spawn_player(&user.username, &starting_room, player_world_tx.clone()).await?;

    // Main loop and clean up
    if let Err(e) = player_loop(socket, &user, player_world_tx, player_world_rx).await {
        tracing::error!("Error in player loop for {}: {:?}", user.username, e);
    }
    world_manager::despawn_player(&user.username).await?;
    tracing::info!("User {} disconnected", user.username);

    Ok(())
}
```

So we check that the magic number is correct, and handle login. Then we ask the world manager for a starting room. We create the player-world communication channel, and tell the world manager to spawn the player. Finally we enter the `player_loop`, which we'll implement next. After the loop exits, we tell the world manager to despawn the player.

So what happens in the player loop? We SPLIT the socket into read and write halves (in synchronous land you can just clone it). then we break the parts up into three tasks --- all waiting for something to do.

```rust
async fn player_loop(
    mut socket: tokio::net::TcpStream,
    user: &User,
    player_world_tx: Sender<MudMessage>,
    mut player_world_rx: Receiver<MudMessage>,
) -> anyhow::Result<()> {
    // Split the socket into read and write halves. This allows us to read and write concurrently.
    let (mut socket_read, mut socket_write) = socket.split();

    // Start a timer to send periodic pings to the client
    let mut ping_interval = tokio::time::interval(std::time::Duration::from_secs(30));

    loop {
        select! {
            // If there is an outbound message for the player, send it.
            Some(message) = player_world_rx.recv() => {
                send_message(&mut socket_write, &message).await?;
            }

            // Process any inbound messages from the player.
            message = read_message(&mut socket_read) => {
                if let PlayerMessageResult::Disconnect = player_message(message?, user).await? {
                    tracing::info!("Player {} requested disconnect", user.username);
                    break;
                }
            }

            // Send periodic pings to keep the connection alive
            _ = ping_interval.tick() => {
                player_world_tx.send(MudMessage::Ping).await?;
            }
        }
    }

    Ok(())
}
```

We're multiplexing between receiving messages from the socket, receiving MudMessages that needs to be sent, and a timer for sending periodic pings to keep the connection alive.

The `player_message` function handles messages from the player:

```rust
enum PlayerMessageResult {
    Continue,
    Disconnect,
}

async fn player_message(message: MudMessage, user: &User) -> anyhow::Result<PlayerMessageResult> {
    match message {
        // Network messages
        MudMessage::Ping => {
            tracing::debug!("Received ping from {}", user.username);
        }
        MudMessage::Disconnect => {
            return Ok(PlayerMessageResult::Disconnect);
        }
        MudMessage::TryExit { direction } => {
            world_manager::move_player(&user.username, &direction).await?;
        }
        MudMessage::PlayerSpeak { message, .. } => {
            world_manager::player_speak(&user.username, &message).await?;
        }
        _ => {}
    }
    Ok(PlayerMessageResult::Continue)
}
```