use std::{ io::{ prelude::*, BufReader }, net::{ TcpListener, TcpStream } };
pub mod utils;
pub mod views;

pub struct HttpRequest {
    pub method: String,
    pub body: String,
    pub content_length: usize,
    pub bearer_token: String,
    pub content_type: String,
}
fn main() {
    const HOST: &str = "127.0.0.0";
    const PORT: &str = "8477";

    let end_point: String = HOST.to_owned() + ":" + PORT ; //listening address

    let listener: TcpListener = TcpListener::bind(end_point).unwrap();
    println!("Server runnning on port {PORT}");
    for stream in listener.incoming() {
        let stream: std::net::TcpStream = stream.unwrap();

        handle_connection(stream);
    }
}
fn handle_connection(mut stream: TcpStream) {
    println!("handling connection...");

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
    let mut type_of_request: String = String::from("GET"); //default

    if let Some(Ok(line)) = buffer.by_ref().lines().next() {
        type_of_request = utils::request_type(line);
    } else {
        println!("Error reading request method");
    }

    let mut content_length: usize = 0;
    for line in buffer.by_ref().lines() {
        let line = line.expect("Failed to read a line");
        println!("{line}");
        if line.starts_with("Content-Length") {
            content_length = utils::get_content_length(line);
            break;
        }
    }
    let request_body = utils::get_req_body(&mut buffer, content_length);

    let http_request: HttpRequest = HttpRequest {
        method: type_of_request,
        body: request_body,
        content_length: content_length,
        bearer_token: String::from(""),
        content_type: String::from(""),
    };
    let response = views::views(http_request);
    stream.write_all(response.as_bytes()).unwrap();
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
