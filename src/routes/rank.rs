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

#[get("/show/<guild_id>")]
pub async fn show_by_guild(guild_id: i64, state: &State<PgPool>) -> Result<Json<Vec<db::GuildRank>>, BadRequest<String>> {
    let rank: Vec<db::GuildRank> = sqlx::query_as("SELECT * FROM rank WHERE guild_id = $1")
        .bind(guild_id)
        .fetch_all(&**state)
        .await
        .map_err(|e| BadRequest(Some(e.to_string())))?;
    Ok(Json(rank.into_iter().collect()))
}
