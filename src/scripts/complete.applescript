on run argv
  tell application "Reminders"
    set completed of (first reminder whose name contains item 1 of argv and completed is false) to true
    quit
  end tell
end run
