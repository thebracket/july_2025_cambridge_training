use std::{io::Write, sync::mpsc::{Receiver, Sender}};
use async_mud_proto::{sync_messaging::{send_message, read_message}, MudMessage};
use colored::Colorize;

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn send_magic_number(socket: &mut std::net::TcpStream) -> anyhow::Result<()> {
    socket.write_all(&[0x4D, 0x55, 0x44, 0x31])?; // 'MUD1'
    Ok(())
}

fn main() -> anyhow::Result<()> {
    println!("{}", "MUD Client".green());

    // Obtain credentials
    println!("{}", "Enter your username:".yellow());
    let username = read_line();

    println!("{}", "Enter your password:".yellow());
    let password = read_line();

    // Connect and login
    println!("{}", "Connecting to server...".yellow());
    let mut socket = std::net::TcpStream::connect("127.0.0.1:8080")?;

    send_magic_number(&mut socket)?;
    println!("{}", "Connected. Sending login...".yellow());
    let login_msg = MudMessage::Login { username, password };
    send_message(&mut socket, &login_msg)?;
    let Ok(MudMessage::LoginSuccess) = read_message(&mut socket) else {
        println!("{}", "Login failed.".red());
        return Ok(());
    };
    println!("{}", "Login successful!".green());

    // Synchronous TCP stream doesn't need splitting, it can be cloned.

    // Make the channel
    let (tcp_tx, tcp_rx) = std::sync::mpsc::channel();
    
    // Start the message sending thread
    let send_thread = std::thread::spawn({
        let socket = socket.try_clone()?;
        move || message_send_loop(tcp_rx, socket)
    });

    // Start the message receiving thread
    let recv_thread = std::thread::spawn({
        let socket = socket.try_clone()?;
        move || message_receive_loop(socket)
    });

    // Start the user input thread
    let input_thread = std::thread::spawn(|| user_input_loop(tcp_tx));

    let _ = send_thread.join();
    let _ = recv_thread.join();
    let _ = input_thread.join();

    Ok(())
}

fn message_send_loop(
    rx: Receiver<MudMessage>,
    mut socket: std::net::TcpStream,
) -> anyhow::Result<()> {
    while let Ok(msg) = rx.recv() {
        send_message(&mut socket, &msg)?;
    }
    Ok(())
}

fn message_receive_loop(
    mut socket: std::net::TcpStream,
) -> anyhow::Result<()> {
    loop {
        let msg = read_message(&mut socket)?;
        match msg {
            MudMessage::EnterRoom { room, other_players } => {
                println!("{}", format!("{}", room.name).green());
                println!("{}", format!("{}", room.description).white());
                println!("{}", format!("Exits: {}", room.exits.iter().map(|exit| exit.direction.clone()).collect::<Vec<String>>().join(", ")).blue());
                if !other_players.is_empty() {
                    println!("{}", format!("Other players here: {}", other_players.join(", ")).magenta());
                } else {
                    println!("{}", "You are alone here.".magenta());
                }
            }
            MudMessage::PlayerEnteredRoom { username } => {
                println!("{}", format!("{} has entered the room.", username).cyan());
            }
            MudMessage::PlayerLeftRoom { username, direction } => {
                println!("{}", format!("{} has left the room, heading {}.", username, direction).cyan());
            }
            MudMessage::PlayerSpeak { username, message } => {
                println!("{}", format!("{} says: {}", username, message).yellow());
            }
            _ => {}
        }
    }
    Ok(())
}

fn user_input_loop(
    tcp_tx: Sender<MudMessage>,
) -> anyhow::Result<()> {
    loop {
        let input = read_line();
        if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
            tcp_tx.send(MudMessage::Disconnect)?;
            println!("{}", "Disconnecting...".yellow());
            break;
        }

        if input.to_lowercase().starts_with("say ") {
            let message = input[4..].trim().to_string();
            if !message.is_empty() {
                tcp_tx.send(MudMessage::PlayerSpeak { username: "".to_string(), message })?;
            } else {
                println!("{}", "Cannot send empty message.".red());
            }
            continue;
        }

        tcp_tx.send(MudMessage::TryExit { direction :input })?;
    }
    Ok(())   
}