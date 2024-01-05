mod app;
mod config;
mod data_model;
mod db;
mod handlers;
mod router;

#[tokio::main]
async fn main() {
    let config = config::Config::from_env();
    app::App::new(config).run().await;
}
