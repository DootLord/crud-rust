use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listner = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listner.incoming() {
        let stream = stream.unwrap();
        handle_collection(stream);
    }
}

fn handle_collection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    print!("Request: {:?}", http_request);
}
