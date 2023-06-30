pub mod account;

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, pool};

pub struct Connection {
    pub pool: PgPool,
}

pub fn new(pool: PgPool) -> Connection {
    println!("Database connection initializing...");
    let conn = Connection { pool };
    return conn;
}


#[derive(Serialize, FromRow)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub last_msg: i64,
}

#[derive(Serialize, FromRow)]
pub struct Guild {
    pub id: i64,
    pub name: String,
}

#[derive(Serialize, FromRow)]
pub struct Rank {
    pub guild_id: i64,
    pub user_id: i64,
    pub xp: i64,
    pub level: i64,
    pub rank: i64,
}

#[derive(FromRow)]
pub struct Watchlist {
    pub guild_id: i64,
    pub user_id: i64,
    pub list: String
}

#[derive(FromRow)]
pub struct Competitions {
    pub id: i32,
    pub name: String,
    pub guild_id: i64,
    pub active: bool,
    pub reg_open: bool,
    pub start_date: i64,
    pub end_date: i64,
    pub winner: i64,
    pub image: String,
    pub color: String,
  }

#[derive(FromRow)]
pub struct CompList {
    pub guild_id: i64,
    pub user_id: i64,
    pub list: String,
    pub comp_id: i64
}

#[derive(FromRow)]
pub struct Message {
    pub guild_id: i64,
    pub user_id: i64,
    pub channel_id: i64,
    pub timestamp: i64,
}