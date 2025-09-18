#[allow(unused_imports)]
use std::net::TcpListener;
use std::{io::Write, net::TcpStream};


fn handle_connection(mut stream: TcpStream) {
    const SUCCESSFUL_RESPONSE: &str = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(SUCCESSFUL_RESPONSE.as_bytes()).unwrap();
}


fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
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
