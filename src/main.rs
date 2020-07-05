mod help;
mod reminders;
mod util;
mod scripts;

use std::env;

fn main() {
    let action = match env::args().nth(1) {
        Some(val) => val,
        None => "add".to_string(),
    };

    match action.as_str() {
        "list" => reminders::list(),
        "add" => reminders::add(),
        "complete" => reminders::complete(),
        "help" => help::usage(),
        _ => help::usage(),
    };
}
