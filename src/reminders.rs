use crate::util;
use std::process::Command;

pub fn add() {
    let reminder = util::get_input("Reminder");
    let date = util::parse_date(util::get_input("When"));

    let command = Command::new("osascript")
        .arg("src/scripts/add.applescript")
        .arg(&reminder)
        .arg(&date)
        .status();

    match command {
        Ok(_val) => println!("Added \"{}\"", &reminder),
        Err(_err) => eprintln!("Error: Unable to complete."),
    };
}

pub fn complete() {
    let options = util::get_reminders();
    let selected = util::get_choice(options);

    let command = Command::new("osascript")
        .arg("src/scripts/complete.applescript")
        .arg(&selected)
        .output();

    match command {
        Ok(_val) => println!("Marked \"{}\" as complete.", selected),
        Err(_err) => eprintln!("Error: Unable to mark as complete."),
    };
}

pub fn list() {
    println!("{}", util::get_reminders().join("\n"));
}
