use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;


pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
    .name("rank")
    .description("Server rank commands")
    // .create_option(|option| {
    //     option
    //         .name("add")
    //         .description("Add a new todo")
    //         .kind(CommandOptionType::SubCommand)
    //         .create_sub_option(|option| {
    //             option
    //                 .name("note")
    //                 .description("The todo note to add")
    //                 .kind(CommandOptionType::String)
    //                 .min_length(2)
    //                 .max_length(100)
    //                 .required(true)
    //         })
    // })
    // .create_option(|option| {
    //     option
    //         .name("complete")
    //         .description("The todo to complete")
    //         .kind(CommandOptionType::SubCommand)
    //         .create_sub_option(|option| {
    //             option
    //                 .name("index")
    //                 .description("The index of the todo to complete")
    //                 .kind(CommandOptionType::Integer)
    //                 .min_int_value(1)
    //                 .required(true)
    //         })
    // })
    .create_option(|option| {
        option
            .name("list")
            .description("List your current rank list")
            .kind(CommandOptionType::SubCommand)
    })
}