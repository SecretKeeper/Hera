extern crate openssl;
#[macro_use]
extern crate diesel_migrations;
extern crate diesel;
extern crate gateway_rust;
use dotenv::dotenv;

use std::env;

use actix::SyncArbiter;
use actix_web::{
    web::{self, Data},
    App, HttpServer,
};

mod controllers;
pub use controllers::auth_controller::{hello, login, register};
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use gateway_rust::repositories::db::DbExecutor;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let addr = Data::new(SyncArbiter::start(12, move || DbExecutor(pool.clone())));

    HttpServer::new(move || {
        App::new()
            .app_data(addr.clone())
            .service(hello)
            .service(web::scope("/auth").service(register).service(login))
    })
    .bind("0.0.0.0:3333")?
    .run()
    .await
}
