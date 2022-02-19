extern crate openssl;
#[macro_use]
extern crate diesel_migrations;
extern crate diesel;
extern crate gateway_rust;

use actix_web::{web, App, HttpServer};

mod controllers;
pub use controllers::auth_controller::{hello, login, register};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(web::scope("/auth").service(register).service(login))
    })
    .bind("0.0.0.0:3333")?
    .run()
    .await
}
