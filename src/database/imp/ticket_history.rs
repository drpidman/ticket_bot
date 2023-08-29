use rusqlite::Connection;
use serenity::async_trait;

use crate::database::models::{TicketHistories, TicketHistory};

impl TicketHistories for TicketHistory {
    fn new(ticket: TicketHistory) -> Result<(), rusqlite::Error> {
        let mut db = Connection::open("config.db")?;
        let transaction = db.transaction()?;

        match transaction.execute(
            "INSERT INTO ticket_history (ticket_id, guild_id, user_id, ticket_channel, ticket_status) VALUES(
                :ticket_id,
                :guild_id,
                :user_id,
                :ticket_channel,
                :ticket_status
            )",
            &[
                (":ticket_id", &ticket.ticket_id.to_string()),
                (":guild_id", &ticket.guild_id.to_string()),
                (":user_id", &ticket.user_id.to_string()),
                (":ticket_channel", &ticket.ticket_channel.to_string()),
                (":ticket_status", &ticket.ticket_status.to_string()),
            ],
        ) {
            Ok(_) => {
                transaction.commit().unwrap();
                db.close().unwrap();
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn get_by_channel(channel_id: u64) -> Result<Option<TicketHistory>, rusqlite::Error> {
        println!("get_by_channel():{}", channel_id);

        let mut db = Connection::open("config.db")?;
        let transaction = db.transaction()?;

        let mut stmt = match transaction
            .prepare("SELECT * FROM ticket_history WHERE ticket_channel = :channelid")
        {
            Ok(status) => status,
            Err(err) => {
                println!("Ocorreu um erro ao preparar o estado:{:?}", err);
                panic!("{:?}", err)
            }
        };

        println!("get_by_channel():stmt");

        let mut query = match stmt.query(&[(":channelid", &channel_id)]) {
            Ok(query) => query,
            Err(err) => {
                println!("Ocorreu um erro ao preparar o estado:{:?}", err);
                panic!("{:?}", err)
            }
        };

        println!("get_by_channel():query");

        let ticket = if let Some(row) = query.next()? {
            println!("query row:{:?}", row);
            Some(TicketHistory {
                ticket_id: row.get::<usize, u64>(0)?,
                guild_id: row.get(1)?,
                user_id: row.get(2)?,
                ticket_channel: row.get(3)?,
                ticket_status: row.get::<usize, String>(4).map(|s| s.to_string()).unwrap(),
            })
        } else {
            println!("None()");
            None
        };

        println!("ticket in query row:{:?}", ticket);

        if ticket.is_some() {
            Ok(Some(ticket.unwrap()))
        } else {
            Ok(None)
        }
    }

    fn close_ticket(ticket_id: u64) -> Result<(), rusqlite::Error> {
        let mut db = Connection::open("config.db")?;
        let transaction = db.transaction()?;

        match transaction.execute(
            "UPDATE ticket_history SET ticket_status=\"fechado\" WHERE ticket_id = :ticket_id",
            &[(":ticket_id", &ticket_id)],
        ) {
            Ok(_) => {
                transaction.commit().unwrap();
                db.close().unwrap();
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}
