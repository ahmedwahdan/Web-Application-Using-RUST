use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let connection_listener: TcpListener = TcpListener::bind("127.0.0.1:3000").unwrap();

    println!("Server listening on port 3000");

    for stream in connection_listener.incoming() {
        let mut _stream = stream.unwrap();

        println!("Connection Established!");
        let mut buffer: [u8; 1024] = [0; 1024];

        _stream.read(&mut buffer);
        _stream.write(&buffer);
    }
}
