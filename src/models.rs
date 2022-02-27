use super::schema::{jwt_tokens, users};
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Identifiable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub solana_pubkey: Option<String>,
    pub ethereum_pubkey: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub avatar: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct Jwt {
    pub id: i32,
    pub user_id: i32,
    pub access_token: String,
    pub access_token_expires_at: NaiveDateTime,
    pub refresh_token: String,
    pub refresh_token_expires_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "jwt_tokens"]
pub struct CreateJWT {
    pub user_id: i32,
    pub access_token: String,
    pub access_token_expires_at: NaiveDateTime,
    pub refresh_token: String,
    pub refresh_token_expires_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub avatar: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    pub uid: Option<i32>,
    pub current_password: String,
    pub new_password: String,
}
