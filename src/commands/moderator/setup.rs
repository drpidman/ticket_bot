use serenity::{
    http::Http,
    model::{
        prelude::{
            application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
            command::{Command, CommandOptionType},
            ChannelId, ChannelType, Interaction, InteractionResponseType,
        },
        Permissions,
    },
    prelude::Context,
};

use crate::utils::components::ticket::ticket_menu;

pub async fn command_run(ctx: &Context, command: &ApplicationCommandInteraction, _i: &Interaction) {
    let options = &command.data.options;
    let option_channel = options.get(0).unwrap().resolved.as_ref().unwrap();
    let option_desc = options.get(1).unwrap().resolved.as_ref().unwrap();

    let option_banner = match options.get(2) {
        Some(attachment) => {
            if let CommandDataOptionValue::Attachment(file) = &attachment.resolved.as_ref().unwrap()
            {
                Some(file.clone())
            } else {
                None
            }
        }
        None => None,
    };

    let channel = if let CommandDataOptionValue::Channel(ch) = &option_channel {
        ch.id
    } else {
        ChannelId::from(0)
    };

    let description = if let CommandDataOptionValue::String(desc) = &option_desc {
        desc
    } else {
        "Empty description"
    };

    let _message = channel
        .send_message(&ctx.http, |msg| {
            msg.add_embed(|embed| {
                embed.title("Ticket").description(description).image({
                    if let Some(banner) = option_banner {
                        banner.url
                    } else {
                        "".to_string()
                    }
                })
            })
            .components(|component| {
                component.create_action_row(|action| {
                    action.add_select_menu(ticket_menu())
                })
            })
        })
        .await
        .unwrap();

    command
        .create_interaction_response(&ctx, |res| {
            res.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|msg| {
                    msg.content("Setup concluido com sucesso").ephemeral(true)
                })
        })
        .await
        .unwrap();
}

pub async fn register(http: &Http) {
    Command::create_global_application_command(http, |cmd| {
        cmd.name("setup")
            .description("Criar um setup ticket")
            .create_option(|option| {
                option
                    .kind(CommandOptionType::Channel)
                    .channel_types(&[ChannelType::Text])
                    .name("channel")
                    .description("Canal onde ficara o ticket")
                    .required(true)
            })
            .create_option(|option| {
                option
                    .kind(CommandOptionType::String)
                    .name("desc")
                    .description("Descrição do ticket")
                    .required(true)
            })
            .create_option(|option| {
                option
                    .kind(CommandOptionType::Attachment)
                    .name("banner")
                    .description("Banner da mensagem, caso necessario algo mais intuitivo.")
            })
            .default_member_permissions(Permissions::MANAGE_CHANNELS | Permissions::ADMINISTRATOR)
    })
    .await
    .unwrap();
}
