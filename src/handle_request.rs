use std::{io::Write, net::TcpStream};

use crate::{
    compression_utils::gz_encoding,
    files_util::handle_file_request,
    http_request::{HttpRequest, VALID_COMPRESSION_METHODS},
    http_response::{construct_http_response, ResponseStatus},
};

pub fn handle_request(mut stream: &TcpStream, request: HttpRequest) {
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
                    body = gz_encoding(&body);
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
