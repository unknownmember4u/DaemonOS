//! Directory file recursive search latency benchmark.

use crate::benchmark::{BenchmarkResult, BenchmarkTest};
use anyhow::Result;
use std::fs;
use std::path::Path;
use std::time::Instant;

/// File search benchmark struct.
pub struct FileSearchBenchmark;

impl FileSearchBenchmark {
    fn scan_dir(&self, path: &Path, count: &mut usize, limit: usize) {
        if *count >= limit {
            return;
        }
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                if *count >= limit {
                    break;
                }
                let p = entry.path();
                *count += 1;
                if p.is_dir() {
                    self.scan_dir(&p, count, limit);
                }
            }
        }
    }
}

impl BenchmarkTest for FileSearchBenchmark {
    fn name(&self) -> &str {
        "File Search Latency"
    }

    fn run(&self) -> Result<BenchmarkResult> {
        let start = Instant::now();
        let mut count = 0;

        // Perform directory search starting from the repository root
        let root_path = Path::new(".");
        self.scan_dir(root_path, &mut count, 500);

        let elapsed = start.elapsed();
        let elapsed_ms = elapsed.as_secs_f64() * 1000.0;

        // Score scaling: lower elapsed time is better.
        let score = (600.0 / (elapsed_ms + 4.0)).clamp(0.0, 100.0) as u32;

        Ok(BenchmarkResult {
            name: self.name().to_string(),
            execution_time: elapsed,
            metric_value: elapsed_ms,
            metric_unit: "ms".to_string(),
            score,
            description: format!(
                "Recursively scanned {} files in {:.2} ms",
                count, elapsed_ms
            ),
        })
    }
}
