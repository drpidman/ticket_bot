use serenity::{
    builder::{CreateChannel, CreateEmbed},
    json,
    model::{
        prelude::{
            message_component::MessageComponentInteraction, Channel, ChannelCategory, ChannelId,
            ChannelType, Interaction, PermissionOverwrite,
            PermissionOverwriteType,
        },
        Permissions,
    },
    prelude::Context,
    utils::Color,
};

use crate::config::{TICKET_CREATION_CATEGORY, TICKET_LOG_CHANNEL};

pub async fn ticket_menu(ctx: &Context, component: &MessageComponentInteraction, i: &Interaction) {
    let logchannel = ChannelId::from(TICKET_LOG_CHANNEL.parse::<u64>().unwrap());
    let category_tickets = ChannelId::from(TICKET_CREATION_CATEGORY.parse::<u64>().unwrap());

    let mut embed_ticket = CreateEmbed::default();

    let mut channel = CreateChannel::default();
    channel.kind(ChannelType::Text);

    let everyone_role = ctx
        .cache
        .guild_roles(component.guild_id.unwrap())
        .unwrap()
        .iter()
        .find(|role| role.1.name == "@everyone")
        .unwrap()
        .0
        .to_owned();

    println!("everyonerole {:?}", everyone_role);

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
        "support".to_string(),
        "question".to_string(),
        "problem".to_string(),
    ];

    let selected_choice = choices
        .iter()
        .find(|ch| ch.to_string() == component.data.values[0].to_string())
        .unwrap()
        .to_string();

    match selected_choice.as_str() {
        "support" => {
            embed_ticket.title("Requisição - Suporte".to_string());
            embed_ticket.color(Color::DARK_GREEN);

            channel.name(format!("suporte-{}", component.user.id));
            channel.permissions(channel_permissions);
            channel.category(category_tickets);
        }
        "question" => {
            embed_ticket.title("Requisição - Duvida".to_string());
            embed_ticket.color(Color::BLUE);

            channel.name(format!("duvida-{}", component.user.id));
            channel.permissions(channel_permissions);
            channel.category(category_tickets);
        }
        "problem" => {
            embed_ticket.title("Requisição - Problema".to_string());
            embed_ticket.color(Color::RED);

            channel.name(format!("problema-{}", component.user.id));
            channel.permissions(channel_permissions);
            channel.category(category_tickets);
        }
        _ => (),
    };

    ctx.http
        .as_ref()
        .create_channel(
            component.guild_id.unwrap().as_u64().to_owned(),
            &json::hashmap_to_json_map(channel.0),
            Some("Ticket de suporte"),
        )
        .await
        .unwrap();

    logchannel
        .send_message(&ctx, |msg| msg.set_embed(embed_ticket))
        .await
        .unwrap();
    println!("Interaction menu {:?}", selected_choice);
}
