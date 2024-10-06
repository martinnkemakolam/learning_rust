use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use server::Spawner;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let spawn = Spawner::new(4);
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        spawn.handle_stream(stream);
    }
}
