use anyhow::Result;
use inquire::{Select, Text};

use crate::{commands::prompt_post_args, gtr};

pub fn run() -> Result<()> {
    let branch = Text::new("Branch name:")
        .with_help_message("Name for the new worktree branch")
        .prompt()?;

    let from_options = vec!["Default (main/master)", "Current branch", "Specific ref"];
    let from = Select::new("Starting point:", from_options).prompt()?;

    let mut args = vec!["new".to_string(), branch.clone()];

    match from {
        "Current branch" => args.push("--from-current".to_string()),
        "Specific ref" => {
            let ref_value = Text::new("Ref (branch/tag/commit):").prompt()?;
            args.push("--from".to_string());
            args.push(ref_value);
        }
        _ => {}
    }

    let (extra, ai_tool) = prompt_post_args()?;
    args.extend(extra);

    let args_str: Vec<&str> = args.iter().map(String::as_str).collect();
    gtr::exec(&args_str)?;

    if let Some(tool) = &ai_tool {
        gtr::exec(&["ai", &branch, "--ai", tool])?;
    }

    Ok(())
}
