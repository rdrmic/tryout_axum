mod db;
mod handlers;
mod router;

#[tokio::main]
async fn main() {
    let router = router::create_router();

    // run our app with hyper, listening globally on port 3000
    let tcp_listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(tcp_listener, router).await.unwrap();
}
