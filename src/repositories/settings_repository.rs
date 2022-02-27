use super::db::DbExecutor;
use crate::diesel::ExpressionMethods;
use crate::models::RevokeTokenRequest;
use crate::schema::jwt_tokens::dsl::*;

use actix::{Handler, Message, SyncContext};
use diesel::result::Error;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};

impl Message for RevokeTokenRequest {
    type Result = Result<String, Error>;
}

impl Handler<RevokeTokenRequest> for DbExecutor {
    type Result = Result<String, Error>;

    fn handle(
        &mut self,
        revoke_token_request: RevokeTokenRequest,
        _: &mut SyncContext<Self>,
    ) -> Self::Result {
        let conn: &PgConnection = &self.0.get().unwrap();

        let target =
            diesel::delete(jwt_tokens.filter(refresh_token.eq(revoke_token_request.refresh_token)))
                .execute(conn)?;

        if target == 1 {
            Ok("token revoked successfully".to_string())
        } else {
            Err(Error::NotFound)
        }
    }
}
