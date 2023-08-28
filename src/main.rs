use database::init;
use dotenv::dotenv;
use serenity::prelude::*;
use std::env;

use crate::events::Handler;

mod interactions;
mod commands;
mod database;
mod config;
mod events;
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

    init();

    if let Err(err) = client.start().await {
        println!("Ocorreu um erro ao inicializar o bot: {:?}", err);
    }
}
