use crate::{
    core::{dek::Dek, session::Session, user::User},
    errors::{Error, Result},
    models::user_model::{
        EmailVerificationResponse, RecentUserPayload, ToggleUserActivationStatusPayload,
        ToggleUserActivationStatusResponse, UpdateUserPayload, UpdateUserResponse,
        UpdateUserRolePayload, UpdateUserRoleResponse, UserEmailPayload, UserEmailResponse,
        UserIdPayload, UserResponse,
    },
    utils::{encryption_utils::Encryption, validation_utils::Validation},
    AppState,
};
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
    Json,
};
use axum_macros::debug_handler;
use bson::{doc, DateTime};
use mongodb::Collection;

pub async fn get_all_users_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserResponse>>> {
    println!(">> HANDLER: get_user_handler called");

    match User::get_all(&state.mongo_client).await {
        Ok(users) => Ok(Json(users)),
        Err(e) => Err(e),
    }
}

pub async fn get_recent_users_handler(
    State(state): State<AppState>,
    payload: Json<RecentUserPayload>,
) -> Result<Json<Vec<UserResponse>>> {
    println!(">> HANDLER: get_recent_users_handler called");

    match User::get_recent(&state.mongo_client, payload.limit).await {
        Ok(users) => Ok(Json(users)),
        Err(e) => Err(e),
    }
}

pub async fn update_user_handler(
    State(state): State<AppState>,
    payload: Json<UpdateUserPayload>,
) -> Result<Json<UpdateUserResponse>> {
    println!(">> HANDLER: update_user_handler called");

    // check if the payload is empty
    if payload.email.is_empty() || payload.name.is_empty() {
        return Err(Error::InvalidPayload {
            message: "Invalid payload".to_string(),
        });
    }

    if !Validation::email(&payload.email) {
        return Err(Error::InvalidPayload {
            message: "Invalid Email".to_string(),
        });
    }

    let db = state.mongo_client.database("auth");
    let collection: Collection<User> = db.collection("users");
    let dek_data = match Dek::get(&state.mongo_client, &payload.email).await {
        Ok(dek) => dek,
        Err(e) => return Err(e),
    };
    // find the user in the users collection using the uid
    match collection
        .update_one(
            doc! {
                "uid": &dek_data.uid,
            },
            doc! {
                "$set": {
                    "name": Encryption::encrypt_data(&payload.name, &dek_data.dek),
                    "updated_at": DateTime::now(),
                }
            },
            None,
        )
        .await
    {
        Ok(res) => {
            if res.modified_count == 0 {
                return Err(Error::UserNotFound {
                    message: "User not found".to_string(),
                });
            }
            Ok(Json(UpdateUserResponse {
                email: payload.email.to_owned(),
                name: payload.name.to_owned(),
            }))
        }
        Err(e) => {
            return Err(Error::ServerError {
                message: e.to_string(),
            })
        }
    }
}

pub async fn update_user_role_handler(
    State(state): State<AppState>,
    payload: Json<UpdateUserRolePayload>,
) -> Result<Json<UpdateUserRoleResponse>> {
    println!(">> HANDLER: update_user_role_handler called");

    // check if the payload is empty
    if payload.email.is_empty() || payload.role.is_empty() {
        return Err(Error::InvalidPayload {
            message: "Invalid payload".to_string(),
        });
    }

    if !Validation::email(&payload.email) {
        return Err(Error::InvalidPayload {
            message: "Invalid Email".to_string(),
        });
    }

    match User::update_role(&State(state).mongo_client, &payload.email, &payload.role).await {
        Ok(role) => {
            return Ok(Json(UpdateUserRoleResponse {
                message: "User role updated".to_string(),
                email: payload.email.to_owned(),
                role,
            }))
        }
        Err(e) => return Err(e),
    }
}

pub async fn toggle_user_activation_status(
    State(state): State<AppState>,
    payload: Json<ToggleUserActivationStatusPayload>,
) -> Result<Json<ToggleUserActivationStatusResponse>> {
    println!(">> HANDLER: update_user_role_handler called");

    match payload.is_active {
        Some(_) => {
            if payload.email.is_empty() {
                return Err(Error::InvalidPayload {
                    message: "Invalid payload".to_string(),
                });
            }
        }
        None => {
            return Err(Error::InvalidPayload {
                message: "Invalid payload".to_string(),
            });
        }
    }

    match User::toggle_account_activation(
        &State(state).mongo_client,
        &payload.email,
        &payload.is_active.unwrap(),
    )
    .await
    {
        Ok(is_active_final) => {
            return Ok(Json(ToggleUserActivationStatusResponse {
                message: "User activation status updated".to_string(),
                email: payload.email.to_owned(),
                is_active: is_active_final,
            }))
        }
        Err(e) => return Err(e),
    }
}

