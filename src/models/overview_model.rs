use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OverviewResponse {
    pub user_count: usize,
    pub active_user_count: usize,
    pub inactive_user_count: usize,
    pub blocked_user_count: usize,
    pub revoked_session_count: usize,
    pub active_session_count: usize,
    pub os_types: Vec<String>,
    pub device_types: Vec<String>,
    pub browser_types: Vec<String>,
}
