use crate::util;
use std::process::Command;

pub fn add() {
    let reminder = util::get_input("Reminder");
    let date = util::parse_date(util::get_input("When"));

    let command = Command::new("osascript")
        .arg("-e")
        .arg(include_str!("scripts/add.applescript"))
        .arg(&reminder)
        .arg(&date)
        .status();

    match command {
        Ok(_val) => println!("Reminder added."),
        Err(_err) => eprintln!("Error: Unable to add reminder."),
    };
}

pub fn complete() {
    let options = util::get_reminders();

    if options.len() == 0 {
        return println!("No reminders.");
    }

    let selected = util::get_choice(options);

    let command = Command::new("osascript")
        .arg("-e")
        .arg(include_str!("scripts/complete.applescript"))
        .arg(&selected)
        .output();

    match command {
        Ok(_val) => println!("Marked reminder as complete."),
        Err(_err) => eprintln!("Error: Unable to mark reminder as complete."),
    };
}

pub fn list() {
    let reminders = util::get_reminders();

    if reminders.len() == 0 {
        return println!("No reminders.");
    }

    for reminder in reminders {
        println!("* {}", reminder);
    }
}
