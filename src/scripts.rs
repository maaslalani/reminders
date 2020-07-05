use std::process::{Command,ExitStatus,Output};
use std::io::Error;

pub fn add_reminder(reminder: String, date: String) -> Result<ExitStatus, Error> {
    Command::new("osascript")
        .arg("-e")
        .arg(include_str!("scripts/add.applescript"))
        .arg(&reminder)
        .arg(&date)
        .status()
}

pub fn complete_reminder(reminder: String) -> Result<Output, Error> {
    Command::new("osascript")
        .arg("-e")
        .arg(include_str!("scripts/complete.applescript"))
        .arg(&reminder)
        .output()
}

pub fn get_reminders() -> Result<Output, Error> {
    Command::new("osascript")
        .arg("-e")
        .arg(include_str!("scripts/list.applescript"))
        .output()
}
