use crate::util;

pub fn add() {
    let reminder = util::get_input("Reminder");
    let date = util::parse_date(util::get_input("When"));
}

pub fn complete() {
    println!("TODO: Complete")
}

pub fn list() {
    println!("TODO: List")
}
