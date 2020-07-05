use dialoguer::{Input,theme};
use chrono_english::{parse_date_string,Dialect};
use chrono::prelude::*;

pub fn parse_date(string: String) -> String {
    match parse_date_string(&string, Local::now(), Dialect::Us) {
        Ok(val) => val.format("%c").to_string(),
        Err(err) => panic!(err),
    }
}

pub fn get_input(prompt: &str) -> String {
    let theme = theme::ColorfulTheme::default();
    match Input::<String>::with_theme(&theme)
        .with_prompt(prompt)
        .interact() {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Error: Failed to open prompt");
                panic!(err)
            }
        }
}
