use crate::todo_api_web::controller::{ping, readiness, todo::create_todo};
use actix_web::{web, HttpResponse};

pub fn app_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(ping)
            .service(readiness)
            .service(create_todo)
            .default_service(web::to(|| HttpResponse::NotFound())),
    );
}
