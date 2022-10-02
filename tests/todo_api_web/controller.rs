mod ping_readiness {
    use todo_server::todo_api_web::routes::app_routes;

    use actix_web::{body, http::StatusCode, test, web, App};

    #[actix_web::test]
    async fn test_ping_pong() {
        let mut app = test::init_service(App::new().configure(app_routes)).await;

        let req = test::TestRequest::get().uri("/ping").to_request();
        let resp = test::call_service(&mut app, req).await;
        let body = resp.into_body();
        let bytes = body::to_bytes(body).await.unwrap();

        assert_eq!(bytes, web::Bytes::from_static(b"pong"));
    }

    #[actix_web::test]
    async fn test_readiness() {
        let mut app = test::init_service(App::new().configure(app_routes)).await;
        let req = test::TestRequest::get().uri("/~/ready").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::ACCEPTED);
    }
}

mod create_todo {
    use todo_server::todo_api_web::{model::todo::TodoIdResponse, routes::app_routes};

    use actix_web::{body, http::header::CONTENT_TYPE, test, App};
    use serde_json::from_str;

    fn post_todo() -> String {
        String::from(
            "{
                \"title\": \"This is a card\",
                \"description\": \"This is the description of the card\",
                \"owner\": \"ae75c4d8-5241-4f1c-8e85-ff380c041442\",
                \"tasks\": [
                    {
                        \"title\": \"title 1\",
                        \"is_done\": true
                    },
                    {
                        \"title\": \"title 2\",
                        \"is_done\": true
                    },
                    {
                        \"title\": \"title 3\",
                        \"is_done\": false
                    }
                ],
                \"state\": \"Doing\"
            }",
        )
    }

    #[actix_web::test]
    async fn valid_todo_post() {
        let mut app = test::init_service(App::new().configure(app_routes)).await;

        let req = test::TestRequest::post()
            .uri("/api/create")
            .insert_header((CONTENT_TYPE, "application/json"))
            .set_payload(post_todo().as_bytes().to_owned())
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        let body = resp.into_body();
        let bytes = body::to_bytes(body).await.unwrap();
        let id = from_str::<TodoIdResponse>(&String::from_utf8(bytes.to_vec()).unwrap()).unwrap();
        assert!(uuid::Uuid::parse_str(&id.get_id()).is_ok());
    }
}
