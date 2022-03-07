extern crate openssl;
#[macro_use]
extern crate diesel_migrations;
extern crate diesel;
extern crate gateway_rust;
use actix_web_httpauth::middleware::HttpAuthentication;
use controllers::{
    auth_controller::revoke_token,
    settings_controller::{change_email, change_password},
    user_controller::change_username,
};
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
use gateway_rust::{db::DbExecutor, extractors::http_auth_extractor::http_auth_extract};

embed_migrations!("./migrations");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let whisper_database_url =
        env::var("WHISPER_DATABASE_URL").expect("WHISPER_DATABASE_URL must be set");
    let whisper_manager = ConnectionManager::<PgConnection>::new(whisper_database_url);

    let whisper_pool = r2d2::Pool::builder()
        .build(whisper_manager)
        .expect("Failed to create whisper database pool.");

    embedded_migrations::run(&pool.get().expect("cant get connection pool")).unwrap();

    let addr = Data::new(SyncArbiter::start(12, move || {
        DbExecutor(pool.clone(), whisper_pool.clone())
    }));

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(http_auth_extract);

        App::new()
            .app_data(addr.clone())
            .service(hello)
            .service(
                web::scope("/user")
                    .wrap(auth)
                    .service(change_password)
                    .service(change_username)
                    .service(change_email),
            )
            .service(
                web::scope("/auth")
                    .service(register)
                    .service(login)
                    .service(revoke_token),
            )
    })
    .bind("0.0.0.0:3333")?
    .run()
    .await
}
