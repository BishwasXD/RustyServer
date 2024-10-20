use std::{fmt::Error, fs, io::{ prelude::*, BufReader }, net::{ TcpListener, TcpStream } };

//TURN THIS TO A SIMPLE TODO WEBSITE
//string,date,bool
use serde::Deserialize;

//The #[derive(Deserialize)] macro allows serde to deserialize JSON data into this struct.
#[derive(Deserialize, Debug)]
struct User{
    name: String,
    id:u8,
}

pub struct HttpRequest{
   pub method: String,
   pub body: String,
   pub content_length: usize,
   pub bearer_token: String,
   pub content_type: String,

}
fn main() {
    const HOST: &str = "127.0.0.0";
    const PORT: &str = "8477";

    let end_point: String = HOST.to_owned() + ":" + PORT; //listening address

    let listener: TcpListener = TcpListener::bind(end_point).unwrap();
    println!("Server runnning on port {PORT}");
    for stream in listener.incoming() {
        let stream: std::net::TcpStream = stream.unwrap(); //unwraping coz it will return an ok type or error type.

        handle_connection(stream);
    }
}
fn handle_connection(mut stream: TcpStream) {
    println!("handling connection...");
    // let mut buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream); //wrapping the stream with a buffer reader
    // let request_line: String = buf_reader.lines().next().unwrap().unwrap();

    // FOR READING DATA DIRECTLY FROM STREAM
    /*
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    use std::str;
    println!("{:?}", buffer);
    let x = str::from_utf8(&buffer).unwrap();
    println!("{}", x);
 */
    //READING DATA AFTER BUFFERING
    //tip: we can create a buffer with predefined capacity as well. with ::with_capacity(cap, f)
    let mut buffer: BufReader<&mut TcpStream> = BufReader::new(&mut stream);

    /* 
    for line in buffer.lines()
    {
        let line = line.expect("Failed to read a line");
        println!("{line}");
        if line.is_empty() {
            break;
        }
      
    } */

    //need to distinguish req type as well.
    let mut type_of_request:String  = String::from("GET"); //default


    if let Some(Ok(line)) = buffer.by_ref().lines().next() {
        type_of_request = request_type(line);
 
    } else {
        println!("Error reading request method");
    }

    let mut content_length: usize = 0;
    for line in buffer.by_ref().lines() {
        let line = line.expect("Failed to read a line");
        println!("{line}");
        if line.starts_with("Content-Length") {
            content_length = get_content_length(line);
            break;
        }
    }
   let request_body = get_req_body(&mut buffer, content_length); 

    /* 
    if type_of_request == "POST"{

        let mut content_length: usize = 0;
        for line in buffer.by_ref().lines() {
            let line = line.expect("Failed to read a line");
            println!("{line}");
            if line.starts_with("Content-Length") {
                content_length = get_content_length(line);
                break;
            }
        }
        let mut req_body = vec![0; content_length + 2]; //prolly whitespace issue without +2, second closing bracket is not read.
        buffer.read_exact(&mut req_body).expect("Failed to read");
        let mut body_str: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&req_body); //look into smart pointers
        let parsed: User = serde_json::from_str(&body_str,).unwrap();
        let name = parsed.name;
        println!("the name of a user that sent a request is {name}");
        
        let response_message = "got response successfully";
        //if we wish to send a response body do it here with content type, size, similar to the request format.
        //TODO: DO WITH HTML AND RESPONSE JSON BODY //do some manipulation on data and send as response along with res message
        let status_line: &str = "HTTP/1.1 200 OK"; //r moves the cursor to the beg of line and n moves the cursor to the next line   
        let length = response_message.len();
        let response  = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{response_message}");
        stream.write_all(response.as_bytes()).unwrap();
    }     
    else if type_of_request == "GET" {
        println!("handling get request");
        let html_content = fs::read_to_string("/home/bishwas/Desktop/RustyServer/server/src/index.html").unwrap();
        let content_length = html_content.len();
        let status_line = "HTTP/1.1 200 OK";
        let response =  format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{html_content}");

        //write_all takes [u8] and send those bytes directly down the connection.
        stream.write_all(response.as_bytes()).unwrap();
        
    }
*/
let mut httpRequest: HttpRequest = HttpRequest{
    method: type_of_request,
    body: request_body,
    content_length: content_length,
    bearer_token: String::from(""),
    content_type: String::from("")
};

}

fn get_content_length(line: String) -> usize {
    let parts: Vec<&str> = line.split(": ").collect();
    let content_length: &&str = parts.get(1).expect("Not Found");
    return content_length.parse().unwrap();
}

//tested with 4000+ byte complex nested data
fn get_req_body(buffer: &mut BufReader<&mut TcpStream>, content_length: usize) -> String{
    let mut req_body = vec![0; content_length + 2];
    buffer.read_exact(&mut req_body).expect("Failed to read");
    let body_str: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&req_body); 
    return body_str.into_owned();
}

fn get_bearer_token(){

}
fn request_type(x:String) -> String{

    let splitted_line: Vec<&str> = x.split(" / ").collect();
    return splitted_line.get(0).expect("Not Found!").to_string()

}


//TODO: TRY TO HANDLE AUTH TOKEN AS WELL.

//the stream will return a connection descriptions:
//1.    
//listening address which is 127.0.0.0:8477 in this casse

//2.
//peer address which is 127.0.0.1:var_port_number
//peer address is a address of a client who made the request
//since both server and client are in same machine it uses loopback addr
//peer address uses Ephemeral Ports
//they are random port assigned by a browser which changed upon each request

//Loopback Address:
//A loopback address refers to the special IP range used to communicate with the same machine.
//It's part of the IPv4 127.0.0.0/8 range, where any address from 127.0.0.1 to 127.255.255.255 points back to your own computer.
//Using a loopback address means the data will never leave your machine—it loops back to the source without ever reaching a physical network.

//Buffer Reader
//it is a rust wrapper that buffers the input
//this helps to read data in chunks instead of reading byte by byte which then improves the performance


