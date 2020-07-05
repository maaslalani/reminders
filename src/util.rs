use chrono::prelude::*;
use chrono_english::{parse_date_string, Dialect};
use dialoguer::{theme, Input, Select};
use std::process::Command;
use std::str;

pub fn parse_date(string: String) -> String {
    match parse_date_string(&string, Local::now(), Dialect::Us) {
        Ok(val) => val.format("%B %d %Y %r").to_string(),
        Err(_err) => {
            eprintln!("Error: Invalid Date.");
            std::process::exit(1);
        }
    }
}

pub fn get_input(prompt: &str) -> String {
    let theme = theme::ColorfulTheme::default();
    match Input::<String>::with_theme(&theme)
        .with_prompt(prompt)
        .interact()
        {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Error: Failed to open prompt");
                panic!(err)
            }
        }
}

pub fn get_choice(options: Vec<String>) -> String {
    let theme = theme::ColorfulTheme::default();
    let selection = match Select::with_theme(&theme)
        .default(0)
        .items(&options[..])
        .interact()
        {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Error: Failed to open prompt");
                panic!(err)
            }
        };

    options[selection].to_string()
}

pub fn get_reminders() -> Vec<String> {
    let command = Command::new("osascript")
        .arg("src/scripts/list.applescript")
        .output();

    match command {
        Ok(val) => match str::from_utf8(&val.stdout) {
            Ok(val) => {
                let reminders = val.trim_end();
                if reminders == "" {
                    return vec![];
                }
                reminders
                    .split(", ")
                    .map(|reminder| reminder.to_string())
                    .collect()
            }
            Err(_err) => panic!("Error: Unable to get reminders."),
        },
        Err(_err) => panic!("Error: Unable to complete."),
    }
}
