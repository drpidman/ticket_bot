use serenity::builder::{CreateSelectMenu};

pub fn ticket_menu() -> CreateSelectMenu {
    let mut menu = CreateSelectMenu::default();

    menu.custom_id("ticket_menu").options(|options| {
        options
            .create_option(|opt| {
                opt.label("Suporte")
                    .description("Contato rapido com a equipe de suporte")
                    .value("suporte")
            })
            .create_option(|opt| {
                opt.label("Duvida")
                    .description("Tire alguma duvida com a equipe de suporte")
                    .value("duvida")
            })
            .create_option(|opt| {
                opt.label("Problema")
                    .description("Relatar um problema para a equipe de suporte")
                    .value("problema")
            })
    });

    menu
}
