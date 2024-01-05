use sqlx::{postgres::PgPoolOptions, query_as, Pool, Postgres};

use crate::data_model::Item;

pub async fn fetch_all(db_connection_pool: Pool<Postgres>) -> Vec<Item> {
    query_as!(Item, "SELECT * FROM item")
        .fetch_all(&db_connection_pool)
        .await
        .unwrap()
}

pub async fn create_db_connection_pool(
    url: &str,
    min_connections: u32,
    max_connections: u32,
) -> Pool<Postgres> {
    PgPoolOptions::new()
        .min_connections(min_connections)
        .max_connections(max_connections)
        .connect(url)
        .await
        .expect("Couldn't connect to the database")
}
