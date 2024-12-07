use core::str;
use std::{ io::{ prelude::*, BufReader }, net::{ TcpListener, TcpStream }, vec };
pub mod utils;
pub mod views;
mod router;
mod models;
pub struct HttpRequest {
    pub method: String,
    pub body: String,
    pub content_length: usize,
    pub bearer_token: String,
    pub content_type: String,
    pub url: String,
}

pub struct RequestBody {}
fn main() {
    const HOST: &str = "127.0.0.1";
    const PORT: &str = "8477";

    let end_point: String = HOST.to_owned() + ":" + PORT; //listening address

    let listener: TcpListener = TcpListener::bind(end_point).expect(
        "Failed listening to the address"
    );
    println!("Server runnning on port {PORT}");
    for stream in listener.incoming() {
        let stream: std::net::TcpStream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("Handling connection...");

    let mut buffer = BufReader::new(&mut stream);

    let mut request_information: Vec<String> = vec!["GET".to_owned(), "/".to_owned()];
    if let Some(Ok(line)) = buffer.by_ref().lines().next() {
        request_information = utils::parse_request(line);
    } else {
        println!("Error reading request method");
        return;
    }

    let method = request_information.get(0).unwrap().to_owned();
    let url = request_information.get(1).unwrap().to_owned();

    let mut content_length = 0;

    for line_result in buffer.by_ref().lines() {
        let line = line_result.expect("Failed to read a line");
    
        if line.starts_with("Content-Length:") {
            content_length = utils::get_content_length(line.clone());
        }

        if line.is_empty() {
            break;
        }
    }
    let mut body = String::new();
    if content_length > 0 {
        let mut body_buffer = buffer.take(content_length as u64);
        body_buffer.read_to_string(&mut body).expect("Failed to read body");
    }

    println!("Request Body: {body}");
    let http_request = HttpRequest {
        method,
        body,
        content_length,
        bearer_token: String::new(),
        content_type: String::new(),
        url,
    };

    let response = router::dispatch_request(http_request);
    stream.write_all(response.as_bytes()).expect("Failed to write response");
    println!("Response send successfully!");
}
