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

#[derive(Debug)]
struct Header {
    maker: u128,
    length: u16,
    // type is Rust's keyword, so need to use alternative
    message_type: MessageType,
}

impl Header {
    fn decode(buf: &[u8]) -> Option<Self> {
        // header length is 19
        if buf.len() < 19 {
            return None;
        }
        let marker = &buf[0..16];
        let marker = u128::from_be_bytes(marker.try_into().unwrap());
        // all marker bits are 1
        if marker != u128::MAX {
            return None;
        }

        let length = &buf[16..18];
        let length = u16::from_be_bytes(length.try_into().unwrap());
        match MessageType::decode(buf[18]) {
            Some(msg_type) => Some(Header {
                maker: u128::MAX,
                length: length,
                message_type: msg_type,
            }),
            None => None,
        }
    }
}

#[derive(Debug)]
enum MessageType {
    Open,
    Update,
    Notification,
    Keepalive,
}

impl MessageType {
    fn decode(msg_type: u8) -> Option<Self> {
        match msg_type {
            1 => Some(Self::Open),
            2 => Some(Self::Update),
            3 => Some(Self::Notification),
            4 => Some(Self::Keepalive),
            _ => None,
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
            Ok(0) => {
                println!("Connection closed");
                break;
            }
            Ok(size) => match Header::decode(&buffer) {
                Some(header) => println!("Header received: {:?}", header),
                None => print!("Decode error"),
            },
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
}
