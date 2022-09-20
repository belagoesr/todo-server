#[cfg(test)]
mod ping_readiness {
    use todo_server::todo_api_web::controller::{ping, readiness};

    use actix_web::{test, App, http::StatusCode};

    #[actix_web::test]
    async fn test_ping_pong() {
        let mut app = test::init_service(App::new().service(ping)).await;

        let req = test::TestRequest::get().uri("/ping").to_request();
        let resp = test::call_service(&mut app, req).await;
        let result = test::read_body(resp).await;
        assert_eq!(std::str::from_utf8(&result).unwrap(), "pong");
    }

    #[actix_web::test]
    async fn test_readiness() {
        let mut app = test::init_service(App::new().service(readiness)).await;

        let req = test::TestRequest::get().uri("/ready").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::ACCEPTED);
    }
}
