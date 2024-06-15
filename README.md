### TODO

- [ ] Implement similar functionality as the telegram in form of a dashboard. [Poise](https://github.com/serenity-rs/poise) should allow for this.
- [ ] Add tracking of specific individuals, per user, alert the user on updates.
- [ ] Since the api endpoint is dead, we have to do something like [this](https://github.com/cjk268/Insider-Trading-Alerts/blob/main/main.py)
- [ ] OPTIONS CHART PLS
- [ ] Perhaps the API should be asynchronous?

# Capitol trades bot

This repository contains the source code for a discord bot to track trades
of American politicians, this was originally a telegram bot made by [TommasoAmic](https://github.com/TommasoAmici).

The [capitoltrades_api](./crates/capitoltrades_api/) crate is a standalone
client for fetching data from <https://www.capitoltrades.com>.
It uses [reqwest](https://docs.rs/reqwest/latest/reqwest/) for synchronous HTTP requests.

The [discord_bot](./crates/discord_bot/) crate contains the code for the Discord
bot, built with [Serenity](https://github.com/serenity-rs/serenity and [Poise](https://github.com/serenity-rs/poise)).

## Setup:

1. Replace the placeholders _DATABASE_URL_ and _DISCORD_TOKEN_ in the [.env](./.env) file.
2. Run the bot using `cargo run`. Use `/help` to get an overview of available commands.

Alternatively (preferred, if just deploying):

-
- Use Docker: Run `docker compose up` (after setting the enviornment variables in the [docker-compose](./docker-compose.yml)).
