use anyhow::Result;
use inquire::{Select, Text};

use crate::gtr;

const CONFIG_KEYS: &[&str] = &[
    "gtr.worktrees.dir",
    "gtr.worktrees.prefix",
    "gtr.defaultBranch",
    "gtr.editor.default",
    "gtr.editor.workspace",
    "gtr.ai.default",
    "gtr.copy.include",
    "gtr.copy.exclude",
    "gtr.copy.includeDirs",
    "gtr.copy.excludeDirs",
    "gtr.hook.postCreate",
    "gtr.hook.preRemove",
    "gtr.hook.postRemove",
    "gtr.hook.postCd",
    "gtr.ui.color",
];

pub fn run() -> Result<()> {
    let actions = vec!["show", "get", "set", "unset"];
    let action = Select::new("Config action:", actions).prompt()?;

    match action {
        "show" => gtr::exec(&["config", "show"]),
        "get" => {
            let key = Select::new("Config key:", CONFIG_KEYS.to_vec()).prompt()?;
            gtr::exec(&["config", "get", key])
        }
        "set" => {
            let key = Select::new("Config key:", CONFIG_KEYS.to_vec()).prompt()?;
            let value = Text::new("Value:").prompt()?;
            gtr::exec(&["config", "set", key, &value])
        }
        "unset" => {
            let key = Select::new("Config key:", CONFIG_KEYS.to_vec()).prompt()?;
            gtr::exec(&["config", "unset", key])
        }
        _ => unreachable!(),
    }
}
