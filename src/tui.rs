use audiotags::Tag;
use cursive::{event::Key, menu, view::{Nameable, Resizable}, views::Dialog};
use cursive_table_view::{TableView, TableViewItem};
use std::{cmp::Ordering, fs, path::Path};
use cursive::align::HAlign;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Column {
    Song,
    Artist,
    Album,
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
            Column::Album => "Album",
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
            Column::Album => todo!(),
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
            Column::Album => todo!(),
        }
    }
}

impl TableViewItem<Column> for AlbumItem {
    fn to_column(&self, column: Column) -> String {
        match column {
            Column::Album => self.album.clone(),
            Column::Artist => self.artist.clone(),
            _ => "".to_string(),
        }
    }

    fn cmp(&self, other: &Self, column: Column) -> Ordering
    where
        Self: Sized,
    {
        match column {
            Column::Album => self.album.cmp(&other.album),
            Column::Artist => self.artist.cmp(&other.artist),
            _ => Ordering::Equal,
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

fn populate_album_vec(artist_directory: &str) -> Vec<AlbumItem> {
    let artist_path = Path::new(artist_directory);
    let artist = artist_path.file_name().unwrap().to_str().unwrap().to_string();

    fs::read_dir(artist_directory)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            if entry.path().is_dir() {
                let album_path = entry.path();
                let album = album_path.file_name().unwrap().to_str().unwrap().to_string();
                let songs = populate_song_vec(album_path.to_str().unwrap());

                Some(AlbumItem {
                    album,
                    artist: artist.clone(),
                    songs,
                })
            } else {
                None
            }
        })
        .collect()
}



pub fn tui_run() {
    let mut siv = cursive::default();

    // Hard coded path to music
    let music_directory = r"C:\Users\tenoc\Music";

    // Assume the folders in the dir are all artists & their sub folders are all albums
    let artists = list_folders(music_directory);
    let mut albums_menu = menu::Tree::new(); // Instantiate empty menu to populate
    let mut artists_menu = menu::Tree::new(); // Empty artist menu

    for artist in &artists { // Loop through the artist dirs and format new paths to pass to list_folders
        let artist_directory = format!("{}/{}", music_directory, artist);
        let albums = list_folders(&artist_directory);
        let artist_name = artist.clone();
        let artist_directory_clone = artist_directory.clone();
        artists_menu.add_leaf(artist, move |s| {
            let items = populate_album_vec(&artist_directory_clone);
            //println!("{:?}", items);
            //s.pop_layer();
            let album_dialog = Dialog::around(TableView::<AlbumItem, Column>::new()
                .column(Column::Album, "Album", |c| c.width_percent(50))
                .column(Column::Artist, "Artist", |c| c.align(HAlign::Center).width_percent(50))
                .with_name("album_table")
                .min_size((50, 20)))
                .title(&format!("Albums - {}", artist_name))
                .with_name("album_dialog");
            s.add_layer(album_dialog);
            s.call_on_name("album_table", |table: &mut TableView<AlbumItem, Column>| {
                table.set_items(items);
            });
        });

        for album in &albums { // For each album we populate the menu with it
            let album_directory = format!("{}/{}", artist_directory, album);
            let album_name = album.clone();
            albums_menu.add_leaf(album, move |s| {
                let items = populate_song_vec(&album_directory);
                //s.pop_layer();
                s.pop_layer();
                let song_dialog = Dialog::around(TableView::<SongItem, Column>::new()
                    .column(Column::Song, "Track", |c| c.width_percent(35))
                    .column(Column::Artist, "Artist", |c| c.align(HAlign::Center).width_percent(20))
                    .column(Column::TrackLength, "Track Length", |c| {
                        c.ordering(Ordering::Greater)
                            .align(HAlign::Right)
                            .width_percent(40)
                    })
                    .with_name("song_table")
                    .min_size((50, 20)))
                    .title(&format!("Tracks - {}", album_name))
                    .with_name("song_dialog");
                    s.add_layer(song_dialog);
                    s.call_on_name("song_table", |table: &mut TableView<SongItem, Column>| {
                        table.set_items(items);
                    });
            });
        }
    }

    siv.menubar().add_subtree("Artists", artists_menu)
        .add_subtree("Albums", albums_menu)
        .add_delimiter()
        .add_leaf("Quit", |s| s.quit());

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());

    let album_table = TableView::<AlbumItem, Column>::new()
        .column(Column::Album, "Album", |c| c.width_percent(50))
        .column(Column::Artist, "Artist", |c| c.align(HAlign::Center).width_percent(50))
        .with_name("album_table");

    let song_table = TableView::<SongItem, Column>::new()
        .column(Column::Song, "Track", |c| c.width_percent(35))
        .column(Column::Artist, "Artist", |c| c.align(HAlign::Center).width_percent(20))
        .column(Column::TrackLength, "Track Length", |c| {
            c.ordering(Ordering::Greater)
                .align(HAlign::Right)
                .width_percent(40)
        })
        .with_name("song_table");

    //siv.add_layer(Dialog::around(album_table.min_size((50, 20))).title("Albums").with_name("dialog"));
    //siv.add_layer(Dialog::around(song_table.min_size((50, 20))).title("Tracks").with_name("dialog"));

    siv.run();
}
