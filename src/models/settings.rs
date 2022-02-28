use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    pub uid: Option<i32>,
    pub current_password: String,
    pub new_password: String,
}
