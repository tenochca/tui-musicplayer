use cursive::{views::{Dialog, TextView}, Cursive};
use rusty_audio::Audio;

pub fn play_audio(file_path: &str, s: &mut Cursive) {
    let mut audio = Audio::new();
    audio.add("song", file_path);
    audio.play("song");
    let now_playing = format!("Now playing...\n{file_path}");
    s.add_layer(Dialog::around(TextView::new(now_playing)));
    audio.wait(); // Block until the sound finishes playing
}
