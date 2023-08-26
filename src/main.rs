use serenity::prelude::*;

use crate::events::Handler;

mod events;
mod commands;

#[tokio::main]
async fn main() {
    let mut client = Client::builder("", 
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
