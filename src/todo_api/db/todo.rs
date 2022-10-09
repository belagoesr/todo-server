use actix_web::{HttpResponse, web, Responder};
use crate::todo_api_web::model::todo::{TodoCard, TodoIdResponse};
use crate::todo_api::models::{TodoCardDb};

pub async fn create_todo(info: web::Json<TodoCard>) -> impl Responder {
    let todo_card = TodoCardDb::new(info);
    let client = get_client().await;
    match put_todo(&client, todo_card).await {
        None => HttpResponse::BadRequest().body("Failed to create todo card"),
        Some(id) => HttpResponse::Created()
            .content_type("application/json")
            .body(serde_json::to_string(&TodoIdResponse::new(id)).expect("Failed to serialize todo card"))
    }
}

/// A partir daqui vamos extrair logo mais
use aws_sdk_dynamodb::{Client};
use serde_dynamo::{to_item};
use crate::{
    todo_api::db::helpers::{TODO_CARD_TABLE},
};

use super::helpers::get_client;

pub async fn put_todo(client: &Client, todo_card: TodoCardDb) ->  Option<uuid::Uuid> {
    let item = to_item(todo_card.clone()).unwrap();
    match client.put_item()
    .table_name(TODO_CARD_TABLE.to_string())
    .set_item(Some(item))
    .send()
    .await {
        Ok(_) => {
            Some(todo_card.id)
        },
        Err(_) => {
            None
        }
    }
}