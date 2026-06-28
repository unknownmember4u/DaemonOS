//! Version subcommand execution handler.

use anyhow::Result;

/// Print version info.
///
/// # Errors
///
/// Returns an error if output writing fails.
pub fn handle() -> Result<()> {
    println!("daemon-cli version 0.1.0");
    Ok(())
}
