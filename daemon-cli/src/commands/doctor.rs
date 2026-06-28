//! Doctor subcommand execution handler.

use crate::system::doctor::{CheckStatus, run_diagnostics};
use anyhow::Result;
use tracing::info;

/// Execute the enhanced doctor diagnostics subcommand.
///
/// # Errors
///
/// Returns an error if writing to stdout fails.
pub fn handle() -> Result<()> {
    info!("Running DaemonOS environment diagnostics...");
    println!("\x1b[1;36m=== DaemonOS Doctor Diagnostics ===\x1b[0m\n");

    let (groups, score) = run_diagnostics();
    let mut recommendations = Vec::new();

    for (group, checks) in groups {
        println!("\x1b[1m[{}]\x1b[0m", group.as_str());
        for check in checks {
            let (status_symbol, status_color, msg_color) = match check.status {
                CheckStatus::Installed => ("✓", "\x1b[32m", "\x1b[0m"), // Green
                CheckStatus::Missing => ("✗", "\x1b[31m", "\x1b[31m"),  // Red
                CheckStatus::Warning => ("⚠", "\x1b[33m", "\x1b[33m"),  // Yellow
            };

            println!(
                "  {}{}\x1b[0m {:<20} : {}{}\x1b[0m",
                status_color, status_symbol, check.name, msg_color, check.message
            );

            if let Some(rec) = check.recommendation {
                recommendations.push((check.name, rec));
            }
        }
        println!();
    }

    println!("\x1b[1m--------------------------------------------------\x1b[0m");

    // Color code health score based on quality boundaries
    let score_color = if score >= 90 {
        "\x1b[1;32m" // Bold Green
    } else if score >= 70 {
        "\x1b[1;33m" // Bold Yellow
    } else {
        "\x1b[1;31m" // Bold Red
    };

    let rating = if score >= 90 {
        "Excellent"
    } else if score >= 70 {
        "Good (Warnings Present)"
    } else if score >= 50 {
        "Poor"
    } else {
        "Critical Suboptimal Environment"
    };

    println!(
        "System Health Score: {}{}/100\x1b[0m ({})\n",
        score_color, score, rating
    );

    if !recommendations.is_empty() {
        println!("\x1b[1;33mRecommendations:\x1b[0m");
        for (name, rec) in recommendations {
            println!("  \x1b[1;33m•\x1b[0m \x1b[1m{:<20}\x1b[0m : {}", name, rec);
        }
        println!();
    } else {
        println!("\x1b[32m✔ Environment is completely healthy and ready for production!\x1b[0m\n");
    }

    Ok(())
}
