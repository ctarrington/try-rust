use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

async fn respond(mut stream: TcpStream) {
    let response = format!("{}", "hello");
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
