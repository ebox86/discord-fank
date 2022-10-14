use std::sync::Arc;
use std::{fmt::Write};
use clokwerk::{Scheduler, TimeUnits, AsyncScheduler, Job};
use chrono_tz::US::Eastern;
use serenity::http::Http;
use sqlx::PgPool;
use crate::db;

pub(crate) fn start_scheduler(serenity: Arc<serenity::http::Http>, pool: PgPool) {
    let mut async_scheduler = AsyncScheduler::with_tz(Eastern);
    //let mut scheduler = Scheduler::with_tz(Eastern);
    let serenity1 = serenity.clone();
    let serenity2 = serenity.clone();


    // scheduler
    //     .every(1.day())
    //         .at("9:30 am")
    //     .run(|| println!("Market Open! 🔔"));
    async_scheduler
        .every(1.day())
            .at("9:30 am")
        .run(move || send_message(serenity.to_owned(), 1024479108983422986, "Market Open! 🔔".to_string()));
    async_scheduler
        .every(1.day())
            .at("9:30 am")
        .run(move || current_competition_job(serenity2.to_owned(), 1024479108983422986, pool.to_owned()));
    async_scheduler
        .every(1.day())
            .at("4:00 pm")
        .run(move || send_message(serenity1.to_owned(), 1024479108983422986, "Market Closed! 🔔".to_string()));
    //scheduler.every(1.day()).at("4:00 pm").run(|| println!("Market Closed! 🔔"));
    // scheduler.every(1.day()).at("3:44 pm")
    // .run(move || { async { send_message(serenity1.to_owned(), 1024479108983422986, "Test 3:44! 🔔".to_string()).await }; });

    // async_scheduler.every(3.seconds())
    // .run(move || send_message(serenity.to_owned(), 1024479108983422986, "Test 3 secs! 🔔".to_string()));
    // async_scheduler.every(10.seconds())
    // .run(move || send_message(serenity1.to_owned(), 1024479108983422986, "Test 10 secs! 🔔🔔🔔".to_string()));

    tokio::spawn(async move {
        loop {
          async_scheduler.run_pending().await;
          tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
      });
}

pub async fn send_message(http: Arc<Http>, channel_id: u64, msg: String){
    let channel = serenity::model::id::ChannelId(channel_id);
    let _ = channel.say(&http, msg).await;
}

pub async fn current_competition_job(http: Arc<Http>, channel_id: u64, pool: PgPool){
    let channel = serenity::model::id::ChannelId(channel_id);
    let current_competitions = db::get_all_comps(&pool).await;
    let mut response = if current_competitions.len() >= 1 {format!("🏁 Current running competitions: {}\n", current_competitions.len())} else {format!("❌ No active competitions\n")};
    for comp in current_competitions{
        writeln!(&mut response, "{}: {} - {}", comp.name, comp.start_date, comp.end_date).unwrap();
    }
    let _ = channel.say(&http, response).await;
}