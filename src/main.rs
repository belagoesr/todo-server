pub mod todo_api;
pub mod todo_api_web;

use actix_web::{web, App, HttpResponse, HttpServer};
use todo_api_web::controller::{ping, readiness};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(readiness)
            .service(ping)
            .default_service(web::to(|| HttpResponse::NotFound()))
    })
    .workers(6)
    .bind(("localhost", 4004))
    .unwrap()
    .run()
    .await
}
