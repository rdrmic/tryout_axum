use sqlx::types::time::PrimitiveDateTime;

#[derive(Debug)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub updated: PrimitiveDateTime,
}
