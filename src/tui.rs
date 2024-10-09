use audiotags::Tag;
use cursive::{event::Key, menu, view::{Nameable, Resizable}, views::Dialog};
use cursive_table_view::{TableView, TableViewItem};
use std::{cmp::Ordering, fmt::format, fs, path::Path, vec};
use cursive::align::HAlign;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Column {
    Song,
    Artist,
    TrackLength,
}

#[derive(Clone, Debug)]
pub struct SongItem {
    song: String,
    artist: String,
    track_length: usize,
}

#[derive(Clone, Debug)]
struct AlbumItem {
    album: String,
    artist: String,
    songs: Vec<SongItem>,
}

#[derive(Clone, Debug)]
struct ArtistItem {
    artist: String,
    albums: Vec<AlbumItem>,
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

pub fn populate_song_vec(album_directory: &str) -> Vec<SongItem> {
    let album_path = Path::new(album_directory);
    let artist = album_path.parent().unwrap().file_name().unwrap().to_str().unwrap().to_string();

    fs::read_dir(album_directory)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            if entry.path().is_file() {
                let file_path = entry.path();
                let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
                //let tag = Tag::default().read_from_path(&file_path).unwrap();

                Some(SongItem {
                    song: file_name,
                    artist: artist.clone(),
                    track_length: 1, //placeholder
                })
            } else {
                None
            }
        })
        .collect()
}


pub fn tui_run () {
    let mut siv = cursive::default();

    //hard coded path to music
    let music_directory = r"C:\Users\tenoc\Music";

    //Assume the folders in the dir are all artists & their sub folders are all albums
    let artists = list_folders(music_directory);
    let mut albums_menu = menu::Tree::new(); //instantiate empty menu to populate
    let mut artists_menu = menu::Tree::new(); //empty artist menu

    for artist in &artists { //loop through the artist dirs and format new paths to pass to list_folders
        let artist_directory = format!("{}/{}", music_directory, artist);
        let albums = list_folders(&artist_directory);
        artists_menu.add_leaf(artist, |_| {});

        for album in albums { //for each album we populate the menu with it
            let albums_directory = format!("{}/{}", artist_directory, album);
            albums_menu.add_leaf(album, |_| {});
        }
    }
    


    siv.menubar().add_subtree("Albums", albums_menu)
    .add_subtree("Artists", artists_menu)       
    .add_delimiter()
    .add_leaf("Quit", |s| s.quit());


    siv.add_global_callback(Key::Esc, |s| s.select_menubar());

    let mut song_table = TableView::<SongItem, Column>::new()
        .column(Column::Song, "Track", |c| c.width_percent(20))
        .column(Column::Artist, "Artist", |c| c.align(HAlign::Center))
        .column(Column::TrackLength, "Track Length", |c| {
            c.ordering(Ordering::Greater)
                .align(HAlign::Right)
                .width_percent(20)
        });
        
        let album_dir: &str = r"C:\Users\tenoc\Music\RadioHead\Fake Plastic Trees";
        let items = populate_song_vec(album_dir);
 

    song_table.set_items(items);

    siv.add_layer(Dialog::around(song_table.with_name("table").min_size((50, 20))).title("Tracks"));


    //siv.add_layer(Dialog::text("Hit <Esc> to show the menu!"));

    siv.run();
}