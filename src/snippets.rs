use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, SqlitePool};

use crate::{auth::AuthState, AppState};

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Snippet {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub created_at: String,
    pub user_id: i64,
}

pub async fn handle_get_snippets(
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<AuthState>,
) -> Json<Vec<Snippet>> {
    let snippets = get_snippets(&state.pool, current_user.id).await;

    Json(snippets)
}

pub async fn get_snippets(pool: &SqlitePool, user_id: i64) -> Vec<Snippet> {
    let snippets = sqlx::query_as::<_, Snippet>("SELECT * FROM snippets WHERE user_id = ?")
        .bind(user_id)
        .fetch_all(pool)
        .await
        .unwrap();

    snippets
}

pub async fn handle_get_snippet(
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<AuthState>,
    Path(id): Path<i64>,
) -> Result<Json<Snippet>, StatusCode> {
    let snippet = get_snippet(&state.pool, id).await;

    if snippet.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    if snippet.clone().unwrap().user_id != current_user.id {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(Json(snippet.unwrap()))
}

pub async fn get_snippet(pool: &SqlitePool, id: i64) -> Option<Snippet> {
    let snippet = sqlx::query_as::<_, Snippet>("SELECT * FROM snippets WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
        .unwrap();

    snippet
}

#[derive(Deserialize)]
pub struct CreateSnippet {
    title: String,
    body: String,
    user_id: Option<i64>,
}

pub async fn handle_create_snippet(
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<AuthState>,
    Json(mut snippet): Json<CreateSnippet>,
) -> Json<i64> {
    snippet.user_id = Some(current_user.id);

    let id: i64 = create_snippet(&state.pool, snippet).await;

    Json(id)
}

pub async fn create_snippet(pool: &SqlitePool, snippet: CreateSnippet) -> i64 {
    let result = sqlx::query("INSERT INTO snippets (title, body, user_id) VALUES (?, ?, ?)")
        .bind(snippet.title)
        .bind(snippet.body)
        .bind(snippet.user_id.unwrap())
        .execute(pool)
        .await
        .unwrap();

    result.last_insert_rowid()
}

pub async fn handle_update_snippet(
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<AuthState>,
    Path(id): Path<i64>,
    Json(snippet): Json<CreateSnippet>,
) -> Result<Json<()>, StatusCode> {
    let existing_snippet = get_snippet(&state.pool, id).await;

    if existing_snippet.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    if existing_snippet.unwrap().user_id != current_user.id {
        return Err(StatusCode::FORBIDDEN);
    }

    update_snippet(&state.pool, id, snippet).await;

    Ok(Json(()))
}

pub async fn update_snippet(pool: &SqlitePool, id: i64, snippet: CreateSnippet) {
    let result = sqlx::query("UPDATE snippets SET title = ?, body = ? WHERE id = ?")
        .bind(snippet.title)
        .bind(snippet.body)
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
}

pub async fn handle_delete_snippet(
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<AuthState>,
    Path(id): Path<i64>,
) -> Result<Json<()>, StatusCode> {
    let existing_snippet = get_snippet(&state.pool, id).await;

    if existing_snippet.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    if existing_snippet.unwrap().user_id != current_user.id {
        return Err(StatusCode::FORBIDDEN);
    }

    delete_snippet(&state.pool, id).await;

    Ok(Json(()))
}

pub async fn delete_snippet(pool: &SqlitePool, id: i64) {
    let result = sqlx::query("DELETE FROM snippets WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
}
