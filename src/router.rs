use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{Pool, Postgres};

use crate::handlers::{fallback, fetch_all, handle_get, handle_post, handle_root};

pub fn create_router(db_connection_pool: Pool<Postgres>) -> Router {
    Router::new()
        .fallback(fallback)
        .route("/", get(handle_root).post(handle_root))
        .route("/user/:user_id/age/:age", get(handle_get))
        .route("/send_data", post(handle_post))
        .route("/fetch_all", get(fetch_all))
        .with_state(db_connection_pool)
}
