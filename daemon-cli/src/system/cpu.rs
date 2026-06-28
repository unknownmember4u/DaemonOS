//! CPU hardware details retriever.

use sysinfo::System;

/// Structured CPU details.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CpuInfo {
    /// Processor product identification brand name
    pub brand: String,
    /// Physical processor core count
    pub physical_cores: usize,
}

impl CpuInfo {
    /// Fetch details from the active runtime hardware.
    pub fn fetch() -> Self {
        let mut sys = System::new();
        sys.refresh_cpu_all();
        let brand = sys
            .cpus()
            .first()
            .map(|c| c.brand().trim().to_string())
            .unwrap_or_else(|| "Unknown CPU".to_string());
        let physical_cores = sys.physical_core_count().unwrap_or(0);
        Self {
            brand,
            physical_cores,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_info_fetch() {
        let info = CpuInfo::fetch();
        assert!(!info.brand.is_empty());
    }
}
