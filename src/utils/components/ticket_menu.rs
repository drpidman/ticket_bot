use serenity::builder::{CreateSelectMenu};

pub fn ticket_menu() -> CreateSelectMenu {
    let mut menu = CreateSelectMenu::default();

    menu.custom_id("ticket_menu").options(|options| {
        options
            .create_option(|opt| {
                opt.label("Suporte")
                    .description("Contato rapido com a equipe de suporte")
                    .value("support")
            })
            .create_option(|opt| {
                opt.label("Duvida")
                    .description("Tire alguma duvida com a equipe de suporte")
                    .value("question")
            })
            .create_option(|opt| {
                opt.label("Problema")
                    .description("Relatar um problema para a equipe de suporte")
                    .value("problem")
            })
    });

    menu
}
