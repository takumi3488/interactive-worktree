use anyhow::{Result, bail};
use inquire::{Select, Text};

use crate::git;
use crate::gtr;

pub fn run() -> Result<()> {
    let branches = git::worktree_branches()?;
    if branches.is_empty() {
        bail!("No worktrees to rename (only main worktree exists)");
    }

    let branch = Select::new("Select worktree to rename:", branches).prompt()?;

    let new_name = Text::new("New name:")
        .with_help_message("New branch/worktree name")
        .prompt()?;

    gtr::exec(&["mv", &branch, &new_name, "--yes"])
}
