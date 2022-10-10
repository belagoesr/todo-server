use crate::todo_api_web::model::todo::{State, TodoCard};
use actix_web::web;
use aws_sdk_dynamodb::model::AttributeValue;
use serde::Serialize;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct TaskDb {
    is_done: bool,
    title: String,
}

#[derive(Debug, Clone, Serialize)]
pub enum StateDb {
    Todo,
    Doing,
    Done,
}

#[derive(Debug, Clone, Serialize)]
pub struct TodoCardDb {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub owner: Uuid,
    pub tasks: Vec<TaskDb>,
    pub state: StateDb,
}

impl TodoCardDb {
    pub fn new(card: web::Json<TodoCard>) -> Self {
        TodoCardDb {
            id: Uuid::new_v4(),
            title: card.title.clone(),
            description: card.description.clone(),
            owner: card.owner,
            tasks: card
                .tasks
                .iter()
                .map(|t| TaskDb {
                    is_done: t.is_done,
                    title: t.title.clone(),
                })
                .collect(),
            state: match card.state {
                State::Todo => StateDb::Todo,
                State::Doing => StateDb::Doing,
                State::Done => StateDb::Done,
            },
        }
    }
}

impl std::fmt::Display for StateDb {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Into<HashMap<String, AttributeValue>> for TodoCardDb {
    fn into(self) -> HashMap<String, AttributeValue> {
        let mut todo_card = HashMap::new();
        todo_card.insert("id".to_string(), val!(S => self.id.to_string()));
        todo_card.insert("title".to_string(), val!(S => self.title));
        todo_card.insert("description".to_string(), val!(S => self.description));
        todo_card.insert("owner".to_string(), val!(S => self.owner.to_string()));
        todo_card.insert("state".to_string(), val!(S => self.state.to_string()));
        todo_card.insert("tasks".to_string(), val!(L => task_to_db_val(self.tasks)));
        todo_card
    }
}

fn task_to_db_val(tasks: Vec<TaskDb>) -> Vec<AttributeValue> {
    tasks
        .iter()
        .map(|t| {
            let mut tasks_hash = HashMap::new();
            tasks_hash.insert("title".to_string(), val!(S => t.title.clone()));
            tasks_hash.insert("is_done".to_string(), val!(B => t.is_done));
            val!(M => tasks_hash)
        })
        .collect::<Vec<AttributeValue>>()
}
