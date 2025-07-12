use std::{collections::HashMap, io::{Read, Write}, net::TcpListener, sync::Arc};
use sync_mud_proto::MudMessage;

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
            MudMessage::TryExit { direction } => {
                if current_user.is_none() {
                    anyhow::bail!("Client tried to exit without logging in");
                }
                let current_room_data = rooms.get(&current_room).ok_or_else(|| anyhow::anyhow!("Current room not found: {}", current_room))?;
                if let Some(exit) = current_room_data.exits.iter().find(|e| e.direction == direction) {
                    if let Some(new_room) = rooms.get(&exit.room_name) {
                        current_room = new_room.name.clone();
                        let enter_msg = MudMessage::EnterRoom { room: new_room.clone() };
                        let enter_bytes = enter_msg.to_bytes()?;
                        stream.write_all(&(enter_bytes.len() as u32).to_be_bytes())?;
                        stream.write_all(&enter_bytes)?;
                        tracing::info!("User {:?} moved to room {}", current_user, new_room.name);
                    } else {
                        tracing::warn!("Exit leads to unknown room: {}", exit.room_name);
                    }
                } else {
                    tracing::warn!("No exit in direction '{}' from room '{}'", direction, current_room);
                }
            }
            MudMessage::Disconnect => {
                tracing::info!("Client requested disconnect");
                break;
            }
            _ => {}
        }

    }
    Ok(())
}