use crate::{util,scripts};

pub fn add() {
    let reminder = util::get_input("Reminder");
    let date = util::parse_date(util::get_input("When"));

    let command = scripts::add_reminder(reminder, date);

    match command {
        Ok(_val) => println!("Reminder added."),
        Err(_err) => eprintln!("Error: Unable to add reminder."),
    };
}

pub fn complete() {
    let reminders = util::get_reminders();

    if reminders.len() == 0 {
        return println!("No reminders.");
    }

    let selected = util::get_choice(reminders);
    let command = scripts::complete_reminder(selected);

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
