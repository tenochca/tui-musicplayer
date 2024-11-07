use cursive::{event::Key, menu, view::{Nameable, Resizable}, views::{Dialog, EditView, TextView}, Cursive};
use cursive_table_view::{TableView, TableViewItem};
use std::{cmp::Ordering, fs, path::Path};
use cursive::align::HAlign;
use std::sync::Arc;
use crate::audio;



#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Column {
    Song,
    Artist,
    Album,
    TrackLength,
}

#[derive(Clone, Debug)]
pub struct SongItem {
    pub song: String,
    artist: String,
    track_length: usize,
}

#[derive(Clone, Debug)]
struct AlbumItem {
    album: String,
    artist: String,
    songs: Vec<SongItem>,
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


fn show_popup(s: &mut Cursive, name: &str) {
    if name.is_empty() {
        // Try again as many times as we need!
        s.add_layer(Dialog::info("Please enter your music directory!"));
    } else {
        let content = format!("Music Directory set to ... \n{name}\n Press <ESC> to pull up menu bar");
        // Remove the initial popup
        s.pop_layer();
        // And put a new one instead
        s.add_layer(Dialog::around(TextView::new(content)));
    }
}




pub fn tui_run(music_directory: &str) {
    let mut siv = cursive::default();

    add_initial_dialog(&mut siv);
    let (artists_menu, albums_menu) = build_menus(music_directory);

    siv.menubar().add_subtree("Artists", artists_menu)
        .add_subtree("Albums", albums_menu)
        .add_delimiter()
        .add_leaf("Quit", |s| s.quit());

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());

    siv.run();
}

fn add_initial_dialog(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Enter your music folder")
            .padding_lrtb(1, 1, 1, 0)
            .content(
                EditView::new()
                    .on_submit(show_popup)
                    .with_name("name")
                    .fixed_width(20),
            )
            .button("Ok", |s| {
                let name = s
                    .call_on_name("name", |view: &mut EditView| {
                        view.get_content()
                    })
                    .unwrap();
                show_popup(s, &name);
            }),
    );
}

fn build_menus(music_directory: &str) -> (menu::Tree, menu::Tree) {
    let artists = list_folders(music_directory);
    let mut albums_menu = menu::Tree::new();
    let mut artists_menu = menu::Tree::new();

    for artist in &artists {
        let artist_directory = format!("{}/{}", music_directory, artist);
        let albums = list_folders(&artist_directory);
        let artist_name = artist.clone();
        let artist_directory_clone = artist_directory.clone();
        artists_menu.add_leaf(artist, move |s| {
            show_albums(s, &artist_name, &artist_directory_clone);
        });

        for album in &albums {
            let album_directory = format!("{}/{}", artist_directory, album);
            let album_name = album.clone();
            let album_directory_clone = Arc::new(album_directory.clone());
            albums_menu.add_leaf(album, move |s| {
                show_songs(s, &album_name, Arc::clone(&album_directory_clone));
            });
        }
    }

    (artists_menu, albums_menu)
}

fn show_albums(s: &mut Cursive, artist_name: &str, artist_directory: &str) {
    let album_dialog = Dialog::around(TableView::<AlbumItem, Column>::new()
        .column(Column::Album, "Album", |c| c.width_percent(50))
        .column(Column::Artist, "Artist", |c| c.align(HAlign::Center).width_percent(50))
        .with_name("album_table")
        .min_size((50, 20)))
        .title(&format!("Albums - {}", artist_name))
        .with_name("album_dialog");
    s.add_layer(album_dialog);
    let items = populate_album_vec(artist_directory);
    s.call_on_name("album_table", |table: &mut TableView<AlbumItem, Column>| {
        table.set_items(items);
    });
}

fn show_songs(s: &mut Cursive, album_name: &str, album_directory: Arc<String>) {
    let song_dialog = Dialog::around(TableView::<SongItem, Column>::new()
        .column(Column::Song, "Track", |c| c.width_percent(35))
        .column(Column::Artist, "Artist", |c| c.align(HAlign::Center).width_percent(20))
        .column(Column::TrackLength, "Track Length", |c| {
            c.ordering(Ordering::Greater)
                .align(HAlign::Right)
                .width_percent(40)
        })
        .on_submit({
            let album_directory_clone = Arc::clone(&album_directory);
            move |s, row, _index| {
                let song_table = s.find_name::<TableView<SongItem, Column>>("song_table").unwrap();
                let song_item = song_table.borrow_item(row).unwrap();
                let song_path = format!("{}/{}", album_directory_clone, song_item.song);
                audio::play_audio(&song_path, s, song_item);
            }
        })
        .with_name("song_table")
        .min_size((50, 20)))
        .title(&format!("Tracks - {}", album_name))
        .with_name("song_dialog");
    s.add_layer(song_dialog);
    let items = populate_song_vec(&album_directory);
    s.call_on_name("song_table", |table: &mut TableView<SongItem, Column>| {
        table.set_items(items);
    });
}