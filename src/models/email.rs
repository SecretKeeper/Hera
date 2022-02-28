use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeEmailRequest {
    pub uid: Option<i32>,
    pub current_password: String,
    pub new_password: String,
}
