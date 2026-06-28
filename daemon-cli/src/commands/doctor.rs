//! Doctor subcommand execution handler.

use crate::system::tools::{ToolStatus, check_tool};
use anyhow::Result;
use tracing::info;

const TOOLS: &[&str] = &[
    "git", "rustc", "cargo", "gcc", "clang", "cmake", "ninja", "docker",
];

/// Execute the developer path tools checking sequence.
///
/// # Errors
///
/// Returns an error if writing to stdout fails.
pub fn handle() -> Result<()> {
    info!("Running developer environment check...");
    println!("--- DaemonOS Toolchain Check ---");

    let mut missing = 0;
    for tool in TOOLS {
        match check_tool(tool) {
            ToolStatus::Found(path) => {
                println!("✔ {:<8} : Found at {}", tool, path);
            }
            ToolStatus::NotFound => {
                println!("✘ {:<8} : NOT found in PATH", tool);
                missing += 1;
            }
        }
    }

    println!();
    if missing == 0 {
        println!("All standard developer tools are available.");
    } else {
        println!("Warning: {} tools are missing from PATH.", missing);
    }

    Ok(())
}
