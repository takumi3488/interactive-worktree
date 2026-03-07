use anyhow::{Result, bail};
use console::style;
use inquire::{Confirm, Select, Text};

use crate::{gh, gtr};

pub fn run() -> Result<()> {
    if !gh::is_available() {
        bail!(
            "gh CLI is not installed or not in PATH.\nInstall it from: {}",
            style("https://cli.github.com").cyan()
        );
    }

    let prs = gh::pr_list()?;
    if prs.is_empty() {
        bail!("No open pull requests found.");
    }

    let pr = Select::new("Select a pull request:", prs)
        .with_page_size(10)
        .prompt()?;

    let branch = pr.head_ref_name;

    // Fetch the latest from remote
    let status = std::process::Command::new("git")
        .args(["fetch", "origin", &branch])
        .status()
        .map_err(|e| anyhow::anyhow!("Failed to execute git fetch: {e}"))?;

    if !status.success() {
        bail!("git fetch origin {} failed", branch);
    }

    let from = format!("origin/{branch}");
    let mut args = vec!["new", &branch, "--from", &from];

    let post_options = vec!["None", "Open in editor", "Start AI tool"];
    let post = Select::new("After creation:", post_options)
        .with_help_message("Action to take after creating the worktree")
        .prompt()?;

    let ai_tool: String;
    match post {
        "Open in editor" => args.push("--editor"),
        "Start AI tool" => {
            ai_tool = Text::new("AI tool:")
                .with_placeholder("claude, aider, copilot, codex, ...")
                .with_help_message("Enter tool name, or press Enter for default")
                .prompt()?;
            args.push("--ai");
            if !ai_tool.is_empty() {
                args.push(&ai_tool);
            }
        }
        _ => {}
    }

    let no_copy = Confirm::new("Skip file copying?")
        .with_default(false)
        .prompt()?;

    if no_copy {
        args.push("--no-copy");
    }

    gtr::exec(&args)
}
