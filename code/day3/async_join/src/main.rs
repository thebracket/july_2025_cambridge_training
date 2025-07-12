use tokio::time::{sleep, Duration};

async fn task1() {
    sleep(Duration::from_secs(2)).await;
    println!("Task 1 complete");
}

async fn task2() {
    sleep(Duration::from_secs(1)).await;
    println!("Task 2 complete");
}

#[tokio::main]
async fn main() {
    tokio::join!(task1(), task2());
    println!("Both tasks complete");
}