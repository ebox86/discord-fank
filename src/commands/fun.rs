use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue
};

pub fn run(options: &[CommandDataOption]) -> String {
    let option = options
    .get(0)
    .expect("Expected string option")
    .resolved
    .as_ref()
    .expect("Expected string object");

    if let CommandDataOptionValue::String(question) = option {
        format!("**{}**: \n> doubtful (currently the only value, will fix soon)", question.to_string())
    } else {
        "Please provide a valid user".to_string()
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
                .description("The question you wish to ask")
                .kind(CommandOptionType::SubCommand)
        },
    )
}
