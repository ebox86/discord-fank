use sqlx::{FromRow, PgPool};
use std::{fmt::Write};
use chrono::Utc;
use crate::{commands::rank, services::{self, stocks::price_diff_formatter}};

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

#[derive(FromRow)]
struct CompList {
    pub user_id: i64,
    pub list: String,
    pub comp_id: i64
}

#[derive(FromRow)]
struct Competitions {
    pub id: i32,
    pub active: bool,
    pub name: String,
    pub start_date: i64,
    pub end_date: i64
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
    for s in &list {
        current_list.push(s.trim().to_string().to_uppercase());
    }
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
            writeln!(&mut response, "{}. {}\t${}\t{}", i + 1, ticker, quote.0.to_string(), price_diff_formatter(quote.0, quote.1)).unwrap();
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

pub(crate) async fn create_comp(pool: &PgPool, name: String, start: i64, end: i64) -> Result<String, sqlx::Error> {
    let _table =
        sqlx::query("INSERT INTO competitions (active, start_date, end_date, name) VALUES ($1, $2, $3, $4)")
            .bind(true)
            .bind(start)
            .bind(end)
            .bind(name)
            .execute(pool)
            .await.unwrap();

    Ok(format!("{}", "Competition created".to_string()))
}

pub(crate) async fn list_comp(pool: &PgPool, user_id: i64) -> Result<String, sqlx::Error> {
    let table: Vec<Competitions> =
        sqlx::query_as("SELECT * FROM competitions WHERE active = true")
            .fetch_all(pool)
            .await?;
    let mut response = if table.len() >= 1 {format!("🏁 Current running competitions: {}\n(to register, take the `id` and run the `/comp register <id> <stock_list>`)\n", table.len())} else {format!("❌ No active competitions\n")};
    for (_, line) in table.iter().enumerate() {
        let is_participant = is_registered(pool, user_id, line.id.into()).await;
        let participant_string = if is_participant {format!("You are registered! ✅ ")} else {format!("")};
        writeln!(&mut response, "ID: {}\tName: {}\t{}", line.id, line.name, participant_string).unwrap();
    }

    Ok(response)
}

pub(crate) async fn register_comp(pool: &PgPool, user_id: i64, comp_id: i64, list: Vec<&str>, iex_api_key: String) -> Result<String, sqlx::Error> {
    print!("adding {} to comp {} for user {}\n", format!("{:?}", list), comp_id, user_id);
    let valudate_comp_id = sqlx::query("SELECT * FROM competitions WHERE id = $1 AND active = true")
        .bind(comp_id)
        .fetch_all(pool)
        .await.unwrap();
    if valudate_comp_id.len() == 0 {
        return Ok(format!("❌ Invalid competition ID"));
    }
    let mut current_list: Vec<String> = get_comp(pool, user_id, comp_id).await;
    if current_list.len() > 0 {
        return Ok(format!("❌ You are already registered for this competition!"));
    }
    for s in &list {
        current_list.push(s.trim().to_string().to_uppercase());
    }

    for symbol in &current_list {
        let quote = services::stocks::get_quote(iex_api_key.to_string(), symbol.to_string()).await;
        if quote.0 == 0.0 {
            return Ok(format!("❌ {} is not a valid stock symbol", symbol));
        }
    }

    let _table: Vec<Competitions> =
        sqlx::query_as("INSERT INTO complist (user_id, comp_id, list) VALUES ($1, $2, $3)")
            .bind(user_id)
            .bind(comp_id)
            .bind(serde_json::to_string(&current_list).unwrap())
            .fetch_all(pool)
            .await?;

    Ok(format!("✅ Registered for competition {} with list {}", comp_id, format!("{:?}\n\nGood Luck!", current_list)))
}

pub(crate) async fn get_comp(pool: &PgPool, user_id: i64, comp_id: i64) -> Vec<String> {
    let table: Vec<CompList> =
        sqlx::query_as("SELECT * FROM complist WHERE user_id = $1 AND comp_id = $2")
            .bind(user_id)
            .bind(comp_id)
            .fetch_all(pool)
            .await.unwrap();

    return if table.get(0).is_some() {serde_json::from_str(table.get(0).unwrap().list.as_str()).expect("Couldn't deserialize recipients") } else {Vec::<String>::new()};
}

pub (crate) async fn is_registered(pool: &PgPool, user_id: i64, comp_id: i64) -> bool {
    let table: Vec<CompList> =
        sqlx::query_as("SELECT * FROM complist WHERE user_id = $1 AND comp_id = $2")
            .bind(user_id)
            .bind(comp_id)
            .fetch_all(pool)
            .await.unwrap();

    return table.len() > 0;
}