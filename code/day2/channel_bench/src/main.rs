use std::sync::mpsc;
use std::thread;
use std::time::Instant;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 0..100_000 {
            tx.send(i).unwrap();
        }
    });

    let start = Instant::now();
    for _ in 0..100_000 {
        rx.recv().unwrap();
    }
    let duration = start.elapsed();

    println!("Time taken: {:?}", duration);
}