use actix_web::{dev::Service, http::StatusCode, test, web, App, HttpResponse};
use todo_server::todo_api_web::controller::{readiness, ping};

mod todo_api_web;

#[actix_web::test]
async fn not_found_route() {
    let app = test::init_service(
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
