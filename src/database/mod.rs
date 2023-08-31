use rusqlite::Connection;

pub mod imp;
pub mod models;

pub fn init() {
    let db = Connection::open("config.db").unwrap();

    db.execute(
        "CREATE TABLE IF NOT EXISTS config(
            guild INTEGER PRIMARY KEY,
            category_id INTEGER,
            ticket INTEGER,
            ticket_log INTEGER
        )",
        (),
    )
    .unwrap();

    db.execute(
        "CREATE TABLE IF NOT EXISTS ticket_history (
        ticket_id INTEGER, guild_id INTEGER, user_id INTEGER, ticket_channel INTEGER,ticket_status TEXT
    )",
        (),
    )
    .unwrap();
}
