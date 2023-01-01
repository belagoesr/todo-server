use crate::todo_api_web::controller::{
    auth::{login, logout, signup_user},
    ping, readiness,
    todo::{create_todo, show_all_todo},
};

use actix_web::{web, HttpResponse};

pub fn app_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(ping)
            .service(readiness)
            .service(create_todo)
            .service(show_all_todo)
            .service(signup_user)
            .service(login)
            .service(logout)
            .default_service(web::to(|| HttpResponse::NotFound())),
    );
}
