pub mod todo_api;
pub mod todo_api_web;

#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;

mod schema;

use todo_server::{
    todo_api::db::helpers::create_table,
    todo_api_web::{model::http::Clients, routes::app_routes},
};

use actix_web::middleware::{DefaultHeaders, Logger};
use actix_web::{web, App, HttpServer};
use env_logger;
use uuid::Uuid;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let client = web::Data::new(Clients::new().await);
    create_table(&client.dynamo.clone()).await;

    HttpServer::new(move|| {
        App::new()
            .app_data(client.clone())
            .wrap(DefaultHeaders::new().add(("x-request-id", Uuid::new_v4().to_string())))
            .wrap(Logger::new("IP:%a DATETIME:%t REQUEST:\"%r\" STATUS: %s DURATION:%D X-REQUEST-ID:%{x-request-id}o"))
            .configure(app_routes)
    })
    .workers(num_cpus::get() - 2)
    .max_connections(30000)
    .bind(("0.0.0.0", 4000))
    .unwrap()
    .run()
    .await
}
