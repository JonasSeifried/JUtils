use std::path::PathBuf;

use crate::{
    error::Result,
    features::hotkey::{Hotkey, MICMUTE},
};
use log::{debug, info, warn};
use rusqlite::Connection;

fn open_db() -> Result<Connection> {
    let mut path = app_dirs2::app_root(app_dirs2::AppDataType::UserData, &crate::APP_INFO)?;
    #[cfg(debug_assertions)]
    path.clear();

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
    debug!("Created hotkeys table");
}

fn init_settings_table(db_connection: &Connection, resource_dir: Option<PathBuf>) {
    db_connection
        .execute(
            "CREATE TABLE IF NOT EXISTS settings (
            id INTEGER PRIMARY KEY,
            resource_dir TEXT,
            auto_launch BOOL,
            start_minimized_state BOOL,
            mute_state BOOL,
            mic_mute_audio_volume REAL
        )",
            (),
        )
        .expect("Failed to create settings Table");
    match resource_dir {
        Some(resource_dir) => {
            let resource_dir = resource_dir
                .to_str()
                .expect("Failed to convert resource_dir: PathBuf to &str");
            db_connection
            .execute(
                "INSERT OR IGNORE INTO settings(id, resource_dir, auto_launch, start_minimized_state, mute_state, mic_mute_audio_volume) VALUES(?1, ?2, ?3, ?4, ?5, ?6)",
                (1, resource_dir, true, false, false, 0.1),
            )
            .expect("Insert or ignore settings");
        }
        None => {
            warn!("No resource dir provided, skipping settings INSERT OR IGNORE");
            let _: String = db_connection
                .query_row("Select resource_path from settings", (), |row| row.get(0))
                .expect("No settings found, terminating");
            warn!("Could not INSERT OR IGNORE but found Settings");
        }
    }

    debug!("Setup settings table")
}

pub fn init_db(resource_dir: Option<PathBuf>) {
    let db_connection = open_db().expect("Failed to connect to database");
    init_hotkey_table(&db_connection);
    init_settings_table(&db_connection, resource_dir);
    info!("Database initialized")
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
    debug!("Fetched Hotkey {} -> {:?}", hotkey.name, hotkey.keys);
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
    debug!("Set hotkey {} -> {:?}", hotkey.name, hotkey.keys);
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
    debug!("Toggled mute_state -> {}", !current_state);
    Ok(!current_state)
}

pub fn fetch_mic_mute_audio_volume() -> Result<f32> {
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
    debug!("Set mic_mute_audio_volume -> {}", new_volume);
    Ok(())
}

#[cfg(not(debug_assertions))]
pub fn fetch_resource_dir() -> Result<String> {
    let conn = open_db()?;
    Ok(conn.query_row("Select resource_dir from settings", (), |row| row.get(0))?)
}

pub fn fetch_start_minimized_state() -> Result<bool> {
    let conn = open_db()?;
    let start_minimized_state =
        conn.query_row("Select start_minimized_state from settings", (), |row| {
            row.get(0)
        })?;
    debug!("Fetched start_minimized_state -> {}", start_minimized_state);
    Ok(start_minimized_state)
}

pub fn set_start_minimized_state(new_state: bool) -> Result<()> {
    let conn = open_db()?;
    conn.execute(
        "UPDATE settings SET start_minimized_state=?1 where id=1",
        (new_state,),
    )?;
    debug!("Set start_minimized_state -> {}", new_state);
    Ok(())
}
