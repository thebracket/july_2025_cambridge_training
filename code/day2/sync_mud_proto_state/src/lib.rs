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