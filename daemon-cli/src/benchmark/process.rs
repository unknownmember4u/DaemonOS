//! Process spawn latency benchmark.

use crate::benchmark::{BenchmarkResult, BenchmarkTest};
use anyhow::{Context, Result};
use std::process::Command;
use std::time::Instant;

/// Process spawning benchmark struct.
pub struct ProcessSpawnBenchmark;

impl BenchmarkTest for ProcessSpawnBenchmark {
    fn name(&self) -> &str {
        "Process Spawn Latency"
    }

    fn run(&self) -> Result<BenchmarkResult> {
        let iterations = 10;
        let start = Instant::now();

        for _ in 0..iterations {
            let mut cmd = Command::new("true");
            let mut child = cmd
                .spawn()
                .or_else(|_| Command::new("echo").spawn())
                .context("Failed to spawn process check commands")?;
            let _ = child.wait();
        }

        let elapsed = start.elapsed();
        let avg_duration = elapsed / iterations;
        let avg_ms = avg_duration.as_secs_f64() * 1000.0;

        // Score scaling: lower spawn latency is better.
        let score = (200.0 / (avg_ms + 1.0)).clamp(0.0, 100.0) as u32;

        Ok(BenchmarkResult {
            name: self.name().to_string(),
            execution_time: elapsed,
            metric_value: avg_ms,
            metric_unit: "ms".to_string(),
            score,
            description: format!("Average process spawn time: {:.2} ms", avg_ms),
        })
    }
}
