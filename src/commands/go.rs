use anyhow::{Result, bail};
use inquire::Select;

use crate::git;
use crate::gtr;

pub fn run() -> Result<()> {
    let branches = git::all_worktree_branches()?;
    if branches.is_empty() {
        bail!("No worktrees found");
    }

    let branch = Select::new("Select worktree:", branches).prompt()?;

    gtr::exec(&["go", &branch])
}
