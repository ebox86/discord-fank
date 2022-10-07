use serenity::{builder::CreateApplicationCommand};



pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    return command
        .name("watchlist")
        .description("Watchlist command")
        .create_option(|option| {
            option
                .name("add")
                .description("Add a stock to the watchlist")
                .kind(serenity::model::prelude::command::CommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("symbol")
                        .description("The symbol of the stock to add")
                        .kind(serenity::model::prelude::command::CommandOptionType::String)
                        .required(true)
                })
        })
        .create_option(|option| {
            option
                .name("remove")
                .description("Remove a stock from the watchlist")
                .kind(serenity::model::prelude::command::CommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("symbol")
                        .description("The symbol of the stock to remove")
                        .kind(serenity::model::prelude::command::CommandOptionType::String)
                        .required(true)
                })
        })
        .create_option(|option| {
            option
                .name("show")
                .description("Show all stocks in the watchlist")
                .kind(serenity::model::prelude::command::CommandOptionType::SubCommand)
        })
        .create_option(|option| {
            option
                .name("list")
                .description("list all lists in the table *DEBUG ONLY*")
                .kind(serenity::model::prelude::command::CommandOptionType::SubCommand)
        })
        .create_option(|option| {
            option
                .name("clear")
                .description("Clear the watchlist")
                .kind(serenity::model::prelude::command::CommandOptionType::SubCommand)
        });
}