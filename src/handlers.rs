use rocket::get;
use rocket_contrib::json::Json;

use crate::types::Project;
use crate::types::Message;

#[get("/")]
pub fn root() -> Json<Message> {
    Json(Message::new("All izz well"))
}

#[get("/projects")]
pub fn get_projects() -> Json<Vec<Project>> {
    Json(vec![Project {
        url: "https://github.com/foo/bar".to_string(),
        name: "Bar Project".to_string(),
        stars: 0,
        watchers: 0,
        forks: 0,
    }])
}

// #[post("/projects")]
// pub fn create_project() -> Project {}
