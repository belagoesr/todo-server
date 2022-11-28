use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Task {
    pub is_done: bool,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum State {
    Todo,
    Doing,
    Done,
}
impl From<&String> for State {
    fn from(s: &String) -> Self {
        match s.as_str() {
            "Todo" | "todo" => State::Todo,
            "Doing" | "doing" => State::Doing,
            "Done" | "done" => State::Done,
            _ => State::Todo,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TodoCard {
    pub id: Option<Uuid>,
    pub title: String,
    pub description: String,
    pub owner: Uuid,
    pub tasks: Vec<Task>,
    pub state: State,
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

#[derive(Serialize, Deserialize, PartialEq)]
pub struct TodoCardsResponse {
    pub cards: Vec<TodoCard>,
}
