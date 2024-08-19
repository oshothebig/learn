use std::io::Read;
use std::net::{TcpListener, TcpStream};

fn main() {
    let port = 179;

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).unwrap();
    println!("Server listening on port {port}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => println!("Error: {}", e),
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let addr = match stream.peer_addr() {
        Ok(peer_addr) => peer_addr.to_string(),
        Err(e) => e.to_string(),
    };
    println!("Incoming connection: {}", addr);

    let mut buffer = [0u8; 1024 * 4];
    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                if size == 0 {
                    println!("Connection closed");
                    break;
                }
                println!("Received {} bytes", size);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
}
