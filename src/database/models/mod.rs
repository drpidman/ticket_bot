use rusqlite::Error;
use serde::*;
use serenity::async_trait;

#[derive(Debug, Serialize, Deserialize)]
pub struct TicketConfig {
    pub guild_id: u64,
    pub ticket_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TicketHistory {
    pub user_id: u64,
    pub guild_id: u64,
    pub ticket_id: u64,
    pub ticket_channel: u64,
    pub ticket_status: String,
}

pub trait Ticket {
    fn new(ticket: TicketConfig) -> Result<(), Error>;
    fn get(guild_id: u64) -> Result<Option<TicketConfig>, Error>;
    fn get_ticket(ticket_id: u64);
}

pub trait TicketHistories {
    fn new(ticket: TicketHistory) -> Result<(), Error>;
    fn get_by_channel(channel_id: u64) -> Result<Option<TicketHistory>, Error>;

    fn close_ticket(ticket_id: u64) -> Result<(), Error>; // sem outra função
}
