use anyhow::Result;
use inquire::{Confirm, Select, Text};

use crate::gtr;

pub fn run() -> Result<()> {
    let branch = Text::new("Branch name:")
        .with_help_message("Name for the new worktree branch")
        .prompt()?;

    let from_options = vec!["Default (main/master)", "Current branch", "Specific ref"];
    let from = Select::new("Starting point:", from_options).prompt()?;

    let mut args = vec!["new", &branch];
    let ref_value: String;
    let ai_tool: String;

    match from {
        "Current branch" => args.push("--from-current"),
        "Specific ref" => {
            ref_value = Text::new("Ref (branch/tag/commit):").prompt()?;
            args.push("--from");
            args.push(&ref_value);
        }
        _ => {}
    }

    let post_options = vec!["None", "Open in editor", "Start AI tool"];
    let post = Select::new("After creation:", post_options)
        .with_help_message("Action to take after creating the worktree")
        .prompt()?;

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
