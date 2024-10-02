use cursive::{event::Key, menu, views::Dialog};

pub fn tui_run () {
    let mut siv = cursive::default();


    siv.menubar().add_subtree("File", menu::Tree::new()
    .leaf("Name", |s| s.add_layer(Dialog::info("New file!"))))        
    .add_delimiter()
    .add_leaf("Quit", |s| s.quit());


    siv.add_global_callback(Key::Esc, |s| s.select_menubar());

    siv.add_layer(Dialog::text("Hit <Esc> to show the menu!"));

    siv.run();
}