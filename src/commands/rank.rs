use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;


pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
    .name("rank")
    .description("Server rank commands")
    .create_option(|option| {
        option
            .name("promote")
            .description("promote a user")
            .kind(CommandOptionType::SubCommand)
            .create_sub_option(|option| {
                option
                    .name("user")
                    .description("the user to promote")
                    .kind(CommandOptionType::User)
                    .min_length(2)
                    .max_length(100)
                    .required(true)
            })
            .create_sub_option(|option| {
                option
                    .name("value")
                    .description("the value to promote by")
                    .kind(CommandOptionType::String)
                    .min_length(2)
                    .max_length(100)
                    .required(true)
            })
    })
    .create_option(|option| {
        option
            .name("demote")
            .description("demote a user")
            .kind(CommandOptionType::SubCommand)
            .create_sub_option(|option| {
                option
                    .name("user")
                    .description("the user to demote")
                    .kind(CommandOptionType::User)
                    .min_int_value(1)
                    .required(true)
            })
            .create_sub_option(|option| {
                option
                    .name("value")
                    .description("The value to demote by")
                    .kind(CommandOptionType::String)
                    .min_length(2)
                    .max_length(100)
                    .required(true)
            })
    })
    .create_option(
        |option| {
            option
                .name("list")
                .description("List your current rank list")
                .kind(CommandOptionType::SubCommand)
    })
}