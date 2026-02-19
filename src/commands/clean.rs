use anyhow::Result;
use inquire::Confirm;

use crate::gtr;

pub fn run() -> Result<()> {
    let merged_only = Confirm::new("Only clean merged worktrees?")
        .with_default(true)
        .prompt()?;

    // Show what would be cleaned (dry-run)
    println!("\n--- Dry run ---");
    let mut dry_args = vec!["clean", "--dry-run"];
    if merged_only {
        dry_args.push("--merged");
    }
    let _ = gtr::exec(&dry_args);

    let proceed = Confirm::new("Proceed with cleanup?")
        .with_default(false)
        .prompt()?;

    if !proceed {
        println!("Cancelled.");
        return Ok(());
    }

    let mut args: Vec<&str> = vec!["clean", "--yes"];
    if merged_only {
        args.push("--merged");
    }

    gtr::exec(&args)
}
