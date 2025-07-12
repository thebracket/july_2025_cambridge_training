use std::collections::HashMap;
use async_mud_proto::MudMessage;
use tokio::sync::{mpsc::{Receiver, Sender}, OnceCell};
use rand::prelude::*;

static WORLD_COMMAND_TX: OnceCell<Sender<WorldCommand>> = OnceCell::const_new();

pub fn run() -> anyhow::Result<()> {
    // Load the rooms
    let rooms = rooms_library2::RoomLibrary::load()?;
    
    // Find starting points
    let starting_rooms: Vec<String> = rooms
        .iter()
        .filter(|(_name, room)| room.start)
        .map(|(name, _)| name.clone())
        .collect::<Vec<_>>();
    if starting_rooms.is_empty() {
        return Err(anyhow::anyhow!("No starting rooms found in the room library"));
    }

    let (tx, rx) = tokio::sync::mpsc::channel(100);
    WORLD_COMMAND_TX.set(tx)?;

    // Start the main loop
    tokio::spawn(async move {
        main_loop(rooms, starting_rooms, rx).await;
    });

    Ok(())
}

enum WorldCommand {
    FindStartingRoom { reply: tokio::sync::oneshot::Sender<String>},
    PlayerSpawn { username: String, room: String, player_tx: Sender<MudMessage> },
    DespawnPlayer { username: String },
    PlayerMove { username: String, direction: String },
    Speak { username: String, message: String },
}

#[derive(Clone, Debug)]
struct Player {
    username: String,
    room: String,
    player_tx: Sender<MudMessage>,
}

async fn main_loop(
    rooms: HashMap<String, rooms_library2::Room>,
    starting_rooms: Vec<String>,
    mut world_commands: Receiver<WorldCommand>,
) {
    let mut players: Vec<Player> = Vec::new();

    while let Some(command) = world_commands.recv().await {
        match command {
            WorldCommand::FindStartingRoom { reply } => {
                let room = starting_rooms
                    .choose(&mut rand::rng())
                    .cloned()
                    .unwrap_or_else(|| starting_rooms[0].clone());
                if reply.send(room).is_err() {
                    tracing::warn!("Failed to send starting room");
                }
            }
            WorldCommand::PlayerSpawn { username, room, player_tx } => {
                if rooms.contains_key(&room) {
                    tracing::info!("Player {} spawned in room {}", username, room);

                    // Send the EnterRoom message to the player
                    let Some(room_details) = rooms.get(&room) else {
                        tracing::error!("Room {} not found after existence check", room);
                        continue;
                    };
                    let other_players: Vec<String> = players.iter()
                        .filter(|p| p.room == room && p.username != username)
                        .map(|p| p.username.clone())
                        .collect();
                    player_tx.send(
                        MudMessage::EnterRoom { room: room_details.clone(), other_players }
                    ).await.unwrap_or_else(|e| {
                        tracing::warn!("Failed to send EnterRoom message to player {}: {:?}", username, e);
                    });

                    // Tell any other players in the room that this player has entered
                    for p in players.iter().filter(|p| p.room == room) {
                        let _ = p.player_tx.send(MudMessage::PlayerEnteredRoom { username: username.clone() }).await;
                    }

                    // Add them to the players list
                    players.push(Player { username, room, player_tx });
                } else {
                    tracing::warn!("Room {} does not exist", room);
                }
            }
            WorldCommand::DespawnPlayer { username } => {
                players.retain(|player| player.username != username);
                tracing::info!("Player {} despawned", username);
            }
            WorldCommand::PlayerMove { username, direction } => {
                // Preconditions

                // Find the player
                let Some(player) = players.iter().find(|p| p.username == username) else {
                    tracing::warn!("Player {} not found for move command", username);
                    continue;
                };
                let Some(current_room) = rooms.get(&player.room) else {
                    tracing::warn!("Current room {} for player {} not found", player.room, username);
                    continue;
                };
                let Some(next_room_name) = current_room.exits.iter().find(|e| e.direction == direction) else {
                    tracing::warn!("No exit {} in room {} for player {}", direction, current_room.name, username);
                    continue;
                };
                // To avoid borrow issues below, clone the player
                let player = (*player).clone();

                // Notify other players in the current room that this player is leaving
                for p in players.iter().filter(|p| p.room == player.room && p.username != username) {
                    let _ = p.player_tx.send(MudMessage::PlayerLeftRoom { username: username.clone(), direction: direction.clone() }).await;
                }

                // Move the player - iterating to avoid borrow issues
                players.iter_mut().find(|p| p.username == username).map(|p| p.room = next_room_name.room_name.clone());

                // Notify the player of the new room
                if let Some(new_room) = rooms.get(&player.room) {
                    let other_players: Vec<String> = players.iter()
                        .filter(|p| p.room == player.room && p.username != username)
                        .map(|p| p.username.clone())
                        .collect();
                    let _ = player.player_tx.send(MudMessage::EnterRoom { room: new_room.clone(), other_players }).await;
                } else {
                    tracing::error!("Next room {} not found for player {}", player.room, username);
                }

                // Notify other players in the new room that this player has entered
                for p in players.iter().filter(|p| p.room == player.room && p.username != username) {
                    let _ = p.player_tx.send(MudMessage::PlayerEnteredRoom { username: username.clone() }).await;
                }
            }
            WorldCommand::Speak { username, message } => {
                // Find the player
                let Some(player) = players.iter().find(|p| p.username == username) else {
                    tracing::warn!("Player {} not found for speak command", username);
                    continue;
                };
                // Send the speak message to all players in the same room
                for p in players.iter().filter(|p| p.room == player.room) {
                    let _ = p.player_tx.send(MudMessage::PlayerSpeak { username: username.clone(), message: message.clone() }).await;
                }
                tracing::info!("Player {} said in room {}: {}", username, player.room, message);
            }
        }
    }
}

