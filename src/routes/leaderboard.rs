
// use http::StatusCode;
// use std::error::Error;
// struct DbError(sqlx::Error);
// use axum::{response::IntoResponse, routing::get, Json, http};
// type Tx = axum_sqlx_tx::Tx<sqlx::Postgres>;


// pub async fn top(mut tx: Tx) -> Result<Json<Vec<String>>, DbError> {
//     let ranks: Vec<(String,)> = sqlx::query_as("SELECT * FROM rank")
//         .fetch_all(&mut tx)
//         .await?;

//     Ok(Json(ranks.into_iter().map(|n| n.0).collect()))
// }