use aws_sdk_dynamodb::Client;

use crate::todo_api::db::helpers::get_client;

#[derive(Clone)]
pub struct Clients {
    pub dynamo: Client,
}

impl Clients {
    pub async fn new() -> Self {
        Self {
            dynamo: get_client().await,
        }
    }
}
