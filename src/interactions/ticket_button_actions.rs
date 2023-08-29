use serenity::{prelude::Context, model::prelude::message_component::MessageComponentInteraction};

use crate::utils::components::ticket::channel_parser;


pub async fn ticket_button_action(ctx: &Context, component: &MessageComponentInteraction) {
    
    component.channel_id.delete(&ctx).await.unwrap();
}