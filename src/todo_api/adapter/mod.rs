use actix_web::web;
use aws_sdk_dynamodb::model::AttributeValue;
use uuid::Uuid;

use super::model::{StateDb, TaskDb, TodoCardDb};
use crate::todo_api_web::model::todo::{State, TodoCard};

#[macro_export]
macro_rules! val {
    (B => $bval:expr) => {{
        AttributeValue::Bool($bval)
    }};
    (L => $val:expr) => {{
        AttributeValue::L($val)
    }};
    (S => $val:expr) => {{
        AttributeValue::S($val)
    }};
    (M => $val:expr) => {{
        AttributeValue::M($val)
    }};
}

pub fn todo_json_to_db(card: web::Json<TodoCard>, id: Uuid) -> TodoCardDb {
    TodoCardDb {
        id,
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

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;
    use crate::{
        todo_api::model::{StateDb, TaskDb, TodoCardDb},
        todo_api_web::model::todo::{State, Task, TodoCard},
    };
    use actix_web::web::Json;

    #[test]
    fn converts_json_to_db() {
        let id = uuid::Uuid::new_v4();
        let owner = uuid::Uuid::new_v4();
        let json = Json(TodoCard {
            title: "title".to_string(),
            description: "description".to_string(),
            owner: owner,
            state: State::Done,
            tasks: vec![Task {
                is_done: true,
                title: "title".to_string(),
            }],
        });
        let expected = TodoCardDb {
            id: id,
            title: "title".to_string(),
            description: "description".to_string(),
            owner: owner,
            state: StateDb::Done,
            tasks: vec![TaskDb {
                is_done: true,
                title: "title".to_string(),
            }],
        };
        assert_eq!(todo_json_to_db(json, id), expected);
    }

    #[test]
    fn task_db_to_db_val() {
        let actual = TaskDb {
            title: "blob".to_string(),
            is_done: true,
        }
        .to_db_val();
        let mut tasks_hash = HashMap::new();
        tasks_hash.insert("title".to_string(), val!(S => "blob".to_string()));
        tasks_hash.insert("is_done".to_string(), val!(B => true));
        let expected = val!(M => tasks_hash);
        assert_eq!(actual, expected);
    }

    #[test]
    fn todo_card_db_to_db_val() {
        let id = uuid::Uuid::new_v4();
        let actual: HashMap<String, AttributeValue> = TodoCardDb {
            id: id,
            title: "title".to_string(),
            description: "description".to_string(),
            owner: id,
            state: StateDb::Done,
            tasks: vec![TaskDb {
                is_done: true,
                title: "title".to_string(),
            }],
        }
        .into();
        let mut expected = HashMap::new();
        expected.insert("id".to_string(), val!(S => id.to_string()));
        expected.insert("title".to_string(), val!(S => "title".to_string()));
        expected.insert(
            "description".to_string(),
            val!(S => "description".to_string()),
        );
        expected.insert("owner".to_string(), val!(S => id.to_string()));
        expected.insert("state".to_string(), val!(S => StateDb::Done.to_string()));
        expected.insert(
            "tasks".to_string(),
            val!(L => vec![TaskDb {is_done: true, title: "title".to_string()}.to_db_val()]),
        );
        assert_eq!(actual, expected);
    }
}
