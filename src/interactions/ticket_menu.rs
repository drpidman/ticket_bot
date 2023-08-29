use serenity::{
    builder::{CreateChannel, CreateEmbed},
    json,
    model::{
        prelude::{
            message_component::MessageComponentInteraction, ChannelId, ChannelType, Interaction,
            InteractionResponseType, PermissionOverwrite, PermissionOverwriteType,
        },
        Permissions, Timestamp,
    },
    prelude::Context,
    utils::Color,
};

use crate::{
    config::{TICKET_CREATION_CATEGORY, TICKET_LOG_CHANNEL},
    database::models::{Ticket, TicketConfig, TicketHistories, TicketHistory},
    utils::components::ticket::{get_ticket_channel, is_user_ticket, ticket_actions},
};

async fn response_to_user(ctx: &Context, component: &MessageComponentInteraction) {
    component
        .create_interaction_response(&ctx, |res| {
            res.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.content("Ticket aberto com sucesso").ephemeral(true)
                })
        })
        .await
        .unwrap();
}

pub async fn ticket_menu(ctx: &Context, component: &MessageComponentInteraction, _i: &Interaction) {
    let tickets_channel = ChannelId::from(TICKET_LOG_CHANNEL.parse::<u64>().unwrap());
    let tickets_category = ChannelId::from(TICKET_CREATION_CATEGORY.parse::<u64>().unwrap());
    let guild_id = component.guild_id.unwrap();

    let ticket = TicketConfig::get(guild_id.0).unwrap();

    if ticket.is_none() {
        component
            .create_interaction_response(&ctx, |res| {
                res.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|msg| {
                        msg.content("O ticket não foi configurado").ephemeral(true)
                    })
            })
            .await
            .unwrap();
        return;
    }

    if ticket.unwrap().ticket_id != component.message.id.0 {
        component
            .create_interaction_response(&ctx, |res| {
                res.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|msg| {
                        msg.content("Este ticket não é valido").ephemeral(true)
                    })
            })
            .await
            .unwrap();
        return;
    }

    let mut ticket_embed = CreateEmbed::default();
    let mut ticket_channel = CreateChannel::default();
    ticket_channel.kind(ChannelType::Text);

    let everyone_role = ctx
        .cache
        .guild_roles(guild_id)
        .unwrap()
        .iter()
        .find(|role| role.1.name == "@everyone")
        .unwrap()
        .0
        .to_owned();

    let channel_permissions = vec![
        // ! PARA O USUARIO
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Member(component.user.id),
        },
        // ! PARA TODOS
        PermissionOverwrite {
            deny: Permissions::VIEW_CHANNEL,
            allow: Permissions::empty(),
            kind: PermissionOverwriteType::Role(everyone_role),
        },
    ];

    let choices: Vec<String> = vec![
        "suporte".to_string(),
        "duvida".to_string(),
        "problema".to_string(),
    ];

    let choice = &choices
        .iter()
        .find(|choice| choice.to_string() == component.data.values[0])
        .unwrap()
        .to_string();

    if is_user_ticket(ctx, component).await {
        return;
    }

    match choice.as_str() {
        "suporte" => {
            ticket_embed.title("Requisição - Suporte".to_string());
            ticket_embed.color(Color::DARK_GREEN);

            ticket_channel.name(format!("suporte-{}", component.user.id));
            ticket_channel.permissions(channel_permissions);
            ticket_channel.category(tickets_category);

            response_to_user(ctx, component).await;
        }
        "duvida" => {
            ticket_embed.title("Requisição - Duvida".to_string());
            ticket_embed.color(Color::BLUE);

            ticket_channel.name(format!("duvida-{}", component.user.id));
            ticket_channel.permissions(channel_permissions);
            ticket_channel.category(tickets_category);

            response_to_user(ctx, component).await;
        }
        "problema" => {
            ticket_embed.title("Requisição - Problema".to_string());
            ticket_embed.color(Color::RED);

            ticket_channel.name(format!("problema-{}", component.user.id));
            ticket_channel.permissions(channel_permissions);
            ticket_channel.category(tickets_category);

            response_to_user(ctx, component).await;
        }
        _ => (),
    };

    ctx.http
        .as_ref()
        .create_channel(
            guild_id.0,
            &json::hashmap_to_json_map(ticket_channel.0),
            Some("Ticket de suporte"),
        )
        .await
        .unwrap();

    let ticket_channel = get_ticket_channel(ctx, component).await.unwrap();

    ticket_embed.field("Usuario", &component.user.name, false);
    ticket_embed.field("Canal", ticket_channel.1.name, false);

    ticket_channel
        .0
        .send_message(&ctx, |msg| {
            msg.add_embed(|embed| {
                embed
                    .title("Gerenciador de ticket")
                    .color(Color::DARK_GOLD)
                    .description("Use o botão abaixo para fechar o ticket")
                    .timestamp(Timestamp::now())
            })
            .components(|components| {
                components.create_action_row(|action| action.add_button(ticket_actions()))
            })
        })
        .await
        .unwrap();

    TicketHistory::new(TicketHistory {
        user_id: component.user.id.0,
        guild_id: guild_id.0,
        ticket_id: rand::random::<u64>(),
        ticket_channel: ticket_channel.1.id.0,
        ticket_status: "aberto".to_string(),
    })
    .unwrap();

    tickets_channel
        .send_message(&ctx, |msg| msg.set_embed(ticket_embed))
        .await
        .unwrap();
}
