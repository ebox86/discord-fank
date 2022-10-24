use serenity::builder::CreateEmbed;

pub mod ping;
pub mod fun;
pub mod rank;
pub mod watchlist;
pub mod comp;

pub enum CommandResult {
    Content(String),
    Embed(CreateEmbed),
}