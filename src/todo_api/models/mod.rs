use crate::todo_api_web::model::todo::{State, TodoCard};
use actix_web::web;
use serde::{Serialize};
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

// impl Into<HashMap<String, AttributeValue>> for TodoCardDb {
//     fn into(self) -> HashMap<String, AttributeValue> {
//         let mut todo_card = HashMap::new();
//         todo_card.insert("id".to_string(), to_attribute_value(self.id));
//         todo_card.insert("title".to_string(),to_attribute_value(self.title));
//         todo_card.insert("description".to_string(), to_attribute_value(self.description));
//         todo_card.insert("owner".to_string(), to_attribute_value(self.owner.to_string()));
//         todo_card.insert("state".to_string(), to_attribute_value(self.state));
//         todo_card.insert("tasks".to_string(), to_attribute_value(self.tasks));
//         todo_card
//     }
// }

// .item("id", AttributeValue::S(todo_card.id.to_string()))
    // .item("title", AttributeValue::S(todo_card.title))
    // .item("description", AttributeValue::S(todo_card.description))
    // .item("owner", AttributeValue::S(todo_card.owner.to_string()))
    // .item("tasks", to_attribute_value(todo_card.tasks).unwrap())
    // .item("state", to_attribute_value(todo_card.state).unwrap())