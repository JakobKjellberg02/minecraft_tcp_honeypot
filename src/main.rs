use std::{io::{Cursor, Read}, net::{TcpListener, TcpStream}};
use varint::VarintRead;

fn handle_connection(mut stream : TcpStream) {
    let packet_length  = match stream.read_unsigned_varint_32() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Failed to read VarInt: {:?}", err);
            return;
        }
    };
    println!("Read VarInt: {}", packet_length);

    let mut buffer = vec![0u8; packet_length as usize];
    match stream.read_exact(&mut buffer) {
        Ok(()) => println!("No errors with reading stream to buffer"),
        Err(e) => eprintln!("Failed to read the numbers of bytes to fill buffer: {:?}", e),
    }

    let mut vector = Cursor::new(buffer);
    let packet_id = match vector.read_unsigned_varint_32() {
        Ok(id) => id,
        Err(err) => {
            eprintln!("Failed to read packet ID: {:?}", err);
            return;
        }
    };
    println!("Packet ID: {}", packet_id);

    let protocol_version = match vector.read_unsigned_varint_32() {
        Ok(id) => id,
        Err(err) => {
            eprintln!("Failed to read packet ID: {:?}", err);
            return;
        }
    };
    println!("Protocol version: {}", protocol_version);
}

fn main() {
    let listener_result = TcpListener::bind("127.0.0.1:25565");
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