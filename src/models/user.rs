use crate::schema::profiles;
use crate::schema::users;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(AsChangeset, Debug, Queryable, Identifiable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub solana_pubkey: Option<String>,
    pub ethereum_pubkey: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub avatar: Option<String>,
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing)]
    pub updated_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Validate, Insertable, Serialize)]
#[table_name = "users"]
pub struct CreateUser {
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Validate, Insertable, Serialize, Deserialize)]
#[table_name = "profiles"]
pub struct CreateProfile {
    pub user_id: i32,
    pub status: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub avatar: String,
}
