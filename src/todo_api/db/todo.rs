use crate::todo_api::model::TodoCardDb;
use aws_sdk_dynamodb::Client;

#[cfg(not(feature = "dynamo"))]
pub async fn put_todo(client: &Client, todo_card: TodoCardDb) -> Option<uuid::Uuid> {
    use crate::todo_api::db::helpers::TODO_CARD_TABLE;

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

#[cfg(feature = "dynamo")]
pub async fn put_todo(_client: &Client, todo_card: TodoCardDb) -> Option<uuid::Uuid> {
    Some(todo_card.id)
}
