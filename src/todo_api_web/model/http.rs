use actix::Addr;
use aws_sdk_dynamodb::Client;

use crate::todo_api::db::helpers::{db_executor_address, get_client, DbExecutor};

#[derive(Clone, Debug)]
pub struct Clients {
    pub dynamo: Client,
    pub postgres: Addr<DbExecutor>,
}
impl Clients {
    pub async fn new() -> Self {
        Self {
            dynamo: get_client().await,
            postgres: db_executor_address(),
        }
    }
}
