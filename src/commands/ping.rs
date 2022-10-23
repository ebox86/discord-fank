

use serenity::builder::{CreateApplicationCommand, CreateMessage};
use serenity::model::prelude::{Message, ChannelId};
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub async fn run(_options: &[CommandDataOption]) -> String {
    return "Pong!".to_string();
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("A ping command")
}
