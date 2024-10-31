mod tui;
mod audio;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 { 
        let first_arg: &str = &args[1];
    tui::tui_run(first_arg);
    }
}
