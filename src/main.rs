use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
mod compression_utils;
mod files_util;
mod handle_request;
mod http_request;
mod http_response;
use crate::handle_request::handle_request;
use crate::http_request::HttpRequest;

fn handle_connection(mut stream: TcpStream) {
    loop {
        let request = match HttpRequest::from_stream(&mut stream) {
            Ok(req) => req,
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                break;
            }
            Err(e) => {
                eprintln!("Bad request: {}", e);
                break;
            }
        };

        if handle_request(&mut stream, request) {
            break;
        }
    }

    println!("FIRST CONNECTION HAS ENDED");
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    // send custom http requests and debug locally. Eg: curl -i -X GET http://localhost:4221/index.html
    // curl -i -X GET http://localhost:4221/echo/ahmad

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // create a thread for each connection
                thread::spawn(|| handle_connection(stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
