use serenity::{
    builder::CreateEmbed,
    http::Http,
    model::{
        prelude::{
            application_command::ApplicationCommandInteraction, command::Command, ChannelId,
            Interaction, InteractionResponseType, UserId,
        },
        Timestamp,
    },
    prelude::Context,
    utils::Color,
};

use crate::{
    database::models::{TicketHistories, TicketHistory},
    utils::{components::ticket::channel_parser, config::guild_config::get_config},
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
    let tickets_channel =
        ChannelId::from(get_config(command.guild_id.unwrap().0).unwrap().ticket_log);

    let current_ticket = command.channel_id.as_ref().name(&ctx.cache).await.unwrap();
    let ticket_metadata = channel_parser(&current_ticket);

    let user_request = ticket_metadata.get(0);
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

    let user = command
        .guild_id
        .unwrap()
        .member(&ctx, user.unwrap().0)
        .await
        .unwrap();

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

    let mut embed = CreateEmbed::default();
    let embed = embed
        .title(format!("Requisição - {}", user_request.unwrap()))
        .description("Este ticket foi fechado")
        .field("Aberto por", user.display_name(), false)
        .field("Tipo", user_request.unwrap().to_string(), false)
        .timestamp(Timestamp::now())
        .color(Color::PURPLE);

    tickets_channel
        .send_message(&ctx, |msg| msg.set_embed(embed.to_owned()))
        .await
        .unwrap();

    command
        .user
        .dm(&ctx, |msg| msg.set_embed(embed.to_owned()))
        .await
        .unwrap();

    TicketHistory::close_ticket(ticket.unwrap().ticket_id).unwrap();
    command.channel_id.delete(&ctx).await.unwrap();
}

pub async fn register(http: &Http) {
    Command::create_global_application_command(http, |cmd| {
        cmd.name("close").description("Fechar um ticket")
    })
    .await
    .unwrap();
}
