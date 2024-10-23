use std::fs;

use crate::utils::{get_from_file,render_template, save_to_file, send_response};
use crate::models::Task;
use crate::HttpRequest;

pub fn get_tasks_lists() -> String
    {
        let response: String = get_from_file();
       
       return  send_response(response, 200);
    }

pub fn post_task(request:HttpRequest) -> String {
        let body: String = request.body;
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
    }

pub fn render_homepage() -> String{
    let html_content: String = fs::read_to_string("/home/bishwas/Desktop/RustyServer/server/static/template/todo.html").unwrap();
    return render_template(html_content, 200)
}