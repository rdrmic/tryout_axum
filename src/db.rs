use sqlx::{postgres::PgPoolOptions, query_as, FromRow, Pool, Postgres};

#[derive(Debug, FromRow, Clone)]
pub struct PwsAlertLevel {
    pub id: i64,
    pub cmas_level: String,
    pub eu_alert_level: String,
}

pub async fn fetch_all_records() -> Vec<PwsAlertLevel> {
    let db_connection_pool = create_db_connection_pool().await;

    query_as::<_, PwsAlertLevel>("SELECT * FROM cbc.pws_alert_level")
        .fetch_all(&db_connection_pool)
        .await
        .expect("Failed to fetch records from cbc.pws_alert_level")
}

async fn create_db_connection_pool() -> Pool<Postgres> {
    PgPoolOptions::new()
        .min_connections(1)
        .max_connections(1)
        .connect("postgres://postgres:postgres@localhost:5432/cbc_db")
        .await
        .expect("Couldn't connect to the database")
}
