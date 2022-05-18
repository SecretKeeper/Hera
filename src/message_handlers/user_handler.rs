use crate::db::DbExecutor;
use crate::errors::ServiceError;
use crate::models::user::{CreateProfile, CreateUser, User};
use crate::schema::profiles::dsl::*;
use crate::schema::users::dsl::*;
use actix::{Handler, Message, SyncContext};
use blake3::Hasher;
use diesel::associations::HasTable;
use diesel::prelude::*;

impl Message for CreateUser {
    type Result = Result<User, ServiceError>;
}

impl Handler<CreateUser> for DbExecutor {
    type Result = Result<User, ServiceError>;

    fn handle(&mut self, mut new_user: CreateUser, _: &mut SyncContext<Self>) -> Self::Result {
        let conn: &PgConnection = &self.0.get().unwrap();

        let whisper_conn: &PgConnection = &self.1.get().unwrap();

        let mut hasher = Hasher::new();

        hasher.update(&new_user.password.as_bytes());

        new_user.password = hasher.finalize().to_hex().chars().collect();

        let inserted_user: User = diesel::insert_into(users::table())
            .values(&new_user)
            .get_result(conn)?;

        // Create also user profile on Whisper App
        let _create_whisper_profile = diesel::insert_into(profiles::table())
            .values(CreateProfile {
                user_id: inserted_user.id,
                status: "".to_string(),
                description: "".to_string(),
            })
            .execute(whisper_conn);

        Ok(inserted_user)
    }
}
