use anyhow::Result;

use crate::gtr;

pub fn run() -> Result<()> {
    gtr::exec(&["list"])
}
