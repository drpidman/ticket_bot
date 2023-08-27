use serenity::{
    builder::CreateEmbed,
    model::prelude::{message_component::MessageComponentInteraction, ChannelId, Interaction},
    prelude::Context,
    utils::Color,
};

use crate::config::TICKET_LOG_CHANNEL;

pub async fn menu_run(ctx: &Context, component: &MessageComponentInteraction, i: &Interaction) {
    let mut embed_ticket = CreateEmbed::default();
    let channel = ChannelId::from(TICKET_LOG_CHANNEL.parse::<u64>().unwrap());

    let choices: Vec<String> = vec![
        "support".to_string(),
        "question".to_string(),
        "problem".to_string(),
    ];

    let selected_choice = &choices
        .iter()
        .find(|ch| ch.to_string() == component.data.values[0].to_string())
        .unwrap()
        .to_string();

    match *&selected_choice.as_str() {
        "support" => {
            embed_ticket.title("Requisição - Suporte".to_string());
            embed_ticket.color(Color::DARK_GREEN);
        }
        "question" => {
            embed_ticket.title("Requisição - Duvida".to_string());
            embed_ticket.color(Color::BLUE);
        }
        "problem" => {
            embed_ticket.title("Requisição - Problema".to_string());
            embed_ticket.color(Color::RED);
        }
        _ => (),
    }

    channel
        .send_message(&ctx, |msg| 
            msg.set_embed(embed_ticket)
            
        )
        .await
        .unwrap();
    println!("Interaction menu {:?}", selected_choice);
}
