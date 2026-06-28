//! Git command execution latency benchmark.

use crate::benchmark::{BenchmarkResult, BenchmarkTest};
use anyhow::Result;
use std::process::Command;
use std::time::Instant;

/// Git latency benchmark struct.
pub struct GitBenchmark;

impl BenchmarkTest for GitBenchmark {
    fn name(&self) -> &str {
        "Git Command Latency"
    }

    fn run(&self) -> Result<BenchmarkResult> {
        let start = Instant::now();
        let output = Command::new("git").arg("--version").output();
        let elapsed = start.elapsed();
        let elapsed_ms = elapsed.as_secs_f64() * 1000.0;

        match output {
            Ok(out) if out.status.success() => {
                // Score scaling: lower latency is better.
                let score = (500.0 / (elapsed_ms + 4.0)).clamp(0.0, 100.0) as u32;

                Ok(BenchmarkResult {
                    name: self.name().to_string(),
                    execution_time: elapsed,
                    metric_value: elapsed_ms,
                    metric_unit: "ms".to_string(),
                    score,
                    description: format!("Resolved git version in {:.2} ms", elapsed_ms),
                })
            }
            _ => Ok(BenchmarkResult {
                name: self.name().to_string(),
                execution_time: elapsed,
                metric_value: 0.0,
                metric_unit: "ms".to_string(),
                score: 0,
                description: "Git command execution failed or git is not installed.".to_string(),
            }),
        }
    }
}
