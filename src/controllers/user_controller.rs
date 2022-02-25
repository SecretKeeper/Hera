use actix::Addr;
use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use gateway_rust::{
    extractors::jwt_data_decode::Auth, models::ChangePasswordRequest, repositories::db::DbExecutor,
};

#[post("/change-password")]
async fn change_password(
    (password_request, sub, addr): (Json<ChangePasswordRequest>, Auth, Data<Addr<DbExecutor>>),
) -> impl Responder {
    println!("Whatttt the fuck? we got the user id: {:?}", sub);
    let actix_message = addr.send(password_request.into_inner()).await;
    let result = actix_message.unwrap();

    Json(result.ok())
}
