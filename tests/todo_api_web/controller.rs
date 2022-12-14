mod ping_readiness {
    use actix_web::{body, http::StatusCode, test, web, App};
    use todo_server::todo_api_web::{model::http::Clients, routes::app_routes};

    #[actix_web::test]
    async fn test_ping_pong() {
        let client = web::Data::new(Clients::new().await);
        let mut app =
            test::init_service(App::new().app_data(client.clone()).configure(app_routes)).await;

        let req = test::TestRequest::get().uri("/ping").to_request();
        let resp = test::call_service(&mut app, req).await;
        let body = resp.into_body();
        let bytes = body::to_bytes(body).await.unwrap();

        assert_eq!(bytes, web::Bytes::from_static(b"pong"));
    }

    #[actix_web::test]
    async fn test_readiness() {
        let client = web::Data::new(Clients::new().await);
        let mut app =
            test::init_service(App::new().app_data(client.clone()).configure(app_routes)).await;
        let req = test::TestRequest::get().uri("/~/ready").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::ACCEPTED);
    }
}

mod create_todo {
    use crate::helpers::read_json;
    use todo_server::{
        todo_api::db::helpers::TODO_FILE,
        todo_api_web::{model::http::Clients, model::todo::TodoIdResponse, routes::app_routes},
    };

    use actix_web::{
        body,
        http::header::{ContentType, CONTENT_TYPE},
        test, web, App,
    };
    use serde_json::from_str;

    #[actix_web::test]
    async fn valid_todo_post() {
        let client = web::Data::new(Clients::new().await);
        let mut app =
            test::init_service(App::new().app_data(client.clone()).configure(app_routes)).await;
        let req = test::TestRequest::post()
            .uri("/api/create")
            .insert_header((CONTENT_TYPE, ContentType::json()))
            .set_payload(read_json(TODO_FILE).as_bytes().to_owned())
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
    use todo_server::todo_api::db::helpers::TODO_FILE;
    use todo_server::todo_api_web::{
        model::http::Clients, model::todo::TodoCardsResponse, routes::app_routes,
    };

    use actix_web::{
        body,
        http::{
            header::{ContentType, CONTENT_TYPE},
            StatusCode,
        },
        test, web, App,
    };

    use crate::helpers::{mock_get_todos, read_json};

    #[actix_web::test]
    async fn test_todo_index_ok() {
        let client = web::Data::new(Clients::new().await);
        let mut app =
            test::init_service(App::new().app_data(client.clone()).configure(app_routes)).await;

        let req = test::TestRequest::get().uri("/api/index").to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_todo_cards_count() {
        let client = web::Data::new(Clients::new().await);
        let mut app =
            test::init_service(App::new().app_data(client.clone()).configure(app_routes)).await;

        let post_req = test::TestRequest::post()
            .uri("/api/create")
            .insert_header((CONTENT_TYPE, ContentType::json()))
            .set_payload(read_json(TODO_FILE).as_bytes().to_owned())
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
        let client = web::Data::new(Clients::new().await);
        let mut app =
            test::init_service(App::new().app_data(client.clone()).configure(app_routes)).await;

        let post_req = test::TestRequest::post()
            .uri("/api/create")
            .insert_header((CONTENT_TYPE, ContentType::json()))
            .set_payload(read_json(TODO_FILE).as_bytes().to_owned())
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

mod auth {
    use crate::helpers::read_json;
    use actix_service::Service;
    use actix_web::{
        http::{
            header::{ContentType, CONTENT_TYPE},
            StatusCode,
        },
        test, App,
    };

    use dotenv::dotenv;
    use todo_server::todo_api_web::model::http::Clients;
    use todo_server::todo_api_web::routes::app_routes;

    // ...
    #[actix_rt::test]
    async fn login_returns_token() {
        let mut app =
            test::init_service(App::new().app_data(Clients::new()).configure(app_routes)).await;

        let login_req = test::TestRequest::post()
            .uri("/auth/login")
            .insert_header((CONTENT_TYPE, ContentType::json()))
            .set_payload(read_json("signup.json").as_bytes().to_owned())
            .to_request();

        let resp_body = test::call_and_read_body(&mut app, login_req).await;

        let jwt: String = String::from_utf8(resp_body.to_vec()).unwrap();

        assert!(jwt.contains("token"));
    }

    #[actix_rt::test]
    async fn signup_returns_created_status() {
        dotenv().ok();
        let app =
            test::init_service(App::new().app_data(Clients::new()).configure(app_routes)).await;

        let signup_req = test::TestRequest::post()
            .uri("/auth/signup")
            .insert_header((CONTENT_TYPE, ContentType::json()))
            .set_payload(read_json("signup.json").as_bytes().to_owned())
            .to_request();

        let resp = app.call(signup_req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[actix_rt::test]
    async fn logout_accepted() {
        dotenv().ok();
        let mut app =
            test::init_service(App::new().data(Clients::new()).configure(app_routes)).await;

        let logout_req = test::TestRequest::delete()
            .uri("/auth/logout")
            .insert_header((CONTENT_TYPE, ContentType::json()))
            .insert_header(("x-auth", "token"))
            .set_payload(read_json("logout.json").as_bytes().to_owned())
            .to_request();

        let resp = test::call_service(&mut app, logout_req).await;
        assert_eq!(resp.status(), StatusCode::ACCEPTED);
    }
}
