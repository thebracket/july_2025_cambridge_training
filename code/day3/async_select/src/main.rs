use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(5);
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));

    tokio::spawn(async move {
        loop {
            tx.send("Hello after 2 seconds").await.unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs_f32(0.1)).await;
        }
    });

    let mut tick_count = 0;
    loop {
        tokio::select! {
            Some(msg) = rx.recv() => {
                println!("Received: {}", msg);
            }
            _ = interval.tick() => {
                println!("Tick");
                tick_count += 1;
                if tick_count >= 5 {
                    println!("Exiting after five ticks.");
                    break;
                }
            }
        }
    }
}