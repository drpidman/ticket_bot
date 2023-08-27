use serenity::model::prelude::component::ComponentType;
use serenity::model::prelude::{Ready, Interaction};
use serenity::prelude::{EventHandler, Context};
use serenity::async_trait;

use crate::commands::moderator::setup;
use crate::interactions::menu_select::menu_run;

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

        if let Interaction::MessageComponent(component) = &interaction {
            if let ComponentType::SelectMenu = component.data.component_type {
                match component.data.custom_id.as_str() {
                    "menu_select" => menu_run(&ctx, &component, &interaction).await,
                    _=> ()
                }
            }
        }
    }
}