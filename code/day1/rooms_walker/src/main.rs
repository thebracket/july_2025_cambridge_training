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
