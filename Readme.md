# Discord Fank
This is an open source discord bot for financial and trading servers which offers the following features:
* Member Ranking / Leveling
* Administrativly set custom watch lists (coming soon)
* User (member) defined custom watch lists (coming soon)
* Integration into many popular financial data providers (coming soon)
* Voice Chat callouts based on triggers or preconfigured price or change levels (coming soon)
* Fun (8Ball, Dice roll)

## Setup
Discord Fank is built using the serenity rust framework for discord. It is deployed using [Shuttle](https://shuttle.rs) service. To deploy your own version, create a free shuttle account and download the shuttle crate:

`cargo install cargo-shuttle`

And then authenticate to the service by running

`cargo shuttle login`

With the api key pasted into the CLI, you are ready to deploy.

To test locally, run

`cargo shuttle run`

To deploy changes to the shuttle hosted environment, run

`cargo shuttle deploy`

## Secrets management
Currently, secrets management is defined by a local file at buildtime which is then loaded into postgres. To define this file, create a `Secrets.toml` file and populate it with the following:

```
# Secrets.toml
DISCORD_TOKEN="YOUR_TOKEN_HERE"
DISCORD_GUILD_ID="YOUR_GUILD_ID_HERE"
```