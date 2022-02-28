use actix::Addr;
use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse, Responder,
};
use gateway_rust::{
    db::DbExecutor,
    models::{auth::LoginRequest, token::RevokeTokenRequest, user::CreateUser},
};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Secret Keeper greats you")
}

#[post("/signup")]
async fn register(
    (new_user, addr): (web::Json<CreateUser>, Data<Addr<DbExecutor>>),
) -> impl Responder {
    let actix_message = addr.send(new_user.into_inner()).await;
    let user = actix_message.unwrap();

    web::Json(user.ok())
}

#[post("/signin")]
async fn login((creds, addr): (web::Json<LoginRequest>, Data<Addr<DbExecutor>>)) -> impl Responder {
    let actix_message = addr.send(creds.into_inner()).await;
    let user = actix_message.unwrap();

    web::Json(user.ok())
}

#[post("/revoke-token")]
async fn revoke_token(
    (revoke_token_request, addr): (web::Json<RevokeTokenRequest>, Data<Addr<DbExecutor>>),
) -> impl Responder {
    let actix_message = addr.send(revoke_token_request.into_inner()).await;
    let result = actix_message.unwrap();

    web::Json(result.ok())
}
