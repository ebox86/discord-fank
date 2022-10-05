mod commands;
mod db;

use log::{error, info};
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
use serenity::model::user::User;
use serenity::prelude::*;
use serenity::{async_trait, model::prelude::GuildId};
use shuttle_service::error::CustomError;
use shuttle_service::SecretStore;
use sqlx::{Executor, PgPool, Pool};

struct Bot {
    client: reqwest::Client,
	discord_guild_id: GuildId,
    database: PgPool,
}

#[shuttle_service::main]
async fn serenity(#[shared::Postgres] pool: PgPool) -> shuttle_service::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml` from the shared Postgres database
    let discord_token = pool
        .get_secret("DISCORD_TOKEN")
        .await
        .map_err(CustomError::new)?;

    let discord_guild_id = pool
        .get_secret("DISCORD_GUILD_ID")
        .await
        .map_err(CustomError::new)?;

    pool.execute(include_str!("../schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let pool = pool;

    let client = get_client(
        &discord_token,
        discord_guild_id.parse().unwrap(),
        pool
    )
    .await;

    Ok(client)
}

pub async fn get_client(
    discord_token: &str,
    discord_guild_id: u64,
    pool: PgPool
) -> Client {
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    Client::builder(discord_token, intents)
        .event_handler(Bot {
            client: reqwest::Client::new(),
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

        let commands = GuildId::set_application_commands(&self.discord_guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::register(command))
                .create_application_command(|command| commands::fun::register(command))
                .create_application_command(|command| commands::rank::register(command))
        })
        .await;
    }

    async fn message(&self, _ctx: Context, msg: Message) {
        let channel_id = msg.channel_id;
        if !msg.author.bot {
            // println!("message: {}, author name: {}, author id: {}, created: {}", 
            //     msg.content, 
            //     msg.author.name, 
            //     msg.author.id,
            //     msg.timestamp.unix_timestamp()
            // );
            let level_up = db::insert(&self.database, msg.author.id.to_string().parse::<i64>().unwrap(), msg.author.name, msg.timestamp.unix_timestamp()).await;
            if level_up {
                channel_id.send_message(&_ctx.http, |m| {m.content("LEVEL UP!")}).await.unwrap();
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command.data.name);

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "rank" => {
                    let command = command.data.options.get(0).expect("Expected command");
                    match command.name.as_str() {
                        // Rank subcommands here
                        "list" => {
                            db::list(&self.database).await.unwrap()
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
                            let new_level = db::calculate_level(current.0 + value, current.1);
                            db::update_level(&self.database, user_id, user_name, current.0 + value, new_level.0).await.unwrap()
                        },
                        // "demote" => {
                        //     let mut user_id: i64 = 0;
                        //     let mut user_name: String = String::new();
                        //     let mut value = 0;
                        //     if command.options.len() < 2
                        //     {
                        //         panic!("Expected User Arguments '[User] [Value]'")
                        //     }
                        //     if let CommandDataOptionValue::User(_user, _member) = command.options[0]
                        //         .resolved
                        //         .as_ref()
                        //         .expect("Expected User Object")
                        //     {
                        //         user_id = _user.id.to_string().parse::<i64>().unwrap().clone();
                        //         user_name = _user.name.clone();
                        //     }

                        //     if let CommandDataOptionValue::Number(_value) = command.options[1]
                        //         .resolved
                        //         .as_ref()
                        //         .expect("Expected User Object")
                        //     {
                        //         value = _value.clone().round() as i64;
                        //     }
                        //     let current = db::get_count_and_level(&self.database, user_id).await;
                        //     let new_level = db::calculate_level(if current.0 - value >= 0 {current.0 - value} else {0}, current.1);
                        //     db::update_level(&self.database, user_id, user_name, current.0 + value, new_level.0).await.unwrap()
                        // }
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

