use crate::models::{CreateUser, User};
use crate::schema::users::dsl::*;
use actix::{Handler, Message, SyncContext};
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::result::Error;

use super::db::DbExecutor;

impl Message for CreateUser {
    type Result = Result<User, Error>;
}

impl Handler<CreateUser> for DbExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, new_user: CreateUser, _: &mut SyncContext<Self>) -> Self::Result {
        let conn: &PgConnection = &self.0.get().unwrap();

        let inserted_user = diesel::insert_into(users::table())
            .values(&new_user)
            .get_result(conn)
            .expect("cant insert user");

        Ok(inserted_user)
    }
}
