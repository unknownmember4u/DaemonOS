//! Disk write and read throughput benchmark.

use crate::benchmark::{BenchmarkResult, BenchmarkTest};
use anyhow::{Context, Result};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// Disk speed benchmark struct.
pub struct DiskBenchmark;

static TEMP_FILE_COUNTER: AtomicU64 = AtomicU64::new(0);

impl DiskBenchmark {
    fn run_io(temp_path: &Path) -> Result<(Duration, Duration, f64)> {
        const FILE_SIZE: usize = 10 * 1024 * 1024; // 10 MB
        const BLOCK_SIZE: usize = 64 * 1024; // 64 KB
        let data = vec![0x55u8; BLOCK_SIZE];

        // 1. Write phase
        let write_start = Instant::now();
        {
            let mut file =
                File::create(temp_path).context("Failed to create temporary benchmark file")?;
            let num_blocks = FILE_SIZE / BLOCK_SIZE;
            for _ in 0..num_blocks {
                file.write_all(&data)
                    .context("Failed to write data to benchmark file")?;
            }
            file.sync_all()
                .context("Failed to sync benchmark file changes")?;
        }
        let write_elapsed = write_start.elapsed();

        // 2. Read phase
        let read_start = Instant::now();
        {
            let mut file =
                File::open(temp_path).context("Failed to open temporary benchmark file")?;
            let mut buffer = vec![0u8; BLOCK_SIZE];
            loop {
                let bytes_read = file
                    .read(&mut buffer)
                    .context("Failed to read block from benchmark file")?;
                if bytes_read == 0 {
                    break;
                }
            }
        }
        let read_elapsed = read_start.elapsed();

        let total_elapsed = write_elapsed + read_elapsed;
        let total_secs = total_elapsed.as_secs_f64();
        let total_mb = (FILE_SIZE as f64 * 2.0) / (1024.0 * 1024.0); // Total MB written + read

        let throughput = if total_secs > 0.0 {
            total_mb / total_secs
        } else {
            0.0
        };

        Ok((write_elapsed, read_elapsed, throughput))
    }
}

impl BenchmarkTest for DiskBenchmark {
    fn name(&self) -> &str {
        "Disk I/O Write & Read Speed"
    }

    fn run(&self) -> Result<BenchmarkResult> {
        let temp_dir = std::env::temp_dir();
        let temp_file_id = TEMP_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
        let temp_path = temp_dir.join(format!(
            "daemon_benchmark_io_file_{}_{}.tmp",
            std::process::id(),
            temp_file_id
        ));

        let result = Self::run_io(&temp_path);
        let _ = fs::remove_file(&temp_path);

        let (write_elapsed, read_elapsed, throughput) = result?;

        // Score scaling: NVMe speed (1000+ MB/s -> 100), SATA SSD (300 MB/s -> 80), HDD (50 MB/s -> 30)
        let score = (throughput / 10.0).clamp(0.0, 100.0) as u32;

        Ok(BenchmarkResult {
            name: self.name().to_string(),
            execution_time: write_elapsed + read_elapsed,
            metric_value: throughput,
            metric_unit: "MB/s".to_string(),
            score,
            description: format!(
                "Throughput: {:.2} MB/s (Write: {:.1}ms, Read: {:.1}ms)",
                throughput,
                write_elapsed.as_secs_f64() * 1000.0,
                read_elapsed.as_secs_f64() * 1000.0
            ),
        })
    }
}
