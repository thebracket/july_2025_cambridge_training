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