use crate::db;
use crate::error::Error;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

fn play_sound_file(file_path: &str, volume: f32) -> Result<(), Error> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    let file = BufReader::new(File::open(file_path)?);
    let source = Decoder::new(file)?;
    sink.append(source);
    sink.set_volume(volume);
    sink.sleep_until_end();
    Ok(())
}

pub async fn play_mute_sound(new_state: bool) -> Result<(), Error> {
    let volume = db::get_mic_mute_audio_volume()?;
    let path = format!(
        "./resources/sounds/mic_{}.wav",
        if new_state { "muted" } else { "activated" }
    );
    let handle = std::thread::spawn(move || play_sound_file(&path, volume));

    handle.join().expect("play_sound_file should not panic")
}
