use axum::{
    extract::{Path, Query},
    http::{HeaderMap, Method},
    response::Html,
    Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub async fn handle_root(method: Method, headers: HeaderMap) -> String {
    println!("-> handle_root");
    format!("method: {method}\nheaders: {headers:?}")
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

#[derive(Deserialize, Serialize, Debug)]
pub struct UserPayload {
    user: String,
    age: u8,
}
