#[allow(unused_imports)]
use std::net::TcpListener;
use std::thread;
use std::{io::Write, net::TcpStream};
mod compression_utils;
mod files_util;
mod http_request;
mod http_response;
use crate::compression_utils::GZ_encoding;
use crate::files_util::handle_file_request;
use crate::http_request::{HttpRequest, VALID_COMPRESSION_METHODS};
use crate::http_response::construct_http_response;
use crate::http_response::ResponseStatus;

fn handle_connection(mut stream: TcpStream) {
    let request = HttpRequest::from_stream(&mut stream);
    let path = request.path.clone();

    let response;

    if path == "/" {
        response = construct_http_response(ResponseStatus::SuccessfulResponse, &[], None);
    } else if path.starts_with("/echo") {
        let parts: Vec<&str> = path.split('/').collect();

        if parts.len() == 3 {
            let mut body = parts[2].to_string().into_bytes();

            let mut response_headers: Vec<(&str, &str)> = vec![("Content-Type", "text/plain")];
            if let Some(accepted_encoding_methods_string) = request.headers.get("Accept-Encoding") {
                let accepted_encoding_methods: Vec<&str> = accepted_encoding_methods_string
                    .split(',')
                    .map(|s| s.trim())
                    .collect();
                let valid_method = accepted_encoding_methods
                    .iter()
                    .find(|method| VALID_COMPRESSION_METHODS.contains(&method));

                if let Some(method) = valid_method {
                    body = GZ_encoding(&body);
                    response_headers.push(("Content-Encoding", method));
                }
            }

            let content_length = body.len().to_string();
            response_headers.push(("Content-Length", &content_length));
            response = construct_http_response(
                ResponseStatus::SuccessfulResponse,
                &response_headers,
                Some(&body),
            );
        } else {
            response = construct_http_response(ResponseStatus::NotFoundResponse, &[], None);
        }
    } else if path.starts_with("/files/") {
        response = handle_file_request(request);
    } else if path == "/user-agent" {
        let body = request
            .headers
            .get("User-Agent")
            .unwrap()
            .clone()
            .into_bytes();

        response = construct_http_response(
            ResponseStatus::SuccessfulResponse,
            &[
                ("Content-Type", "text/plain"),
                ("Content-Length", &body.len().to_string()),
            ],
            Some(&body),
        );
    } else {
        response = construct_http_response(ResponseStatus::NotFoundResponse, &[], None);
    }

    stream.write_all(&response).unwrap();
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
