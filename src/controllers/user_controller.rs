use actix::Addr;
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder, ResponseError,
};
use hera::{
    db::DbExecutor,
    extractors::jwt_data_decode::Auth,
    models::settings::{ChangePasswordRequest, ChangeUsernameRequest},
};

#[post("/change-password")]
async fn change_password(
    (mut password_request, sub, addr): (Json<ChangePasswordRequest>, Auth, Data<Addr<DbExecutor>>),
) -> impl Responder {
    password_request.uid = Some(sub.user_id);

    let actix_message = addr.send(password_request.into_inner()).await;
    let result = actix_message.unwrap();

    match result {
        Ok(response) => HttpResponse::Ok().body(response),
        Err(error) => ResponseError::error_response(&error),
    }
}

#[post("/change-username")]
async fn change_username(
    (mut change_username_request, sub, addr): (
        Json<ChangeUsernameRequest>,
        Auth,
        Data<Addr<DbExecutor>>,
    ),
) -> impl Responder {
    change_username_request.uid = Some(sub.user_id);

    let actix_message = addr.send(change_username_request.into_inner()).await;
    let result = actix_message.unwrap();

    match result {
        Ok(response) => HttpResponse::Ok().body(response),
        Err(error) => ResponseError::error_response(&error),
    }
}
