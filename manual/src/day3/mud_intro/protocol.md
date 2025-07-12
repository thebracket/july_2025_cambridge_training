# Workshop: MUD Protocol

We've talked a bit about how it can be painful to support both sync and async in a protocol. Previously, we went with a simple `to_bytes` and `from_bytes` approach - and left the protocol handling to the callers.

Let's improve this!

## Feature Flags

We're going to provide some transmission/receipt support for both sync and async. We don't want to bloat synchronous code with Tokio, or async code with std::io - so we'll use feature flags.

In the `mud_protocol` crate, add this to your `Cargo.toml`:

```toml
[features]
default-features = []
async = ["tokio"]
```

And we'll update our dependencies to only include Tokio when the `tokio` feature is enabled:

```toml
[dependencies]
tokio = { workspace = true, optional = true }
```

## Async Read/Write

We'll add support for reading/writing messages to/from async streams, and *gate* it so that it's only compiled when the `tokio` feature is enabled:

```rust
#[cfg(feature = "tokio")]
pub mod async_messaging {
    use super::MudMessage;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    pub async fn read_message(socket: &mut (impl AsyncReadExt + Unpin)) -> anyhow::Result<MudMessage> {
        let mut len_buf = [0u8; 4];
        socket.read_exact(&mut len_buf).await
            .inspect_err(|e| tracing::error!("Failed to read message length: {:?}", e))?;
        let len = u32::from_be_bytes(len_buf) as usize;
        let mut msg_buf = vec![0u8; len];
        socket.read_exact(&mut msg_buf).await
            .inspect_err(|e| tracing::error!("Failed to read message: {:?}", e))?;
        let msg = MudMessage::from_bytes(&msg_buf)?;
        Ok(msg)
    }

    pub async fn send_message(socket: &mut (impl AsyncWriteExt + Unpin), msg: &MudMessage) -> anyhow::Result<()> {
        let msg_bytes = msg.to_bytes()?;
        let len = msg_bytes.len() as u32;
        socket.write_all(&len.to_be_bytes()).await
            .inspect_err(|e| tracing::error!("Failed to write message length: {:?}", e))?;
        socket.write_all(&msg_bytes).await
            .inspect_err(|e| tracing::error!("Failed to write message: {:?}", e))?;
        Ok(())
    }
}
```

The `AsyncReadExt`, `AsyncWriteExt` traits provide the necessary methods for reading from and writing to async streams. By using these traits, we can seamlessly integrate our messaging protocol with async I/O operations, allowing for efficient and non-blocking communication in our MUD server.

It doesn't matter if its a TCP stream, a websocket stream, a gRPC stream or something else - as long as it can handle async reads and writes, it will work.

## Sync Read/Write

We'll also provide synchronous versions of the same functions, gated on the absence of the `tokio` feature:

```rust
#[cfg(not(feature = "tokio"))]
pub mod sync_messaging {
    use super::MudMessage;
    use std::io::{Read, Write};

    pub fn read_message(socket: &mut impl Read) -> anyhow::Result<MudMessage> {
        let mut len_buf = [0u8; 4];
        socket.read_exact(&mut len_buf)
            .inspect_err(|e| tracing::error!("Failed to read message length: {:?}", e))?;
        let len = u32::from_be_bytes(len_buf) as usize;
        let mut msg_buf = vec![0u8; len];
        socket.read_exact(&mut msg_buf)
            .inspect_err(|e| tracing::error!("Failed to read message: {:?}", e))?;
        let msg = MudMessage::from_bytes(&msg_buf)?;
        Ok(msg)
    }

    pub fn send_message(socket: &mut impl Write, msg: &MudMessage) -> anyhow::Result<()> {
        let msg_bytes = msg.to_bytes().inspect_err(|e| tracing::error!("Failed to serialize message: {:?}", e))?;
        let len = msg_bytes.len() as u32;
        socket.write_all(&len.to_be_bytes())
            .inspect_err(|e| tracing::error!("Failed to write message length: {:?}", e))?;
        socket.write_all(&msg_bytes)
            .inspect_err(|e| tracing::error!("Failed to write message: {:?}", e))?;
        Ok(())
    }
}
```

Now you have a threaded version that can handle anything that supports the `Read` and `Write` traits, such as TCP streams, files, or in-memory buffers.

## Protocol Extension

We want to cleanly support a few features, so the protocol has been extended a bit:

```rust
#[derive(Serialize, Deserialize, Debug)]
pub enum MudMessage {
    Login { username: String, password: String },
    LoginSuccess,
    LoginFail,
    EnterRoom { room: Room, other_players: Vec<String> },
    TryExit { direction: String },
    Disconnect,

    Ping,
    PlayerEnteredRoom { username: String },
    PlayerLeftRoom { username: String, direction: String },
    PlayerSpeak { username: String, message: String },
}
```

We've added `Ping` to act as keep-alives. We're announcing player movement to other players, and letting players speak in rooms.

Go ahead and update your `mud_protocol` crate with these changes.