use database::init;
use dotenv::dotenv;
use serenity::prelude::*;
use std::{env, fs};

use crate::events::Handler;

mod commands;
mod database;
mod events;
mod interactions;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut client = Client::builder(
        env::var("BOT_TOKEN").expect("Token não foi providenciado"),
        GatewayIntents::non_privileged() | GatewayIntents::GUILD_MESSAGES,
    )
    .event_handler(Handler)
    .await
    .expect("Erro ao encontrar o token do bot");

    if fs::metadata("config.db").is_err() {
        fs::File::create("config.db").unwrap();
    }

    init();

    if let Err(err) = client.start().await {
        println!("Ocorreu um erro ao inicializar o bot: {:?}", err);
    }
}
