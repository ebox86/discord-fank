use sqlx::{FromRow, PgPool};
use std::{fmt::Write};
use chrono::Utc;
use crate::{commands::rank, services};

#[derive(FromRow)]
struct Rank {
    pub user_id: i64,
    pub last_msg: i64,
    pub points: i64,
    pub level: i64
}
#[derive(FromRow)]
struct Watchlist {
    pub user_id: i64,
    pub list: String
}

pub(crate) async fn update_level(pool: &PgPool, user_id: i64, user_name: String, points: i64, level: i64) -> Result<String, sqlx::Error> {
    print!("updating user with new points: {} and level: {}\n", points, level);
    let _table: Vec<Rank> =
        sqlx::query_as("INSERT INTO rank (user_id, user_name, last_msg, points, level) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (user_id) WHERE user_id = $1 DO UPDATE SET points=$4, level=$5")
            .bind(user_id)
            .bind(user_name)
            .bind(Utc::now().timestamp())
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
    return if table.get(0).is_some(){(table.get(0).unwrap().points, table.get(0).unwrap().level)} else {(0, 0)};
}

pub(crate) async fn increment_level(pool: &PgPool, user_id: i64, user_name: String, last_msg: i64, points: i64) -> (i64,bool) {
    let count_and_level = get_count_and_level(pool, user_id).await;
    let level = rank::calculate_level(count_and_level.0, count_and_level.1);
    let _insert = sqlx::query("INSERT INTO rank (user_id, user_name, last_msg, points, level) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (user_id) WHERE user_id = $1 DO UPDATE SET last_msg=$3, points=rank.points + $4, level=$5")
        .bind(user_id)
        .bind(user_name)
        .bind(last_msg)
        .bind(points)
        .bind(level.0)
        .execute(pool)
        .await;
    return level;
}

pub(crate) async fn list_rank(pool: &PgPool) -> Result<String, sqlx::Error> {
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

pub(crate) async fn add_watchlist(pool: &PgPool, user_id: i64, list: Vec<&str>) -> Result<String, sqlx::Error> {
    print!("adding {} to watchlist for user {}\n", format!("{:?}", list), user_id);
    let mut current_list: Vec<String> = get_watchlist(pool, user_id).await;
    print!("current list: {:?}\n", current_list);
    //current_list.append(list.iter().map(|s| s.as_str()).collect());
    for s in &list {
        current_list.push(s.to_string());
    }
    print!("new list: {:?}\n", current_list);
    let _table: Vec<Watchlist> =
        sqlx::query_as("INSERT INTO watchlist (user_id, list) VALUES ($1, $2) ON CONFLICT (user_id) WHERE user_id = $1 DO UPDATE SET list=$2")
            .bind(user_id)
            .bind(serde_json::to_string(&current_list).unwrap())
            .fetch_all(pool)
            .await?;

    Ok(format!("{}", "Added 1 row"))
}

pub(crate) async fn get_watchlist(pool: &PgPool, user_id: i64) -> Vec<String> {
    let table: Vec<Watchlist> =
        sqlx::query_as("SELECT * FROM watchlist WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(pool)
            .await.unwrap();

    return if table.get(0).is_some() {serde_json::from_str(table.get(0).unwrap().list.as_str()).expect("Couldn't deserialize recipients") } else {Vec::<String>::new()};
}

pub(crate) async fn clear_watchlist(pool: &PgPool, user_id: i64) -> Result<String, sqlx::Error> {
    let _table: Vec<Rank> =
        sqlx::query_as("DELETE FROM watchlist WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(pool)
            .await?;

    Ok(format!("{}", "Removed 1 row"))
}

pub(crate) async fn show_watchlist(pool: &PgPool, iex_api_key: &String, user_id: i64) -> Result<String, sqlx::Error> {
    let table: Vec<String> = get_watchlist(pool, user_id).await;
    if table.len() == 0 {
        return Ok(format!("❌ Your watchlist is empty!\n Use `/watchlist add <symbol>` to add one"));
    } else {
        let mut response = "Your watchlist 👇:\n".to_string();
        for (i, ticker) in table.iter().enumerate() {
            let quote = services::stocks::get_quote(iex_api_key.to_string(), ticker.to_string()).await;
            writeln!(&mut response, "{}. {}\t${}", i + 1, ticker, quote.as_str()).unwrap();
        }
        Ok(response)
    }
}

pub(crate) async fn list_watchlist(pool: &PgPool) -> Result<String, sqlx::Error> {
    let table: Vec<Watchlist> =
        sqlx::query_as("SELECT * FROM watchlist")
            //.bind(user_id)
            .fetch_all(pool)
            .await?;

    let mut response = format!("Current table is size {}:\n", table.len());
    for (i, line) in table.iter().enumerate() {
        writeln!(&mut response, "{}. userid: {} list: {}", i + 1, line.user_id, line.list).unwrap();
    }

    Ok(response)
}