use crate::{
    error::Result,
    features::hotkey::{Hotkey, MICMUTE},
};
use rusqlite::Connection;

fn open_db() -> Result<Connection> {
    let mut path = app_dirs2::app_root(app_dirs2::AppDataType::UserData, &crate::APP_INFO)?;
    #[cfg(debug_assertions)]
    {
        path.clear();
    }

    path.push(".db");
    Ok(Connection::open(path)?)
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
        .expect("Failed to create hotkeys Table");
    db_connection
        .execute(
            "INSERT OR IGNORE INTO hotkeys(name, keys) VALUES(?1, ?2)",
            (MICMUTE, ""),
        )
        .expect("Insert or ignore settings");
    println!("Debug: Created hotkeys table");
}

fn init_settings_table(db_connection: &Connection) {
    db_connection
        .execute(
            "CREATE TABLE IF NOT EXISTS settings (
            id INTEGER PRIMARY KEY,
            auto_launch BOOL,
            mute_state BOOL,
            mic_mute_audio_volume REAL
        )",
            (),
        )
        .expect("Failed to create settings Table");
    db_connection
        .execute(
            "INSERT OR IGNORE INTO settings(id, auto_launch, mute_state, mic_mute_audio_volume) VALUES(?1, ?2, ?3, ?4)",
            (1, true, false, 0.1),
        )
        .expect("Insert or ignore settings");
    println!("Debug: Created settings table");
}

pub fn init_db() {
    let db_connection = open_db().expect("Failed to connect to database");
    init_hotkey_table(&db_connection);
    init_settings_table(&db_connection);
    println!("Debug: Database initialized");
}

pub fn fetch_hotkey(hotkey_name: &str) -> Result<Hotkey> {
    let conn = open_db()?;
    let hotkey = conn.query_row(
        "SELECT * FROM hotkeys where name = ?1",
        (hotkey_name,),
        |row| {
            Ok(Hotkey {
                name: row.get::<usize, String>(0)?,
                keys: row
                    .get::<usize, String>(1)?
                    .split("&&")
                    .map(|s| s.to_string())
                    .filter(|s| s.len() != 0)
                    .collect::<Vec<String>>(),
            })
        },
    )?;
    println!("Debug: Fetched Hotkey {} -> {:?}", hotkey.name, hotkey.keys);
    Ok(hotkey)
}

pub fn set_hotkey(hotkey: Hotkey) -> Result<()> {
    let conn = open_db()?;
    conn.execute(
        "INSERT INTO hotkeys (name, keys)
        VALUES(?1, ?2) 
        ON CONFLICT(name) 
        DO UPDATE SET keys=?2;",
        (&hotkey.name, &hotkey.keys.join("&&")),
    )?;
    println!("Debug: Set hotkey {} -> {:?}", hotkey.name, hotkey.keys);

    Ok(())
}

pub fn toggle_mute_state() -> Result<bool> {
    let conn = open_db()?;
    let current_state: bool =
        conn.query_row("Select mute_state from settings", (), |row| row.get(0))?;
    conn.execute(
        "UPDATE settings SET mute_state=?1 where id=1",
        (!current_state,),
    )?;
    Ok(!current_state)
}

pub fn get_mic_mute_audio_volume() -> Result<f32> {
    let conn = open_db()?;
    Ok(
        conn.query_row("Select mic_mute_audio_volume from settings", (), |row| {
            row.get(0)
        })?,
    )
}

pub fn set_mic_mute_audio_volume(new_volume: f32) -> Result<()> {
    let conn = open_db()?;
    conn.execute(
        "UPDATE settings SET mic_mute_audio_volume=?1 where id=1",
        (new_volume,),
    )?;
    Ok(())
}
