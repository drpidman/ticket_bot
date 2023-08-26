use serenity::model::prelude::{Ready, interaction, Interaction};
use serenity::prelude::{EventHandler, Context};
use serenity::async_trait;

use crate::commands::moderator::setup;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Bot ready: {:?}", &ready.user.bot);

        setup::register(&ctx.http).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = &interaction {
            
            match command.data.name.as_str() {
                "setup" => setup::command_run(&ctx, &command, &interaction).await,
                _=> ()
            };

        }
    }
}