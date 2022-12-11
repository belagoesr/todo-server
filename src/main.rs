pub mod todo_api;
pub mod todo_api_web;

use todo_server::{todo_api::db::helpers::create_table, todo_api_web::routes::app_routes};

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
//use bastion::prelude::*;
use env_logger;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    //env_logger::init();
    create_table().await;
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new(
                "IP:%a DATETIME:%t REQUEST:\"%r\" STATUS: %s DURATION:%D",
            ))
            .configure(app_routes)
    })
    .workers(num_cpus::get() - 2)
    .max_connections(30000)
    .bind(("0.0.0.0", 4000))
    .unwrap()
    .run()
    .await
}

// #[fort::root(redundancy = 10)]
// async fn main(_: BastionContext) -> Result<(), ()> {
//     std::env::set_var("RUST_LOG", "actix_web=info");
//     env_logger::init();
//     create_table().await;

//     let _ = web_main();

//     // TODO: still throwing error since init() is called more then 1 time
//     Ok(())
// }