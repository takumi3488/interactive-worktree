use anyhow::{Result, bail};
use inquire::{Confirm, MultiSelect};

use crate::git;
use crate::gtr;

pub fn run() -> Result<()> {
    let branches = git::worktree_branches()?;
    if branches.is_empty() {
        bail!("No worktrees to remove (only main worktree exists)");
    }

    let selected = MultiSelect::new("Select worktrees to remove:", branches)
        .with_help_message("Space to select, Enter to confirm")
        .prompt()?;

    if selected.is_empty() {
        println!("No worktrees selected.");
        return Ok(());
    }

    let delete_branch = Confirm::new("Also delete the branch(es)?")
        .with_default(true)
        .prompt()?;

    let force = Confirm::new("Force removal (even if dirty)?")
        .with_default(false)
        .prompt()?;

    for branch in &selected {
        let mut args: Vec<&str> = vec!["rm", branch, "--yes"];
        if delete_branch {
            args.push("--delete-branch");
        }
        if force {
            args.push("--force");
        }
        if let Err(e) = gtr::exec(&args) {
            eprintln!("Failed to remove {branch}: {e}");
        }
    }

    Ok(())
}
