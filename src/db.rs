use serenity::model::{prelude::Message, user::User};
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

pub fn calculate_level() -> i64 {
    1
}

pub(crate) async fn get_level(pool: &PgPool, user: User) -> Result<i64, sqlx::Error> {
    let table: Vec<Rank> =
        sqlx::query_as("SELECT level FROM rank WHERE user_id = $1")
            .bind(user.id.to_string())
            .fetch_all(pool)
            .await?;

    let response = table.first().unwrap().level;
    Ok(response)
}

pub(crate) async fn get_msg_count(pool: &PgPool, user: User) -> Result<i64, sqlx::Error> {
    let table: Vec<Rank> =
        sqlx::query_as("SELECT msg_count FROM rank WHERE user_id = $1")
            .bind(user.id.to_string())
            .fetch_all(pool)
            .await?;

    let response = table.first().unwrap().level;
    Ok(response)
}

pub(crate) async fn insert(pool: &PgPool, msg: Message) -> Result<String, sqlx::Error> {
    let user_id_string = msg.author.id.to_string();
    //let current_msg_count = get_msg_count(pool, msg.author).await?;
    sqlx::query("INSERT INTO rank (user_id, user_name, last_msg, msg_count, level) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (user_id) WHERE user_id = $1 DO UPDATE SET last_msg=$3, msg_count=rank.msg_count + 1, level=$5")
        .bind(user_id_string.parse::<i64>().unwrap())
        .bind(msg.author.name)
        .bind(msg.timestamp.unix_timestamp())
        .bind(1)
        .bind(calculate_level())
        .execute(pool)
        .await?;

    Ok(format!("updated 1 row"))
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