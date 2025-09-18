pub enum ResponseStatus {
    SuccessfulResponse,
    NotFoundResponse,
}

pub fn construct_http_response(
    status: ResponseStatus,
    headers: &[(&str, &str)], // slice of key-value header pairs
    body: Option<&str>,
) -> String {
    // Add status line
    let status_line = match status {
        ResponseStatus::SuccessfulResponse => "HTTP/1.1 200 OK",
        ResponseStatus::NotFoundResponse => "HTTP/1.1 404 Not Found",
    };
    let mut response = format!("{}\r\n", status_line);

    // Add headers
    for (key, value) in headers {
        response.push_str(&format!("{}: {}\r\n", key, value));
    }
    response.push_str("\r\n");

    // Add the body
    if let Some(body) = body {
        response.push_str(body);
    }

    response
}
