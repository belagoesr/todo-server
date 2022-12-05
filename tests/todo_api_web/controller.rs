const todo_file: &str = "post_todo.json";
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
    use crate::helpers::read_json;
    use todo_server::todo_api_web::{model::todo::TodoIdResponse, routes::app_routes};

    use actix_web::{
        body,
        http::header::{ContentType, CONTENT_TYPE},
        test, App,
    };
    use serde_json::from_str;

    #[actix_web::test]
    async fn valid_todo_post() {
        // use crate::todo_api::db::helpers::create_table().await;

        let mut app = test::init_service(App::new().configure(app_routes)).await;
        let req = test::TestRequest::post()
            .uri("/api/create")
            .insert_header((CONTENT_TYPE, ContentType::json()))
            .set_payload(read_json(todo_file).as_bytes().to_owned())
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        let body = resp.into_body();
        let bytes = body::to_bytes(body).await.unwrap();
        let id = from_str::<TodoIdResponse>(&String::from_utf8(bytes.to_vec()).unwrap()).unwrap();
        assert!(uuid::Uuid::parse_str(&id.get_id()).is_ok());
    }
}

mod read_all_todos {
    use serde_json::from_str;
    use todo_server::todo_api_web::{model::todo::TodoCardsResponse, routes::app_routes};

    use actix_web::{
        body,
        http::{
            header::{ContentType, CONTENT_TYPE},
            StatusCode,
        },
        test, App,
    };

    use crate::helpers::{mock_get_todos, read_json};

    #[actix_web::test]
    async fn test_todo_index_ok() {
        let mut app = test::init_service(App::new().configure(app_routes)).await;

        let req = test::TestRequest::get().uri("/api/index").to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_todo_cards_count() {
        let mut app = test::init_service(App::new().configure(app_routes)).await;

        let post_req = test::TestRequest::post()
            .uri("/api/create")
            .insert_header((CONTENT_TYPE, ContentType::json()))
            .set_payload(read_json(todo_file).as_bytes().to_owned())
            .to_request();

        let _ = test::call_service(&mut app, post_req).await;
        let get_req = test::TestRequest::get().uri("/api/index").to_request();
        let resp_body = test::call_service(&mut app, get_req).await.into_body();
        let bytes = body::to_bytes(resp_body).await.unwrap();
        let todo_cards =
            from_str::<TodoCardsResponse>(&String::from_utf8(bytes.to_vec()).unwrap()).unwrap();

        assert_eq!(todo_cards.cards.len(), 1);
    }

    #[actix_web::test]
    async fn test_todo_cards_with_value() {
        let mut app = test::init_service(App::new().configure(app_routes)).await;

        let post_req = test::TestRequest::post()
            .uri("/api/create")
            .insert_header((CONTENT_TYPE, ContentType::json()))
            .set_payload(read_json(todo_file).as_bytes().to_owned())
            .to_request();

        let _ = test::call_service(&mut app, post_req).await;
        let req = test::TestRequest::with_uri("/api/index").to_request();
        let resp_body = test::call_service(&mut app, req).await.into_body();
        let bytes = body::to_bytes(resp_body).await.unwrap();
        let todo_cards: TodoCardsResponse =
            from_str(&String::from_utf8(bytes.to_vec()).unwrap()).unwrap();

        assert_eq!(todo_cards.cards, mock_get_todos());
    }
}
