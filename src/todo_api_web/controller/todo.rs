use crate::todo_api::adapter;

use crate::todo_api::db::helpers::get_client;
use crate::todo_api::db::todo::{get_todos, put_todo};
use crate::todo_api_web::model::todo::{TodoCard, TodoCardsResponse, TodoIdResponse};

use actix_web::get;
use actix_web::{http::header::ContentType, post, web, HttpResponse, Responder};
use uuid::Uuid;

#[post("/api/create")]
pub async fn create_todo(info: web::Json<TodoCard>) -> impl Responder {
    let id = Uuid::new_v4();
    let todo_card = adapter::todo_json_to_db(info, id);
    let client = get_client().await;
    match put_todo(&client, todo_card).await {
        None => HttpResponse::BadRequest().body("Failed to create todo card"),
        Some(id) => HttpResponse::Created()
            .content_type(ContentType::json())
            .body(
                serde_json::to_string(&TodoIdResponse::new(id))
                    .expect("Failed to serialize todo card"),
            ),
    }
}

#[get("/api/index")]
pub async fn show_all_todo() -> impl Responder {
    let client = get_client().await;
    let resp = get_todos(&client).await;
    match resp {
        None => HttpResponse::InternalServerError().body("Failed to read todo cards"),
        Some(todos) => HttpResponse::Ok().content_type("application/json").body(
            serde_json::to_string(&TodoCardsResponse { cards: todos })
                .expect("Failed to serialize todo cards"),
        ),
    }
}
