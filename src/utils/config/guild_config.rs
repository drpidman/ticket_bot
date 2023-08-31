use crate::database::models::{TicketConfig, Ticket};


pub fn get_config(guild_id: u64) -> Option<TicketConfig> {
    let config = TicketConfig::get(guild_id).unwrap();

    if config.is_none() {
        return None
    }

    config
}