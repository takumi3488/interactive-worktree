use anyhow::{Result, bail};
use inquire::Select;

use crate::git;
use crate::gtr;

const EDITORS: &[&str] = &[
    "default", "cursor", "vscode", "zed", "idea", "nvim", "vim", "emacs", "sublime", "nano",
];

pub fn run() -> Result<()> {
    let branches = git::all_worktree_branches()?;
    if branches.is_empty() {
        bail!("No worktrees found");
    }

    let branch = Select::new("Select worktree:", branches).prompt()?;

    let editor = Select::new("Editor:", EDITORS.to_vec())
        .with_help_message("'default' uses gtr.editor.default config")
        .prompt()?;

    let mut args = vec!["editor", &branch];
    if editor != "default" {
        args.push("--editor");
        args.push(editor);
    }

    gtr::exec(&args)
}
