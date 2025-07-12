# Workshop: Sync MUD Client

> The code is in the slightly poorly named `code/day3/async_mud_client` directory.

With a working server, it's a lot easier to test if we have a client! We want to do a few things differently from last time.

Let's start with dependencies:

```toml
[dependencies]
async_mud_proto = { path = "../async_mud_proto" }
colored.workspace = true
anyhow.workspace = true
```

Notice that we're NOT providing the `tokio` feature flag, so we aren't pulling in the async runtime.

Previously, you didn't see updates until you sent a message. That's not great for multi-player games! So instead, we'll follow a similar pattern to the server:

```rust
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
```

We get the login information, connect, send our magic number (did you notice that we bumped it so you can't accidentally talk to the previous version?), and then send a login message. If we don't get a `LoginSuccess` back, we bail out.

Then we create a channel for sending messages to the server, and spawn three threads: one for sending messages, one for receiving messages, and one for user input. All of these block on OS notifications, so they won't waste CPU time.

## Sending Messages

The message sending thread looks like this:

```rust
fn message_send_loop(
    rx: Receiver<MudMessage>,
    mut socket: std::net::TcpStream,
) -> anyhow::Result<()> {
    while let Ok(msg) = rx.recv() {
        send_message(&mut socket, &msg)?;
    }
    Ok(())
}
```

Very simple: just wait for messages to arrive on the channel, and send them to the server.

## Message Receiving

The message receiving thread looks like this:

```rust
n message_receive_loop(
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
```

Again, deliberately simple. The text formatting makes up the bulk of the code!

## User Input

```rust
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
```

The user input loop reads lines from standard input. If the user types "quit" or "exit", it sends a `Disconnect` message and breaks the loop. If the user types something starting with "say ", it treats the rest of the line as a chat message to send. Otherwise, it treats the input as a direction to move.

Overall, this design is much nicer than the previous blocking client. Concerns are nicely separated, messages are sent and received asynchronously (not async, but threads!), and the user can see updates from the server as they happen.