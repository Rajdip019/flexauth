use axum::{extract::State, Json};
use axum_macros::debug_handler;
use bson::doc;
use bson::DateTime;

use crate::core::session::Session;
use crate::errors::Result;
use crate::models::overview_model::OverviewResponse;
use crate::{core::user::User, AppState};

#[debug_handler]
pub async fn get_all_overview_handler(
    State(state): State<AppState>,
) -> Result<Json<OverviewResponse>> {
    println!(">> HANDLER: get_all_overview_handler called");

    let users = User::get_all(&state.mongo_client).await.unwrap();
    let user_count = users.len();
    let active_user_count = users.iter().filter(|u| u.is_active).count();
    let inactive_user_count = users.iter().filter(|u| !u.is_active).count();
    let blocked_user_count = users
        .iter()
        .filter(|u| u.blocked_until.map_or(false, |time| time > DateTime::now()))
        .count();

    let all_sessions = Session::get_all(&state.mongo_client).await.unwrap();
    println!(">> all_sessions Length: {:?}", all_sessions.len());

    let active_session_count = all_sessions.iter().filter(|s| !s.is_revoked).count();
    let revoked_session_count = all_sessions.iter().filter(|s| s.is_revoked).count();

    let os_types: Vec<String> = all_sessions
        .iter()
        .map(|session| session.os.clone())
        .collect();

    let device_types: Vec<String> = all_sessions
        .iter()
        .map(|session| session.device.clone())
        .collect();

    let browser_types: Vec<String> = all_sessions
        .iter()
        .map(|session| session.browser.clone())
        .collect();

    println!(">> os_types: {:?}", os_types);
    println!(">> device_types: {:?}", device_types);
    println!(">> browser_types: {:?}", browser_types);

    let response = OverviewResponse {
        user_count,
        active_user_count,
        inactive_user_count,
        blocked_user_count,
        revoked_session_count,
        active_session_count,
        os_types,
        device_types,
        browser_types,
    };

    Ok(Json(response))
}
