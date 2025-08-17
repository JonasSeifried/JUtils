// use crate::db;
use crate::error::Result;
use log::{debug, error};
use rodio::OutputStreamBuilder;
use std::{fs::File, io::BufReader};

fn play_sound_file(file_path: &str, volume: f32) -> Result<()> {
    let mut stream_handle = OutputStreamBuilder::open_default_stream()?;
    stream_handle.log_on_drop(false);
    let file = BufReader::new(File::open(file_path)?);
    let sink = rodio::play(&stream_handle.mixer(), file)?;
    sink.set_volume(volume);

    sink.sleep_until_end();
    Ok(())
}

pub fn play_mute_sound(new_state: bool, volume: f32) -> Result<()> {
    let relative_path = format!(
        "resources\\sounds\\mic_{}.wav",
        if new_state { "muted" } else { "activated" }
    );
    #[cfg(not(debug_assertions))]
    let path = format!("{}\\{}", db::fetch_resource_dir()?, relative_path);
    #[cfg(debug_assertions)]
    let path = relative_path;
    std::thread::spawn(move || match play_sound_file(&path, volume) {
        Ok(_) => debug!("Played sound file: {:?}", path),
        Err(e) => {
            error!("Failed to play sound file: {};\nerror: {}", path, e);
        }
    });
    Ok(())
}
