use aws_sdk_dynamodb::model::AttributeValue;
use serde::Serialize;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct TaskDb {
    pub is_done: bool,
    pub title: String,
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
    #[allow(dead_code)]
    pub fn get_id(self) -> Uuid {
        self.id
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
        todo_card.insert("tasks".to_string(), 
            val!(L => self.tasks.into_iter().map(|t| t.to_db_val()).collect::<Vec<AttributeValue>>()));
        todo_card
    }
}

impl TaskDb {
    fn to_db_val(self) -> AttributeValue {
        let mut tasks_hash = HashMap::new();
            tasks_hash.insert("title".to_string(), val!(S => self.title.clone()));
            tasks_hash.insert("is_done".to_string(), val!(B => self.is_done));
            val!(M => tasks_hash)
    }
}
