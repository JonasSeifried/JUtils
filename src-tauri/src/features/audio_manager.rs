use crate::error::Result;
use log::{debug, error};
use rodio::OutputStreamBuilder;
use tauri::{path::BaseDirectory, AppHandle, Manager};
use std::{fs::File, io::BufReader, path::PathBuf};

fn play_sound_file(file_path: &PathBuf, volume: f32) -> Result<()> {
    let mut stream_handle = OutputStreamBuilder::open_default_stream()?;
    stream_handle.log_on_drop(false);
    let file = BufReader::new(File::open(file_path)?);
    let sink = rodio::play(&stream_handle.mixer(), file)?;
    sink.set_volume(volume);

    sink.sleep_until_end();
    Ok(())
}

pub fn play_mute_sound(new_state: bool, volume: f32, app_handle: AppHandle) -> Result<()> {
    let path = app_handle.path().resolve(format!("resources/sounds/mic_{}.wav", if new_state { "muted" } else { "activated" }), BaseDirectory::Resource)?;


    std::thread::spawn(move || match play_sound_file(&path, volume) {
        Ok(_) => debug!("Played sound file: {:?}", path),
        Err(e) => {
            error!("Failed to play sound file: {};\nerror: {}", path.to_str().unwrap_or("<coudn't convert path to string>"), e);
        }
    });
    Ok(())
}
