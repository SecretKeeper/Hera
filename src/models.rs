use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub solana_pubkey: String,
    pub ethereum_pubkey: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub avatar: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}
