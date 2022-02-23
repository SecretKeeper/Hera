use std::fmt;

use chrono::prelude::*;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::errors::ServiceError;

// const BEARER: &str = "Bearer ";
const JWT_SECRET: &[u8] = b"secret";

#[derive(Clone, PartialEq)]
pub enum Role {
    User,
    Admin,
}

impl Role {
    pub fn from_str(role: &str) -> Role {
        match role {
            "Admin" => Role::Admin,
            _ => Role::User,
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::User => write!(f, "User"),
            Role::Admin => write!(f, "Admin"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: i32,
    role: String,
    exp: usize,
}

pub fn create_jwt(uid: &i32, role: &Role) -> Result<String, ServiceError> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(180))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: *uid,
        role: role.to_string(),
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| ServiceError::JWTTokenCreationError)
}
