//! CPU single-thread performance benchmark.

use crate::benchmark::{BenchmarkResult, BenchmarkTest};
use anyhow::Result;
use std::time::Instant;

/// CPU speed benchmark struct.
pub struct CpuBenchmark;

impl BenchmarkTest for CpuBenchmark {
    fn name(&self) -> &str {
        "CPU Single-Thread Core"
    }

    fn run(&self) -> Result<BenchmarkResult> {
        let start = Instant::now();

        // Compute prime numbers up to 20,000 using trial division
        let mut count = 0;
        for n in 2..20000 {
            let mut is_prime = true;
            let limit = (n as f64).sqrt() as u32;
            for i in 2..=limit {
                if n % i == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                count += 1;
            }
        }

        let elapsed = start.elapsed();
        let elapsed_ms = elapsed.as_secs_f64() * 1000.0;

        // Score scaling: lower elapsed time is better.
        let score = (5000.0 / (elapsed_ms + 40.0)).clamp(0.0, 100.0) as u32;

        Ok(BenchmarkResult {
            name: self.name().to_string(),
            execution_time: elapsed,
            metric_value: count as f64,
            metric_unit: "primes".to_string(),
            score,
            description: format!(
                "Calculated {} primes up to 20,000 in {:.2} ms",
                count, elapsed_ms
            ),
        })
    }
}
