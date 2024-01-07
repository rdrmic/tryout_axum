use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, Method, StatusCode, Uri},
    response::{Html, IntoResponse},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;

use crate::db;

pub async fn fallback(uri: Uri) -> impl IntoResponse {
    println!("-> fallback // {uri}");

    let page_404 = include_str!("../404.html").replacen(":@", uri.path(), 1);
    (StatusCode::NOT_FOUND, Html(page_404))
}

pub async fn handle_root(method: Method, headers: HeaderMap) -> impl IntoResponse {
    println!("-> handle_root");

    (
        StatusCode::IM_A_TEAPOT,
        format!("method: {method}\nheaders: {headers:?}"),
    )
}

pub async fn handle_get(
    Path((user_id, age)): Path<(u8, u8)>,
    Query(params): Query<HashMap<String, String>>,
) -> Html<String> {
    println!("-> handle_get");

    Html(format!(
        "<ul style='color:green'><li>user: {user_id}</li><li>age: {age}</li></ul><p style='color:red'>{params:?}</p>"
    ))
}

pub async fn handle_post(Json(mut payload): Json<UserPayload>) -> Json<UserPayload> {
    println!("-> handle_post");

    payload.user += " Šaulić";
    payload.age += 10;
    Json(payload)
}

#[derive(Deserialize, Serialize)]
pub struct UserPayload {
    user: String,
    age: u8,
}

// TODO tokio::sync::Mutex

pub async fn fetch_all(State(db_connection_pool): State<PgPool>) -> String {
    println!("-> fetch_all");

    let records = db::fetch_all(db_connection_pool).await;
    format!("{records:#?}")
}

pub async fn find_by_id(State(db_connection_pool): State<PgPool>) -> String {
    println!("-> find_by_id");

    let record = db::find_by_id(db_connection_pool).await;
    format!("{record:#?}")
}

pub async fn insert(State(db_connection_pool): State<PgPool>) -> String {
    println!("-> insert");

    let inserted_record = db::insert(db_connection_pool).await;
    format!("{inserted_record:#?}")
}

pub async fn update(State(db_connection_pool): State<PgPool>) -> String {
    println!("-> update");

    let updated_record = db::update(db_connection_pool).await;
    format!("{updated_record:#?}")
}

pub async fn delete_by_id(State(db_connection_pool): State<PgPool>) -> String {
    println!("-> delete_by_id");

    let deleted_record = db::delete_by_id(db_connection_pool).await;
    format!("deleted: {}", deleted_record.is_some())
}
