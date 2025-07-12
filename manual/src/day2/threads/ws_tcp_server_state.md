# Workshop: Synchronous MUD with State

Conspicuously missing from the previous workshop was any kind of state management. Each client was completely independent of one another, and could simply wander around.

There's a divide in the Rust world as to which method for state handling is better. The "shared state" model, where multiple threads can access a common data structure, and the "actor model", where each thread has its own state and communicates with other threads via message passing. Both have their pros and cons.

In *this* workshop, we're going to use shared state (and we'll go with actors tomorrow, in async land).

We're keeping the server almost the same, but we're going to introduce a `WorldState` struct that will hold the state of the game world, including the occupants of each room. You'll need to add a depenendency on `once_cell` to your `Cargo.toml`:

```toml
once_cell = "1.18"
```

Here's my implementation of `WorldState` in `src/main.rs`:

```rust
struct WorldState {
    room_occupants: Mutex<HashMap<String, Vec<String>>>,
}

impl WorldState {
    pub fn new() -> Self {
        WorldState {
            room_occupants: Mutex::new(HashMap::new()),
        }
    }

    pub fn remove_user(&self, username: &str) {
        tracing::info!("Removing user {} from world state", username);
        let mut occupants = self.room_occupants.lock().unwrap();
        for room in occupants.values_mut() {
            room.retain(|user| user != username);
        }
    }

    pub fn add_user_to_room(&self, username: &str, room_name: &str) {
        let mut occupants = self.room_occupants.lock().unwrap();
        occupants.entry(room_name.to_string()).or_default().push(username.to_string());
    }

    pub fn get_occupants(&self, room_name: &str) -> Vec<String> {
        let occupants = self.room_occupants.lock().unwrap();
        occupants.get(room_name).cloned().unwrap_or_default()
    }

    pub fn remove_user_from_room(&self, username: &str, room_name: &str) {
        let mut occupants = self.room_occupants.lock().unwrap();
        if let Some(room) = occupants.get_mut(room_name) {
            room.retain(|user| user != username);
        }
    }
}

static WORLD_STATE: once_cell::sync::Lazy<Arc<WorldState>> = once_cell::sync::Lazy::new(|| Arc::new(WorldState::new()));
```

This is enough to let you start adding tracking.

We'll stop here for now---this is actually a lot *easier* in async land. It should have given you a taste of how to manage shared state in a synchronous server, and how to update the client to show other players in the room.


> My versions are in `code/day2/sync_mud_server_state`, `code/day2/sync_mud_proto_state`, and `code/day2/sync_mud_client_state`.

## Discussion: 

* Given how tricky it can be to manage shared state, do you think this is a good approach? 
* How would you handle a more real-time setup, where players can see each other move around in real time?
