use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Room {
    pub name: String,
    pub description: String,
    pub exits: Vec<Exit>,
    pub start: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Exit {
    pub direction: String,
    pub room_name: String,
}

#[derive(thiserror::Error, Debug)]
pub enum RoomError {
    #[error("Room not found")]
    NotFound,
    #[error("Failed to load rooms")]
    LoadFailed,
    #[error("Invalid exit in room: {0}")]
    InvalidExit(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoomLibrary {
    rooms: HashMap<String, Room>,
}


impl RoomLibrary {
    pub fn load() -> Result<HashMap<String, Room>, RoomError> {
        let path = std::path::Path::new("rooms.json");
        let rooms: HashMap<String, Room> = if path.exists() {
            let data = std::fs::read_to_string(path)
                .map_err(|_e| RoomError::LoadFailed)?;
            serde_json::from_str(&data)
                .map_err(|_e| RoomError::LoadFailed)?
        } else {
            return Err(RoomError::LoadFailed);
        };

        // Validate the exits
        for room in rooms.values() {
            for exit in &room.exits {
                if !rooms.contains_key(&exit.room_name) {
                    return Err(RoomError::InvalidExit(exit.room_name.clone()));
                }
            }
        }

        Ok(rooms)
    }
}