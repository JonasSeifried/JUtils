use crate::{
    error::{Error, ToError},
    hotkey::Hotkey,
};
use rusqlite::Connection;

pub fn init_db() {
    let conn = open_db().expect("Failed to connect to database");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS hotkeys (
            name TEXT PRIMARY KEY,
            keys TEXT NOT NULL
        )",
        (),
    )
    .expect("Failed to connect to database");
}

fn open_db() -> Result<Connection, Error> {
    Connection::open(".db").map_err(|e| e.to_error())
}

pub fn fetch_hotkey(hotkey_name: &str) -> Result<Hotkey, Error> {
    let conn = open_db()?;
    conn.query_row(
        "SELECT * FROM hotkeys where name = ?1",
        (hotkey_name,),
        |row| {
            Ok(Hotkey {
                name: row.get(0)?,
                keys: row.get(1)?,
            })
        },
    )
    .map_err(|e| e.to_error())
}

pub fn set_hotkey(hotkey: Hotkey) -> Result<(), Error> {
    let conn = open_db()?;
    conn.execute(
        "INSERT INTO hotkeys (name, keys)
        VALUES(?1, ?2) 
        ON CONFLICT(name) 
        DO UPDATE SET keys=?2;",
        (hotkey.name, hotkey.keys),
    )
    .map_err(|err| err.to_error())
    .map(|_| ())
}
