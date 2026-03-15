use anyhow::{Result, bail};
use inquire::{Select, Text};

use crate::{git, worktree_ops};

pub fn run() -> Result<()> {
    let wts = git::worktree_list()?;
    let branches: Vec<String> = wts.iter().skip(1).map(|w| w.branch.clone()).collect();
    if branches.is_empty() {
        bail!("No worktrees to rename (only main worktree exists)");
    }

    let branch = Select::new("Select worktree to rename:", branches).prompt()?;

    let old_path = wts
        .iter()
        .find(|w| w.branch == branch)
        .map(|w| w.path.clone())
        .ok_or_else(|| anyhow::anyhow!("Worktree not found for branch '{branch}'"))?;

    let new_name = Text::new("New name:")
        .with_help_message("New branch/worktree name")
        .prompt()?;

    let new_path = worktree_ops::worktree_dir_path(&new_name)?;

    git::branch_rename(&branch, &new_name)?;
    git::worktree_move(&old_path, &new_path)?;

    println!("Renamed '{branch}' → '{new_name}'");
    Ok(())
}
