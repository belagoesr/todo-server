mod todo_api_web;
use todo_api_web::controller::{ping, readiness};

use actix_web::{dev::Service, http::StatusCode, test, web, App, HttpResponse, HttpServer};

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

#[actix_web::test]
async fn not_found_route() {
    let mut app = test::init_service(
        App::new()
            .service(readiness)
            .service(ping)
            .default_service(web::to(|| HttpResponse::NotFound())),
    )
    .await;

    let req = test::TestRequest::with_uri("/crazy-path").to_request();

    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
