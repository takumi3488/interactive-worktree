Output demo.gif
Set FontSize 14
Set Width 800
Set Height 600
Set PlaybackSpeed 0.7

Type "iwt"
Sleep 300ms
Enter

Sleep 800ms

# "new" is the first item, already selected - just press Enter
Enter

Sleep 400ms

# Enter branch name
Type "feature/demo-branch"
Sleep 200ms
Enter

Sleep 400ms

# Select "Default (main/master)" - first option
Enter

Sleep 400ms

# Select "None" for post-creation action - first option
Enter

Sleep 400ms

# Skip file copying? No (default)
Enter

Sleep 5s

# Now select "list" (2nd item)
Down
Sleep 200ms
Enter

Sleep 800ms

# Select "rm" (3rd item)
Down
Down
Sleep 200ms
Enter

Sleep 400ms

# Space to select the new worktree
Space
Sleep 200ms
Enter

Sleep 400ms

# Delete branch? No (default)
Enter

Sleep 400ms

# Force removal? No (default)
Enter

Sleep 800ms

# List again to confirm removal (2nd item)
Down
Sleep 200ms
Enter

Sleep 5s

# Quit
Ctrl+C
