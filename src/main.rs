#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&stream);
    let request = reader
        .lines() // gives an iterator over each line in the request, then read the first line
        .next() // grap the first line
        .unwrap()
        .unwrap();

    // e.g: GET /index.html HTTP/1.1
    let parts: Vec<&str> = request.split_whitespace().collect();
    let _method = parts[0];
    let path = parts[1];
    let _http_version = parts[2];

    println!("parts are {:?}", parts);

    const SUCCESSFUL_RESPONSE: &str = "HTTP/1.1 200 OK\r\n\r\n";
    const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 Not Found\r\n\r\n";

    match path {
        "/index.html" => stream.write_all(SUCCESSFUL_RESPONSE.as_bytes()).unwrap(),
        _ => stream.write_all(NOT_FOUND_RESPONSE.as_bytes()).unwrap(),
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    // send custom http requests and debug locally. Eg: curl -i -X GET http://localhost:4221/index.html

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
