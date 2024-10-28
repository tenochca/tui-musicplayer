mod tui;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect(); // Check if there is at least one argument (excluding the program name) 
    if args.len() > 1 { 
        let first_arg: &str = &args[1];
    tui::tui_run(first_arg);
    }
}
