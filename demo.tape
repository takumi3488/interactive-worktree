Output demo.gif
Set FontSize 14
Set Width 800
Set Height 600
Set TypingSpeed 100ms

Type "iwt"
Sleep 750ms
Enter

Sleep 2s

# "new" is the first item, already selected - just press Enter
Enter

Sleep 1s

# Enter branch name
Type "feature/demo-branch"
Sleep 500ms
Enter

Sleep 1s

# Select "Default (main/master)" - first option
Enter

Sleep 1s

# Select "None" for post-creation action - first option
Enter

Sleep 1s

# Skip file copying? No (default)
Enter

Sleep 12s

# Now select "list" (2nd item)
Down
Sleep 500ms
Enter

Sleep 2s

# Select "rm" (3rd item)
Down
Down
Sleep 500ms
Enter

Sleep 1s

# Space to select the new worktree
Space
Sleep 500ms
Enter

Sleep 1s

# Delete branch? No (default)
Enter

Sleep 1s

# Force removal? No (default)
Enter

Sleep 2s

# List again to confirm removal (2nd item)
Down
Sleep 500ms
Enter

Sleep 12s

# Quit
Ctrl+C
