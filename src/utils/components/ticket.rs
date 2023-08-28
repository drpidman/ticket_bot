use serenity::{
    builder::CreateSelectMenu,
    model::prelude::{message_component::MessageComponentInteraction, InteractionResponseType},
    prelude::Context,
};

pub async fn is_ticket(ctx: &Context, component: &MessageComponentInteraction) -> bool {
    let channels = component.guild_id.unwrap().channels(&ctx).await.unwrap();

    let channel = channels
        .iter()
        .find(|ch| ch.1.name.contains(&format!("{}", component.user.id)));

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
