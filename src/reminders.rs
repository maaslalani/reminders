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
    println!("TODO: Complete")
}

pub fn list() {
    println!("TODO: List")
}
