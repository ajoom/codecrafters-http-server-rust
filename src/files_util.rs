use std::path::Path;

use crate::{
    http_request::{HttpRequest, RequestMethod},
    http_response::{construct_http_response, ResponseStatus},
};

pub fn handle_file_request(request: HttpRequest) -> Vec<u8> {
    let filename = &request.path["/files/".len()..];
    let directory = std::env::args()
        .skip_while(|arg| arg != "--directory")
        .nth(1)
        .expect("Please provide --directory <path>");

    let full_path = Path::new(&directory).join(filename);

    match request.method {
        RequestMethod::GET => {
            // file doesnt exist
            if !full_path.exists() || !full_path.is_file() {
                return construct_http_response(ResponseStatus::NotFoundResponse, &[], None);
            }

            let contents = std::fs::read(&full_path).unwrap();
            return construct_http_response(
                ResponseStatus::SuccessfulResponse,
                &[
                    ("Content-Type", "application/octet-stream"),
                    ("Content-Length", &contents.len().to_string()),
                ],
                Some(&contents),
            );
        }

        RequestMethod::POST => {
            let body = request.body.unwrap();

            match std::fs::write(&full_path, body) {
                Ok(_) => {
                    // 3- Return 201 Created
                    construct_http_response(ResponseStatus::CreatedResponse, &[], None)
                }
                Err(_) => {
                    // if writing fails, return 404 for simplicity
                    construct_http_response(ResponseStatus::NotFoundResponse, &[], None)
                }
            }
        }
    }
}
