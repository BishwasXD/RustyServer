use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[derive(serde::Serialize)]
pub struct Task {
    pub title: String,
    pub description: String,
    pub completed: bool,
 
    pub created_at: String,
}

