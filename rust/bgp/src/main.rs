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
    marker: u128,
    length: u16,
    // type is Rust's keyword, so need to use alternative
    message_type: MessageType,
}

#[derive(Debug)]
struct DecodeError;

impl TryFrom<&[u8]> for Header {
    type Error = DecodeError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 19 {
            return Err(DecodeError);
        }

        let marker = &bytes[0..16];
        let marker = marker
            .try_into()
            .map(|bs| u128::from_be_bytes(bs))
            .map_err(|_| DecodeError)
            .and_then(|m| match m {
                u128::MAX => Ok(m),
                _ => Err(DecodeError),
            })?;

        let length = &bytes[16..18];
        let length = u16::from_be_bytes(length.try_into().unwrap());
        MessageType::try_from(bytes[18]).map(|message_type| Header {
            marker,
            length,
            message_type,
        })
    }
}

#[derive(Debug)]
enum MessageType {
    Open,
    Update,
    Notification,
    Keepalive,
}

impl TryFrom<u8> for MessageType {
    type Error = DecodeError;

    fn try_from(msg_type: u8) -> Result<Self, Self::Error> {
        match msg_type {
            1 => Ok(Self::Open),
            2 => Ok(Self::Update),
            3 => Ok(Self::Notification),
            4 => Ok(Self::Keepalive),
            _ => Err(DecodeError),
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
            Ok(size) => match Header::try_from(&buffer[..size]) {
                Ok(header) => println!("Header received: {:?}", header),
                Err(_) => print!("Decode error"),
            },
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
}
