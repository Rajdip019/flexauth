use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct SignUpPayload {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignInPayload {
    pub email: String,
    pub password: String,
}
