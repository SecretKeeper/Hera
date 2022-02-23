use crate::auth::auth::{self, Role};
use crate::errors::ServiceError;
use crate::models::{CreateUser, LoginRequest, LoginResponse, User, UserResponse};
use crate::schema::users::dsl::*;
use actix::{Handler, Message, SyncContext};
use blake3::Hasher;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::result::Error;
use jsonwebtoken::{Algorithm, Header};

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

impl Message for LoginRequest {
    type Result = Result<LoginResponse, ServiceError>;
}

impl Handler<LoginRequest> for DbExecutor {
    type Result = Result<LoginResponse, ServiceError>;

    fn handle(&mut self, creds: LoginRequest, _: &mut SyncContext<Self>) -> Self::Result {
        let conn: &PgConnection = &self.0.get().unwrap();

        let mut hasher = Hasher::new();

        hasher.update(&creds.password.as_bytes());

        let mut found_users = users
            .filter(username.eq(&creds.username))
            .load::<User>(conn)?;

        let mut header = Header::new(Algorithm::HS512);
        header.kid = Some("blabla".to_owned());

        if let Some(user) = found_users.pop() {
            if user.password.as_deref().unwrap_or("")
                == hasher
                    .finalize()
                    .to_hex()
                    .chars()
                    .collect::<String>()
                    .to_string()
            {
                let token = auth::create_jwt(&user.id, &Role::from_str("User"))
                    .map_err(|_e| ServiceError::InternalServerError)?;

                return Ok(LoginResponse {
                    user: UserResponse {
                        user_id: user.id,
                        username: user.username.as_deref().unwrap_or("").to_string(),
                        email: user.email.as_deref().unwrap_or("").to_string(),
                        avatar: user.avatar.as_deref().unwrap_or("").to_string(),
                    },
                    token,
                });
            }
        }

        Err(ServiceError::Unauthorized)
    }
}
