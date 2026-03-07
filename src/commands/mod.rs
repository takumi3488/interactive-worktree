pub mod ai;
pub mod clean;
pub mod config;
pub mod copy;
pub mod doctor;
pub mod editor;
pub mod go;
pub mod list;
pub mod mv;
pub mod new;
pub mod pr;
pub mod rm;
pub mod run;

use anyhow::Result;
use inquire::{Confirm, Select, Text};

/// Prompt for post-creation action and return the extra args to pass to `gtr new`.
pub(crate) fn prompt_post_args() -> Result<Vec<String>> {
    let mut extra: Vec<String> = Vec::new();

    let post = Select::new(
        "After creation:",
        vec!["None", "Open in editor", "Start AI tool"],
    )
    .with_help_message("Action to take after creating the worktree")
    .prompt()?;

    match post {
        "Open in editor" => extra.push("--editor".to_string()),
        "Start AI tool" => {
            let ai_tool = Text::new("AI tool:")
                .with_placeholder("claude, aider, copilot, codex, ...")
                .with_help_message("Enter tool name, or press Enter for default")
                .prompt()?;
            extra.push("--ai".to_string());
            if !ai_tool.is_empty() {
                extra.push(ai_tool);
            }
        }
        _ => {}
    }

    if Confirm::new("Skip file copying?")
        .with_default(false)
        .prompt()?
    {
        extra.push("--no-copy".to_string());
    }

    Ok(extra)
}
