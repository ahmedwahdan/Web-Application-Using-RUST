use std::net::TcpStream;
use std::io::{Read, Write};
use std::str;

fn main() {
    let mut _stream : TcpStream = TcpStream::connect("127.0.0.1:3000").unwrap();

    let mut buffer : [u8; 5] = [0; 5];

    _stream.write("Hello".as_bytes()).unwrap();
    _stream.read(&mut buffer).unwrap();

    println!("Got response from server {:?}", str::from_utf8(&buffer).unwrap());

}
