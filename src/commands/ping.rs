

use serenity::builder::{CreateApplicationCommand, CreateMessage, CreateEmbed};
use serenity::model::prelude::{Message, ChannelId};
use serenity::model::prelude::interaction::application_command::CommandDataOption;

use super::CommandResult;


pub async fn run(_options: &[CommandDataOption]) -> CommandResult {
    //CommandResult::Content("pong".to_string())
    let mut embed = CreateEmbed::default();
    embed.title("Pong!");
    CommandResult::Embed(embed)
    //return "Pong!".to_string();
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("A ping command")
}
