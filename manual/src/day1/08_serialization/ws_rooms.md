# Workshop: Rooms JSON

You have everything you need now to build a simple room system for your upcoming MUD.

## Task 1: Create a new library

> The code for this workshop is in `code/day1/rooms_library`.

Make a new library called `rooms_library` in your workspace.

The library should have a `Room` structure, and a structure that holds a `HashMap<String, Room>`. It should also implement serialization and deserialization for these structures using Serde, and allow you to load and save the rooms from a JSON file.

Rooms should:
* Have a name (e.g. "Herbert's Room")
* Have a description (e.g. "You are in a server closet, surrounded by blinking lights and humming machines.")
* Define exits - in a struct. Exits are a pair: a *direction* (e.g. "north") and a *room name* (e.g. "Herbert's Room").
* Have a boolean "starting" field that indicates if this is a room players can spawn into.

Your library needs to load the rooms and make them available to library consumers. It should also perform some validation on load: ensure that all exits point to valid rooms.

> For bonus points, loading the rooms should return `Result<HashMap<String, Room>, String>` so that you can handle errors gracefully. Return an error message if the rooms fail to load. We'll dive more deeply into proper error handling in a later chapter. For now, just return a string error message. You can use `map_err` on a result to transform the error into a string, and the `?` operator to propagate errors.

## Task 2: Create a Rooms Walker

Create a new binary crate called `rooms_walker` in your workspace. This will be a simple CLI tool that allows you to explore the rooms.

You'll need to create a `rooms.json` file in the `rooms_walker` directory. Feel free to be creative - but keep it PG-13! Here's mine:

```json
{
    "Herbert's Room": {
        "name": "Herbert's Room",
        "description": "You are in a server closet, surrounded by blinking lights and humming machines.",
        "exits": [
            {
                "direction": "north",
                "room_name": "Main Hall"
            }
        ],
        "start": true
    },
    "Main Hall": {
        "name": "Main Hall",
        "description": "You are in the main hall of the building.",
        "exits": [
            {
                "direction": "south",
                "room_name": "Herbert's Room"
            }
        ],
        "start": false
    }
}
```

> The code for this workshop is in `code/day1/rooms_walker`.

Start simple:
* Load the rooms from the library.
* Iterate and find the starting room.
* Set a variable to indicate the current room.
* In a loop:
    * Print the current room's description.
    * Print the exits.
    * Ask the user for a direction to go.
    * If the direction is valid, change the current room to the new room.
    * If the direction is invalid, print an error message and ask again.
    * Quit if the user types "exit".

Here's a handy function for you:

```rust
pub fn read_line() -> String {
    // <- Public function
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}
```

You can use this function to read user input. It reads a line from standard input, trims it, and returns it as a `String`. It's also a good idea to add `.to_lowercase()` to the input!

Let's work on that.

![](../../images/ScrollTime.png)

I ended up with this:

```rust
use colored::Colorize;

pub fn read_line() -> String {
    // <- Public function
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main() -> Result<(), String> {
    // Load the rooms
    let rooms = rooms_library::RoomLibrary::load()?;

    // Find a starting room
    let Some(start_room) = rooms.values().find(|room| room.start) else {
        return Err("No starting room found".to_string());
    };

    // Walk around the rooms
    let mut current_room = start_room.name.clone();
    loop {
        let Some(room) = rooms.get(&current_room) else {
            return Err(format!("Room '{}' not found", current_room));
        };

        println!("{}", room.name.bright_green());
        println!("{}", room.description);
        if room.exits.is_empty() {
            println!("No exits available.");
        } else {
            println!("Exits:");
            for exit in &room.exits {
                println!(" - {}: {}", exit.direction.bright_blue(), exit.room_name);
            }
        }

        println!("\nEnter a command (or 'exit' to quit):");
        let command = read_line().to_lowercase();
        if command == "exit" {
            println!("Exiting the game. Goodbye!");
            break;
        }
        let exit = room.exits.iter().find(|e| e.direction.to_lowercase() == command);
        if let Some(exit) = exit {
            current_room = exit.room_name.clone();
            println!("You move to the {} room.", exit.room_name.bright_yellow());
        } else {
            println!("{}", "Invalid command or exit. Please try again.".red());
        }
    }

    Ok(())
}
```

> Bonus points for creativity!