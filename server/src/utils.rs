
use std::{ io:: BufReader , net::TcpStream };
use std::{ fs::OpenOptions , io::{ Read, Write } };
use crate::models::Task;
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

pub fn parse_request(request_line: String) -> Vec<String> {
 
    let parts: Vec<&str> = request_line.split('/').collect();
    println!("{parts:?}");
    let method_with_url = parts.get(1).expect("Error: No URL found in the request!");   
    let url_parts: Vec<&str> = method_with_url.split_whitespace().collect();
    println!("{url_parts:?}");
    let url = url_parts.get(0).expect("Error: Unable to extract the URL!").to_string();
    let method = parts.get(0).expect("Error: HTTP method not found!").to_string();
    vec![method, url]
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

pub fn render_template(response_body:String, status_code: usize)->String{

    let status_message = match status_code {
        200 => "OK",
        201 => "Created",
        400 => "Bad Request",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Unknown Status",
    };

    let status_line: String = format!("HTTP/1.1 {} {}\r\n", status_code, status_message);
    let content_type: &str = "Content-Type: text/html\r\n";
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

pub fn save_to_file(data: &Vec<Task>) {
    //to append to a exisiting file, we will have to use OpenOption, this gives us the ability to configure how file is opened and what type of operations are allowed.
    let json_data = serde_json::to_string_pretty(data).unwrap();
    let mut file = OpenOptions::new()
        .read(true)
        .truncate(true)
        .write(true)
        .open("/home/bishwas/Desktop/RustyServer/server/todos.json")
        .unwrap();
    file.write_all(json_data.as_bytes()).unwrap();
}

pub fn get_from_file() -> String {
    let mut file = OpenOptions::new()
        .read(true)
        .write(false)
        .open("/home/bishwas/Desktop/RustyServer/server/todos.json")
        .unwrap();
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();
    return file_contents;
}
