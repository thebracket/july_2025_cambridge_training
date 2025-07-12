# Workshop: Threaded TCP Server and Client

We're going to wrap up the day by building a simple TCP server that can handle multiple clients at once. Then you'll take over, and integrate your Rooms and Login code from earlier. You'll have the beginnings of a MUD server!

Before you start, let's make an example TCP echo server and client (all wrapped together). The code is in `code/day2/tcp_echo`:

```rust
use std::{io::{Read, Write}, thread};

fn server() {
    let listener = std::net::TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        thread::spawn(move || {
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();
            stream.write(&buffer).unwrap();
        });
    }
}

fn main() {
    thread::spawn(|| server());
    thread::sleep(std::time::Duration::from_millis(100)); // Give server time to start

    let mut stream = std::net::TcpStream::connect("127.0.0.1:8080").unwrap();
    stream.write(b"Hello, world!").unwrap();
    println!("Sent: Hello, world!");
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Received: {}", String::from_utf8_lossy(&buffer));

    drop(stream); // Close the client connection
    // Note: In a real application, you'd want a way to gracefully shut down the server    
}
```

This program spawns a TCP server in a thread. A client TcpStream connects to it, sends a message, and reads the echoed message back. The server spawns a new thread for each incoming connection (there won't ever be more than one in this case).

### Task 1: Build a Common Protocol Library

> My version is in `code/day2/sync_mud_proto`.

Create a new shared library for your MUD protocol. We're going to use a serialized command protocol. I used `bincode` --- which is only a good choice when you control both client and server and know they have the same architecture. You can also use `serde_json` or `serde_cbor` or whatever you like.

I defined a `Command` enum:

```rust
use rooms_library2::Room;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum MudMessage {
    Login { username: String, password: String },
    LoginSuccess,
    LoginFail,
    EnterRoom { room: Room },
    TryExit { direction: String },
    Disconnect,
}

impl MudMessage {
    pub fn to_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let bytes = bincode::serialize(self)?;
        Ok(bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        let msg = bincode::deserialize(bytes)?;
        Ok(msg)
    }
}
```

You shouldn't need much more than the scaffolding (Cargo.toml, dependencies).

### Task 2: Build a Synchronous MUD Server

> My version is in `code/day2/sync_mud_server`.

I've made a start on the server for you:

```rust
fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();
    tracing::info!("Starting server");
    
    let rooms = Arc::new(rooms_library2::RoomLibrary::load()?);

    // Start the server
    let server = TcpListener::bind("0.0.0.0:8080")?;
    while let Ok((stream, addr)) = server.accept() {
        tracing::info!("Accepted connection from {}", addr);
        let my_rooms = rooms.clone();
        std::thread::spawn(move || {
            match handle_client(stream, my_rooms) {
                Ok(()) => tracing::info!("Client {} disconnected cleanly", addr),
                Err(e) => tracing::error!("Client {} error: {:?}", addr, e),
            }
        });
    }

    Ok(())
}
```

And to get a sense for how to handle a client connnection:

```rust
fn handle_client(mut stream: std::net::TcpStream, rooms: Arc<HashMap<String, rooms_library2::Room>>) -> anyhow::Result<()> {
    // First, we demand a magic number
    let mut buf = [0u8; 4];
    stream.read_exact(&mut buf)?;
    let magic = u32::from_be_bytes(buf);
    if magic != 0x4D554400 { // "MUD\0"
        anyhow::bail!("Invalid magic number: {:X}", magic);
    }
    tracing::info!("Client sent valid magic number");

    // Placeholder for the login data
    let mut current_user = None;
    let mut current_room = String::new();

    // Now we loop
    loop {
        // We expect a usize containing the number of bytes to read
        let mut len_buf = [0u8; 4];
        stream.read_exact(&mut len_buf)?;
        let len = u32::from_be_bytes(len_buf) as usize;
        if len > 10_000 {
            anyhow::bail!("Client sent too large message length: {}", len);
        }

        // Now we read the actual message
        let mut msg_buf = vec![0u8; len];
        stream.read_exact(&mut msg_buf)?;

        // The message is a bincode-encoded MudMessage
        let msg = MudMessage::from_bytes(&msg_buf)?;
        match msg {
            MudMessage::Login { username, password } => {
                let users = login_library2::LoginManager::new()?;
                if let Some(_user) = users.verify_user(&username, &password) {
                    // Successful login
                    current_user = Some(username.clone());
                    tracing::info!("User {} logged in successfully", username);
                    let resp = MudMessage::LoginSuccess;
                    let resp_bytes = resp.to_bytes()?;
                    stream.write_all(&(resp_bytes.len() as u32).to_be_bytes())?;
                    stream.write_all(&resp_bytes)?;

                    // Pick a starting room
                    let Some(start_room) = rooms.values().find(|room| room.start) else {
                        anyhow::bail!("No starting room found");
                    };
                    current_room = start_room.name.clone();

                    // Send the initial room
                    let enter_msg = MudMessage::EnterRoom { room: start_room.clone() };
                    let enter_bytes = enter_msg.to_bytes()?;
                    stream.write_all(&(enter_bytes.len() as u32).to_be_bytes())?;
                    stream.write_all(&enter_bytes)?;
                } else {
                    tracing::warn!("Failed login attempt for user {}", username);
                    let resp = MudMessage::LoginFail;
                    let resp_bytes = resp.to_bytes()?;
                    stream.write_all(&(resp_bytes.len() as u32).to_be_bytes())?;
                    stream.write_all(&resp_bytes)?;
                }
            }
            // Continues
```

This should be enough to get you started. You'll need to handle `TryExit` messages (trying to move), and `Disconnect` messages (which should just break the loop and close the connection).

### Task 3: Build a Simple Client

The client is very much a single-threaded mirror of the server. It connects, sends a magic number, then sends a `Login` message, waits for a response, and if successful, waits for the initial room. Then it can send `TryExit` messages to move around.

> My version is in `code/day2/sync_mud_client`.