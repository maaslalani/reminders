on run argv
  tell application "Reminders"
    set reminderName to item 1 of argv

    set dateTimeString to item 2 of argv
    set dateTime to date dateTimeString

    make new reminder with properties { name: reminderName, due date: dateTime}

    quit
  end tell
end run
