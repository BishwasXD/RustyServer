//contains fns that returns type of request and request body.
use std::{ io::{ prelude::*, BufReader }, net:: TcpStream  };


pub fn get_content_length(line: String) -> usize {
    let parts: Vec<&str> = line.split(": ").collect();
    let content_length: &&str = parts.get(1).expect("Not Found");
    return content_length.parse().unwrap();
}

//tested with 4000+ byte complex nested data
pub fn get_req_body(buffer: &mut BufReader<&mut TcpStream>, content_length: usize) -> String {
    let mut req_body = vec![0; content_length + 2];
    buffer.read_exact(&mut req_body).expect("Failed to read");
    let body_str: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&req_body);
    return body_str.into_owned();
}

pub fn get_bearer_token() {}

pub fn request_type(x: String) -> String {
    let splitted_line: Vec<&str> = x.split(" / ").collect();
    return splitted_line.get(0).expect("Not Found!").to_string();
}

pub fn send_response(response_body: String, status_code: u8) -> String{
    let content_length: usize = response_body.len();
    let status_line = format!("HTTP/1.1 {status_code} OK");
    let res = format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{response_body}");
    return res

}
