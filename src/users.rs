use std::sync::Arc;

use axum::{debug_handler, extract::State, Json};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, sqlite::SqlitePool};

use crate::AppState;

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub last_login: Option<String>,
}

#[debug_handler]
pub async fn handle_get_users(State(state): State<Arc<AppState>>) -> Json<Vec<User>> {
    let users = get_users(&state.pool).await;

    Json(users)
}

pub async fn get_users(pool: &SqlitePool) -> Vec<User> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool)
        .await
        .unwrap();

    users
}

pub async fn get_user_by_name<'a>(pool: &'a SqlitePool, username: &'a str) -> Option<User> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(pool)
        .await
        .unwrap();

    user
}

pub async fn get_user(pool: &SqlitePool, id: i64) -> Option<User> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
        .unwrap();

    user
}

pub async fn update_last_login(pool: &SqlitePool, id: i64) -> String {
    let result = sqlx::query("UPDATE users SET last_login = datetime('now') WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();

    get_user(pool, id).await.unwrap().last_login.unwrap()
}

pub async fn clear_last_login(pool: &SqlitePool, id: i64) {
    let result = sqlx::query("UPDATE users SET last_login = NULL WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
}
