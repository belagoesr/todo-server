mod todo_api_web;
use todo_api_web::controller::{ping, readiness};

use actix_web::{web, App, HttpResponse, HttpServer};

use num_cpus;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(readiness)
            .service(ping)
            .default_service(web::to(|| HttpResponse::NotFound()))
    })
    .workers(num_cpus::get() + 2)
    .bind(("localhost", 4004))
    .unwrap()
    .run()
    .await
}
