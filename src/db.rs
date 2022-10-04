use serde::__private::de;
use serenity::{model::{prelude::Message, user::User}, futures::future::ok};
use sqlx::{FromRow, PgPool};
use std::fmt::Write;

#[derive(FromRow)]
struct Rank {
    pub user_id: i64,
    pub user_name: String,
    pub last_msg: i64,
    pub msg_count: i64,
    pub level: i64
}

pub fn calculate_level(msg_count: i64, mut level: i64) -> (i64, bool) {
    print!("calling calculate level\n");
    print!("msg_count: {} level: {}\n", msg_count, level);
    let cost = 50.0;
    let lvl_as_flt = level as f64;
    let derived_cost = lvl_as_flt * cost * f64::powf(1.07, lvl_as_flt) + (25.0 * lvl_as_flt);
    print!("derived_cost: {}\n", derived_cost.to_string());
    let mut level_up = false;
    if msg_count as f64 >= derived_cost {
        level_up = true;
        print!("incrementing to next level!\n");
        level+=1;
    }
    return (level, level_up);
}

pub(crate) async fn get_count_and_level(pool: &PgPool, user_id: i64) -> (i64, i64) {
    let table: Vec<Rank> =
        sqlx::query_as("SELECT * FROM rank WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(pool)
            .await.unwrap();
    return if table.get(0).is_some(){(table.get(0).unwrap().msg_count, table.get(0).unwrap().level)} else {(1, 0)};
}

pub(crate) async fn insert(pool: &PgPool, user_id: i64, user_name: String, last_msg: i64) -> bool {
    let count_and_level = get_count_and_level(pool, user_id).await;
    let level = calculate_level(count_and_level.0, count_and_level.1);
    sqlx::query("INSERT INTO rank (user_id, user_name, last_msg, msg_count, level) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (user_id) WHERE user_id = $1 DO UPDATE SET last_msg=$3, msg_count=rank.msg_count + 1, level=$5")
        .bind(user_id)
        .bind(user_name)
        .bind(last_msg)
        .bind(1)
        .bind(level.0)
        .execute(pool)
        .await;
    return level.1;
}

pub(crate) async fn list(pool: &PgPool) -> Result<String, sqlx::Error> {
    let table: Vec<Rank> =
        sqlx::query_as("SELECT * FROM rank")
            //.bind(user_id)
            .fetch_all(pool)
            .await?;

    let mut response = format!("Current table is size {}:\n", table.len());
    for (i, line) in table.iter().enumerate() {
        writeln!(&mut response, "{}. userid: {} msg_count: {} last_msg: {} level: {}", i + 1, line.user_id, line.msg_count, line.last_msg, line.level).unwrap();
    }

    Ok(response)
}