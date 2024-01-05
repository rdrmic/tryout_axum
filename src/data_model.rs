use sqlx::types::time::PrimitiveDateTime;

#[derive(Debug, Clone)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub updated: PrimitiveDateTime,
}
