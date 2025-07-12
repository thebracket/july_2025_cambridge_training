# State Maintenance - Channels and Actors

You can use `Mutex` and shared state in async code, just like in synchronous code. However, it can get messy quite quickly when you start having locks everywhere!

The actor model is a popular way to manage state in async applications. You saw a simple version of this in the threaded channel example from Day 2.

> This example is in `code/day3/actor_example` in the repository.

The idea is to isolate state into a single task, that has no purpose beyond managing that state. Here's a self-contained key-value store example:

```rust
use tokio::sync::OnceCell;

enum WorldCommand {
    SetValue { key: String, value: String },
    GetValue { key: String, respond_to: tokio::sync::oneshot::Sender<Option<String>> },
    DeleteValue { key: String },    
}

static SENDER: OnceCell<tokio::sync::mpsc::Sender<WorldCommand>> = OnceCell::const_new();

pub async fn run() -> anyhow::Result<()> {
    let (tx, rx) = tokio::sync::mpsc::channel(32);
    SENDER.set(tx)?;

    tokio::spawn(main_loop(rx));

    Ok(())
}

async fn main_loop(mut rx: tokio::sync::mpsc::Receiver<WorldCommand>) {
    let mut key_value_store = std::collections::HashMap::<String, String>::new();

    while let Some(command) = rx.recv().await {
        match command {
            WorldCommand::SetValue { key, value } => {
                key_value_store.insert(key, value);
            }
            WorldCommand::GetValue { key, respond_to } => {
                let value = key_value_store.get(&key).cloned();
                let _ = respond_to.send(value);
            }
            WorldCommand::DeleteValue { key } => {
                key_value_store.remove(&key);
            }
        }
    }
}

pub async fn set_value(key: String, value: String) -> anyhow::Result<()> {
    let sender = SENDER.get().ok_or_else(|| anyhow::anyhow!("World actor not running"))?;
    sender.send(WorldCommand::SetValue { key, value }).await?;
    Ok(())
}

pub async fn get_value(key: String) -> anyhow::Result<Option<String>> {
    let sender = SENDER.get().ok_or_else(|| anyhow::anyhow!("World actor not running"))?;
    let (tx, rx) = tokio::sync::oneshot::channel();
    sender.send(WorldCommand::GetValue { key, respond_to: tx }).await?;
    let value = rx.await?;
    Ok(value)
}

pub async fn delete_value(key: String) -> anyhow::Result<()> {
    let sender = SENDER.get().ok_or_else(|| anyhow::anyhow!("World actor not running"))?;
    sender.send(WorldCommand::DeleteValue { key }).await?;
    Ok(())
}
```

The `main.rs` simply uses it:

```rust
mod world_state;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    world_state::run().await?;

    world_state::set_value("foo".to_string(), "bar".to_string()).await?;
    if let Some(value) = world_state::get_value("foo".to_string()).await? {
        println!("Got foo: {}", value);
    } else {
        println!("Key 'foo' not found");
    }

    world_state::set_value("hello".to_string(), "world".to_string()).await?;
    if let Some(value) = world_state::get_value("hello".to_string()).await? {
        println!("Got hello: {}", value);
    }

    world_state::delete_value("foo".to_string()).await?;
    match world_state::get_value("foo".to_string()).await? {
        Some(value) => println!("Got foo after delete: {}", value),
        None => println!("Key 'foo' correctly not found after delete"),
    }

    Ok(())    
}
```

In this case, we haven't gain a lot. In a large application, this pattern helps a lot:

* You have a well-defined API for interacting with the state.
* You can easily change the implementation of the state management without affecting the rest of the code.
* You can have multiple actors managing different pieces of state, communicating via channels - but the channel complexity is hidden behind a simple API.
* If you need to scale out, you can replace the API with a networked version.
* You can easily separate it into a separate crate for compilation speed - and for easy division between teams.

There are downsides, too (like everything in engineering!):

* There's more boilerplate code to write initially.
* Performance is bounded by the single-threaded actor. You might need to start using an MPMC channel and multiple actors if you need more throughput.
* If the actor crashes, the whole system might be affected. You can mitigate this with supervision strategies, but that adds complexity.

There are actor frameworks to help with this. Netflix are heavily using [ractor](https://github.com/slawlor/ractor) in production. [Actix](https://actix.rs/) is another popular choice in the Rust ecosystem.