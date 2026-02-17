use anyhow::{Result, bail};
use inquire::Select;

use crate::git;
use crate::gtr;

const AI_TOOLS: &[&str] = &[
    "default", "claude", "aider", "copilot", "codex", "auggie", "continue", "gemini", "opencode",
];

pub fn run() -> Result<()> {
    let branches = git::all_worktree_branches()?;
    if branches.is_empty() {
        bail!("No worktrees found");
    }

    let branch = Select::new("Select worktree:", branches).prompt()?;

    let tool = Select::new("AI tool:", AI_TOOLS.to_vec())
        .with_help_message("'default' uses gtr.ai.default config")
        .prompt()?;

    let mut args = vec!["ai", &branch];
    if tool != "default" {
        args.push("--ai");
        args.push(tool);
    }

    gtr::exec(&args)
}
