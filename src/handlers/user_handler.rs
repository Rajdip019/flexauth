use crate::{
    core::{dek::Dek, user::User},
    errors::{Error, Result},
    models::user_model::{
        ToggleUserActivationStatusPayload, ToggleUserActivationStatusResponse, UpdateUserPayload,
        UpdateUserResponse, UpdateUserRolePayload, UpdateUserRoleResponse, UserEmailPayload,
        UserEmailResponse, UserIdPayload, UserResponse,
    },
    AppState,
};
use axum::{extract::State, Json};
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

    let db = state.mongo_client.database("test");
    let collection: Collection<User> = db.collection("users");
    let dek_data = match Dek::get(&state.mongo_client, &payload.email).await {
        Ok(dek) => dek,
        Err(e) => return Err(e),
    };

    // find the user in the users collection using the uid
    match collection
        .update_one(
            doc! {
                "uid": dek_data.uid,
            },
            doc! {
                "$set": {
                    "name": payload.name.clone(),
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
                message: "User updated".to_string(),
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

    match User::get_from_email(&state.mongo_client, &payload.email).await {
        Ok(user) => return Ok(Json(user)),
        Err(e) => return Err(e),
    }
}

#[debug_handler]
pub async fn get_user_id_handler(
    State(state): State<AppState>,
    payload: Json<UserIdPayload>,
) -> Result<Json<UserResponse>> {
    println!(">> HANDLER: get_user_by_id handler called");

    match User::get_from_uid(&state.mongo_client, &payload.uid).await {
        Ok(user) => return Ok(Json(user)),
        Err(e) => return Err(e),
    }
}

pub async fn delete_user_handler(
    State(state): State<AppState>,
    payload: Json<UserEmailPayload>,
) -> Result<Json<UserEmailResponse>> {
    println!(">> HANDLER: delete_user_handler called");

    // check if the payload is empty
    if payload.email.is_empty() {
        return Err(Error::InvalidPayload {
            message: "Invalid payload".to_string(),
        });
    }

    match User::delete(&State(state).mongo_client, &payload.email).await {
        Ok(_) => {
            return Ok(Json(UserEmailResponse {
                message: "User deleted".to_string(),
                email: payload.email.to_owned(),
            }))
        }
        Err(e) => return Err(e),
    }
}
