# Async TCP Echo Server

Let's make the TCP echo server from day 2, but async. We'll use the "current_thread" runtime, which is single-threaded.

> This code is in `code/day3/async_tcp_echo` in the repository.

```rust
use tokio::io::AsyncWriteExt; // Instead of std::io::Write
use tokio::io::AsyncReadExt; // Instead of std::io::Read

async fn server() -> anyhow::Result<()> {
    // Use Tokio's TcpListener instead of std::net::TcpListener
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        // Accept a new connection asynchronously
        // This will yield until a connection is available
        let (mut socket, _) = listener.accept().await?;

        // We're spawning a new task for each connection instead of a thread
        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            loop {
                match socket.read(&mut buffer).await {
                    Ok(0) => break, // Connection closed
                    Ok(n) => {
                        if socket.write_all(&buffer[..n]).await.is_err() {
                            break; // Write error
                        }
                    }
                    Err(_) => break, // Read error
                }
            }
        });
    }
    //Ok(()) - Unreachable, server runs forever
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // We spawn the server in a task, so we can also run a client in the same program
    tokio::spawn(async {
        if let Err(e) = server().await {
            eprintln!("Server error: {:?}", e);
        }
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await; // Give server time to start

    // Using tokio's TcpStream instead of std::net::TcpStream
    match tokio::net::TcpStream::connect("127.0.0.1:8080").await {
        Ok(mut stream) => {
            let msg = b"Hello, world!";
            stream.write_all(msg).await?;
            let mut buffer = [0; 1024];
            let n = stream.read(&mut buffer).await?;
            println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
        }
        Err(e) => {
            eprintln!("Failed to connect: {:?}", e);
        }
    }

    Ok(())
}
```

The big takeaway is that it's basically the same code as before - but with `await` sprinkled in --- and on just the one thread!