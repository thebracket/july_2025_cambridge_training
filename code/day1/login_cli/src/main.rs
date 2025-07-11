use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// List all users
    List,
    /// Add a user
    Add { 
        /// Username
        #[arg(required = true)]
        username: String, 
        /// Password
        #[arg(required = true)]
        password: String 
    },
    /// Delete a user
    Delete { 
        /// Username
        #[arg(required = true)]
        username: String, 
    },
    /// Update a user's password
    Update { 
        /// Username
        #[arg(required = true)]
        username: String, 
        /// New password
        #[arg(required = true)]
        password: String 
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            let users = login_library::LoginManager::new();
            if users.users.is_empty() {
                println!("No users found");
                return;
            }
            users.users.iter().for_each(|u| println!("{}", u.username));
        }
        Commands::Add { username, password } => {
            let mut users = login_library::LoginManager::new();
            users.add_user(&username, &password);
            users.save();
            println!("User {} added", username);
        }
        Commands::Delete { username } => {
            let mut users = login_library::LoginManager::new();
            users.users.retain(|u| u.username != username);
            users.save();
            println!("User {} deleted", username);
        }
        Commands::Update { username, password } => {
            let mut users = login_library::LoginManager::new();
            if let Some(user) = users.users.iter_mut().find(|u| u.username == username) {
                *user = login_library::User::new(&username, &password);
                users.save();
                println!("User {} updated", username);
            } else {
                println!("User {} not found", username);
            }
        }        
    }
}
