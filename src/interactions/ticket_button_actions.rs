use serenity::{
    builder::CreateEmbed,
    model::{
        prelude::{
            message_component::MessageComponentInteraction, ChannelId, InteractionResponseType,
            UserId,
        },
        Timestamp,
    },
    prelude::Context,
    utils::Color,
};

use crate::{
    database::models::{TicketHistories, TicketHistory},
    utils::{components::ticket::get_metadata, config::guild_config::get_config},
};

pub async fn ticket_button_action(ctx: &Context, component: &MessageComponentInteraction) {
    let tickets_channel = ChannelId::from(get_config(component.guild_id.unwrap().0).unwrap().ticket_log);
    let current_ticket = TicketHistory::get_by_channel(component.channel_id.0);

    let ticket_metadata = get_metadata(
        &component
            .channel_id
            .name(&ctx.cache)
            .await
            .unwrap()
            .to_string(),
    );
    let metadata = ticket_metadata.unwrap();

    let user = if let Ok(user) = metadata.user.parse::<u64>() {
        Some(UserId::from(user))
    } else {
        None
    };

    let user = component
        .guild_id
        .unwrap()
        .member(&ctx, user.unwrap().0)
        .await
        .unwrap();

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

    let ticket = current_ticket.unwrap();

    let mut embed = CreateEmbed::default();
    let embed = embed
        .title(format!("Requisição - {}", metadata.request))
        .description("Este ticket foi fechado")
        .field("Aberto por", user.display_name(), false)
        .field("Tipo", metadata.request, false)
        .timestamp(Timestamp::now())
        .color(Color::PURPLE);

    tickets_channel
        .send_message(&ctx, |msg| msg.set_embed(embed.to_owned()))
        .await
        .unwrap();

    component
        .user
        .dm(&ctx, |msg| msg.set_embed(embed.to_owned()))
        .await
        .unwrap();

    TicketHistory::close_ticket(ticket.unwrap().ticket_id).unwrap();
    component.channel_id.delete(&ctx).await.unwrap();
}
