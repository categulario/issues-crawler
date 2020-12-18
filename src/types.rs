use serde::Serialize;

#[derive(Serialize)]
pub struct Project {
    pub name: String,
    pub url: String,
    pub watchers: usize,
    pub forks: usize,
    pub stars: usize,
}

pub struct Issue {
    author: String,
    url: String,
    title: String,
    content: String,
}

#[derive(Serialize)]
pub struct Message {
    msg: String,
}

impl Message {
    pub fn new(msg: &str) -> Message {
        Message {
            msg: msg.into(),
        }
    }
}
