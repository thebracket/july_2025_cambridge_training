# Workshop: Login CLI

Now that we have a library, let's make a simple CLI tool to manage users.

> The code for this workshop is in `code/day1/login_cli`.

## Task 1: Create a new binary crate

Use `cargo new` to create a new binary crate called `login_cli`. Make sure it's in your workspace.

We'll need to add some dependencies to `login_cli/Cargo.toml`:

```toml
[dependencies]
login_library = { path = "../login_library" }
clap = { version = "4.5", features = ["derive"] }
```

> Bonus points if you added them to your workspace dependencies!

## Task 2: Create a CLI tool

I'll give you a head-start on this one:

```rust
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
    }
}
```

The `clap` crate is a command-line argument parser, used by most Rust CLI tools. It does a fair amount of magic for you. If you `cargo run`, it generates usage information for you. It even adds `--help` to every command.

The `cargo run` formatting is a little strange when you want to pass arguments to your program. You need to use `--` to separate the arguments for `cargo` and the arguments for your program:

```bash
cargo run -- list
```

## Task 3: Add a user

I'll help out a little since the goal isn't to learn `clap` in detail:

```rust
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
}
```

Now implement adding a user in the `main` function.

![](../../images/ScrollTime.png)

## Task 4: Finish the CRUD!

**C**reate, **R**ead, **U**pdate, **D**elete. The bane of every developer's existence.

We've already got Create and Read. Add Update and Delete commands to the CLI tool.

![](../../images/ScrollTime.png)

The program is now complete! You can add, list, update and delete users. The users are stored in a JSON file in the current directory.
