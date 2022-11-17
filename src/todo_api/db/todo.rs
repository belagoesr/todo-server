use std::collections::HashMap;

use crate::todo_api::model::TodoCardDb;
use aws_sdk_dynamodb::{model::AttributeValue, Client};

use crate::{todo_api::db::helpers::TODO_CARD_TABLE, todo_api_web::model::todo::TodoCard};

#[cfg(feature = "dynamo")]
pub async fn put_todo(client: &Client, todo_card: TodoCardDb) -> Option<uuid::Uuid> {
    match client
        .put_item()
        .table_name(TODO_CARD_TABLE.to_string())
        .set_item(Some(todo_card.clone().into()))
        .send()
        .await
    {
        Ok(_) => Some(todo_card.id),
        Err(e) => {
            println!("{:?}", e);
            None
        }
    }
}

#[cfg(not(feature = "dynamo"))]
pub async fn put_todo(_client: &Client, todo_card: TodoCardDb) -> Option<uuid::Uuid> {
    Some(todo_card.id)
}

#[cfg(feature = "dynamo")]
pub async fn get_todos(client: &Client) -> Option<Vec<TodoCard>> {
    println!("starting db call");
    use tokio_stream::StreamExt;

    use crate::todo_api::adapter;

    let items: Result<Vec<HashMap<String, AttributeValue>>, _> = client
        .scan()
        .table_name(TODO_CARD_TABLE.to_string())
        .into_paginator()
        .items()
        .send()
        .collect()
        .await;

    println!("Items in table:");
    for item in items {
        println!("   {:?}", item);
    }

    Some(vec![])
    //     match items {
    //             Ok(_) => Some(adapter::scanoutput_to_todocards()),
    //             Err(_) => None
    //         }
}

#[cfg(not(feature = "dynamo"))]
pub async fn get_todos(client: &Client) -> Option<Vec<TodoCard>> {
    Some(vec![])
}
