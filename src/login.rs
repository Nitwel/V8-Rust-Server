use std::sync::Arc;

use axum::{debug_handler, extract::State, http::StatusCode, Extension, Json};
use serde::{Deserialize, Serialize};

use crate::{
    auth::AuthState,
    users::{clear_last_login, get_user_by_name, update_last_login},
    AppState,
};

#[derive(Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[debug_handler]
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Login>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let user = get_user_by_name(&state.pool, &payload.username).await;

    match user {
        Some(user) => {
            if payload.password == user.password {
                let time = update_last_login(&state.pool, user.id).await;
                let token = bcrypt::hash(user.password + &time, 4).unwrap();
                Ok(Json(LoginResponse { token }))
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}

#[debug_handler]
pub async fn logout(
    Extension(current_user): Extension<AuthState>,
    State(state): State<Arc<AppState>>,
) {
    clear_last_login(&state.pool, current_user.id).await;
}
