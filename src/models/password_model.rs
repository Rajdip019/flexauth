use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct ResetPasswordPayload {
    pub email: String,
    pub old_password: String,
    pub new_password: String,
}