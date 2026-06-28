//! OS and kernel information retriever.

use sysinfo::System;

/// Structured operating system release and architecture details.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OsInfo {
    /// OS release identifier (e.g. Linux/macOS/Windows)
    pub name: String,
    /// OS running kernel build version
    pub kernel_version: String,
    /// Target binary architecture
    pub architecture: String,
}

impl OsInfo {
    /// Fetch details from the active runtime system.
    pub fn fetch() -> Self {
        Self {
            name: System::name().unwrap_or_else(|| "Unknown".to_string()),
            kernel_version: System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
            architecture: System::cpu_arch().unwrap_or_else(|| "Unknown".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_os_info_fetch() {
        let info = OsInfo::fetch();
        assert!(!info.name.is_empty());
        assert!(!info.kernel_version.is_empty());
        assert!(!info.architecture.is_empty());
    }
}
