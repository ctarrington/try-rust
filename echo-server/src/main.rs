use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

async fn respond(mut stream: TcpStream) {
    let mut raw = String::new();
    BufReader::new(&stream)
        .read_to_string(&mut raw)
        .expect("error reading stream");

    let response = format!("{}", raw);
    println!("reply {}", response);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        respond(stream.unwrap()).await;
    }
}
