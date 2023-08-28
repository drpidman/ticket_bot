use rusqlite::Connection;

pub mod imp;
pub mod models;

pub fn init() {
    let db = Connection::open("config.db").unwrap();

    db.execute(
        "CREATE TABLE IF NOT EXISTS config(guild INTEGER PRIMARY KEY, ticket INTEGER)",
        (),
    )
    .unwrap();
}
