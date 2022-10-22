mod todo_api;
mod todo_api_web;
use todo_api_web::routes::app_routes;

use actix_web::{App, HttpServer};
use num_cpus;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(app_routes))
        .workers(num_cpus::get() + 2)
        .bind(("localhost", 4004))
        .unwrap()
        .run()
        .await
}

// #[actix_web::main]
// async fn main() {
//  use todo_api::db::helpers::{create_table, get_client, list_items};
//     // create_table().await;
//     list_items().await
// }
