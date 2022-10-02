use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    is_done: bool,
    title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum State {
    Todo,
    Doing,
    Done,
}

#[derive(Serialize, Deserialize, Debug)]
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