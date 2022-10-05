use serde::__private::de;
use serenity::{model::{prelude::Message, user::User}, futures::future::ok};
use sqlx::{FromRow, PgPool};
use std::fmt::Write;

#[derive(FromRow)]
struct Rank {
    pub user_id: i64,
    pub user_name: String,
    pub last_msg: i64,
    pub points: i64,
    pub level: i64
}

pub fn level_cost(level: f64) -> f64 {
    let cost = 50.0;
    return level * cost * f64::powf(1.07, level) + (25.0 * level);
}

pub fn calculate_level(points: i64, level: i64) -> (i64, bool) {
    print!("calling calculate level\n");
    print!("points: {} level: {}\n", points, level);
    let mut lvl_as_flt = level as f64;
    let mut level_up = false;
    while points as f64 >= level_cost(lvl_as_flt) {
        print!("points: {} derived cost: {}\nContinuing..\n", points, level_cost(lvl_as_flt));
        level_up = true;
        print!("incrementing to next level!\nCurrent level: {}, New Level: {}\n", lvl_as_flt, (lvl_as_flt + 1.0).round() as i64);
        lvl_as_flt += 1.0;
    }
    return (lvl_as_flt.round() as i64, level_up);
}

pub(crate) async fn update_level(pool: &PgPool, user_id: i64, user_name: String, points: i64, level: i64) -> Result<String, sqlx::Error> {
    print!("updating user with new points: {} and level: {}\n", points, level);
    let table: Vec<Rank> =
        sqlx::query_as("INSERT INTO rank (user_id, user_name, points, level) VALUES ($1, $2, $3, $4) ON CONFLICT (user_id) WHERE user_id = $1 DO UPDATE SET points=$3, level=$4")
            .bind(user_id)
            .bind(user_name)
            .bind(points)
            .bind(level)
            .fetch_all(pool)
            .await?;

    Ok(format!("{}", "Updated 1 row"))
}

pub(crate) async fn get_count_and_level(pool: &PgPool, user_id: i64) -> (i64, i64) {
    let table: Vec<Rank> =
        sqlx::query_as("SELECT * FROM rank WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(pool)
            .await.unwrap();
    return if table.get(0).is_some(){(table.get(0).unwrap().points, table.get(0).unwrap().level)} else {(1, 0)};
}

pub(crate) async fn insert(pool: &PgPool, user_id: i64, user_name: String, last_msg: i64) -> bool {
    let count_and_level = get_count_and_level(pool, user_id).await;
    let level = calculate_level(count_and_level.0, count_and_level.1);
    sqlx::query("INSERT INTO rank (user_id, user_name, last_msg, points, level) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (user_id) WHERE user_id = $1 DO UPDATE SET last_msg=$3, points=rank.points + 1, level=$5")
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
        writeln!(&mut response, "{}. userid: {} points: {} last_msg: {} level: {}", i + 1, line.user_id, line.points, line.last_msg, line.level).unwrap();
    }

    Ok(response)
}