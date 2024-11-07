use cursive::{view::{Offset, Position}, views::{Dialog, TextView}, Cursive};
use rusty_audio::Audio;
use std::thread;

use crate::tui::SongItem;

pub fn play_audio(file_path: &str, s: &mut Cursive, song_item: &SongItem) {
    // Display the now playing dialog
    let now_playing = format!("Now playing...\n{}", song_item.song);
    let dialog = Dialog::around(TextView::new(now_playing));
    s.screen_mut().add_layer_at(Position::new(Offset::Absolute(95), Offset::Absolute(10)), dialog);

    // Clone the file_path to move it into the new thread
    let file_path = file_path.to_string();

    // Spawn a new thread for audio playback
    thread::spawn(move || {
        let mut audio = Audio::new();
        audio.add("song", &file_path);
        audio.play("song");
        audio.wait(); // Block until the sound finishes playing
    });
}

//TODO: find a way to stop the song table from being cleared. Find a way to queueu songs. Allow user to select album and be show the songs
// Get user directpory input working
