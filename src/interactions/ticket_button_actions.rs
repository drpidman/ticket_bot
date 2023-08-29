use serenity::{model::prelude::message_component::MessageComponentInteraction, prelude::Context};

use crate::{
    database::models::{TicketHistories, TicketHistory},
    utils::components::ticket::channel_parser,
};

pub async fn ticket_button_action(ctx: &Context, component: &MessageComponentInteraction) {
    let ticket = if let Ok(ticket) = TicketHistory::get_by_channel(component.channel_id.0) {
        Some(ticket.unwrap())
    } else {
        None
    };

    TicketHistory::close_ticket(ticket.unwrap().ticket_id).unwrap();

    component.channel_id.delete(&ctx).await.unwrap();
}
