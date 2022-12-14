mod commands;
mod db;
mod services;
mod routes;
mod scheduler;
mod util;

#[macro_use]
extern crate rocket;

use anyhow::Context as _;
use chrono::{Utc, Duration};
use log::info;
use serenity::builder::{CreateMessage, CreateInteractionResponseData, CreateEmbed};
use serenity::http::CacheHttp;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;
use serenity::model::prelude::application_command::ApplicationCommand;
use serenity::model::prelude::command::Command;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
use serenity::prelude::*;
use serenity::{async_trait, model::prelude::GuildId};
use shuttle_service::error::CustomError;
use shuttle_secrets::SecretStore;
use sqlx::{Executor, PgPool, FromRow};
use sync_wrapper::SyncWrapper;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use shuttle_service::ResourceBuilder;

use crate::commands::CommandResult;

struct Handler {
    iex_api_key: String,
	discord_guild_id: GuildId,
    database: PgPool
}

struct BotService {
    serenity: serenity::Client,
    database: PgPool
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[shuttle_service::async_trait]
impl shuttle_service::Service for BotService {
    async fn bind( mut self: Box<Self>,  addr: std::net::SocketAddr) -> Result<(), shuttle_service::Error> {
        let shutdown = rocket::config::Shutdown {
            ctrlc: false,
            ..rocket::config::Shutdown::default()
        };

        let config = rocket::Config::figment()
            .clone()
            .merge((rocket::Config::ADDRESS, addr.ip()))
            .merge((rocket::Config::PORT, addr.port()))
            .merge((rocket::Config::LOG_LEVEL, rocket::config::LogLevel::Off))
            .merge((rocket::Config::SHUTDOWN, shutdown));
        tokio::select! {
            _ = self.serenity.start() => Ok(()),
            _ = rocket::build()
            .mount("/hello", routes![routes::test::world])
            .mount("/rank", routes![routes::rank::show, routes::rank::show_by_guild])
            .attach(CORS)
            .manage(self.database)
            .configure(config)
            .launch() => Ok(())
        }
    }
}

#[shuttle_service::main]
async fn init( 
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> Result<BotService, shuttle_service::Error> {
    // discord keys and guild id here
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("Failed to get discord token")?;

    let discord_guild_id = secret_store
        .get("DISCORD_GUILD_ID")
        .context("Failed to get discord guild id")?;

    // api keys here
    let iex_api_key = secret_store
        .get("IEX_API_KEY")
        .context("Failed to get IEX API key")?;

    pool.execute(include_str!("../schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let pool = pool.clone();
    let pool1 = pool.clone();
    let pool2 = pool.clone();

    let handler = Handler {
        iex_api_key,
        discord_guild_id: GuildId(discord_guild_id.parse().unwrap()),
        database: pool,
    };
    // serenity client
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let serenity = Client::builder(discord_token, intents)
        .event_handler(handler)
        .await
        .expect("Err creating client");
    
    scheduler::start_scheduler(serenity.cache_and_http.http.clone(), pool1, discord_guild_id.parse().unwrap(), 698169764861837375);

    Ok(
        BotService{
            serenity,
            database: pool2
        }
    )
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        // registers global commands
        // Command::create_global_application_command(&ctx.http, |c| commands::ping::register(c)).await.unwrap();
        // Command::create_global_application_command(&ctx.http, |c| commands::rank::register(c)).await.unwrap();
        // Command::create_global_application_command(&ctx.http, |c| commands::fun::register(c)).await.unwrap();
        // Command::create_global_application_command(&ctx.http, |c| commands::watchlist::register(c)).await.unwrap();
        // Command::create_global_application_command(&ctx.http, |c| commands::comp::register(c)).await.unwrap();
        
        // registers guild commands
        let _commands = GuildId::set_application_commands(&self.discord_guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::register(command))
                .create_application_command(|command| commands::fun::register(command))
                .create_application_command(|command| commands::rank::register(command))
                .create_application_command(|command| commands::watchlist::register(command))
                .create_application_command(|command| commands::comp::register(command))
        })
        .await;
    
    }

    async fn message(&self, _ctx: Context, msg: Message) {
        let message_xp = 3;
        //let channel_id = msg.channel_id;
        if !msg.author.bot {
            let level_up = db::increment_level(&self.database, msg.guild_id.unwrap().0 as i64, msg.author.id.to_string().parse::<i64>().unwrap(), msg.author.name, msg.timestamp.unix_timestamp(), message_xp).await;
            if level_up.1 {
                let mut lvl_up_msg = CreateMessage::default();
                lvl_up_msg.embed(|embed| embed
                            .title(format!("Level up! Welcome to level {}", level_up.0))
                            .color(0x00FF00)
                            .image("https://i.imgur.com/qgpcufH.gif")
                            .field("Next Level:", format!("{} points",  commands::rank::level_cost(level_up.0 as f64)), true)
                ).content(format!("<@{}>", msg.author.id));
                // bot testing channel
                //util::send_message(&_ctx.http, 1024479108983422986, lvl_up_msg).await.unwrap();
                // real channel
                util::send_message(&_ctx.http, 1015709479179923516, lvl_up_msg).await.unwrap();
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            //let invoking_user = &interaction.application_command().unwrap().member.unwrap().user
            let invoking_user = &command.member.as_ref().unwrap().user;
            let guild_id = command.guild_id.unwrap().0 as i64;
            let http = &ctx.http;
            let channel_id = command.channel_id.0;
            println!("Received command interaction: {:#?}", command.data.name);

            let content: CommandResult = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options).await,
                "fun" => commands::fun::run(&command.data.options).await,
                "watchlist" => {
                    let command = command.data.options.get(0).expect("Expected a command");
                    match command.name.as_ref() {
                        "add" => {
                            let mut ticker: Vec<&str> = Vec::new();
                            if let CommandDataOptionValue::String (_ticker) = command.options[0]
                                .resolved
                                .as_ref()
                                .expect("Expected String")
                            {
                                ticker = _ticker.split(",").collect::<Vec<&str>>();
                            }
                            CommandResult::Content(db::add_watchlist(&self.database, &self.iex_api_key, guild_id, invoking_user.id.to_string().parse::<i64>().unwrap(), ticker).await.unwrap())
                        },
                        "list" => CommandResult::Content(db::list_watchlist(&self.database, guild_id).await.unwrap()),
                        "clear" => {
                            let user_id: i64 = invoking_user.id.to_string().parse::<i64>().unwrap();
                            CommandResult::Content(db::clear_watchlist(&self.database, guild_id, user_id).await.unwrap())
                        },
                        "show" => {
                            let user_id: i64 = invoking_user.id.to_string().parse::<i64>().unwrap();
                            CommandResult::Content(db::show_watchlist(&self.database, &self.iex_api_key, guild_id, user_id).await.unwrap())
                        },
                        _ => CommandResult::Content("Please enter a watchlist command".to_string()),
                    }
                },
                "comp" => {
                    let command = command.data.options.get(0).expect("Expected a command");
                    match command.name.as_str() {
                        "create" => {
                            let mut name = String::new();
                            let start_date = Utc::now();
                            let mut end_date = Utc::now();
                            if let CommandDataOptionValue::String (_name) = command.options[0]
                                .resolved
                                .as_ref()
                                .expect("Expected String")
                            {
                                name = _name.to_string();
                            }
                            if let CommandDataOptionValue::Integer (_length) = command.options[1]
                            .resolved
                            .as_ref()
                            .expect("Expected String")
                            {
                                end_date = start_date + Duration::days(*_length);
                            }
                            CommandResult::Content(db::create_comp(&self.database, guild_id, name, start_date.timestamp(), end_date.timestamp()).await.unwrap())
                        },
                        "list" => commands::CommandResult::Content(db::list_comp(&self.database, guild_id, invoking_user.id.to_string().parse::<i64>().unwrap()).await.unwrap()),
                        "register" => {
                            let mut comp_id: i64 = 0;
                            let mut tickers: Vec<&str> = Vec::new();
                            if let CommandDataOptionValue::Integer (_comp_id) = command.options[0]
                                .resolved
                                .as_ref()
                                .expect("Expected String")
                            {
                                comp_id = *_comp_id;
                            }
                            if let CommandDataOptionValue::String (_ticker) = command.options[1]
                                .resolved
                                .as_ref()
                                .expect("Expected String")
                            {
                                tickers = _ticker.split(",").collect::<Vec<&str>>();
                            }
                            CommandResult::Content(db::register_comp(&self.database, guild_id, invoking_user.id.to_string().parse::<i64>().unwrap(), comp_id, tickers, self.iex_api_key.to_string()).await.unwrap())
                        },
                        // "show" => {
                        //     let mut comp_id: i64 = 0;
                        //     if let CommandDataOptionValue::Integer (_comp_id) = command.options[0]
                        //         .resolved
                        //         .as_ref()
                        //         .expect("Expected String")
                        //     {
                        //         comp_id = *_comp_id;
                        //     }
                        //     db::show_comp(&self.database, comp_id).await.unwrap()
                        // },
                        "edit" => {
                            let mut comp_id: i64 = 0;
                            let mut reg: bool = false;
                            if let CommandDataOptionValue::Integer (_comp_id) = command.options[0]
                                .resolved
                                .as_ref()
                                .expect("Expected String")
                            {
                                comp_id = *_comp_id;
                            }
                            if command.options.len() > 1 {
                                if let CommandDataOptionValue::Boolean (_reg) = command.options[1]
                                    .resolved
                                    .as_ref()
                                    .expect("Expected String")
                                {
                                    reg = *_reg;
                                }
                                db::set_registration_status(&self.database, guild_id, comp_id, reg).await.unwrap();
                            }
                            CommandResult::Content(db::show_comp_metadata(&self.database, guild_id, comp_id).await.unwrap())
                        }
                        _ => CommandResult::Content("Please enter a comp command".to_string()),
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
                            let current = db::get_count_and_level(&self.database, guild_id, user_id).await;
                            let next_cost = commands::rank::level_cost(current.1 as f64);
                            let watchlist = db::get_watchlist(&self.database, guild_id, user_id).await;
                            let watchlist_count = watchlist.len();
                            CommandResult::Content(format!("{} is level {} with {} xp.\nNext level at {} xp.{}", user_name, current.1, current.0, next_cost, "\nWatchlist count: ".to_owned() + &watchlist_count.to_string()))
                        },
                        "list" => {
                            CommandResult::Content(db::list_rank(&self.database, guild_id).await.unwrap())
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
                            let current = db::get_count_and_level(&self.database, guild_id, user_id).await;
                            let new_level = commands::rank::calculate_level(current.0 + value, current.1);
                            CommandResult::Content(db::update_level(&self.database, guild_id, user_id, user_name, current.0 + value, new_level.0).await.unwrap())
                        },
                        _ => CommandResult::Content("Please enter a valid todo".to_string()),
                    }
                }
                _ => CommandResult::Content("test".to_string()),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| match content {
                            CommandResult::Content(content) => message.content(content),
                            CommandResult::Embed(embed) => message.add_embed(embed),
                          })
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn application_command_permissions_update(
        &self,
        _ctx: Context,
        _permission: serenity::model::prelude::command::CommandPermission,
    ) {
    }

    async fn auto_moderation_rule_create(&self, _ctx: Context, _rule: serenity::model::prelude::automod::Rule) {}

    async fn auto_moderation_rule_update(&self, _ctx: Context, _rule: serenity::model::prelude::automod::Rule) {}

    async fn auto_moderation_rule_delete(&self, _ctx: Context, _rule: serenity::model::prelude::automod::Rule) {}

    async fn auto_moderation_action_execution(&self, _ctx: Context, _execution: serenity::model::prelude::automod::ActionExecution) {}

    async fn channel_create(&self, _ctx: Context, _channel: &serenity::model::prelude::GuildChannel) {}

    async fn category_create(&self, _ctx: Context, _category: &serenity::model::prelude::ChannelCategory) {}

    async fn category_delete(&self, _ctx: Context, _category: &serenity::model::prelude::ChannelCategory) {}

    async fn channel_delete(&self, _ctx: Context, _channel: &serenity::model::prelude::GuildChannel) {}

    async fn channel_pins_update(&self, _ctx: Context, _pin: serenity::model::prelude::ChannelPinsUpdateEvent) {}

    async fn channel_update(&self, _ctx: Context, _new_data: serenity::model::prelude::Channel) {}

    async fn guild_ban_addition(&self, _ctx: Context, _guild_id: GuildId, _banned_user: serenity::model::user::User) {}

    async fn guild_ban_removal(&self, _ctx: Context, _guild_id: GuildId, _unbanned_user: serenity::model::user::User) {}

    async fn guild_create(&self, _ctx: Context, _guild: serenity::model::prelude::Guild) {}

    async fn guild_delete(&self, _ctx: Context, _incomplete: serenity::model::prelude::UnavailableGuild) {}

    async fn guild_emojis_update(
        &self,
        _ctx: Context,
        _guild_id: GuildId,
        _current_state: std::collections::HashMap<serenity::model::prelude::EmojiId, serenity::model::prelude::Emoji>,
    ) {
    }

    async fn guild_integrations_update(&self, _ctx: Context, _guild_id: GuildId) {}

    async fn guild_member_addition(&self, _ctx: Context, _new_member: serenity::model::prelude::Member) {}

    async fn guild_member_removal(&self, _ctx: Context, _guild_id: GuildId, _kicked: serenity::model::user::User) {}

    async fn guild_member_update(&self, _ctx: Context, _new: serenity::model::prelude::GuildMemberUpdateEvent) {}

    async fn guild_members_chunk(&self, _ctx: Context, _chunk: serenity::model::prelude::GuildMembersChunkEvent) {}

    async fn guild_role_create(&self, _ctx: Context, _new: serenity::model::prelude::Role) {}

    async fn guild_role_delete(&self, _ctx: Context, _guild_id: GuildId, _removed_role_id: serenity::model::prelude::RoleId) {
    }

    async fn guild_role_update(&self, _ctx: Context, _new_data: serenity::model::prelude::Role) {}

    async fn guild_stickers_update(
        &self,
        _ctx: Context,
        _guild_id: GuildId,
        _current_state: std::collections::HashMap<serenity::model::prelude::StickerId, serenity::model::sticker::Sticker>,
    ) {
    }

    async fn guild_unavailable(&self, _ctx: Context, _guild_id: GuildId) {}

    async fn guild_update(&self, _ctx: Context, _new_but_incomplete_data: serenity::model::prelude::PartialGuild) {}

    async fn invite_create(&self, _ctx: Context, _data: serenity::model::prelude::InviteCreateEvent) {}

    async fn invite_delete(&self, _ctx: Context, _data: serenity::model::prelude::InviteDeleteEvent) {}

    async fn message_delete(
        &self,
        _ctx: Context,
        _channel_id: serenity::model::prelude::ChannelId,
        _deleted_message_id: serenity::model::prelude::MessageId,
        _guild_id: Option<GuildId>,
    ) {
    }

    async fn message_delete_bulk(
        &self,
        _ctx: Context,
        _channel_id: serenity::model::prelude::ChannelId,
        _multiple_deleted_messages_ids: Vec<serenity::model::prelude::MessageId>,
        _guild_id: Option<GuildId>,
    ) {
    }

    async fn message_update(&self, _ctx: Context, _new_data: serenity::model::prelude::MessageUpdateEvent) {}

    async fn reaction_add(&self, _ctx: Context, _add_reaction: serenity::model::prelude::Reaction) {}

    async fn reaction_remove(&self, _ctx: Context, _removed_reaction: serenity::model::prelude::Reaction) {}

    async fn reaction_remove_all(
        &self,
        _ctx: Context,
        _channel_id: serenity::model::prelude::ChannelId,
        _removed_from_message_id: serenity::model::prelude::MessageId,
    ) {
    }

    async fn presence_replace(&self, _ctx: Context, _: Vec<serenity::model::prelude::Presence>) {}

    async fn presence_update(&self, _ctx: Context, _new_data: serenity::model::prelude::Presence) {}

    async fn resume(&self, _ctx: Context, _: serenity::model::prelude::ResumedEvent) {}

    async fn shard_stage_update(&self, _ctx: Context, _: serenity::client::bridge::gateway::event::ShardStageUpdateEvent) {}

    async fn typing_start(&self, _ctx: Context, _: serenity::model::prelude::TypingStartEvent) {}

    async fn unknown(&self, _ctx: Context, _name: String, _raw: serenity::json::Value) {}

    async fn user_update(&self, _ctx: Context, _new_data: serenity::model::user::CurrentUser) {}

    async fn voice_server_update(&self, _ctx: Context, _: serenity::model::prelude::VoiceServerUpdateEvent) {}

    async fn voice_state_update(&self, _ctx: Context, _: serenity::model::voice::VoiceState) {}

    async fn webhook_update(
        &self,
        _ctx: Context,
        _guild_id: GuildId,
        _belongs_to_channel_id: serenity::model::prelude::ChannelId,
    ) {
    }

    async fn integration_create(&self, _ctx: Context, _integration: serenity::model::prelude::Integration) {}

    async fn integration_update(&self, _ctx: Context, _integration: serenity::model::prelude::Integration) {}

    async fn integration_delete(
        &self,
        _ctx: Context,
        _integration_id: serenity::model::prelude::IntegrationId,
        _guild_id: GuildId,
        _application_id: Option<serenity::model::prelude::ApplicationId>,
    ) {
    }

    async fn stage_instance_create(&self, _ctx: Context, _stage_instance: serenity::model::prelude::StageInstance) {}

    async fn stage_instance_update(&self, _ctx: Context, _stage_instance: serenity::model::prelude::StageInstance) {}

    async fn stage_instance_delete(&self, _ctx: Context, _stage_instance: serenity::model::prelude::StageInstance) {}

    async fn thread_create(&self, _ctx: Context, _thread: serenity::model::prelude::GuildChannel) {}

    async fn thread_update(&self, _ctx: Context, _thread: serenity::model::prelude::GuildChannel) {}

    async fn thread_delete(&self, _ctx: Context, _thread: serenity::model::prelude::PartialGuildChannel) {}

    async fn thread_list_sync(&self, _ctx: Context, _thread_list_sync: serenity::model::prelude::ThreadListSyncEvent) {}

    async fn thread_member_update(&self, _ctx: Context, _thread_member: serenity::model::prelude::ThreadMember) {}

    async fn thread_members_update(
        &self,
        _ctx: Context,
        _thread_members_update: serenity::model::prelude::ThreadMembersUpdateEvent,
    ) {
    }

    async fn guild_scheduled_event_create(&self, _ctx: Context, _event: serenity::model::prelude::ScheduledEvent) {}

    async fn guild_scheduled_event_update(&self, _ctx: Context, _event: serenity::model::prelude::ScheduledEvent) {}

    async fn guild_scheduled_event_delete(&self, _ctx: Context, _event: serenity::model::prelude::ScheduledEvent) {}

    async fn guild_scheduled_event_user_add(
        &self,
        _ctx: Context,
        _subscribed: serenity::model::prelude::GuildScheduledEventUserAddEvent,
    ) {
    }

    async fn guild_scheduled_event_user_remove(
        &self,
        _ctx: Context,
        _unsubscribed: serenity::model::prelude::GuildScheduledEventUserRemoveEvent,
    ) {
    }
}