pub async fn find_starting_room() -> anyhow::Result<String> {
    let tx = WORLD_COMMAND_TX.get().ok_or_else(|| anyhow::anyhow!("World command channel not initialized"))?;
    let (reply_tx, reply_rx) = tokio::sync::oneshot::channel();
    
    tx.send(WorldCommand::FindStartingRoom { reply: reply_tx })
        .await
        .map_err(|_| anyhow::anyhow!("Failed to send command to world manager"))?;
    
    reply_rx.await.map_err(|_| anyhow::anyhow!("Failed to receive starting room"))
}

pub async fn spawn_player(username: &str, room: &str, player_tx: Sender<MudMessage>) -> anyhow::Result<()> {
    let tx = WORLD_COMMAND_TX.get().ok_or_else(|| anyhow::anyhow!("World command channel not initialized"))?;
    
    tx.send(WorldCommand::PlayerSpawn {
        username: username.to_string(),
        room: room.to_string(),
        player_tx,
    }).await.map_err(|_| anyhow::anyhow!("Failed to send player spawn command"))?;
    
    Ok(())
}

pub async fn despawn_player(username: &str) -> anyhow::Result<()> {
    let tx = WORLD_COMMAND_TX.get().ok_or_else(|| anyhow::anyhow!("World command channel not initialized"))?;
    
    tx.send(WorldCommand::DespawnPlayer {
        username: username.to_string(),
    }).await.map_err(|_| anyhow::anyhow!("Failed to send player despawn command"))?;
    
    Ok(())
}

pub async fn move_player(username: &str, direction: &str) -> anyhow::Result<()> {
    let tx = WORLD_COMMAND_TX.get().ok_or_else(|| anyhow::anyhow!("World command channel not initialized"))?;
    
    tx.send(WorldCommand::PlayerMove {
        username: username.to_string(),
        direction: direction.to_string(),
    }).await.map_err(|_| anyhow::anyhow!("Failed to send player move command"))?;
    
    Ok(())
}

pub async fn player_speak(username: &str, message: &str) -> anyhow::Result<()> {
    let tx = WORLD_COMMAND_TX.get().ok_or_else(|| anyhow::anyhow!("World command channel not initialized"))?;
    
    tx.send(WorldCommand::Speak {
        username: username.to_string(),
        message: message.to_string(),
    }).await.map_err(|_| anyhow::anyhow!("Failed to send player speak command"))?;
    
    Ok(())
}