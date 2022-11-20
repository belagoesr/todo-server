use std::collections::HashMap;

use actix_web::web;
use aws_sdk_dynamodb::model::AttributeValue;
use uuid::Uuid;

use crate::todo_api_web::model::todo::{State, Task, TodoCard};

use super::model::{StateDb, TaskDb, TodoCardDb};

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

pub fn scanoutput_to_todocards(output: Vec<HashMap<String, AttributeValue>>) -> Vec<TodoCard> {
    output
        .into_iter()
        .map(|item| {
            let id = item.get("id").unwrap().as_s().unwrap();
            let owner = item.get("owner").unwrap().as_s().unwrap();
            let title = item.get("title").unwrap().as_s().unwrap();
            let description = item.get("description").unwrap().as_s().unwrap();
            let state = item.get("state").unwrap().as_s().unwrap();
            let tasks = item.get("tasks").unwrap().as_l().unwrap();

            TodoCard {
                id: Some(uuid::Uuid::parse_str(id).unwrap()),
                owner: uuid::Uuid::parse_str(owner).unwrap(),
                title: title.to_string(),
                description: description.to_string(),
                state: State::from(state),
                tasks: tasks
                    .iter()
                    .map(|t| Task {
                        title: t
                            .as_m()
                            .unwrap()
                            .get("title")
                            .unwrap()
                            .as_s()
                            .unwrap()
                            .to_string(),
                        is_done: *t.as_m().unwrap().get("is_done").unwrap().as_bool().unwrap(),
                    })
                    .collect::<Vec<Task>>(),
            }
        })
        .collect()
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
            id: None,
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
// TODO: conserta
#[cfg(test)]
mod scan_to_cards {
    use aws_sdk_dynamodb::model::AttributeValue;

    use super::scanoutput_to_todocards;
    use crate::todo_api_web::model::todo::{State, Task, TodoCard};

    fn scan_with_one() -> Option<Vec<std::collections::HashMap<String, AttributeValue>>> {
        let mut tasks_hash = std::collections::HashMap::new();
        tasks_hash.insert("title".to_string(), AttributeValue::S("blob".to_string()));
        tasks_hash.insert("is_done".to_string(), AttributeValue::Bool(true));

        let mut hash = std::collections::HashMap::new();
        hash.insert("title".to_string(), AttributeValue::S("title".to_string()));
        hash.insert(
            "description".to_string(),
            AttributeValue::S("description".to_string()),
        );
        hash.insert(
            "owner".to_string(),
            AttributeValue::S("90e700b0-2b9b-4c74-9285-f5fc94764995".to_string()),
        );
        hash.insert(
            "id".to_string(),
            AttributeValue::S("646b670c-bb50-45a4-ba08-3ab684bc4e95".to_string()),
        );
        hash.insert("state".to_string(), AttributeValue::S("Done".to_string()));
        hash.insert(
            "tasks".to_string(),
            AttributeValue::L(vec![AttributeValue::M(tasks_hash)]),
        );

        Some(vec![hash])
    }

    #[test]
    fn scanoutput_has_one_item() {
        let scan = scan_with_one().unwrap();
        let todos = vec![TodoCard {
            title: "title".to_string(),
            description: "description".to_string(),
            state: State::Done,
            id: Some(uuid::Uuid::parse_str("646b670c-bb50-45a4-ba08-3ab684bc4e95").unwrap()),
            owner: uuid::Uuid::parse_str("90e700b0-2b9b-4c74-9285-f5fc94764995").unwrap(),
            tasks: vec![Task {
                is_done: true,
                title: "blob".to_string(),
            }],
        }];

        assert_eq!(scanoutput_to_todocards(scan), todos)
    }
}
