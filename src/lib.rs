mod commands;
mod db;
mod services;

use log::info;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
use serenity::prelude::*;
use serenity::{async_trait, model::prelude::GuildId};
use shuttle_service::error::CustomError;
use shuttle_service::SecretStore;
use sqlx::{Executor, PgPool};

struct Bot {
    iex_api_key: String,
    _client: reqwest::Client,
	discord_guild_id: GuildId,
    database: PgPool,
}

#[shuttle_service::main]
async fn serenity(#[shared::Postgres] pool: PgPool) -> shuttle_service::ShuttleSerenity {
    // discord keys and guild id here
    let discord_token = pool
        .get_secret("DISCORD_TOKEN")
        .await
        .map_err(CustomError::new)?;

    let discord_guild_id = pool
        .get_secret("DISCORD_GUILD_ID")
        .await
        .map_err(CustomError::new)?;

    // api keys here
    let iex_api_key = pool
        .get_secret("IEX_API_KEY")
        .await
        .map_err(CustomError::new)?;

    pool.execute(include_str!("../schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let pool = pool;

    let client = get_client(
        &discord_token,
        &iex_api_key,
        discord_guild_id.parse().unwrap(),
        pool
    )
    .await;

    Ok(client)
}

pub async fn get_client(
    discord_token: &str,
    polygon_api_key: &str,
    discord_guild_id: u64,
    pool: PgPool
) -> Client {
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    Client::builder(discord_token, intents)
        .event_handler(Bot {
            iex_api_key: polygon_api_key.to_owned(),
            _client: reqwest::Client::new(),
            discord_guild_id: GuildId(discord_guild_id),
            database: pool,
        })
        .await
        .expect("Err creating client")
}

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let _commands = GuildId::set_application_commands(&self.discord_guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::register(command))
                .create_application_command(|command| commands::fun::register(command))
                .create_application_command(|command| commands::rank::register(command))
                .create_application_command(|command| commands::watchlist::register(command))
        })
        .await;
    }

    async fn message(&self, _ctx: Context, msg: Message) {
        let message_xp = 3;
        let channel_id = msg.channel_id;
        if !msg.author.bot {
            let level_up = db::increment_level(&self.database, msg.author.id.to_string().parse::<i64>().unwrap(), msg.author.name, msg.timestamp.unix_timestamp(), message_xp).await;
            if level_up.1 {
                channel_id.send_message(&_ctx.http, 
                    |m| {
                        m.content(
                            format!(
                                "Welcome to level {}!\nNext level at {} xp.", 
                                level_up.0, 
                                commands::rank::level_cost(level_up.0 as f64)))}).await.unwrap();
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            //let invoking_user = &interaction.application_command().unwrap().member.unwrap().user
            let invoking_user = &command.member.as_ref().unwrap().user;
            println!("Received command interaction: {:#?}", command.data.name);

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "watchlist" => {
                    let command = command.data.options.get(0).expect("Expected a command");
                    match command.name.as_str() {
                        "add" => {
                            let mut ticker: Vec<&str> = Vec::new();
                            if let CommandDataOptionValue::String (_ticker) = command.options[0]
                                .resolved
                                .as_ref()
                                .expect("Expected String")
                            {
                                ticker = _ticker.split(",").collect::<Vec<&str>>();
                            }

                            db::add_watchlist(&self.database, invoking_user.id.to_string().parse::<i64>().unwrap(), ticker).await.unwrap()
                        },
                        "list" => db::list_watchlist(&self.database).await.unwrap(),
                        "clear" => {
                            let user_id: i64 = invoking_user.id.to_string().parse::<i64>().unwrap();
                            db::clear_watchlist(&self.database, user_id).await.unwrap()
                        },
                        "show" => {
                            let user_id: i64 = invoking_user.id.to_string().parse::<i64>().unwrap();
                            db::show_watchlist(&self.database, &self.iex_api_key, user_id).await.unwrap()
                        },
                        _ => "Please enter a watchlist command".to_string(),
                    }
                },
                "rank" => {
                    let command = command.data.options.get(0).expect("Expected command");
                    match command.name.as_str() {
                        //Rank subcommands here
                        "stats" => {
                            let mut user_id: i64 = 0;
                            let mut user_name: String = String::new();
                            if command.options.len() < 1
                            {
                                user_id = invoking_user.id.to_string().parse::<i64>().unwrap();
                                user_name = invoking_user.name.clone();
                            } else {
                                let user = command.options.get(0).expect("Expected user");
                                match user.resolved.as_ref().unwrap() {
                                    CommandDataOptionValue::User(_user, _) => {
                                        user_id = _user.id.to_string().parse::<i64>().unwrap();
                                        user_name = user.name.clone();
                                    },
                                    _ => {
                                        println!("Expected user");
                                    }
                                }
                            }
                            let current = db::get_count_and_level(&self.database, user_id).await;
                            let next_cost = commands::rank::level_cost(current.1 as f64);
                            format!("{} is level {} with {} xp.\nNext level at {} xp.", user_name, current.1, current.0, next_cost)
                        },
                        "list" => {
                            db::list_rank(&self.database).await.unwrap()
                        },
                        "promote" => {
                            let mut user_id: i64 = 0;
                            let mut user_name: String = String::new();
                            let mut value = 0;
                            if command.options.len() < 2
                            {
                                panic!("Expected User Arguments '[User] [Value]'")
                            }
                            if let CommandDataOptionValue::User(_user, _member) = command.options[0]
                                .resolved
                                .as_ref()
                                .expect("Expected User Object")
                            {
                                user_id = _user.id.to_string().parse::<i64>().unwrap().clone();
                                user_name = _user.name.clone();
                            }

                            if let CommandDataOptionValue::Number(_value) = command.options[1]
                                .resolved
                                .as_ref()
                                .expect("Expected User Object")
                            {
                                value = _value.clone().round() as i64;
                            }
                            let current = db::get_count_and_level(&self.database, user_id).await;
                            let new_level = commands::rank::calculate_level(current.0 + value, current.1);
                            db::update_level(&self.database, user_id, user_name, current.0 + value, new_level.0).await.unwrap()
                        },
                        _ => "Please enter a valid todo".to_string(),
                    }
                },
                "fun" => commands::fun::run(&command.data.options),
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
}

