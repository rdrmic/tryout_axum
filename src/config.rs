#[derive(Debug)]
pub struct Config {
    pub app_url: String,
    pub db_url: String,
    pub db_min_connections: u32,
    pub db_max_connections: u32,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        Self {
            app_url: format!(
                "{}:{}",
                std::env::var("APP_HOST").unwrap(),
                std::env::var("APP_PORT").unwrap()
            ),
            db_url: std::env::var("DATABASE_URL").unwrap(),
            db_min_connections: std::env::var("DATABASE_MIN_CONNECTIONS")
                .unwrap()
                .parse()
                .unwrap(),
            db_max_connections: std::env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap()
                .parse()
                .unwrap(),
        }
    }
}
