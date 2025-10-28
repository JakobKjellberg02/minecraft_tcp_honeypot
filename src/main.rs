use std::{io::{Cursor, Read, Write}, net::{TcpListener, TcpStream}};
use varint::{VarintRead, VarintWrite};

fn handle_ping(stream: &mut TcpStream) {
    let packet_length = stream.read_unsigned_varint_32().unwrap();
    let mut buffer = vec![0u8; packet_length as usize];
    stream.read_exact(&mut buffer).unwrap();
    let mut cursor = Cursor::new(buffer);
    
    let packet_id = cursor.read_unsigned_varint_32().unwrap();
    if packet_id != 1 { return };
    
    let mut payload_bytes = [0u8; 8];
    cursor.read_exact(&mut payload_bytes).unwrap();
    
    let mut vector = Cursor::new(vec![0u8; 0]);
    vector.write_unsigned_varint_32(1).unwrap();
    vector.write(&payload_bytes).unwrap();
    
    let mut packet = Cursor::new(vec![0u8; 0]);
    packet.write_unsigned_varint_32(vector.get_ref().len() as u32).unwrap();
    packet.write(vector.get_ref()).unwrap();

    stream.write_all(packet.get_ref()).unwrap();
    stream.flush().unwrap();
}

fn handle_status(stream: &mut TcpStream) {
    let parse_json = r#"{
        "version": {"name": "1.21.10", "protocol": 773}, 
        "players": {"max": 20, "online": 3},
        "description": {"text": "Joe Minecraft Server"}
    }"#;
    let json_bytes = parse_json.as_bytes();
    let mut vector = Cursor::new(vec![0u8; 0]);
    vector.write_unsigned_varint_32(0).unwrap();
    vector.write_unsigned_varint_32(json_bytes.len() as u32).unwrap();
    vector.write(json_bytes).unwrap();

    let mut packet = Cursor::new(vec![0u8; 0]);
    packet.write_unsigned_varint_32(vector.get_ref().len() as u32).unwrap();
    packet.write(vector.get_ref()).unwrap();

    stream.write_all(packet.get_ref()).unwrap();
    stream.flush().unwrap();
}

fn handle_login(stream: &mut TcpStream) {
    let packet_length = stream.read_unsigned_varint_32().unwrap();
    println!("Read VarInt: {}", packet_length);

    let mut buffer = vec![0u8; packet_length as usize];
    stream.read_exact(&mut buffer).unwrap();
    let mut vector = Cursor::new(buffer);

    let packet_id = vector.read_unsigned_varint_32().unwrap();
    println!("Packet ID version: {}", packet_id);

    let name_len = vector.read_unsigned_varint_32().unwrap() as usize;
    let mut name_bytes = vec![0u8; name_len];
    vector.read_exact(&mut name_bytes).unwrap();
    let username = String::from_utf8_lossy(&name_bytes);
    println!("Username = {}", username);

    let parse_response = r#"{"text":"Don't try funny stuff script kiddie"}"#;
    let response_bytes = parse_response.as_bytes();
    let mut vector = Cursor::new(vec![0u8; 0]);
    vector.write_unsigned_varint_32(0).unwrap();
    vector.write_unsigned_varint_32(response_bytes.len() as u32).unwrap();
    vector.write(response_bytes).unwrap();

    let mut packet = Cursor::new(vec![0u8; 0]);
    packet.write_unsigned_varint_32(vector.get_ref().len() as u32).unwrap();
    packet.write(vector.get_ref()).unwrap();

    stream.write_all(packet.get_ref()).unwrap();
}

fn handle_connection(mut stream : TcpStream) {
    let packet_length = stream.read_unsigned_varint_32().unwrap();
    println!("Read VarInt: {}", packet_length);

    let mut buffer = vec![0u8; packet_length as usize];
    stream.read_exact(&mut buffer).unwrap();
    let mut vector = Cursor::new(buffer);

    let packet_id = vector.read_unsigned_varint_32().unwrap();
    println!("Packet ID version: {}", packet_id);

    let protocol_version = vector.read_unsigned_varint_32().unwrap();
    println!("Protocol version: {}", protocol_version);

    let server_adr_length = vector.read_unsigned_varint_32().unwrap() as usize;
    let mut server_bytes = vec![0u8; server_adr_length];
    vector.read_exact(&mut server_bytes).unwrap(); 
    let server_adr = String::from_utf8_lossy(&server_bytes);
    println!("Server Address: {}", server_adr);
    
    let mut port_bytes = [0u8; 2];
    vector.read_exact(&mut port_bytes).unwrap();
    let server_port = u16::from_be_bytes(port_bytes);
    println!("Server port: {}", server_port);

    let next_state = vector.read_unsigned_varint_32().unwrap();
    println!("State: {}", next_state);
    match next_state {
        1 => { handle_status(&mut stream); handle_ping(&mut stream); }
        2 => handle_login(&mut stream),
        _ => eprintln!("Unknown state: {}", next_state)
    }

}

fn main() {
    let listener_result = TcpListener::bind("0.0.0.0:25565");
    let listener: TcpListener = match listener_result {
        Ok(tcp) => tcp,
        Err(error) => { panic!("Problem establishing TCP Listener: {error:?}") }
    };
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Established connection: {:?}", stream.peer_addr());
                handle_connection(stream);
            }
            Err(error) => eprintln!("Incoming connection failed: {}", error),
        }
    }
}