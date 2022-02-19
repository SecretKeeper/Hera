use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Secret Keeper greats you")
}

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    username: String,
    email: String,
    password: String,
}

#[post("/signup")]
async fn register(auth_data: web::Json<CreateUser>) -> impl Responder {
    return web::Json(CreateUser {
        username: auth_data.username.to_string(),
        email: auth_data.username.to_string(),
        password: "4563221".to_string(),
    });
}

#[post("/signin")]
async fn login(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
