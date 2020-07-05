const HELP: &str = r#"
reminders is a command line interface for Reminders.app.

Usage: reminders [ACTION]

    reminders add       - Add a reminder
    reminders complete  - Mark a reminder as complete
    reminders help      - Print this usage information
    reminders list      - List reminders

Running reminders will automatically run reminders add.
"#;

pub fn usage() {
    println!("{}", HELP);
}
