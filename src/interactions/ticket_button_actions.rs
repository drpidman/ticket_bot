use serenity::{
    model::prelude::{message_component::MessageComponentInteraction, InteractionResponseType},
    prelude::Context,
};

use crate::{
    database::models::{TicketHistories, TicketHistory},
    utils::components::ticket::channel_parser,
};

pub async fn ticket_button_action(ctx: &Context, component: &MessageComponentInteraction) {
    let current_ticket = TicketHistory::get_by_channel(component.channel_id.0);

    if current_ticket.is_err() {
        component
            .create_interaction_response(&ctx, |res| {
                res.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|msg| {
                        msg.content("Ocorreu um erro ao processar as informações")
                            .ephemeral(true)
                    })
            })
            .await
            .unwrap();
        return;
    }

    let ticket = if let Some(ticket) = current_ticket.unwrap() {
        Some(ticket)
    } else {
        None
    };

    TicketHistory::close_ticket(ticket.unwrap().ticket_id).unwrap();
    component.channel_id.delete(&ctx).await.unwrap();
}
