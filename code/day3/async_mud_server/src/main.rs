mod world_manager;
use async_mud_proto::{async_messaging::{read_message, send_message}, MudMessage};
use login_library2::User;
use tokio::{io::AsyncReadExt, select, sync::mpsc::{Receiver, Sender}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setup Logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .compact()
        .init();

    // Setup the World Manager
    world_manager::run()?;

    // Start the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    tracing::info!("Server listening on {}", listener.local_addr()?);
    
    loop {
        let (socket, addr) = listener.accept().await?;
        tracing::info!("New connection from {}", addr);
        tokio::spawn(async move {            
            if let Err(e) = handle_connection(socket, addr).await {
                tracing::error!("Error handling connection from {}: {:?}", addr, e);
            }
        });
    }
}

async fn check_magic_number(socket: &mut tokio::net::TcpStream) -> anyhow::Result<()> {
    let mut buf = [0u8; 4];
    socket.read_exact(&mut buf).await?;
    if buf == [0x4D, 0x55, 0x44, 0x31] { // 'MUD1'
        tracing::info!("Received valid magic number MUD1");
        Ok(())
    } else {
        anyhow::bail!("Invalid magic number: expected MUD1, got {:?}", buf);
    }
}

async fn handle_login(socket: &mut tokio::net::TcpStream) -> anyhow::Result<User> {
    let MudMessage::Login { username, password } = read_message(socket).await? else {
        anyhow::bail!("Expected Login message");
    };
    tracing::info!("Login attempt for user {}", username);
    let logins = login_library2::LoginManager::new()?;
    let Some(user)  = logins.verify_user(&username, &password) else {
        send_message(socket, &MudMessage::LoginFail).await?;
        anyhow::bail!("Login failed for user {}", username);
    };
    send_message(socket, &MudMessage::LoginSuccess).await?;
    tracing::info!("User {} logged in successfully", username);
    Ok(user.clone())
}

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
