use crate::todo_api::{core::decode_jwt, model::core::JwtValue};
use actix_web_lab::middleware::Next;

use actix_web::Error;
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    web::Data,
};

use super::model::http::Clients;

pub async fn authentication_mw(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let data = req.extract::<Data<Clients>>().await.unwrap();
    let jwt = req.headers().get("x-auth");

    match jwt {
        None => Err(actix_web::error::ErrorInternalServerError(
            "error in error authentication mw",
        )),
        Some(token) => {
            let decoded_jwt: JwtValue = serde_json::from_value(decode_jwt(token.to_str().unwrap()))
                .expect("Failed to parse Jwt");

            let valid_jwt = data.postgres.send(decoded_jwt);
            let fut = next.call(req).await?;

            match valid_jwt.await {
                Ok(true) => {
                    let (req, res) = fut.into_parts();
                    let res = ServiceResponse::new(req, res);
                    Ok(res)
                }
                _ => Err(actix_web::error::ErrorInternalServerError(
                    "error in error authentication mw",
                )),
            }
        }
    }
}
