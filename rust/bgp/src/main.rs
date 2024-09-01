use std::io::Read;
use std::net::{Ipv4Addr, TcpListener, TcpStream};

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
enum Message {
    Open(OpenMessage),
}

impl TryFrom<&[u8]> for Message {
    type Error = DecodeError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let header = Header::try_from(bytes)?;

        if bytes.len() < header.length as usize {
            return Err(DecodeError);
        }

        let body = &bytes[19..header.length as usize];
        match header.message_type {
            MessageType::Open => OpenMessage::try_from(body).map(|m| Message::Open(m)),
            _ => Err(DecodeError),
        }
    }
}

#[derive(Debug)]
struct OpenMessage {
    version: u8,
    my_as: u16,
    hold_time: u16,
    identifier: Ipv4Addr,
    opt_param_length: u8,
    capabilities: Vec<u8>,
}

impl TryFrom<&[u8]> for OpenMessage {
    type Error = DecodeError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let version_range = 0..1;
        let my_asn_range = 1..3;
        let hold_time_range = 3..5;
        let identifier_range = 5..9;
        let opt_param_length_range = 9..10;

        let version: u8 = (&bytes[version_range])
            .try_into()
            .map(|bs| u8::from_be_bytes(bs))
            .map_err(|_| DecodeError)?;
        let my_as = (&bytes[my_asn_range])
            .try_into()
            .map(|bs| u16::from_be_bytes(bs))
            .map_err(|_| DecodeError)?;
        let hold_time = (&bytes[hold_time_range])
            .try_into()
            .map(|bs| u16::from_be_bytes(bs))
            .map_err(|_| DecodeError)?;
        let identifier = (&bytes[identifier_range])
            .try_into()
            .map(|bs: [u8; 4]| Ipv4Addr::from(bs))
            .map_err(|_| DecodeError)?;
        let opt_param_length = (&bytes[opt_param_length_range])
            .try_into()
            .map(|bs| u8::from_be_bytes(bs))
            .map_err(|_| DecodeError)?;
        let capabilities = Vec::from(&bytes[10..]);

        Ok(OpenMessage {
            version,
            my_as,
            hold_time,
            identifier,
            opt_param_length,
            capabilities,
        })
    }
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
        match parse_message(&mut stream, &mut buffer) {
            Ok(message) => {
                println!("Received message: {:?}", message)
            }
            Err(_) => {
                println!("Closing connection");
                break;
            }
        }
    }
}

fn parse_message(stream: &mut TcpStream, buf: &mut [u8]) -> Result<Message, DecodeError> {
    const HEADER_LENGTH: usize = 19;

    let header_buf = &mut buf[0..HEADER_LENGTH];
    if let Err(e) = stream.read_exact(header_buf) {
        println!("Error while reading header: {}", e);
        return Err(DecodeError);
    }

    let header = match Header::try_from(header_buf.as_ref()) {
        Ok(header) => header,
        Err(e) => {
            println!("Error while parsing header: {:?}", e);
            return Err(DecodeError);
        }
    };

    println!("Header received: {:?}", header);

    let body_buf = &mut buf[HEADER_LENGTH..header.length as usize];

    if let Err(e) = stream.read_exact(body_buf) {
        println!("Error while reading body: {}", e);
        return Err(DecodeError);
    }

    let message = match Message::try_from(&buf[0..header.length as usize]) {
        Ok(message) => message,
        Err(e) => {
            println!("Error while parsing message: {:?}", e);
            return Err(DecodeError);
        }
    };

    Ok(message)
}
