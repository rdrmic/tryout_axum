mod handlers;

use axum::{
    routing::{get, post},
    Router,
};
use handlers::{handle_get, handle_post, handle_root};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(handle_root).post(handle_root))
        .route("/user/:user_id/age/:age", get(handle_get))
        .route("/send_data", post(handle_post));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
