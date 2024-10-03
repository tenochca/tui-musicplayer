use cursive::{event::Key, menu, views::Dialog};
use std::fs;
extern crate cursive_table_view;

struct SongItem {
    name: String,
}

pub fn list_folders(music_directory: &str) -> Vec<String> {
    fs::read_dir(music_directory) //read the contents of the directory
    .unwrap() //handles errors
    .filter_map(|entry| { //iterates through directory
        let entry = entry.unwrap();
        if entry.path().is_dir() { //checks if entry is directory
            Some(entry.file_name().into_string().unwrap()) //valid entries get turned into strings
        } else {
            None
        }
    })
    .collect() //collect all valid entries into Vec
}

pub fn list_files(music_directory: &str) -> Vec<String> {
    fs::read_dir(music_directory) //read the contents of the directory
    .unwrap() //handles errors
    .filter_map(|entry| { //iterates through directory
        let entry = entry.unwrap();
        if entry.path().is_file() { //checks if entry is a file
            Some(entry.file_name().into_string().unwrap()) //valid entries get turned into strings
        } else {
            None
        }
    })
    .collect() //collect all valid entries into Vec
}



pub fn tui_run () {
    let mut siv = cursive::default();

    //hard coded path to music
    let music_directory = r"C:\Users\tenoc\Music";

    //Assume the folders in the dir are all artists & their sub folders are all albums
    let artists = list_folders(music_directory);

    let mut albums_menu = menu::Tree::new(); //instantiate empty menu to populate

    for artist in &artists { //loop through the artist dirs and format new paths to pass to list_folders
        let artist_directory = format!("{}/{}", music_directory, artist);
        let albums = list_folders(&artist_directory);

        for album in albums { //for each album we populate the menu with it
            albums_menu.add_leaf(album, |_| {});
        }
    }
        let mut artists_menu = menu::Tree::new(); //empty artist menu

        for artist in artists { //populate artist menu
            artists_menu.add_leaf(artist, |_| {});
    }



    siv.menubar().add_subtree("Albums", albums_menu) 
    .add_subtree("Artists", artists_menu)       
    .add_delimiter()
    .add_leaf("Quit", |s| s.quit());


    siv.add_global_callback(Key::Esc, |s| s.select_menubar());

    siv.add_layer(Dialog::text("Hit <Esc> to show the menu!"));

    siv.run();
}