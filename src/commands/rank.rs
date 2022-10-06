use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;

pub fn level_cost(level: f64) -> i64 {
    let cost: f64 = 8.0;
    let initial_modifer: f64 = 15.0;
    let multiplier: f64 = 1.75;
    if level == 0.0 {
        return 1;
    } else {
        return (multiplier * level * (cost * level + level.log(1.03))).round() as i64;
        //return (cost * multiplier.powf(level) + (initial_modifer * level)).round() as i64;
    }
}

// Current points minus previous level cost

pub fn calculate_level(points: i64, level: i64) -> (i64, bool) {
    print!("calling calculate level\n");
    let mut lvl_as_flt = level as f64;
    print!("points: {} level: {} level_cost: {}, increment_level: {}\n", points, level, level_cost(lvl_as_flt), points >= level_cost(lvl_as_flt));
    let mut level_up = false;
    while points >= level_cost(lvl_as_flt) {
        print!("points: {} derived cost: {}\nContinuing..\n", points, level_cost(lvl_as_flt));
        level_up = true;
        print!("incrementing to next level!\nCurrent level: {}, New Level: {}\n", lvl_as_flt, (lvl_as_flt + 1.0).round() as i64);
        lvl_as_flt += 1.0;
    }
    return (lvl_as_flt.round() as i64, level_up);
}

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
                    .kind(CommandOptionType::Number)
                    .min_length(2)
                    .max_int_value(100)
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
                    .kind(CommandOptionType::Number)
                    .min_length(2)
                    .max_int_value(100)
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