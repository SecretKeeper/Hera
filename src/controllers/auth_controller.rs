use actix::Addr;
use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse, Responder,
};
use gateway_rust::{
    models::{CreateUser, Login},
    repositories::db::DbExecutor,
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
async fn login((creds, addr): (web::Json<Login>, Data<Addr<DbExecutor>>)) -> impl Responder {
    let actix_message = addr.send(creds.into_inner()).await;
    let user = actix_message.unwrap();

    web::Json(user.ok())
}
