use axum::{extract::State, Json};
use axum_macros::debug_handler;
use bson::doc;
use bson::DateTime;
use woothee::parser::{Parser, WootheeResult};

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

    // create a user-agent map from all_sessions
    let user_agents = all_sessions
        .iter()
        .map(|s| s.user_agent.clone())
        .collect::<Vec<String>>();

    println!(">> user_agents: {:?}", user_agents);

    let parser = Parser::new();

    // find out os_types, device_types, browser_types from all_sessions using user-agent-parser
    let results: Vec<Option<WootheeResult>> = all_sessions
        .iter()
        .map(|s| parser.parse(s.user_agent.as_str()))
        .collect();

    // get os_types as a string[] from results
    let os_types: Vec<String> = results
        .iter()
        .map(|r| {
            r.as_ref()
                .map_or_else(String::new, |result| result.os.to_string())
        })
        .collect();

    // get device_types as a string[] from results
    let device_types: Vec<String> = results
        .iter()
        .map(|r| {
            r.as_ref()
                .map_or_else(String::new, |result| result.category.to_string())
        })
        .collect();

    // get browser_types as a string[] from results
    let browser_types: Vec<String> = results
        .iter()
        .map(|r| {
            r.as_ref()
                .map_or_else(String::new, |result| result.name.to_string())
        })
        .collect();

    println!(">> results USER AGENTSSS: {:?}", results);
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
