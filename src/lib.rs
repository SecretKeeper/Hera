#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod auth;
pub mod db;
pub mod errors;
pub mod extractors;
pub mod message_handlers;
pub mod models;
pub mod schema;
pub mod utils;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
