use anyhow::{Result, bail};
use inquire::Select;

use crate::{commands::prompt_post_args, gh, git, gtr};

pub fn run() -> Result<()> {
    let prs = gh::pr_list()?;
    if prs.is_empty() {
        bail!("No open pull requests found.");
    }

    let pr = Select::new("Select a pull request:", prs)
        .with_page_size(10)
        .prompt()?;

    let branch = pr.head_ref_name;
    git::fetch("origin", &branch)?;

    let from = format!("origin/{branch}");
    let mut args = vec!["new".to_string(), branch, "--from".to_string(), from];

    args.extend(prompt_post_args()?);

    let args_str: Vec<&str> = args.iter().map(String::as_str).collect();
    gtr::exec(&args_str)
}
