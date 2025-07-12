use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;

async fn server() -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
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
    tokio::spawn(async {
        if let Err(e) = server().await {
            eprintln!("Server error: {:?}", e);
        }
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await; // Give server time to start

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
