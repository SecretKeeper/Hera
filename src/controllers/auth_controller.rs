use actix::Addr;
use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse, Responder,
};
use gateway_rust::{models::CreateUser, repositories::db::DbExecutor};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Secret Keeper greats you")
}

#[post("/signup")]
async fn register(
    (new_user, addr): (web::Json<CreateUser>, Data<Addr<DbExecutor>>),
) -> impl Responder {
    let create_message = addr.send(new_user.into_inner()).await;
    let user = create_message.unwrap();

    web::Json(user.ok())
}

#[post("/signin")]
async fn login(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
