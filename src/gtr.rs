#![allow(dead_code)]

use std::process::Command;

use anyhow::{Context, Result, bail};

/// Run `git gtr <args>` and inherit stdio (interactive).
pub fn exec(args: &[&str]) -> Result<()> {
    let status = Command::new("git")
        .arg("gtr")
        .args(args)
        .status()
        .context("Failed to execute git gtr")?;

    if !status.success() {
        bail!(
            "git gtr {} exited with {}",
            args.join(" "),
            status.code().unwrap_or(-1)
        );
    }
    Ok(())
}

/// Run `git gtr <args>` and capture stdout.
pub fn exec_capture(args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .arg("gtr")
        .args(args)
        .output()
        .context("Failed to execute git gtr")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("git gtr {} failed: {}", args.join(" "), stderr.trim());
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Check if gtr is available.
pub fn is_available() -> bool {
    Command::new("git")
        .args(["gtr", "version"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
