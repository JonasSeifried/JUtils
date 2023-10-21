use crate::{error::Error, hotkey::Hotkey};
use rusqlite::Connection;

fn open_db() -> Result<Connection, Error> {
    Ok(Connection::open(".db")?)
}

fn init_hotkey_table(db_connection: &Connection) {
    db_connection
        .execute(
            "CREATE TABLE IF NOT EXISTS hotkeys (
                name TEXT PRIMARY KEY,
                keys TEXT NOT NULL
        )",
            (),
        )
        .expect("Failed to create hotkey Table");
}

fn init_settings_table(db_connection: &Connection) {
    db_connection
        .execute(
            "CREATE TABLE IF NOT EXISTS settings (
            id INTEGER PRIMARY KEY,
            mute_state BOOL
        )",
            (),
        )
        .expect("Failed to create settings Table");
    db_connection
        .execute(
            "INSERT OR IGNORE INTO settings(id, mute_state) VALUES(?1, ?2)",
            (1, false),
        )
        .expect("Insert or ignore settings");
}

pub fn init_db() {
    let db_connection = open_db().expect("Failed to connect to database");
    init_hotkey_table(&db_connection);
    init_settings_table(&db_connection);
}

pub fn fetch_hotkey(hotkey_name: &str) -> Result<Hotkey, Error> {
    let conn = open_db()?;
    let hotkey = conn.query_row(
        "SELECT * FROM hotkeys where name = ?1",
        (hotkey_name,),
        |row| {
            Ok(Hotkey {
                name: row.get(0)?,
                keys: row.get(1)?,
            })
        },
    )?;
    Ok(hotkey)
}

pub fn set_hotkey(hotkey: Hotkey) -> Result<(), Error> {
    let conn = open_db()?;
    conn.execute(
        "INSERT INTO hotkeys (name, keys)
        VALUES(?1, ?2) 
        ON CONFLICT(name) 
        DO UPDATE SET keys=?2;",
        (hotkey.name, hotkey.keys),
    )?;

    Ok(())
}

pub fn toggle_mute_state() -> Result<bool, Error> {
    let conn = open_db()?;
    let current_state: bool =
        conn.query_row("Select mute_state from settings", (), |row| row.get(0))?;
    conn.execute(
        "UPDATE settings SET mute_state=?1 where id=1",
        (!current_state,),
    )?;
    Ok(!current_state)
}
