use anyhow::{Result, bail};
use inquire::{MultiSelect, Text};

use crate::git;
use crate::gtr;

pub fn run() -> Result<()> {
    let branches = git::worktree_branches()?;
    if branches.is_empty() {
        bail!("No worktrees to copy to (only main worktree exists)");
    }

    let targets = MultiSelect::new("Copy files to which worktrees:", branches)
        .with_help_message("Space to select, Enter to confirm")
        .prompt()?;

    if targets.is_empty() {
        println!("No worktrees selected.");
        return Ok(());
    }

    let pattern = Text::new("File pattern (optional, Enter to skip):")
        .with_help_message("e.g. *.json, package-lock.json")
        .prompt()?;

    for target in &targets {
        let mut args: Vec<&str> = vec!["copy", target];
        if !pattern.is_empty() {
            args.push(&pattern);
        }
        if let Err(e) = gtr::exec(&args) {
            eprintln!("Failed to copy to {target}: {e}");
        }
    }

    Ok(())
}
