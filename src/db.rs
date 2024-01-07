use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    query_as, PgPool, Pool, Postgres,
};

use crate::data_model::Item;

pub async fn fetch_all(db_connection_pool: Pool<Postgres>) -> Vec<Item> {
    query_as!(Item, "SELECT * FROM item ORDER BY id ASC")
        .fetch_all(&db_connection_pool)
        .await
        .unwrap()
}

pub async fn find_by_id(db_connection_pool: Pool<Postgres>) -> Option<Item> {
    query_as!(Item, "SELECT * FROM item WHERE id = $1", 2)
        .fetch_optional(&db_connection_pool)
        .await
        .unwrap()
}

pub async fn insert(db_connection_pool: Pool<Postgres>) -> Item {
    query_as!(
        Item,
        // FIXME now()
        "INSERT INTO item(name, updated) VALUES ($1, now()) RETURNING *",
        "the first name"
    )
    .fetch_one(&db_connection_pool)
    .await
    .unwrap()
}

pub async fn update(db_connection_pool: Pool<Postgres>) -> Option<Item> {
    query_as!(
        Item,
        "UPDATE item SET name = '1st' WHERE id = $1 RETURNING *",
        1
    )
    .fetch_optional(&db_connection_pool)
    .await
    .unwrap()
}

pub async fn delete_by_id(db_connection_pool: Pool<Postgres>) -> Option<PgRow> {
    query_as!(Item, "DELETE FROM item WHERE id = $1", 3)
        .fetch_optional(&db_connection_pool)
        .await
        .unwrap()
}

pub async fn create_db_connection_pool(
    db_url: &str,
    min_connections: u32,
    max_connections: u32,
) -> PgPool {
    PgPoolOptions::new()
        .min_connections(min_connections)
        .max_connections(max_connections)
        .connect(db_url)
        .await
        .expect("Couldn't connect to the database")
}
