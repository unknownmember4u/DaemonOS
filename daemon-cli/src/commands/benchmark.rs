//! Benchmark command handler to display system diagnostics and calculate scores.

use crate::benchmark::run_benchmarks;
use anyhow::Result;
use tracing::info;

/// Execute the benchmarking sequence and print the formatted score sheet.
///
/// # Errors
///
/// Returns an error if benchmark execution fails.
pub fn handle() -> Result<()> {
    info!("Initializing hardware performance benchmarks...");
    println!("\x1b[1;36m=== DaemonOS Performance Benchmarks ===\x1b[0m");
    println!("Running hardware diagnostics... Please wait.\n");

    let (results, score) = run_benchmarks()?;
    let mut bottlenecks = Vec::new();

    for res in &results {
        let score_color = if res.score >= 90 {
            "\x1b[32m" // Green
        } else if res.score >= 70 {
            "\x1b[33m" // Yellow
        } else {
            "\x1b[31m" // Red
        };

        // Format execution time and metric cleanly
        let metric_str = if res.metric_value > 0.0 {
            format!("{:.2} {}", res.metric_value, res.metric_unit)
        } else {
            "N/A".to_string()
        };

        println!(
            "  \x1b[1m•\x1b[0m {:<28} : {:<20} - Score: {}{}/100\x1b[0m",
            res.name, metric_str, score_color, res.score
        );
        println!("    \x1b[2m[{}]\x1b[0m\n", res.description);

        // Identify bottlenecks based on threshold scores
        if res.name == "CPU Single-Thread Core" && res.score < 75 {
            bottlenecks.push((
                "CPU Core Performance",
                "Processor core speeds are sub-optimal. Close heavy background threads or inspect thermal throttling."
            ));
        } else if res.name == "RAM Allocation & Write Speed" && res.score < 75 {
            bottlenecks.push((
                "Memory Throughput",
                "Memory write throughput is low. Check motherboard dual-channel configurations or RAM speeds."
            ));
        } else if res.name == "Disk I/O Write & Read Speed" && res.score < 75 {
            bottlenecks.push((
                "Disk Read/Write Latency",
                "Local disk write/read transfer speeds are slow. Consider migrating project builds to an NVMe SSD."
            ));
        } else if res.name == "Process Spawn Latency" && res.metric_value > 10.0 {
            bottlenecks.push((
                "Process Spawn Overhead",
                "Spawning short-lived commands is slow. Background antivirus or file security scanners might be intercepting runs."
            ));
        } else if res.name == "File Search Latency" && res.metric_value > 80.0 {
            bottlenecks.push((
                "File Directory Scans",
                "Traversing directories is slow. Exclude build directories (target/) from indexers."
            ));
        } else if (res.name == "Git Command Latency" || res.name == "Rust Compiler Latency")
            && res.score > 0
            && res.score < 75
        {
            bottlenecks.push((
                "Toolchain Invocation Lag",
                "Git/Rustc compilation check speeds are slow. Verify shell hooks or local path setups."
            ));
        }
    }

    println!("\x1b[1m--------------------------------------------------\x1b[0m");

    // Color code overall score
    let score_color = if score >= 90 {
        "\x1b[1;32m" // Bold Green
    } else if score >= 70 {
        "\x1b[1;33m" // Bold Yellow
    } else {
        "\x1b[1;31m" // Bold Red
    };

    let rating = if score >= 90 {
        "Excellent (Developer Grade)"
    } else if score >= 70 {
        "Moderate"
    } else if score >= 50 {
        "Poor"
    } else {
        "Critical Performance Degradation"
    };

    println!(
        "Developer Performance Score: {}{}/100\x1b[0m ({})\n",
        score_color, score, rating
    );

    if !bottlenecks.is_empty() {
        println!("\x1b[1;33mDetected Bottlenecks & Optimization Suggestions:\x1b[0m");
        for (source, suggestion) in bottlenecks {
            println!(
                "  \x1b[1;33m•\x1b[0m \x1b[1m{:<25}\x1b[0m : {}",
                source, suggestion
            );
        }
        println!();
    } else {
        println!(
            "\x1b[32m✔ No performance bottlenecks detected. System is running at full speed!\x1b[0m\n"
        );
    }

    Ok(())
}
