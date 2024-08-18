use std::io::Read;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:179").unwrap();
    println!("Server listening on port 179");

    let (mut stream, _) = listener.accept().unwrap();
    println!("Incoming connection: {}", stream.peer_addr().unwrap());

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
