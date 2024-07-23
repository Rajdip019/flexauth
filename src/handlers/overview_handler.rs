use axum::{extract::State, Json};
use axum_macros::debug_handler;
use bson::doc;
use serde_json::{json, Value};
use user_agent_parser::UserAgentParser;

use crate::core::session::Session;
use crate::errors::Result;
use crate::{core::user::User, AppState};

#[debug_handler]
pub async fn get_all_overview_handler(State(state): State<AppState>) -> Result<Json<Value>> {
    println!(">> HANDLER: get_all_overview_handler called");
    let db = &state.mongo_client.database("auth");
    let collection_user: mongodb::Collection<User> = db.collection("users");
    // get count of users
    let user_count = collection_user.count_documents(None, None).await.unwrap();

    let collection_session: mongodb::Collection<Session> = db.collection("sessions");
    // session count of revoked ones with condition is_revoked is true
    let revoked_session_count = collection_session
        .count_documents(Some(doc! { "is_revoked": true }), None)
        .await
        .unwrap();
    // session count of active ones with condition is_revoked is false
    let active_session_count = collection_session
        .count_documents(Some(doc! { "is_revoked": false }), None)
        .await
        .unwrap();

    let all_sessions = Session::get_all(&state.mongo_client).await.unwrap();
    println!(">> all_sessions Length: {:?}", all_sessions.len());
    // create a user-agent map from all_sessions
    let user_agents = all_sessions
        .iter()
        .map(|s| s.user_agent.clone())
        .collect::<Vec<String>>();

    println!(">> user_agents: {:?}", user_agents);

    let ua_parser = UserAgentParser::from_path("regexes.yaml").unwrap();

    // find out os_types, device_types, browser_types from all_sessions using user-agent-parser
    let os_types = all_sessions
        .iter()
        .map(|s| {
            ua_parser
                .parse_os(&s.user_agent)
                .name
                .map_or(String::new(), |cow_str| cow_str.to_string())
        })
        .collect::<Vec<String>>();

    let device_types = all_sessions
        .iter()
        .map(|s| {
            ua_parser
                .parse_device(&s.user_agent)
                .name
                .map_or(String::new(), |cow_str| cow_str.to_string())
        })
        .collect::<Vec<String>>();

    let browser_types = all_sessions
        .iter()
        .map(|s| {
            ua_parser
                .parse_product(&s.user_agent)
                .name
                .map_or(String::new(), |cow_str| cow_str.to_string())
        })
        .collect::<Vec<String>>();

    println!(">> all_sessions: {:?}", all_sessions);
    println!(">> os_types: {:?}", os_types);
    println!(">> device_types: {:?}", device_types);
    println!(">> browser_types: {:?}", browser_types);

    let response = json!({
        "user_count":user_count,
        "revoked_session_count": revoked_session_count,
        "active_session_count": active_session_count,
    });
    Ok(Json(response))
}
