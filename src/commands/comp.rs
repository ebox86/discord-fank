use std::option;

use serenity::{builder::CreateApplicationCommand};
use serenity::model::Permissions;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    return command
        .name("comp")
        .description("Competitions command")
        .create_option(|option| {
            option
                .name("register")
                .description("registers a user to a current competition")
                .kind(serenity::model::prelude::command::CommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("competition")
                        .description("The competition to register to")
                        .kind(serenity::model::prelude::command::CommandOptionType::Integer)
                        .required(true)
                })
                .create_sub_option(|option| {
                    option
                        .name("symbol")
                        .description("The 5 stocks to add for the competition")
                        .kind(serenity::model::prelude::command::CommandOptionType::String)
                        .required(true)
                })
        })
        .create_option(|option| {
            option
                .name("create")
                .description("creates a new competition")
                .kind(serenity::model::prelude::command::CommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("name")
                        .description("The name of the competition")
                        .kind(serenity::model::prelude::command::CommandOptionType::String)
                        .required(true)
                })
                .create_sub_option(|option| {
                    option
                        .name("length")
                        .description("The length of the competition")
                        .kind(serenity::model::prelude::command::CommandOptionType::Integer)
                        .required(true)
                        .add_int_choice("1d", 1)
                        .add_int_choice("2d", 2)
                        .add_int_choice("3d", 3)
                        .add_int_choice("1w", 7)
                        .add_int_choice("2w", 14)
                })
        }).default_member_permissions(Permissions::MODERATE_MEMBERS)
        .create_option(|option| {
            option
                .name("list")
                .description("Lists all current competitions")
                .kind(serenity::model::prelude::command::CommandOptionType::SubCommand)
        })
        .create_option(|option| {
            option
                .name("stats")
                .description("Show your competition stats")
                .kind(serenity::model::prelude::command::CommandOptionType::SubCommand)
        })
        .create_option(|option| {
            option
                .name("lb")
                .description("show leaderboard for current competition")
                .kind(serenity::model::prelude::command::CommandOptionType::SubCommand)
        })
        .create_option(|option| {
            option
                .name("alltime")
                .description("show all time leaderboard")
                .kind(serenity::model::prelude::command::CommandOptionType::SubCommand)
        });
}