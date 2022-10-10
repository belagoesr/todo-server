use crate::todo_api::adapter;
use crate::todo_api::db:: {helpers::get_client, todo::put_todo};
use crate::todo_api_web::model::todo::{TodoCard, TodoIdResponse};
use actix_web::{http::header::ContentType, post, web, HttpResponse, Responder};

#[post("/api/create")]
pub async fn create_todo(info: web::Json<TodoCard>) -> impl Responder {
    let todo_card = adapter::todo_json_to_db(info);
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