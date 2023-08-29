use serenity::async_trait;
use serenity::model::prelude::component::ComponentType;
use serenity::model::prelude::{Interaction, Ready};
use serenity::prelude::{Context, EventHandler};

use crate::commands::moderator::setup;
use crate::commands::user::close_ticket;
use crate::interactions::ticket_button_actions::ticket_button_action;
use crate::interactions::ticket_menu::ticket_menu;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Bot ready: {:?}", &ready.user.bot);

        setup::register(&ctx.http).await;
        close_ticket::register(&ctx.http).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = &interaction {
            match command.data.name.as_str() {
                "setup" => setup::run(&ctx, command, &interaction).await,
                "close" => close_ticket::run(&ctx, command, &interaction).await,
                _ => (),
            };
        }

        if let Interaction::MessageComponent(component) = &interaction {
            if let ComponentType::SelectMenu = component.data.component_type {
                match component.data.custom_id.as_str() {
                    "ticket_menu" => ticket_menu(&ctx, component, &interaction).await,
                    "ticket_button_actions" => ticket_button_action(&ctx, component).await,
                    _ => (),
                }
            }
        }
    }
}
