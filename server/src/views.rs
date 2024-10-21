use std::{ fs::{ File, OpenOptions }, io::{ Read, Write } };

use crate::{ utils::send_response, HttpRequest };
use serde::Deserialize;

//for handling logic
#[derive(Deserialize, Debug)]
#[derive(serde::Serialize)]
struct Task {
    title: String,
    description: String,
    completed: bool,
    created_at: String,
}

pub fn views(http_request: HttpRequest) -> String {
    //here will have the post and get logic for our todo app
    //views will always returns a response, the response type shall be enums.
    if http_request.method == "POST" {
        let body: String = http_request.body;
        let new_task: Task = serde_json::from_str(&body).unwrap();

        let saved_data: String = get_from_file();
        let mut tasks: Vec<Task> = if saved_data.is_empty() {
            vec![] 
        } else {
            serde_json::from_str(&saved_data).unwrap() 
        
        };


        tasks.push(new_task);
        println!("{tasks:?}");
        save_to_file(&tasks);

        let response: String = String::from("Data saved successfully");

        return send_response(response, 201);
    } else if http_request.method == "GET" {
        let response: String = get_from_file();
        return send_response(response, 200);
    } else {
        return "Only post and get handled for now".to_owned();
    }
}
fn save_to_file(data: &Vec<Task>) {
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

fn get_from_file() -> String {
    let mut file = OpenOptions::new()
        .read(true)
        .write(false)
        .open("/home/bishwas/Desktop/RustyServer/server/todos.json")
        .unwrap();
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();
    return file_contents;
}
