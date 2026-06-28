//! System metrics and hardware information querying.

use sysinfo::System;

/// SystemInfo struct wrapping sysinfo controls.
pub struct SystemInfo {
    sys: System,
}

impl SystemInfo {
    /// Create a new SystemInfo instance and refresh system statistics.
    pub fn new() -> Self {
        let mut sys = System::new();
        sys.refresh_all();
        Self { sys }
    }

    /// Refresh hardware counters and system statistics.
    pub fn refresh(&mut self) {
        self.sys.refresh_all();
    }

    /// Total system RAM in bytes.
    pub fn total_memory(&self) -> u64 {
        self.sys.total_memory()
    }

    /// Free system RAM in bytes.
    pub fn free_memory(&self) -> u64 {
        self.sys.free_memory()
    }

    /// Used system RAM in bytes.
    pub fn used_memory(&self) -> u64 {
        self.sys.used_memory()
    }

    /// Retrieve OS display name.
    pub fn os_name(&self) -> Option<String> {
        System::name()
    }

    /// Retrieve Linux kernel build version.
    pub fn kernel_version(&self) -> Option<String> {
        System::kernel_version()
    }

    /// Count of virtual CPU threads.
    pub fn cpu_count(&self) -> usize {
        self.sys.cpus().len()
    }
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self::new()
    }
}