#[debug_handler]
pub async fn get_user_email_handler(
    State(state): State<AppState>,
    payload: Json<UserEmailPayload>,
) -> Result<Json<UserResponse>> {
    println!(">> HANDLER: get_user_by_email_handler called");

    if !Validation::email(&payload.email) {
        return Err(Error::InvalidPayload {
            message: "Invalid Email".to_string(),
        });
    }

    match User::get_from_email(&state.mongo_client, &payload.email).await {
        Ok(user) => {
            return Ok(Json(UserResponse {
                uid: user.uid,
                email: user.email,
                name: user.name,
                role: user.role,
                is_active: user.is_active,
                email_verified: user.email_verified,
                blocked_until: user.blocked_until,
                created_at: user.created_at,
                updated_at: user.updated_at,
            }))
        }
        Err(e) => return Err(e),
    }
}

#[debug_handler]
pub async fn get_user_id_handler(
    State(state): State<AppState>,
    payload: Json<UserIdPayload>,
) -> Result<Json<UserResponse>> {
    println!(">> HANDLER: get_user_by_id handler called");
    if payload.uid.is_empty() {
        return Err(Error::InvalidPayload {
            message: "Invalid payload".to_string(),
        });
    }

    match User::get_from_uid(&state.mongo_client, &payload.uid).await {
        Ok(user) => {
            return Ok(Json(UserResponse {
                uid: user.uid,
                email: user.email,
                name: user.name,
                role: user.role,
                is_active: user.is_active,
                email_verified: user.email_verified,
                blocked_until: user.blocked_until,
                created_at: user.created_at,
                updated_at: user.updated_at,
            }))
        }
        Err(e) => return Err(e),
    }
}

#[debug_handler]
pub async fn verify_email_request_handler(
    State(state): State<AppState>,
    payload: Json<UserEmailPayload>,
) -> Result<Json<UserEmailResponse>> {
    println!(">> HANDLER: verify_email_request_handler called");

    if !Validation::email(&payload.email) {
        return Err(Error::InvalidPayload {
            message: "Invalid Email".to_string(),
        });
    }

    match User::verify_email_request(&State(&state).mongo_client, &payload.email).await {
        Ok(_) => {
            return Ok(Json(UserEmailResponse {
                message: "Verification email sent".to_string(),
                email: payload.email.to_owned(),
            }));
        }
        Err(e) => return Err(e),
    }
}

#[debug_handler]
pub async fn verify_email_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<EmailVerificationResponse>> {
    println!(">> HANDLER: verify_email_handler called");

    match User::verify_email(&State(&state).mongo_client, &id).await {
        Ok(req_id) => {
            return Ok(Json(EmailVerificationResponse {
                message: "Email verified successfully".to_string(),
                req_id: req_id.to_owned(),
            }));
        }
        Err(e) => return Err(e),
    }
}

pub async fn delete_user_handler(
    State(state): State<AppState>,
    payload: Json<UserEmailPayload>,
) -> Result<Json<UserEmailResponse>> {
    println!(">> HANDLER: delete_user_handler called");

    if !Validation::email(&payload.email) {
        return Err(Error::InvalidPayload {
            message: "Invalid Email".to_string(),
        });
    }

    match User::delete(&State(&state).mongo_client, &payload.email).await {
        Ok(uid) => {
            match Session::delete_all(&State(&state).mongo_client, &uid).await {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
            return Ok(Json(UserEmailResponse {
                message: "User deleted".to_string(),
                email: payload.email.to_owned(),
            }));
        }
        Err(e) => return Err(e),
    }
}

#[debug_handler]
pub async fn show_verification_page_email(Path(id): Path<String>) -> impl IntoResponse {
    Html(format!(
        r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Verify Email</title>
        <style>
            body {{ font-family: Arial, sans-serif; margin: 0; background-color: #060A13; color: #f2f2f2; }}
            .navbar {{ background-color: #060A13; overflow: hidden; border-bottom: 0.5px solid #1E293B; }}
            .navbar h1 {{ color: #f2f2f2; text-align: center; padding: 14px 0px; margin: 0; }}
            .content {{ display: flex; justify-content: center; align-items: center; height: 80vh; }}
            .message {{ text-align: center; }}
            .message h2 {{ color: #f2f2f2; }}
        </style>
    </head>
    <body>
        <div class='navbar'>
            <h1>FlexAuth</h1>
        </div>
        <div class="content">
            <div class="message">
                <h2 id="message">Verifying...</h2>
            </div>
        </div>
        <script>
            document.addEventListener('DOMContentLoaded', function() {{
                fetch('http://localhost:8080/api/user/verify-email/{id}', {{
                    headers: {{
                        'Content-Type': 'application/json',
                        'x-api-key': '{api_key}'
                    }},
                }})
                    .then(response => {{
                        if (response.ok) {{
                            document.getElementById('message').textContent = 'Email Verified 🎉';
                        }} else {{
                            document.getElementById('message').textContent = 'Verification Link Expired';
                        }}
                    }})
                    .catch(error => {{
                        document.getElementById('message').textContent = 'An error occurred: ' + error.message;
                    }});
            }});
        </script>
    </body>
    </html>
    "#,
        id = id,
        api_key = dotenv::var("X_API_KEY").unwrap()
    ))
}
