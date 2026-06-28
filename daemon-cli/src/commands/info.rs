//! Info subcommand execution handler.

use crate::system::{cpu::CpuInfo, memory::MemoryInfo, os::OsInfo};
use anyhow::Result;
use tracing::info;

/// Execute system information diagnostics query.
///
/// # Errors
///
/// Returns an error if any system query or stdout write fails.
pub fn handle() -> Result<()> {
    info!("Querying system hardware and OS metadata...");
    let os = OsInfo::fetch();
    let cpu = CpuInfo::fetch();
    let mem = MemoryInfo::fetch();

    println!("--- DaemonOS System Diagnostics ---");
    println!("OS Name:      {}", os.name);
    println!("Kernel:       {}", os.kernel_version);
    println!("Architecture: {}", os.architecture);
    println!("CPU Model:    {}", cpu.brand);
    println!("CPU Cores:    {}", cpu.physical_cores);
    println!("Total RAM:    {} MB", mem.total_memory / (1024 * 1024));
    println!("Free RAM:     {} MB", mem.free_memory / (1024 * 1024));

    Ok(())
}
