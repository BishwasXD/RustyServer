//contains fns that returns type of request and request body.
use std::{ any, io::{ prelude::*, BufReader }, net::TcpStream };
use serde::Deserialize;
use serde_json::json;

use crate::views::Task;
pub fn get_content_length(line: String) -> usize {
    let parts: Vec<&str> = line.split(": ").collect();
    let content_length: &&str = parts.get(1).expect("Not Found");
    return content_length.parse().unwrap();
}

//tested with 4000+ byte complex nested data
pub fn get_req_body(buffer: &mut BufReader<&mut TcpStream>, content_length: usize) -> String {
    let mut req_body: Vec<u8> = vec![0; content_length + 2];
    buffer.read_exact(&mut req_body).expect("Failed to read");
    let body_str: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&req_body);
    return body_str.into_owned();
}

pub fn get_bearer_token() {}

pub fn request_type(x: String) -> String {
    let splitted_line: Vec<&str> = x.split(" / ").collect();
    return splitted_line.get(0).expect("Not Found!").to_string();
}

pub fn send_response(response_body: String, status_code: usize) -> String {
    // let json_body: String = serde_json::to_string(&response_body).unwrap();
    // // Remove the extra "tasks:"

    let status_message = match status_code {
        200 => "OK",
        201 => "Created",
        400 => "Bad Request",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Unknown Status",
    };

    let status_line: String = format!("HTTP/1.1 {} {}\r\n", status_code, status_message);
    let content_type: &str = "Content-Type: application/json\r\n";
    let content_length: String = format!("Content-Length: {}\r\n", response_body.len());
    let access_control: &str = "Access-Control-Allow-Origin: *\r\n";

    let header_end: &str = "\r\n";

    let response: String = format!(
        "{}{}{}{}{}{}",
        status_line,
        content_type,
        content_length,
        access_control,
        header_end,
        response_body
    );
    println!("{response}");
    response
}