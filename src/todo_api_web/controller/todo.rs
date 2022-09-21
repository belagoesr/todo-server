use actix_web::{post, HttpResponse, Responder};

#[post("/api/create")]
pub async fn create_todo() -> impl Responder {
    HttpResponse::NotImplemented()
}
