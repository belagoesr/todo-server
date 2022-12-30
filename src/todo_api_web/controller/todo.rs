use crate::todo_api::adapter;
use crate::todo_api::db::helpers::{ERROR_CREATE, ERROR_READ};
use crate::todo_api::db::todo::{get_todos, put_todo};
use crate::todo_api_web::model::http::Clients;
use crate::todo_api_web::model::todo::{TodoCard, TodoCardsResponse, TodoIdResponse};

use actix_web::get;
use actix_web::{http::header::ContentType, post, web, HttpResponse, Responder};
use log::error;
use uuid::Uuid;

#[post("/api/create")]
pub async fn create_todo(state: web::Data<Clients>, info: web::Json<TodoCard>) -> impl Responder {
    let id = Uuid::new_v4();
    let todo_card = adapter::todo_json_to_db(info, id);
    let client = state.dynamo.clone();

    match put_todo(&client, todo_card).await {
        None => {
            error!("Failed to create todo card {}", ERROR_CREATE);
            HttpResponse::BadRequest().body(ERROR_CREATE)
        }
        Some(id) => HttpResponse::Created()
            .content_type(ContentType::json())
            .json(TodoIdResponse::new(id)),
    }
}

#[get("/api/index")]
pub async fn show_all_todo(state: web::Data<Clients>) -> impl Responder {
    let client = state.dynamo.clone();
    let resp = get_todos(&client).await;
    match resp {
        None => {
            error!("Failed to read todo cards");
            HttpResponse::InternalServerError().body(ERROR_READ)
        }
        Some(cards) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(TodoCardsResponse { cards }),
    }
}
