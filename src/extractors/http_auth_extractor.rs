use actix_web::dev::ServiceRequest;
use actix_web_grants::permissions::AttachPermissions;
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::auth::auth::authorize;

pub async fn http_auth_extract(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, actix_web::Error> {
    let user_roles = authorize(credentials.token()).await;

    req.attach(user_roles.unwrap());

    Ok(req)
}
