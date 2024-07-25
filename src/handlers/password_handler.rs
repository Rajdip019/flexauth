use crate::{
    core::user::User,
    errors::{Error, Result},
    models::password_model::{
        ForgetPasswordPayload, ForgetPasswordResetPayload, ResetPasswordPayload,
    },
    utils::validation_utils::Validation,
    AppState,
};
use axum::{
    extract::{Path, State}, response::{Html, IntoResponse}, Json
};
use axum_macros::debug_handler;
use bson::doc;
use serde_json::{json, Value};

#[debug_handler]
pub async fn reset_password_handler(
    State(state): State<AppState>,
    payload: Json<ResetPasswordPayload>,
) -> Result<Json<Value>> {
    // check if payload is valid
    if payload.email.is_empty() | payload.old_password.is_empty() | payload.new_password.is_empty()
    {
        return Err(Error::InvalidPayload {
            message: "Email and password are required.".to_string(),
        });
    }

    if !Validation::password(&payload.new_password) || !Validation::password(&payload.old_password)
    {
        return Err(Error::InvalidPayload {
            message: "Password must be at least 8 characters long.".to_string(),
        });
    }

    match User::change_password(
        &state.mongo_client,
        &payload.email,
        &payload.old_password,
        &payload.new_password,
    )
    .await
    {
        Ok(_) => {
            return Ok(Json(json!({
                "message": "Password updated successfully. Please login with the new password."
            })));
        }
        Err(e) => return Err(e),
    }
}

#[debug_handler]
pub async fn forget_password_request_handler(
    State(state): State<AppState>,
    payload: Json<ForgetPasswordPayload>,
) -> Result<Json<Value>> {
    // check if payload.email exists
    if payload.email.is_empty() {
        return Err(Error::InvalidPayload {
            message: "Email is required.".to_string(),
        });
    }

    match User::forget_password_request(&state.mongo_client, &payload.email).await {
        Ok(_) => {
            return Ok(Json(json!({
                "message": "Password reset request sent successfully. Please check your email."
            })));
        }
        Err(e) => return Err(e),
    }
}

#[debug_handler]
pub async fn forget_password_reset_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
    payload: Json<ForgetPasswordResetPayload>,
) -> Result<Json<Value>> {
    // check if payload is valid
    if payload.email.is_empty() | payload.password.is_empty() {
        return Err(Error::InvalidPayload {
            message: "Invalid Payload".to_string(),
        });
    }
    match User::forget_password_reset(&state.mongo_client, &id, &payload.email, &payload.password)
        .await
    {
        Ok(_) => {
            return Ok(Json(json!({
                "message": "Password updated successfully. Please login with the new password."
            })));
        }
        Err(e) => return Err(e),
    }
}

#[debug_handler]
pub async fn forget_password_form(Path(id): Path<String>) -> impl IntoResponse {
    Html(format!(r#"
        <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Reset Password</title>
        <style>
            body {{ font-family: Arial, sans-serif; margin: 0; background-color: #060A13; }}
            .navbar {{ background-color: #060A13; overflow: hidden; border-bottom: 0.5px solid #1E293B; }}
            .navbar h1 {{ color: #f2f2f2; text-align: center; padding: 14px 0px; }}
            .form-div {{ margin: 0 auto; display: flex; justify-content: center; align-items: center; }}
            form {{ margin-top: 20px; display: flex; flex-direction: column; align-items: left; width: 40%; }}
            label {{ display: block; margin-top: 10px; text-align: left; color: #f2f2f2; }}
            input {{ width: 100%; padding: 8px; margin-top: 5px; margin-bottom: 10px; border: 1px solid #ccc; border-radius: 4px; box-sizing: border-box; }}
            button {{ padding: 10px 20px; background-color: #3B81F6; color: #f2f2f2; border: none; cursor: pointer; border-radius: 5px; width: 100%; }}
            h1 {{ color: #f2f2f2; text-align: center; padding: 14px 0px; }}
            h2 {{ text-align: center; color: #f2f2f2; }}
            p {{ color: #f2f2f2; margin-top: 30px; }}
            .success {{ display: flex; justify-content: center; align-items: center; height: 100vh; }}
        </style>
    </head>
    <body>
        <div class='navbar'>
            <h1>FlexAuth</h1>
        </div>
        <h2>Reset Password</h2>
        <div class='form-div'>
            <form id="resetForm">
                <label for="email">Enter Email:</label>
                <input type="email" id="email" name="email" required placeholder="Enter email">
                <label for="password">Enter Password:</label>
                <input type="password" id="password" name="password" required placeholder="Enter new password">
                <label for="confirm_password">Confirm Password:</label>
                <input type="password" id="confirm_password" name="confirm_password" required placeholder="Confirm new password">
                <br />
                <button type="submit" id="submitBtn">Submit</button>
                <p><b>Note:</b> Password must be at least 8 characters long and include at least one uppercase letter, one lowercase letter, one number, and one special character.</p>
            </form>
        </div>
        <script>
            document.getElementById('resetForm').addEventListener('submit', function(event) {{
                event.preventDefault();

                const email = document.getElementById('email').value;
                const password = document.getElementById('password').value;
                const confirmPassword = document.getElementById('confirm_password').value;
                const passwordPattern = /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{{8,}}$/;
                const submitBtn = document.getElementById('submitBtn');

                if (!passwordPattern.test(password)) {{
                    alert('Password must be at least 8 characters long and include at least one uppercase letter, one lowercase letter, one number, and one special character.');
                }} else if (password !== confirmPassword) {{
                    alert('Passwords do not match.');
                }} else {{
                    submitBtn.textContent = 'Loading...';
                    submitBtn.disabled = true;
                    fetch(`/api/password/forget-reset/{id}`, {{
                        method: 'POST',
                        headers: {{
                            'Content-Type': 'application/json',
                            'x-api-key': '{api_key}'
                        }},
                        body: JSON.stringify({{
                            email: email,
                            password: password
                        }})
                    }}).then(response => {{
                        if (response.ok) {{
                            document.body.innerHTML = "<div class='success'><h1>Password changed successfully.</h1> <h2>You can close this window now and proceed to login with your new password.</h2></div>";
                        }} else {{
                            response.json().then(res => {{
                                alert('Error: ' + res.error.type);
                                submitBtn.textContent = 'Submit';
                                submitBtn.disabled = false;
                            }}).catch(error => {{
                                alert('An error occurred: ' + error.message);
                                submitBtn.textContent = 'Submit';
                                submitBtn.disabled = false;
                            }});
                        }}
                    }}).catch(error => {{
                        alert('An error occurred: ' + error.message);
                        submitBtn.textContent = 'Submit';
                        submitBtn.disabled = false;
                    }});
                }}
            }});
        </script>
    </body>
    </html>
    "#, id = id, api_key = dotenv::var("X_API_KEY").unwrap()))
}

