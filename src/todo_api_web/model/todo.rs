use serde::{Serialize, Deserialize};
use uuid::Uuid;
pub struct Task {
    is_done: bool,
    title: String,
}

pub enum State {
    Todo,
    Doing,
    Done,
}

pub struct TodoCard {
    title: String,
    description: String,
    owner: Uuid,
    tasks: Vec<Task>,
    state: State,
}

pub struct TodoCardId {
    id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct TodoIdResponse {
    id: Uuid,
}

impl TodoIdResponse {
    pub fn new(id: Uuid) -> Self {
        TodoIdResponse { id: id }
    }
    
    #[allow(dead_code)]
    pub fn get_id(self) -> String {
        format!("{}", self.id)
    }
}