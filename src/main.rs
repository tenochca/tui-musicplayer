mod tui;

fn main() {
    //tui::tui_run();
    let music = r"C:\Users\tenoc\Music";
    let folder = tui::list_folders(music);
    println! ("{:?}", folder);
}
