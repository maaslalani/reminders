on run argv
  tell application "Reminders"
    set selected to first reminder whose name contains item 1 of argv and completed is false
    set completed of selected to true
  end tell
end run
