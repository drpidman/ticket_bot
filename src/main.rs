use serenity::prelude::*;
use std::env;
use dotenv::dotenv;

use crate::events::Handler;

mod events;
mod config;
mod commands;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut client = Client::builder(env::var("BOT_TOKEN").expect("Token não foi providenciado"), 
        GatewayIntents::non_privileged() |
                GatewayIntents::GUILD_MESSAGES
    )
    .event_handler(Handler)
    .await
    .expect("Erro ao encontrar o token do bot");

    if let Err(err) = client.start().await {
        println!("Ocorreu um erro ao inicializar o bot: {:?}", err);
    }
}
