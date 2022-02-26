use std::env;

use actix_web::http::header::{HeaderMap, AUTHORIZATION};
use actix_web::{dev, Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::Deserialize;

use dotenv::dotenv;

use crate::auth::auth::Claims;
use crate::errors::ServiceError;

const BEARER: &str = "Bearer ";

#[derive(Debug, Deserialize)]
pub struct Auth {
    pub user_id: i32,
}

impl FromRequest for Auth {
    type Error = Error;
    type Future = Ready<Result<Auth, Error>>;

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        dotenv().ok();

        let jwt_secret =
            env::var("JWT_ACCESS_TOKEN_SECRET").expect("JWT_ACCESS_TOKEN_SECRET must be set");

        match jwt_from_header(req.headers()) {
            Ok(jwt) => {
                // return ok(Auth { user_id: jwt });
                let decoded = decode::<Claims>(
                    &jwt,
                    &DecodingKey::from_secret(jwt_secret.as_bytes()),
                    &Validation::new(Algorithm::HS512),
                )
                .unwrap();
                return ok(Auth {
                    user_id: decoded.claims.sub,
                });
            }
            Err(_e) => return err(ServiceError::Unauthorized.into()),
        }
    }
}

fn jwt_from_header(headers: &HeaderMap) -> Result<String, ServiceError> {
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(ServiceError::Unauthorized),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => return Err(ServiceError::Unauthorized),
    };
    if !auth_header.starts_with(BEARER) {
        return Err(ServiceError::Unauthorized);
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}
