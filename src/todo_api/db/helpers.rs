use std::env;

use actix::{Actor, Addr, SyncArbiter, SyncContext};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::Connection;
use diesel_migrations::run_pending_migrations;

use actix_web::web;
use aws_sdk_dynamodb::{
    model::{
        AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
    },
    Client, Endpoint,
};
use log::{debug, error};

pub static TODO_CARD_TABLE: &str = "TODO_CARDS";
pub static TODO_FILE: &str = "post_todo.json";
pub static ERROR_SERIALIZE: &str = "Failed to serialize todo cards";
pub static ERROR_CREATE: &str = "Failed to create todo card";
pub static ERROR_READ: &str = "Failed to read todo card";

pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub fn db_executor_address() -> Addr<DbExecutor> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    SyncArbiter::start(4, move || DbExecutor(pool.clone()))
}

pub async fn get_client() -> Client {
    let config = aws_config::load_from_env().await;

    let addr = if let Ok(db_endpoint) = std::env::var("DYNAMODB_ENDPOINT") {
        format!("http://{}:8000", db_endpoint)
    } else {
        "http://0.0.0.0:8000".to_string()
    };

    let dynamodb_local_config = aws_sdk_dynamodb::config::Builder::from(&config)
        .endpoint_resolver(Endpoint::immutable(addr.parse().expect("Invalid URI")))
        .build();
    Client::from_conf(dynamodb_local_config)
}

fn run_migrations() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pg_conn = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
    match run_pending_migrations(&pg_conn) {
        Ok(_) => debug!("auth database created"),
        Err(_) => error!("auth database creation failed"),
    };
}

pub async fn create_table(client: &Client) {
    run_migrations();
    match client.list_tables().send().await {
        Ok(list) => {
            match list.table_names {
                Some(table_vec) => {
                    if table_vec.len() > 0 {
                        println!("Error: {:?}", "Table already exists");
                    } else {
                        create_table_input(&client).await
                    }
                }
                None => create_table_input(&client).await,
            };
        }
        Err(_) => {
            create_table_input(&client).await;
        }
    }
}

fn build_key_schema() -> KeySchemaElement {
    KeySchemaElement::builder()
        .attribute_name("id")
        .key_type(KeyType::Hash)
        .build()
}

fn build_provisioned_throughput() -> ProvisionedThroughput {
    ProvisionedThroughput::builder()
        .read_capacity_units(1)
        .write_capacity_units(1)
        .build()
}

fn build_attribute_definition() -> AttributeDefinition {
    AttributeDefinition::builder()
        .attribute_name("id")
        .attribute_type(ScalarAttributeType::S)
        .build()
}

async fn create_table_input(client: &Client) {
    let table_name = TODO_CARD_TABLE.to_string();
    let ad = build_attribute_definition();
    let ks = build_key_schema();
    let pt = build_provisioned_throughput();

    match client
        .create_table()
        .table_name(table_name)
        .key_schema(ks)
        .attribute_definitions(ad)
        .provisioned_throughput(pt)
        .send()
        .await
    {
        Ok(output) => {
            debug!("Table created {:?}", output);
        }
        Err(error) => {
            error!("Could not create table due to error: {:?}", error);
        }
    }
}

use tokio_stream::StreamExt;

use crate::todo_api_web::model::http::Clients;
pub async fn list_items(state: web::Data<Clients>) {
    let client = state.dynamo.clone();
    let items = client
        .scan()
        .table_name(TODO_CARD_TABLE.to_string())
        .into_paginator()
        .items()
        .send()
        .collect::<Result<Vec<_>, _>>()
        .await;

    println!("Items in table:");
    for item in items {
        println!("   {:?}", item);
    }
    ()
}
