use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use log::error;

use crate::{
    todo_api::{
        core::{decode_jwt, generate_jwt, validate_jwt_date, validate_jwt_info},
        model::core::JwtValue,
    },
    todo_api_web::model::{
        auth::{Auth, SignUp},
        http::Clients,
    },
};

#[post("/login")]
pub async fn login(state: web::Data<Clients>, info: web::Json<Auth>) -> impl Responder {
    let login_user = info.clone();
    if !is_email_pswd_valids(&login_user.email, &login_user.password.clone().unwrap()) {
        return HttpResponse::BadRequest().finish();
    }

    let resp = state.postgres.send(login_user).await;

    match resp {
        Err(e) => {
            error!("{:?}", e);
            HttpResponse::NoContent().finish()
        }
        Ok(user) => {
            let usr = user.unwrap();
            match usr.verify(info.clone().password.unwrap()) {
                Ok(true) => generate_jwt(usr, state).await,
                Ok(false) => HttpResponse::NoContent().finish(),
                Err(_) => HttpResponse::NoContent().finish(),
            }
        }
    }
}
#[post("/signup")]
pub async fn signup_user(state: web::Data<Clients>, info: web::Json<SignUp>) -> impl Responder {
    let signup = info.into_inner();
    if !is_email_pswd_valids(&signup.email, &signup.password) {
        return HttpResponse::BadRequest();
    }

    let resp = state.postgres.send(signup).await;

    match resp {
        Ok(_) => HttpResponse::Created(),
        Err(e) => {
            error!("{:?}", e);
            HttpResponse::InternalServerError()
        }
    }
}

#[post("/logout")]
pub async fn logout(
    req: HttpRequest,
    state: web::Data<Clients>,
    info: web::Json<Auth>,
) -> impl Responder {
    use regex::Regex;

    let jwt = req.headers().get("x-auth");
    let logout_user = info.clone();
    let email_regex = Regex::new("\\w{1,}@\\w{2,}.[a-z]{2,3}(.[a-z]{2,3})?$").unwrap();

    if !email_regex.is_match(&logout_user.email) {
        return HttpResponse::BadRequest().finish();
    }

    let resp = state.postgres.send(logout_user.clone());

    match jwt {
        None => return HttpResponse::BadRequest().finish(),
        Some(jwt) => {
            let jwt_value: JwtValue = serde_json::from_value(decode_jwt(jwt.to_str().unwrap()))
                .expect("failed to parse JWT Value");
            match validate_jwt_date(jwt_value.expires_at) {
                false => HttpResponse::Unauthorized().finish(),
                true => {
                    validate_jwt_info(
                        jwt_value.email,
                        logout_user.email,
                        resp.await.expect("Failed to read contact info"),
                        state,
                    )
                    .await
                }
            }
        }
    }
}

pub fn is_email_pswd_valids(email: &str, pswd: &str) -> bool {
    use regex::Regex;

    let email_regex = Regex::new("\\w{1,}@\\w{2,}.[a-z]{2,3}(.[a-z]{2,3})?$").unwrap();
    let pswd_regex = Regex::new("[[a-z]+[A-Z]+[0-9]+(\\s@!=_#&~\\[\\]\\{\\}\\?)]{32,64}").unwrap();

    email_regex.is_match(email) && pswd_regex.is_match(pswd)
}

#[cfg(test)]
mod valid_email_pswd {
    use super::is_email_pswd_valids;

    #[test]
    fn valid_email_and_pswd() {
        assert!(is_email_pswd_valids(
            "my@email.com",
            "My cr4zy P@ssw0rd My cr4zy P@ssw0rd"
        ));
    }

    #[test]
    fn invalid_emails() {
        assert!(!is_email_pswd_valids(
            "my_email.com",
            "My cr4zy P@ssw0rd My cr4zy P@ssw0rd"
        ));
        assert!(!is_email_pswd_valids(
            "my@email.com.br.us",
            "My cr4zy P@ssw0rd My cr4zy P@ssw0rd"
        ));
    }

    #[test]
    fn invalid_passwords() {
        assert!(!is_email_pswd_valids(
            "my@email.com.br",
            "My cr4zy P@ssw0rd"
        ));
        assert!(is_email_pswd_valids(
            "my@email.com",
            "my cr4zy p@ssw0rd my cr4zy p@ssw0rd"
        ));
        assert!(is_email_pswd_valids(
            "my@email.com",
            "My crazy P@ssword My crazy P@ssword"
        ));
        assert!(is_email_pswd_valids(
            "my@email.com",
            "My cr4zy Passw0rd My cr4zy Passw0rd"
        ));
    }
}
