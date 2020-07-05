mod reminders;
mod help;
mod util;

use std::env;

fn main() {
    match env::args().nth(1) {
        Some(val) => {
            match val.as_str() {
                "list" => reminders::list(),
                "add" => reminders::add(),
                "complete" => reminders::complete(),
                _ => help::usage(),
            }
        }
        None => reminders::add(),
    };
}
