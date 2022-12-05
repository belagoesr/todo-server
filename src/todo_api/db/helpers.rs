use actix_web::http::Uri;
use aws_sdk_dynamodb::{
    model::{
        AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
    },
    Client, Endpoint,
};

pub static TODO_CARD_TABLE: &str = "TODO_CARDS";

pub static TODO_FILE: &str = "post_todo.json";

pub async fn get_client() -> Client {
    let config = aws_config::load_from_env().await;
    let dynamodb_local_config = aws_sdk_dynamodb::config::Builder::from(&config)
        .endpoint_resolver(Endpoint::immutable(Uri::from_static(
            "http://localhost:8000",
        )))
        .build();

    Client::from_conf(dynamodb_local_config)
}

pub async fn create_table() {
    let client = get_client().await;
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
            println!("Output: {:?}", output);
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}

use tokio_stream::StreamExt;
pub async fn list_items() {
    let client = get_client().await;
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
