//! System memory details retriever.

use sysinfo::System;

/// Structured system physical memory details.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryInfo {
    /// Total system physical memory capacity (in bytes)
    pub total_memory: u64,
    /// Available system physical memory (in bytes)
    pub free_memory: u64,
}

impl MemoryInfo {
    /// Fetch details from the active runtime hardware.
    pub fn fetch() -> Self {
        let mut sys = System::new();
        sys.refresh_memory();
        Self {
            total_memory: sys.total_memory(),
            free_memory: sys.free_memory(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_info_fetch() {
        let info = MemoryInfo::fetch();
        assert!(info.total_memory > 0);
    }
}
