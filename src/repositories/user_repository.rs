use crate::errors::ServiceError;
use crate::models::{CreateUser, Login, User};
use crate::schema::users::dsl::*;
use actix::{Handler, Message, SyncContext};
use blake3::Hasher;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::result::Error;

use super::db::DbExecutor;

impl Message for CreateUser {
    type Result = Result<User, Error>;
}

impl Handler<CreateUser> for DbExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, mut new_user: CreateUser, _: &mut SyncContext<Self>) -> Self::Result {
        let conn: &PgConnection = &self.0.get().unwrap();

        let mut hasher = Hasher::new();

        hasher.update(&new_user.password.as_bytes());

        new_user.password = hasher.finalize().to_hex().chars().collect();

        let inserted_user = diesel::insert_into(users::table())
            .values(&new_user)
            .get_result(conn)
            .expect("cant insert user");

        Ok(inserted_user)
    }
}

impl Message for Login {
    type Result = Result<User, ServiceError>;
}

impl Handler<Login> for DbExecutor {
    type Result = Result<User, ServiceError>;

    fn handle(&mut self, creds: Login, _: &mut SyncContext<Self>) -> Self::Result {
        let conn: &PgConnection = &self.0.get().unwrap();

        let mut hasher = Hasher::new();

        hasher.update(&creds.password.as_bytes());

        let mut found_users = users
            .filter(username.eq(&creds.username))
            .load::<User>(conn)?;

        if let Some(user) = found_users.pop() {
            if user.password.as_deref().unwrap_or("")
                == hasher
                    .finalize()
                    .to_hex()
                    .chars()
                    .collect::<String>()
                    .to_string()
            {
                return Ok(user.into());
            }
        }

        Err(ServiceError::Unauthorized)
    }
}
