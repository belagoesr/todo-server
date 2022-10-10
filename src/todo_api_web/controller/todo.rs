use crate::todo_api::db::todo::create_todo as put_todo;
use crate::todo_api_web::model::todo::{TodoCard, TodoIdResponse};
use actix_web::{http::header::ContentType, post, web, HttpResponse, Responder};
use uuid::Uuid;

#[post("/api/create")]
pub async fn create_todo(payload: web::Json<TodoCard>) -> impl Responder {
    let new_id = Uuid::new_v4();
    let str = serde_json::to_string(&TodoIdResponse::new(new_id));

    put_todo(payload).await;

    HttpResponse::Created()
        .content_type(ContentType::json())
        .body(str.expect("failed to serialize ContactsBatchResponseId"))
}
