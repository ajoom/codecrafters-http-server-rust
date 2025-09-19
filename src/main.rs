#[allow(unused_imports)]
use std::net::TcpListener;
use std::thread;
use std::{io::Write, net::TcpStream};
mod http_request;
mod http_response;
use crate::http_request::HttpRequest;
use crate::http_response::construct_http_response;
use crate::http_response::ResponseStatus;

fn handle_connection(mut stream: TcpStream) {
    let request = HttpRequest::from_stream(&mut stream);
    let path = request.path;

    let response_string;

    if path == "/" {
        response_string = construct_http_response(ResponseStatus::SuccessfulResponse, &[], None);
    } else if path.starts_with("/echo") {
        let parts: Vec<&str> = path.split('/').collect();

        if parts.len() == 3 {
            let body = parts[2];

            response_string = construct_http_response(
                ResponseStatus::SuccessfulResponse,
                &[
                    ("Content-Type", "text/plain"),
                    ("Content-Length", &body.bytes().len().to_string()),
                ],
                Some(body),
            );
        } else {
            response_string = construct_http_response(ResponseStatus::NotFoundResponse, &[], None);
        }
    } else if path == "/user-agent" {
        let body = request
            .headers
            .iter()
            .find(|(key, _)| key == "User-Agent")
            .map(|(_, value)| value)
            .unwrap();

        response_string = construct_http_response(
            ResponseStatus::SuccessfulResponse,
            &[
                ("Content-Type", "text/plain"),
                ("Content-Length", &body.bytes().len().to_string()),
            ],
            Some(&body),
        );
    } else {
        response_string = construct_http_response(ResponseStatus::NotFoundResponse, &[], None);
    }

    stream.write_all(response_string.as_bytes()).unwrap();
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
               thread::spawn(|| handle_connection(stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
