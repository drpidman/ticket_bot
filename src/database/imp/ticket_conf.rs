use rusqlite::{Connection, Error};

use crate::database::models::{Ticket, TicketConfig};

impl Ticket for TicketConfig {
    fn new(ticket: TicketConfig) -> Result<(), Error> {
        let mut db = Connection::open("config.db").unwrap();
        let transaction = db.transaction().unwrap();

        match transaction.execute(
            "INSERT INTO config (guild, ticket) VALUES(:guild, :ticket)",
            &[(":guild", &ticket.guild_id), (":ticket", &ticket.ticket_id)],
        ) {
            Ok(_) => Ok(()),
            Err(err) => {
                println!("Ocorreu um erro ao inserir valores: {:?}", err);
                Err(Error::ExecuteReturnedResults)
            }
        }
        .unwrap();

        match transaction.commit() {
            Ok(_) => {
                db.close().unwrap();
                Ok(())
            }
            Err(err) => {
                println!("Erro ao confirmar alterações: {:?}", err);
                Err(Error::ExecuteReturnedResults)
            }
        }
    }

    fn get(guild_id: u64) -> Result<Option<TicketConfig>, Error> {
        let db = Connection::open("config.db").unwrap();

        let mut stmt = db.prepare("SELECT * FROM config WHERE guild = :guild")?;
        let mut query = stmt.query(&[(":guild", &guild_id)]).unwrap();

        let ticket = if let Some(row) = query.next()? {
            Some(TicketConfig {
                guild_id: row.get(0)?,
                ticket_id: row.get(1)?,
            })
        } else {
            None
        };

        if ticket.is_some() {
            Ok(Some(ticket.unwrap()))
        } else {
            Ok(None)
        }
    }

    fn get_ticket(_ticket_id: u64) {}
}
