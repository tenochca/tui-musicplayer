use cursive::{event::Key, menu, views::Dialog};
use std::fs;

pub fn list_folders(music_directory: &str) -> Vec<String> {
    fs::read_dir(music_directory)
    .unwrap()
    .filter_map(|entry| {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            Some(entry.file_name().into_string().unwrap())
        } else {
            None
        }
    })
    .collect()
}



pub fn tui_run () {
    let mut siv = cursive::default();

    siv.menubar().add_subtree("Albums", menu::Tree::new()) 
    .add_subtree("Artists", menu::Tree::new())       
    .add_delimiter()
    .add_leaf("Quit", |s| s.quit());


    siv.add_global_callback(Key::Esc, |s| s.select_menubar());

    siv.add_layer(Dialog::text("Hit <Esc> to show the menu!"));

    siv.run();
}