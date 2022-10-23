use std::sync::Arc;

use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::http::Http;
use serenity::model::prelude::Embed;
use serenity::model::prelude::command::{CommandOptionType, self};
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue
};

pub async fn run(options: &[CommandDataOption]) -> String {
    let command = options.get(0).unwrap();
    match command.name.as_str() {
        "8ball" => {
            //let _ = client.send_message(channel_id, &embed).await;
            "Outcome unlikely!".to_string()
        },
        _ => "Please enter a watchlist command".to_string(),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
    .name("fun")
    .description("Fun commands")
    .create_option(
        |option| {
            option
                .name("8ball")
                .description("Ask the 8ball a question")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(
                    |option| {
                        option
                            .name("question")
                            .description("The question you wish to ask")
                            .kind(CommandOptionType::String)
                            .required(true)
                    }
                )
        },
    )
}
