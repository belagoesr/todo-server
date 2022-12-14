use actix::prelude::*;
use serde::{Deserialize, Serialize};

use crate::todo_api::{
    adapter,
    db::helpers::DbExecutor,
    model::{auth::User, error::DbError},
};

// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// pub struct Login {
//     pub email: String,
//     pub password: String,
// }

// impl Message for Login {
//     type Result = Result<User, DbError>;
// }

// impl Handler<Login> for DbExecutor {
//     type Result = Result<User, DbError>;

//     fn handle(&mut self, msg: Login, _: &mut Self::Context) -> Self::Result {
//         use crate::todo_api::db::auth::scan_user;

//         scan_user(msg.email, &self.0.get().expect("Failed to open connection"))
//     }
// }

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SignUp {
    pub email: String,
    pub password: String,
}

impl Message for SignUp {
    type Result = Result<(), DbError>;
}

impl Handler<SignUp> for DbExecutor {
    type Result = Result<(), DbError>;

    fn handle(&mut self, msg: SignUp, _: &mut Self::Context) -> Self::Result {
        use crate::todo_api::db::auth::insert_new_user;

        let user = adapter::auth::signup_to_hash_user(msg);

        insert_new_user(user, &mut self.0.get().expect("Failed to open connection"))
    }
}

// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// pub struct Logout {
//     pub email: String,
// }

// impl Message for Logout {
//     type Result = Result<User, DbError>;
// }

// impl Handler<Logout> for DbExecutor {
//     type Result = Result<User, DbError>;

//     fn handle(&mut self, msg: Logout, _: &mut Self::Context) -> Self::Result {
//         use crate::todo_api::db::auth::scan_user;

//         scan_user(msg.email, &self.0.get().expect("Failed to open connection"))
//     }
// }

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Auth {
    pub email: String,
    pub password: Option<String>,
}

impl Message for Auth {
    type Result = Result<User, DbError>;
}

impl Handler<Auth> for DbExecutor {
    type Result = Result<User, DbError>;

    fn handle(&mut self, msg: Auth, _: &mut Self::Context) -> Self::Result {
        use crate::todo_api::db::auth::scan_user;

        scan_user(
            msg.email,
            &mut self.0.get().expect("Failed to open connection"),
        )
    }
}
