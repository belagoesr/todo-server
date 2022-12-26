use crate::todo_api::model::TodoCardDb;
use aws_sdk_dynamodb::Client;

use crate::{todo_api::db::helpers::TODO_CARD_TABLE, todo_api_web::model::todo::TodoCard};
use log::{debug, error};

#[cfg(not(feature = "dynamo"))]
pub async fn put_todo(client: &Client, todo_card: TodoCardDb) -> Option<uuid::Uuid> {
    match client
        .put_item()
        .table_name(TODO_CARD_TABLE.to_string())
        .set_item(Some(todo_card.clone().into()))
        .send()
        .await
    {
        Ok(_) => {
            debug!("item created with id {:?}", todo_card.id);
            Some(todo_card.id)
        }
        Err(e) => {
            error!("error when creating item {:?}", e);
            None
        }
    }
}

#[cfg(feature = "dynamo")]
pub async fn put_todo(_client: &Client, todo_card: TodoCardDb) -> Option<uuid::Uuid> {
    Some(todo_card.id)
}

#[cfg(not(feature = "dynamo"))]
pub async fn get_todos(client: &Client) -> Option<Vec<TodoCard>> {
    use crate::todo_api::adapter;

    let scan_output = client
        .scan()
        .table_name(TODO_CARD_TABLE.to_string())
        .limit(100i32)
        .send()
        .await;

    match scan_output {
        Ok(dbitems) => {
            let res = adapter::scanoutput_to_todocards(dbitems)?.to_vec();
            debug!("Scanned {:?} todo cards", res);
            Some(res)
        }
        Err(e) => {
            error!("Could not scan todocards due to error {:?}", e);
            None
        }
    }
}

#[cfg(feature = "dynamo")]
pub async fn get_todos(_client: &Client) -> Option<Vec<TodoCard>> {
    use crate::todo_api_web::model::todo::{State, Task};

    Some(vec![TodoCard {
        id: Some(uuid::Uuid::parse_str("be75c4d8-5241-4f1c-8e85-ff380c041664").unwrap()),
        title: String::from("This is a card"),
        description: String::from("This is the description of the card"),
        owner: uuid::Uuid::parse_str("ae75c4d8-5241-4f1c-8e85-ff380c041442").unwrap(),
        tasks: vec![
            Task {
                title: String::from("title 1"),
                is_done: true,
            },
            Task {
                title: String::from("title 2"),
                is_done: true,
            },
            Task {
                title: String::from("title 3"),
                is_done: false,
            },
        ],
        state: State::Doing,
    }])
}
