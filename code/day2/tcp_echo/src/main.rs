use std::{io::{Read, Write}, thread};

fn server() {
    let listener = std::net::TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        thread::spawn(move || {
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();
            stream.write(&buffer).unwrap();
        });
    }
}

fn main() {
    thread::spawn(|| server());
    thread::sleep(std::time::Duration::from_millis(100)); // Give server time to start

    let mut stream = std::net::TcpStream::connect("127.0.0.1:8080").unwrap();
    stream.write(b"Hello, world!").unwrap();
    println!("Sent: Hello, world!");
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Received: {}", String::from_utf8_lossy(&buffer));

    drop(stream); // Close the client connection
    // Note: In a real application, you'd want a way to gracefully shut down the server    
}
