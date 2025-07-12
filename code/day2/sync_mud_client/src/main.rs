use std::io::{Read, Write};
use std::net::TcpStream;
use sync_mud_proto::MudMessage;

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main() -> anyhow::Result<()> {
    // Connect to the server
    let mut stream = TcpStream::connect("localhost:8080")?;
    println!("Connected to server");

    // Send the magic number
    let magic: u32 = 0x4D554400; // "MUD\0"
    stream.write_all(&magic.to_be_bytes())?;

    // Ask for username and password
    println!("Enter username:");
    let username = read_line();
    println!("Enter password:");
    let password = read_line();

    // Send login message
    let login_msg = MudMessage::Login { username, password };
    let login_bytes = login_msg.to_bytes()?;
    stream.write_all(&(login_bytes.len() as u32).to_be_bytes())?;
    stream.write_all(&login_bytes)?;

    // Read login response
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf)?;
    let len = u32::from_be_bytes(len_buf) as usize;
    
    let mut msg_buf = vec![0u8; len];
    stream.read_exact(&mut msg_buf)?;
    
    let response = MudMessage::from_bytes(&msg_buf)?;
    match response {
        MudMessage::LoginSuccess => {
            println!("Login successful!");
        }
        MudMessage::LoginFail => {
            println!("Login failed!");
            return Ok(());
        }
        _ => {
            println!("Unexpected response from server");
            return Ok(());
        }
    }

    // Main game loop
    loop {
        // Read message from server
        let mut len_buf = [0u8; 4];
        stream.read_exact(&mut len_buf)?;
        let len = u32::from_be_bytes(len_buf) as usize;
        
        let mut msg_buf = vec![0u8; len];
        stream.read_exact(&mut msg_buf)?;
        
        let msg = MudMessage::from_bytes(&msg_buf)?;
        match msg {
            MudMessage::EnterRoom { room } => {
                println!("\n{}", room.name);
                println!("{}", room.description);
                
                if room.exits.is_empty() {
                    println!("No exits available.");
                } else {
                    println!("Exits:");
                    for exit in &room.exits {
                        println!(" - {}: {}", exit.direction, exit.room_name);
                    }
                }
                
                // Get user input
                println!("\nEnter a command (or 'quit' to disconnect):");
                let command = read_line().to_lowercase();
                
                if command == "quit" {
                    // Send disconnect message
                    let disconnect_msg = MudMessage::Disconnect;
                    let disconnect_bytes = disconnect_msg.to_bytes()?;
                    stream.write_all(&(disconnect_bytes.len() as u32).to_be_bytes())?;
                    stream.write_all(&disconnect_bytes)?;
                    println!("Disconnecting...");
                    break;
                }
                
                // Check if it's a valid exit
                if room.exits.iter().any(|e| e.direction.to_lowercase() == command) {
                    // Send TryExit message
                    let exit_msg = MudMessage::TryExit { direction: command };
                    let exit_bytes = exit_msg.to_bytes()?;
                    stream.write_all(&(exit_bytes.len() as u32).to_be_bytes())?;
                    stream.write_all(&exit_bytes)?;
                } else {
                    println!("Invalid command or exit. Try again.");
                }
            }
            _ => {
                println!("Unexpected message from server");
            }
        }
    }

    Ok(())
}