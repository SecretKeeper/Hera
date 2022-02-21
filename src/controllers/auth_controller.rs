use actix_web::{get, post, web, HttpResponse, Responder};
use gateway_rust::models::CreateUser;

use crate::AppState;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Secret Keeper greats you")
}

#[post("/signup")]
async fn register(
    (new_user, state): (web::Json<CreateUser>, web::Data<AppState>),
) -> impl Responder {
    let create_message = state.db.send(new_user.into_inner()).await;
    let user = create_message.unwrap();

    web::Json(user.ok())
}

#[post("/signin")]
async fn login(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
