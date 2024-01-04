use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::{fallback, handle_get, handle_get_records, handle_post, handle_root};

pub fn create_router() -> Router {
    Router::new()
        .fallback(fallback)
        .route("/", get(handle_root).post(handle_root))
        .route("/user/:user_id/age/:age", get(handle_get))
        .route("/send_data", post(handle_post))
        .route("/get_records", get(handle_get_records))
}
