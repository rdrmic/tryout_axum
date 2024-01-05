use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, Method, StatusCode, Uri},
    response::{Html, IntoResponse},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
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

//tokio::sync::Mutex

pub async fn fetch_all(State(db_connection_pool): State<Pool<Postgres>>) -> String {
    println!("-> fetch_all");

    //let db_connection = db_connection_pool.acquire().await.unwrap();
    //let db_connection = db_connection_pool.begin().await.unwrap();

    let records = db::fetch_all(db_connection_pool).await;
    format!("{records:#?}")
}
