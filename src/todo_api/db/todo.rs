use crate::todo_api::{db::helpers::TODO_CARD_TABLE, model::TodoCardDb};
use aws_sdk_dynamodb::Client;

pub async fn put_todo(client: &Client, todo_card: TodoCardDb) -> Option<uuid::Uuid> {
    match client
        .put_item()
        .table_name(TODO_CARD_TABLE.to_string())
        .set_item(Some(todo_card.clone().into()))
        .send()
        .await
    {
        Ok(_) => Some(todo_card.id),
        Err(_) => None,
    }
}
