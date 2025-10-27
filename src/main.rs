use std::net::TcpListener;
fn main() {
    let listener_result = TcpListener::bind("127.0.0.1:25565");
    let listener = match listener_result {
        Ok(tcp) => tcp,
        Err(error) => { panic!("Problem establishing TCP Listener: {error:?}") }
    };
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("established connection");
    }
}