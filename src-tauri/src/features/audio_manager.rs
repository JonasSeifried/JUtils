use crate::db;
use crate::error::Result;
use log::{debug, error};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

fn play_sound_file(file_path: &str, volume: f32) -> Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    let file = BufReader::new(File::open(file_path)?);
    let source = Decoder::new(file)?;
    sink.append(source);
    sink.set_volume(volume);
    sink.sleep_until_end();
    Ok(())
}

pub fn play_mute_sound(new_state: bool) -> Result<()> {
    let volume = db::fetch_mic_mute_audio_volume()?;
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
