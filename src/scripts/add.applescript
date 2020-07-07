on run argv
  tell application "Reminders"
    set reminderName to item 1 of argv
    set dateTime to item 2 of argv

    make new reminder with properties { name: reminderName, due date: date dateTime }
    quit
  end tell
end run
