pub mod auth;

use actix_web::web;
use aws_sdk_dynamodb::output::ScanOutput;
use uuid::Uuid;

use super::model::{StateDb, TaskDb, TodoCardDb};
use crate::todo_api_web::model::todo::{State, Task, TodoCard};

#[macro_export]
macro_rules! val {
    (B => $bval:expr) => {{
        aws_sdk_dynamodb::model::AttributeValue::Bool($bval)
    }};
    (L => $val:expr) => {{
        aws_sdk_dynamodb::model::AttributeValue::L($val)
    }};
    (S => $val:expr) => {{
        aws_sdk_dynamodb::model::AttributeValue::S($val)
    }};
    (M => $val:expr) => {{
        aws_sdk_dynamodb::model::AttributeValue::M($val)
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

pub fn scanoutput_to_todocards(output: ScanOutput) -> Option<Vec<TodoCard>> {
    Some(
        output
            .items()?
            .into_iter()
            .filter_map(|item| {
                let id = item.get("id")?.as_s().ok();
                let owner = item.get("owner")?.as_s().ok();
                let title = item.get("title")?.as_s().ok();
                let description = item.get("description")?.as_s().ok();
                let state = item.get("state")?.as_s().ok();
                let tasks = item.get("tasks")?.as_l().ok();

                Some(TodoCard {
                    id: uuid::Uuid::parse_str(id?).ok(),
                    owner: uuid::Uuid::parse_str(owner?).ok()?,
                    title: title?.to_string(),
                    description: description?.to_string(),
                    state: State::from(state?),
                    tasks: tasks?
                        .iter()
                        .filter_map(|t| {
                            let is_done = *t.as_m().ok()?.get("is_done")?.as_bool().ok()?;
                            Some(Task {
                                title: t.as_m().ok()?.get("title")?.as_s().ok()?.to_string(),
                                is_done,
                            })
                        })
                        .collect::<Vec<Task>>(),
                })
            })
            .collect(),
    )
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
        let actual: HashMap<String, aws_sdk_dynamodb::model::AttributeValue> = TodoCardDb {
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

#[cfg(test)]
mod scan_to_cards {
    use std::{collections::HashMap, vec};

    use aws_sdk_dynamodb::{model::AttributeValue, output::ScanOutput};

    use super::scanoutput_to_todocards;
    use crate::todo_api_web::model::todo::{State, Task, TodoCard};

    fn attr_values() -> HashMap<String, AttributeValue> {
        let tasks = vec![
            ("is_done".to_string(), AttributeValue::Bool(true)),
            ("title".to_string(), AttributeValue::S("blob".to_string())),
        ];
        let tasks_hash = HashMap::<String, AttributeValue>::from_iter(tasks);

        let values = vec![
            ("title".to_string(), AttributeValue::S("title".to_string())),
            (
                "description".to_string(),
                AttributeValue::S("description".to_string()),
            ),
            (
                "owner".to_string(),
                AttributeValue::S("90e700b0-2b9b-4c74-9285-f5fc94764995".to_string()),
            ),
            (
                "id".to_string(),
                AttributeValue::S("646b670c-bb50-45a4-ba08-3ab684bc4e95".to_string()),
            ),
            ("state".to_string(), AttributeValue::S("Done".to_string())),
            (
                "tasks".to_string(),
                AttributeValue::L(vec![AttributeValue::M(tasks_hash)]),
            ),
        ];
        let hash = HashMap::<String, AttributeValue>::from_iter(values);
        hash
    }

    fn scan_with_one() -> ScanOutput {
        let hash = attr_values();

        let mut output = ScanOutput::builder().build();
        output.consumed_capacity = None;
        output.count = 1;
        output.items = Some(vec![hash]);
        output.scanned_count = 1;
        output.last_evaluated_key = None;

        output
    }

    fn scan_with_two() -> ScanOutput {
        let hash = attr_values();
        let mut output = ScanOutput::builder().build();

        output.consumed_capacity = None;
        output.count = 2;
        output.items = Some(vec![hash.clone(), hash]);
        output.scanned_count = 2;
        output.last_evaluated_key = None;

        output
    }

    #[test]
    fn scanoutput_has_one_item() {
        let scan = scan_with_one();
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

        assert_eq!(scanoutput_to_todocards(scan).unwrap(), todos)
    }

    #[test]
    fn scanoutput_has_two_items() {
        let scan = scan_with_two();
        let todo = TodoCard {
            title: "title".to_string(),
            description: "description".to_string(),
            state: State::Done,
            id: Some(uuid::Uuid::parse_str("646b670c-bb50-45a4-ba08-3ab684bc4e95").unwrap()),
            owner: uuid::Uuid::parse_str("90e700b0-2b9b-4c74-9285-f5fc94764995").unwrap(),
            tasks: vec![Task {
                is_done: true,
                title: "blob".to_string(),
            }],
        };
        let todos = vec![todo.clone(), todo];

        assert_eq!(scanoutput_to_todocards(scan).unwrap(), todos)
    }
}
