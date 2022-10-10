use actix_web::web;
use uuid::Uuid;

use crate::todo_api_web::model::todo::{TodoCard, State};

use super::model::{TodoCardDb, TaskDb, StateDb};

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


pub fn todo_json_to_db(card: web::Json<TodoCard>) -> TodoCardDb {
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
