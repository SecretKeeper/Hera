use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    pub uid: Option<i32>,
    pub current_password: String,
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeUsernameRequest {
    pub uid: Option<i32>,
    pub current_password: String,
    pub new_username: String,
}
