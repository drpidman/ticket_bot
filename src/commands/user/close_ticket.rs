use serenity::{
    http::Http,
    model::prelude::{
        application_command::ApplicationCommandInteraction, command::Command, Interaction,
        InteractionResponseType, UserId,
    },
    prelude::Context,
};

use crate::{
    database::models::{TicketHistories, TicketHistory},
    utils::components::ticket::channel_parser,
};

pub async fn response_error(ctx: &Context, command: &ApplicationCommandInteraction) {
    command
        .create_interaction_response(&ctx, |res| {
            res.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|msg| {
                    msg.content(
        "Este não é um canal de ticket valido, tente executar o comando dentro de um"
    )
    .ephemeral(true)
                })
        })
        .await
        .unwrap();
}

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction, _i: &Interaction) {
    let channel = command.channel_id.as_ref().name(&ctx.cache).await.unwrap();

    let ticket_metadata = channel_parser(channel);
    let user_option = ticket_metadata.get(1);

    if user_option.is_none() {
        response_error(ctx, command).await;
        return;
    }

    let user = if let Ok(user) = user_option.unwrap().parse::<u64>() {
        Some(UserId::from(user))
    } else {
        None
    };

    if user.is_none() {
        response_error(ctx, command).await;
        return;
    }

    if !ticket_metadata.contains(&"suporte".to_string())
        & !ticket_metadata.contains(&"duvida".to_string())
        & !ticket_metadata.contains(&"problema".to_string())
    {
        response_error(ctx, command).await;
        return;
    }

    let ticket = if let Ok(ticket) = TicketHistory::get_by_channel(command.channel_id.0) {
        Some(ticket.unwrap())
    } else {
        None
    };

    if ticket.is_some() {
        TicketHistory::close_ticket(ticket.unwrap().ticket_id).unwrap();
    }

    command.channel_id.delete(&ctx).await.unwrap();
}

pub async fn register(http: &Http) {
    Command::create_global_application_command(http, |cmd| {
        cmd.name("close").description("Fechar um ticket")
    })
    .await
    .unwrap();
}
