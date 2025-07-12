use rooms_library2::Room;
use serde::{Deserialize, Serialize};

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