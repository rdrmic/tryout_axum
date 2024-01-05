use tokio::net::TcpListener;

use crate::{config::Config, db, router};

pub struct App {
    config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn run(&self) {
        let tcp_listener = TcpListener::bind(&self.config.app_url).await.unwrap();

        let db_connection_pool = db::create_db_connection_pool(
            &self.config.db_url,
            self.config.db_min_connections,
            self.config.db_max_connections,
        )
        .await;
        let router = router::create_router(db_connection_pool);

        axum::serve(tcp_listener, router).await.unwrap();
    }
}
