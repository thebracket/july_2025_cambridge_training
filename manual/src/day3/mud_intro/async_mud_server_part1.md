# Workshop: Async MUD Server, Part 1

In your MUD server, make sure you are depending upon `tokio`, and the `mud_protocol` crate with the `tokio` feature enabled. I've also added `rand` for random number generation, and the login and rooms libraries from Day 2:

```toml
[dependencies]
tokio.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
anyhow.workspace = true
async_mud_proto = { path = "../async_mud_proto", features = [ "tokio" ] }
login_library2 = { path = "../../day2/login_library2" }
rooms_library2 = { path = "../../day2/rooms_library2" }
rand = "0.9.1"
```

Now setup you `main.rs` to use Tokio, and initialize tracing:

```rust
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
            /*if let Err(e) = handle_connection(socket, addr).await {
                tracing::error!("Error handling connection from {}: {:?}", addr, e);
            }*/
        });
    }
}
```

This is a very standard loop: we set everything up, bind to a TCP port, and then loop accepting connections. Each connection is handled in its own task using `tokio::spawn`. I've commented out the actual connection handler. Let's implement the `world_manager`.

Make a new file, `world_manager.rs`. In your `main.rs` file, add `mod world_manager;` at the top. We're following a basic actor pattern:

```rust
// Store the command sender
static WORLD_COMMAND_TX: OnceCell<Sender<WorldCommand>> = OnceCell::const_new();

// Function to start the world manager
pub fn run() -> anyhow::Result<()> {
    // Load the rooms
    let rooms = rooms_library2::RoomLibrary::load()?;
    
    // Find starting points
    let starting_rooms: Vec<String> = rooms
        .iter()
        .filter(|(_name, room)| room.start)
        .map(|(name, _)| name.clone())
        .collect::<Vec<_>>();
    if starting_rooms.is_empty() {
        return Err(anyhow::anyhow!("No starting rooms found in the room library"));
    }

    let (tx, rx) = tokio::sync::mpsc::channel(100);
    WORLD_COMMAND_TX.set(tx)?;

    // Start the main loop
    tokio::spawn(async move {
        // Call into the main loop
        main_loop(rooms, starting_rooms, rx).await;
    });

    Ok(())
}

// List of commands the world manager can handle
enum WorldCommand {
    FindStartingRoom { reply: tokio::sync::oneshot::Sender<String>},
    PlayerSpawn { username: String, room: String, player_tx: Sender<MudMessage> },
    DespawnPlayer { username: String },
    PlayerMove { username: String, direction: String },
    Speak { username: String, message: String },
}
```

And then each command exposes a public function for use elsewhere:

```rust
pub async fn find_starting_room() -> anyhow::Result<String> {
    let tx = WORLD_COMMAND_TX.get().ok_or_else(|| anyhow::anyhow!("World command channel not initialized"))?;
    let (reply_tx, reply_rx) = tokio::sync::oneshot::channel();
    
    tx.send(WorldCommand::FindStartingRoom { reply: reply_tx })
        .await
        .map_err(|_| anyhow::anyhow!("Failed to send command to world manager"))?;
    
    reply_rx.await.map_err(|_| anyhow::anyhow!("Failed to receive starting room"))
}
```

The main loop handles commands, maintaining world state. There's no synchronization needed, since only the main loop task accesses the state:

```rust
#[derive(Clone, Debug)]
struct Player {
    username: String,
    room: String,
    player_tx: Sender<MudMessage>,
}

async fn main_loop(
    rooms: HashMap<String, rooms_library2::Room>,
    starting_rooms: Vec<String>,
    mut world_commands: Receiver<WorldCommand>,
) {
    let mut players: Vec<Player> = Vec::new();

    while let Some(command) = world_commands.recv().await {
        match command {
            WorldCommand::FindStartingRoom { reply } => {
                let room = starting_rooms
                    .choose(&mut rand::rng())
                    .cloned()
                    .unwrap_or_else(|| starting_rooms[0].clone());
                if reply.send(room).is_err() {
                    tracing::warn!("Failed to send starting room");
                }
            }
            // etc...
```

Notice how each `Player` has a `Sender<MudMessage>` to send messages back to the player. This is how we will notify players of events in the world. Sending sockets around gets tricky, but channels are *designed* for communication between tasks (and threads). So we use channels to talk *into* the world - and channels for the world to send messages *back* to players.
