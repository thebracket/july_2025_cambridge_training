use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Room {
    pub name: String,
    pub description: String,
    pub exits: Vec<Exit>,
    pub start: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Exit {
    pub direction: String,
    pub room_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoomLibrary {
    rooms: HashMap<String, Room>,
}


impl RoomLibrary {
    pub fn load() -> Result<HashMap<String, Room>, String> {
        let path = std::path::Path::new("rooms.json");
        let rooms: HashMap<String, Room> = if path.exists() {
            let data = std::fs::read_to_string(path)
                .map_err(|e| format!("Failed to read rooms.json: {}", e))?;
            serde_json::from_str(&data)
                .map_err(|e| format!("Failed to parse rooms.json: {}", e))?
        } else {
            return Err("rooms.json does not exist".to_string());
        };

        // Validate the exits
        for room in rooms.values() {
            for exit in &room.exits {
                if !rooms.contains_key(&exit.room_name) {
                    return Err(format!("Invalid exit in room {}: {}", room.name, exit.room_name));
                }
            }
        }

        Ok(rooms)
    }
}