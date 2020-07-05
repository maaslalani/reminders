on run argv
  tell application "Reminders"
    set completed of (last reminder whose name is item 1 of argv) to true
    quit
  end tell
end run
