use crate::{
    compression_utils::gz_encoding,
    files_util::handle_file_request,
    http_request::{HttpRequest, VALID_COMPRESSION_METHODS},
    http_response::{construct_http_response, ResponseStatus},
};
use std::{io::Write, net::TcpStream};

pub fn handle_request(mut stream: &TcpStream, request: HttpRequest) -> bool {
    let should_close_connection = should_close_connection(&request);
    let response = route_request(&request);

    stream.write_all(&response).unwrap();
    should_close_connection
}

fn should_close_connection(request: &HttpRequest) -> bool {
    request
        .headers
        .get("Connection")
        .is_some_and(|value| value == "close")
}

fn route_request(request: &HttpRequest) -> Vec<u8> {
    let path = &request.path;

    match path.as_str() {
        "/" => handle_root(),
        path if path.starts_with("/echo") => handle_echo(request),
        path if path.starts_with("/files/") => handle_file_request(request),
        "/user-agent" => handle_user_agent(request),
        _ => handle_not_found(),
    }
}

fn handle_root() -> Vec<u8> {
    construct_http_response(ResponseStatus::SuccessfulResponse, &[], None)
}

fn handle_echo(request: &HttpRequest) -> Vec<u8> {
    let path_parts: Vec<&str> = request.path.split('/').collect();

    if path_parts.len() != 3 {
        return handle_not_found();
    }

    let mut headers: Vec<(String, String)> = Vec::new();
    headers.push(("Content-Type".to_string(), "text/plain".to_string()));

    // Handle content encoding
    let (encoded_body, encoding_headers) = handle_content_encoding(request);

    // Add encoding headers
    headers.extend(encoding_headers);

    if let Some(connection_header) = request.headers.get("Connection") {
        if connection_header == "close" {
            headers.push(("Connection".to_string(), "close".to_string()));
        }
    }

    // Add content length
    let content_length = encoded_body.len().to_string();
    headers.push(("Content-Length".to_string(), content_length));

    let headers: Vec<(&str, &str)> = headers
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();

    construct_http_response(
        ResponseStatus::SuccessfulResponse,
        &headers,
        Some(&encoded_body),
    )
}

fn handle_user_agent(request: &HttpRequest) -> Vec<u8> {
    let user_agent = request
        .headers
        .get("User-Agent")
        .expect("User-Agent header missing")
        .clone();

    let body = user_agent.into_bytes();
    let content_length = body.len().to_string();

    let headers = [
        ("Content-Type", "text/plain"),
        ("Content-Length", content_length.as_str()),
    ];

    construct_http_response(ResponseStatus::SuccessfulResponse, &headers, Some(&body))
}

fn handle_not_found() -> Vec<u8> {
    construct_http_response(ResponseStatus::NotFoundResponse, &[], None)
}

fn handle_content_encoding(request: &HttpRequest) -> (Vec<u8>, Vec<(String, String)>) {
    let Some(accepted_encodings) = request.headers.get("Accept-Encoding") else {
        return get_unencoded_echo_body(request);
    };

    let encoding_methods: Vec<&str> = accepted_encodings.split(',').map(str::trim).collect();

    let valid_encoding = encoding_methods
        .iter()
        .find(|method| VALID_COMPRESSION_METHODS.contains(method));

    match valid_encoding {
        Some(method) => {
            let body = request
                .body
                .as_ref()
                .expect("Echo request should have body")
                .as_bytes();
            let compressed_body = gz_encoding(body);
            (
                compressed_body,
                vec![("Content-Encoding".to_string(), method.to_string())],
            )
        }
        None => get_unencoded_echo_body(request),
    }
}

fn get_unencoded_echo_body(request: &HttpRequest) -> (Vec<u8>, Vec<(String, String)>) {
    let body = request
        .body
        .as_ref()
        .expect("Echo request should have body")
        .as_bytes()
        .to_vec();
    (body, vec![])
}
