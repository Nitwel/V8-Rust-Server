use dotenv::dotenv;
use sqlx::sqlite::SqlitePool;
use std::env;
use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
mod auth;
mod login;
mod post_run_js;
mod snippets;
mod users;

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let shared_state = Arc::new(AppState { pool: pool });

    init_v8().await;

    let app = Router::new()
        .route("/api/users", get(users::handle_get_users))
        .route("/api/logout", post(login::logout))
        .route(
            "/api/snippets",
            get(snippets::handle_get_snippets).post(snippets::handle_create_snippet),
        )
        .route(
            "/api/snippets/:id",
            get(snippets::handle_get_snippet)
                .put(snippets::handle_update_snippet)
                .delete(snippets::handle_delete_snippet),
        )
        .route_layer(middleware::from_fn_with_state(
            shared_state.clone(),
            auth::auth,
        ))
        .route("/api/login", post(login::login))
        .route("/api/run", post(post_run_js::post_run_js))
        .nest_service("/", ServeDir::new("public"))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn init_v8() {
    let platform = v8::new_default_platform(0, false).make_shared();

    v8::V8::initialize_platform(platform);
    v8::V8::initialize();
}
