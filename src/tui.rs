use cursive::{event::Key, menu, view::{Nameable, Resizable}, views::{BoxedView, Dialog, Layer, OnEventView}, View};
use cursive_table_view::{TableView, TableViewItem};
use std::{cmp::Ordering, fs};
use cursive::align::HAlign;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Column {
    Song,
    Artist,
    TrackLength,
}

#[derive(Clone, Debug)]
struct SongItem {
    song: String,
    artist: String,
    track_length: usize,
}


impl Column {
    fn as_str(&self) -> &str {
        match *self {
            Column::Song => "Song Name",
            Column::Artist => "Artist",
            Column::TrackLength => "Track Length",
        }
    }
}

impl TableViewItem<Column> for SongItem {
    //converts the column into a string
    fn to_column(&self, column: Column) -> String {
        match column {
            Column::Song => self.song.to_string(),
            Column::Artist => self.artist.to_string(),
            Column::TrackLength => format!("{}", self.track_length),
        }
    }

    fn cmp(&self, other: &Self, column: Column) -> Ordering
    where
        Self: Sized,
    {
        match column {
            Column::Song => self.song.cmp(&other.song),
            Column::Artist => self.artist.cmp(&other.artist),
            Column::TrackLength => self.track_length.cmp(&other.track_length),
        }
    }
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

    let mut table = TableView::<SongItem, Column>::new()
        .column(Column::Song, "Song", |c| c.width_percent(20))
        .column(Column::Artist, "Artist", |c| c.align(HAlign::Center))
        .column(Column::TrackLength, "Track Length", |c| {
            c.ordering(Ordering::Greater)
                .align(HAlign::Right)
                .width_percent(20)
        });

    let mut items = Vec::new();
        for i in 0..50 {
            items.push(SongItem {
                song: format!("Name {}", i),
                artist: format!("Artist {}", i),
                track_length: 1,
        });
    }

    table.set_items(items);

    siv.add_layer(Dialog::around(table.with_name("table").min_size((50, 20))).title("Table View"));



    //siv.add_layer(Dialog::text("Hit <Esc> to show the menu!"));

    siv.run();
}