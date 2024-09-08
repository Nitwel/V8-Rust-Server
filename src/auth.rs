use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
    middleware::{self, Next},
    response::Response,
};

use crate::{
    users::{get_users, User},
    AppState,
};

#[derive(Clone)]
pub struct AuthState {
    pub id: i64,
}

pub async fn auth(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = headers
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    println!("{:?}", auth_header);

    match auth_header {
        Some(auth_header) => {
            let user = token_is_valid(&state.pool, auth_header).await;

            if user.is_none() {
                return Err(StatusCode::UNAUTHORIZED);
            }

            let user = user.unwrap();

            let auth_state = AuthState { id: user.id };

            req.extensions_mut().insert(auth_state);

            Ok(next.run(req).await)
        }

        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

async fn token_is_valid(pool: &sqlx::SqlitePool, token: &str) -> Option<User> {
    let users = get_users(pool).await;

    for user in users {
        let pw = user.password.clone();
        // add user.last_login to the token

        if user.last_login.is_none() {
            continue;
        }

        let pw = pw + &user.clone().last_login.unwrap_or(String::new());

        if bcrypt::verify(&pw, token).unwrap_or(false) {
            return Some(user);
        }
    }

    None
}
