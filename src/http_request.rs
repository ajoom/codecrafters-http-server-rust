use std::{
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

pub enum RequestMethod {
    POST,
    GET,
}

impl RequestMethod {
    pub fn from_str(s: &str) -> Self {
        match s {
            "GET" => RequestMethod::GET,
            "POST" => RequestMethod::POST,
            _ => panic!("Non-supported request method"),
        }
    }
}

pub struct HttpRequest {
    // 1- request line:
    pub method: RequestMethod,
    pub path: String,
    pub _http_version: String,

    // 2- headers:
    pub headers: Vec<(String, String)>,

    // 3- optional body:
    pub body: Option<String>,
}

impl HttpRequest {
    pub fn from_stream(stream: &mut TcpStream) -> Self {
        let mut reader = BufReader::new(stream);

        // 1- Parse request line
        let mut request_line = String::new();
        reader.read_line(&mut request_line).unwrap();
        let parts: Vec<&str> = request_line.trim_end().split_whitespace().collect();

        if parts.len() != 3 {
            panic!("Non-valid request line");
        }

        let method = RequestMethod::from_str(parts[0]);
        let path = parts[1].to_string();
        let http_version = parts[2].to_string();

        // 2- Parse headers
        let mut headers = Vec::new();
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            let line = line.trim_end();
            if line.is_empty() {
                break; // empty line marks end of headers
            }
            if let Some((key, value)) = line.split_once(":") {
                headers.push((key.trim().to_string(), value.trim().to_string()));
            }
        }

        // 3- Parse body
        let mut body = String::new();
        reader.read_to_string(&mut body).ok();
        let body = if body.is_empty() { None } else { Some(body) };

        HttpRequest {
            method,
            path,
            _http_version: http_version,
            headers,
            body,
        }
    }
}
