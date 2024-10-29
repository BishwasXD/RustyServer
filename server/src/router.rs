use crate::HttpRequest;
use crate::views::{get_tasks_lists,post_task, render_homepage};

pub fn dispatch_request(request: HttpRequest) -> String{

match request.url.as_str() {
    "post-data" => post_task(request),
    "get-data" => get_tasks_lists(),
    _ => render_homepage(),
}
}