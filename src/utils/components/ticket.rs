use serenity::{
    builder::{CreateButton, CreateSelectMenu},
    model::prelude::{
        component::ButtonStyle, message_component::MessageComponentInteraction, ChannelId,
        GuildChannel, InteractionResponseType,
    },
    prelude::Context,
};

pub struct Metadata {
    pub request: String,
    pub user: String,
}

pub fn channel_parser(channel: &str) -> Vec<String> {
    channel
        .replace('-', " ")
        .split(' ')
        .map(|s| s.to_string())
        .collect()
}

pub fn get_metadata(channel: &str) -> Option<Metadata> {
    let metadata = channel_parser(channel);

    if metadata.len() >= 2 {
        return Some(Metadata {
            request: metadata.get(0).unwrap().to_owned(),
            user: metadata.get(1).unwrap().to_owned(),
        });
    } else {
        None
    }
}

pub async fn get_ticket_channel(
    ctx: &Context,
    component: &MessageComponentInteraction,
) -> Option<(ChannelId, GuildChannel)> {
    let channels = component.guild_id.unwrap().channels(ctx).await.unwrap();

    let user_id_str = component.user.id.to_string();
    let channel = channels
        .iter()
        .find(|(_, ch)| ch.name.contains(&user_id_str));

    channel.map(|(channel_id, channel)| (*channel_id, channel.clone()))
}

pub async fn is_user_ticket(ctx: &Context, component: &MessageComponentInteraction) -> bool {
    let channel = get_ticket_channel(ctx, component).await;

    if channel.is_some() {
        component
            .create_interaction_response(&ctx, |res| {
                res.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|msg| {
                        msg.content("Você já abriu um ticket anteriormente")
                            .ephemeral(true)
                    })
            })
            .await
            .unwrap();

        return true;
    }

    false
}

pub fn ticket_actions() -> CreateButton {
    let mut button = CreateButton::default();

    button
        .custom_id("ticket_button_actions")
        .label("Fechar Ticket")
        .style(ButtonStyle::Danger);

    button
}

pub fn ticket_menu() -> CreateSelectMenu {
    let mut menu = CreateSelectMenu::default();

    menu.custom_id("ticket_menu").options(|options| {
        options
            .create_option(|opt| {
                opt.label("Suporte")
                    .description("Contato rapido com a equipe de suporte")
                    .value("suporte")
                    .default_selection(false)
            })
            .create_option(|opt| {
                opt.label("Duvida")
                    .description("Tire alguma duvida com a equipe de suporte")
                    .value("duvida")
                    .default_selection(false)
            })
            .create_option(|opt| {
                opt.label("Problema")
                    .description("Relatar um problema para a equipe de suporte")
                    .value("problema")
                    .default_selection(false)
            })
    });

    menu
}
