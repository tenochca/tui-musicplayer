use cursive::views::{Dialog, TextView};

pub fn simple_ui () {
    let mut siv = cursive::default();

    siv.add_layer(Dialog::around(TextView::new("Hello World!"))
        .title("Cursive TUI")
        .button("Quit", |s| s.quit()));
    
    siv.run();
}