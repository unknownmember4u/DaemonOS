//! RAM allocation and memory write throughput benchmark.

use crate::benchmark::{BenchmarkResult, BenchmarkTest};
use anyhow::Result;
use std::time::Instant;

/// RAM speed benchmark struct.
pub struct RamBenchmark;

impl BenchmarkTest for RamBenchmark {
    fn name(&self) -> &str {
        "RAM Allocation & Write Speed"
    }

    fn run(&self) -> Result<BenchmarkResult> {
        const BUFFER_SIZE: usize = 64 * 1024 * 1024; // 64 MB
        const CHUNK_SIZE: usize = 4 * 1024; // 4 KB

        let start = Instant::now();
        let mut buffer = vec![0u8; BUFFER_SIZE];

        // Access and write chunks to force page fault mapping
        for chunk in buffer.chunks_mut(CHUNK_SIZE) {
            if let Some(first) = chunk.first_mut() {
                *first = 0xAA;
            }
        }

        let elapsed = start.elapsed();
        let elapsed_secs = elapsed.as_secs_f64();
        let mb_written = (BUFFER_SIZE as f64) / (1024.0 * 1024.0);

        let throughput = if elapsed_secs > 0.0 {
            mb_written / elapsed_secs
        } else {
            0.0
        };

        // Score: RAM throughput.
        let score = (throughput / 35.0).clamp(0.0, 100.0) as u32;

        Ok(BenchmarkResult {
            name: self.name().to_string(),
            execution_time: elapsed,
            metric_value: throughput,
            metric_unit: "MB/s".to_string(),
            score,
            description: format!(
                "Allocated and wrote 64MB in 4KB chunks at {:.2} MB/s",
                throughput
            ),
        })
    }
}
