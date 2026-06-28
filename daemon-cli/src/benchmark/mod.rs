//! Benchmark suite definitions and runner orchestration.

pub mod cpu;
pub mod disk;
pub mod git;
pub mod process;
pub mod ram;
pub mod rustc;
pub mod search;

use cpu::CpuBenchmark;
use disk::DiskBenchmark;
use git::GitBenchmark;
use process::ProcessSpawnBenchmark;
use ram::RamBenchmark;
use rustc::RustcBenchmark;
use search::FileSearchBenchmark;

use anyhow::Result;
use std::time::Duration;

/// Trait for defining benchmark tests.
pub trait BenchmarkTest {
    /// Friendly name of the test.
    fn name(&self) -> &str;
    /// Execute the performance test.
    fn run(&self) -> Result<BenchmarkResult>;
}

/// The result returned by a benchmark test.
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    /// Test identifier
    pub name: String,
    /// Total duration elapsed
    pub execution_time: Duration,
    /// Calculated throughput or count metric
    pub metric_value: f64,
    /// Metric units (e.g. MB/s, ms, files)
    pub metric_unit: String,
    /// Score resolved from 0 to 100
    pub score: u32,
    /// Human readable output explanation
    pub description: String,
}

/// Run all benchmark suites and calculate overall normalized performance score.
///
/// # Errors
///
/// Returns an error if any benchmark execution encounters fatal failures.
pub fn run_benchmarks() -> Result<(Vec<BenchmarkResult>, u32)> {
    let tests: Vec<Box<dyn BenchmarkTest>> = vec![
        Box::new(CpuBenchmark),
        Box::new(RamBenchmark),
        Box::new(DiskBenchmark),
        Box::new(FileSearchBenchmark),
        Box::new(ProcessSpawnBenchmark),
        Box::new(GitBenchmark),
        Box::new(RustcBenchmark),
    ];

    let mut results = Vec::new();
    let mut total_score = 0;
    let mut active_tests = 0;

    for test in tests {
        let res = test.run()?;

        // If tools like rustc or git are not installed, their score evaluates to 0
        // and we exclude them from the denominator to avoid penalizing hardware performance.
        let is_missing_tool = (res.name == "Rust Compiler Latency"
            || res.name == "Git Command Latency")
            && res.score == 0;

        if !is_missing_tool {
            total_score += res.score;
            active_tests += 1;
        }
        results.push(res);
    }

    let overall_score = total_score.checked_div(active_tests).unwrap_or(100);

    Ok((results, overall_score))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_all_benchmarks() -> Result<()> {
        let (results, score) = run_benchmarks()?;
        assert!(score <= 100);
        assert_eq!(results.len(), 7);
        Ok(())
    }

    #[test]
    fn test_cpu_benchmark() -> Result<()> {
        let test = cpu::CpuBenchmark;
        let r = test.run()?;
        assert_eq!(r.name, "CPU Single-Thread Core");
        assert!(r.score <= 100);
        assert!(r.metric_value > 0.0);
        Ok(())
    }

    #[test]
    fn test_ram_benchmark() -> Result<()> {
        let test = ram::RamBenchmark;
        let r = test.run()?;
        assert_eq!(r.name, "RAM Allocation & Write Speed");
        assert!(r.score <= 100);
        Ok(())
    }

    #[test]
    fn test_disk_benchmark() -> Result<()> {
        let test = disk::DiskBenchmark;
        let r = test.run()?;
        assert_eq!(r.name, "Disk I/O Write & Read Speed");
        assert!(r.score <= 100);
        Ok(())
    }

    #[test]
    fn test_search_benchmark() -> Result<()> {
        let test = search::FileSearchBenchmark;
        let r = test.run()?;
        assert_eq!(r.name, "File Search Latency");
        assert!(r.score <= 100);
        Ok(())
    }

    #[test]
    fn test_process_spawn_benchmark() -> Result<()> {
        let test = process::ProcessSpawnBenchmark;
        let r = test.run()?;
        assert_eq!(r.name, "Process Spawn Latency");
        assert!(r.score <= 100);
        Ok(())
    }
}
