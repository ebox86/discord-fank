// use std::sync::Arc;

// use axum::response::IntoResponse;
// use reqwest::StatusCode;
// use shuttle_service::database;
// use sqlx::PgPool;

// use crate::db;

// pub async fn top(pool: PgPool) -> Result<axum::Json<Vec<String>>, (StatusCode, String)> {
//     let table = db::list_rank(&pool).await.unwrap();
//     let mut msg = String::new();
//     for (i, row) in table.chars().enumerate() {
//         writeln!(&mut msg, "{}. userid: {} points: {} last_msg: {} level: {}", i + 1, line.user_id, line.points, line.last_msg, line.level).unwrap();    
//     }
//     Ok(axum::Json(vec![msg]))
// }