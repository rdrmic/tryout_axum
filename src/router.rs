use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;

use crate::handlers::{
    delete_by_id, fallback, fetch_all, find_by_id, handle_get, handle_post, handle_root, insert,
    update,
};

pub fn create_router(db_connection_pool: PgPool) -> Router {
    Router::new()
        .fallback(fallback)
        .route("/", get(handle_root).post(handle_root))
        .route("/user/:user_id/age/:age", get(handle_get))
        .route("/send_data", post(handle_post))
        // DB CRUD
        .route("/fetch_all", get(fetch_all))
        .route("/find_by_id", get(find_by_id))
        .route("/insert", get(insert))
        .route("/update", put(update))
        .route("/delete_by_id", delete(delete_by_id))
        .with_state(db_connection_pool)
}
