use std::net::TcpStream;
use std::{fs, thread};
use std::io::prelude::*;
pub struct  Spawner{
    workers: Vec<Jobs>
}
impl Spawner {
    pub fn new(length: usize)-> Spawner{
        assert!(length > 0);
        let mut workers = Vec::with_capacity(length);
        for _ in [..length].iter() {
            workers.push(Jobs::new())
        }
        Spawner{
            workers
        }
    }
    pub fn handle_stream(&self, stream: TcpStream){
        handle_connection(stream)
    }
}

struct Jobs{
}
impl Jobs {
    fn new()-> Jobs {
        Jobs{}
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer));
    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {
        response(stream, "HTTP/1.1 200 OK", "index.html")
    }else{
        response(stream, "HTTP/1.1 404 NOT FOUND", "404.html")
    }
}

fn response(mut stream: TcpStream, status_line: &str, path: &str){
    let content = fs::read_to_string(path).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap()
}