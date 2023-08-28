use rusqlite::Error;
use serde::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct TicketConfig {
    pub guild_id: u64,
    pub ticket_id: u64
}

pub trait Ticket {
    fn new(ticket: TicketConfig) -> Result<(), Error>;
    fn get(guild_id: u64) -> Result<Option<TicketConfig>, Error>;
    fn get_ticket(ticket_id: u64);
}