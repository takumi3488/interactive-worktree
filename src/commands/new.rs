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
    let mut ai_tool = String::new();

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
            if ai_tool.is_empty() {
                // デフォルトツール: --ai フラグのみ
                args.push("--ai");
            }
            // ツール指定ありの場合は作成後に別途 git gtr ai で起動
        }
        _ => {}
    }

    let no_copy = Confirm::new("Skip file copying?")
        .with_default(false)
        .prompt()?;

    if no_copy {
        args.push("--no-copy");
    }

    gtr::exec(&args)?;

    // ツール名を指定した場合、別途 git gtr ai で指定ツールを起動
    if !ai_tool.is_empty() {
        gtr::exec(&["ai", &branch, "--ai", &ai_tool])?;
    }

    Ok(())
}
