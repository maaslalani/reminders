mod reminders;
mod help;

use std::env;

fn main() {
    match env::args().nth(1) {
        Some(val) => {
            match val.as_str() {
                "list" => reminders::list(),
                "complete" => reminders::complete(),
                _ => help::usage(),
            }
        }
        None => {
            reminders::add();
        }
    };
}
