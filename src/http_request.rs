use std::{
    io::{BufRead, BufReader},
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
    pub _method: RequestMethod,
    pub path: String,
    pub _http_version: String,

    // 2- headers:
    pub headers: Vec<(String, String)>,

    // 3- optional body:
    pub _body: Option<String>,
}

impl HttpRequest {
    pub fn from_stream(stream: &mut TcpStream) -> Self {
        let reader = BufReader::new(stream);
        let mut lines_iter = reader.lines();

        // 1- Parse request line
        let request_line = lines_iter.next().unwrap().unwrap();
        let parts: Vec<&str> = request_line.split_whitespace().collect();

        if parts.len() != 3 {
            panic!("Non-valid request line");
        }

        let method = RequestMethod::from_str(parts[0]);
        let path = parts[1].to_string();
        let http_version = parts[2].to_string();

        // 2- Parse headers
        let mut headers = Vec::new();
        for line in lines_iter.by_ref() {
            let line = line.ok().unwrap();

            if line.is_empty() {
                break; // empty line marks end of headers
            }
            if let Some((key, value)) = line.split_once(":") {
                headers.push((key.trim().to_string(), value.trim().to_string()));
            }
        }

        HttpRequest {
            _method: method,
            path,
            _http_version: http_version,
            headers,
            _body: None, // TODO next
        }
    }
}
