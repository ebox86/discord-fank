use sqlx::PgPool;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
use crate::db;

#[get("/show")]
pub async fn show(state: &State<PgPool>) -> Result<Json<Vec<db::Rank>>, BadRequest<String>> {
    let rank: Vec<db::Rank> = sqlx::query_as("SELECT * FROM rank")
        .fetch_all(&**state)
        .await
        .map_err(|e| BadRequest(Some(e.to_string())))?;
    Ok(Json(rank.into_iter().collect()))
}