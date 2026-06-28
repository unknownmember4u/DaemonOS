//! System diagnostic checking suite definitions and execution logic.

use crate::system::{
    memory::MemoryInfo,
    os::OsInfo,
    tools::{ToolStatus, check_tool},
};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use sysinfo::Disks;

/// Categories of diagnostics results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum CheckGroup {
    /// System details (OS, CPU, RAM)
    System,
    /// Storage drive capacities
    Storage,
    /// Internet connectivity
    Network,
    /// Compiler, runtime, and container tools
    Development,
}

impl CheckGroup {
    /// String label for the group.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::System => "System",
            Self::Storage => "Storage",
            Self::Network => "Network",
            Self::Development => "Development",
        }
    }
}

/// The state of a diagnostic evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckStatus {
    /// Requirement fully satisfied
    Installed,
    /// Requirement missing or failed
    Missing,
    /// Requirement partially satisfied or sub-optimal
    Warning,
}

/// Diagnostics outcome wrapper.
#[derive(Debug, Clone)]
pub struct CheckResult {
    /// Specific name of the check
    pub name: String,
    /// Group category
    pub group: CheckGroup,
    /// Status resolved
    pub status: CheckStatus,
    /// Message string
    pub message: String,
    /// Importance weight (used to calculate final score)
    pub weight: u32,
    /// Guidance for resolving missing or warning checks
    pub recommendation: Option<String>,
}

/// Extensible trait for executing individual diagnostics requirements.
pub trait DoctorCheck {
    /// Name of the diagnostic test.
    fn name(&self) -> &str;
    /// Category group.
    fn group(&self) -> CheckGroup;
    /// Execute the check logic.
    fn run(&self) -> CheckResult;
}

// ==============================================================================
// Implementation of Specific Checks
// ==============================================================================

/// Validates Operating System name and kernel metadata.
pub struct OsKernelCheck;

impl DoctorCheck for OsKernelCheck {
    fn name(&self) -> &str {
        "OS & Kernel"
    }

    fn group(&self) -> CheckGroup {
        CheckGroup::System
    }

    fn run(&self) -> CheckResult {
        let os = OsInfo::fetch();
        let status = if os.name == "Unknown" || os.kernel_version == "Unknown" {
            CheckStatus::Warning
        } else {
            CheckStatus::Installed
        };

        CheckResult {
            name: self.name().to_string(),
            group: self.group(),
            status,
            message: format!("{} (kernel: {})", os.name, os.kernel_version),
            weight: 10,
            recommendation: if status == CheckStatus::Warning {
                Some("System reports an unrecognized operating system kernel variant.".to_string())
            } else {
                None
            },
        }
    }
}

/// Validates CPU target architecture.
pub struct CpuArchCheck;

impl DoctorCheck for CpuArchCheck {
    fn name(&self) -> &str {
        "CPU Architecture"
    }

    fn group(&self) -> CheckGroup {
        CheckGroup::System
    }

    fn run(&self) -> CheckResult {
        let os = OsInfo::fetch();
        let is_64bit = os.architecture.contains("64")
            || os.architecture.contains("arm64")
            || os.architecture.contains("aarch64");

        let (status, message, recommendation) = if is_64bit {
            (
                CheckStatus::Installed,
                format!("64-bit architecture ({})", os.architecture),
                None,
            )
        } else {
            (
                CheckStatus::Warning,
                format!("32-bit architecture ({})", os.architecture),
                Some("DaemonOS recommends running on a 64-bit CPU architecture to support modern virtualization/sandboxing.".to_string())
            )
        };

        CheckResult {
            name: self.name().to_string(),
            group: self.group(),
            status,
            message,
            weight: 10,
            recommendation,
        }
    }
}

/// Validates physical RAM availability.
pub struct AvailableRamCheck;

impl DoctorCheck for AvailableRamCheck {
    fn name(&self) -> &str {
        "Available RAM"
    }

    fn group(&self) -> CheckGroup {
        CheckGroup::System
    }

    fn run(&self) -> CheckResult {
        let mem = MemoryInfo::fetch();
        let total_gb = (mem.total_memory as f64) / (1024.0 * 1024.0 * 1024.0);

        let (status, message, recommendation) = if total_gb >= 3.5 {
            (
                CheckStatus::Installed,
                format!("{:.1} GB RAM detected", total_gb),
                None,
            )
        } else if total_gb >= 1.9 {
            (
                CheckStatus::Warning,
                format!("{:.1} GB RAM detected (minimal)", total_gb),
                Some("DaemonOS recommends at least 4 GB RAM for optimal performance.".to_string()),
            )
        } else {
            (
                CheckStatus::Missing,
                format!("{:.1} GB RAM detected (critically low)", total_gb),
                Some(
                    "A minimum of 2 GB of RAM is required to run DaemonOS services safely."
                        .to_string(),
                ),
            )
        };

        CheckResult {
            name: self.name().to_string(),
            group: self.group(),
            status,
            message,
            weight: 15,
            recommendation,
        }
    }
}

/// Validates free storage capacity in the root directory partition.
pub struct DiskFreeSpaceCheck;

impl DoctorCheck for DiskFreeSpaceCheck {
    fn name(&self) -> &str {
        "Disk Free Space"
    }

    fn group(&self) -> CheckGroup {
        CheckGroup::Storage
    }

    fn run(&self) -> CheckResult {
        let disks = Disks::new_with_refreshed_list();
        let root_path = std::path::Path::new("/");
        let available_space = disks
            .list()
            .iter()
            .find(|d| d.mount_point() == root_path)
            .map(|d| d.available_space())
            .unwrap_or_else(|| {
                disks
                    .list()
                    .first()
                    .map(|d| d.available_space())
                    .unwrap_or(0)
            });

        let available_gb = (available_space as f64) / (1024.0 * 1024.0 * 1024.0);

        let (status, message, recommendation) = if available_gb >= 20.0 {
            (
                CheckStatus::Installed,
                format!("{:.1} GB free on system drive", available_gb),
                None,
            )
        } else if available_gb >= 10.0 {
            (
                CheckStatus::Warning,
                format!("{:.1} GB free (low storage)", available_gb),
                Some("Clean up unused files to free space on your system partition (20 GB recommended).".to_string()),
            )
        } else {
            (
                CheckStatus::Warning,
                format!("{:.1} GB free (critically low storage)", available_gb),
                Some("Free up space on your disk to prevent installation failures.".to_string()),
            )
        };

        CheckResult {
            name: self.name().to_string(),
            group: self.group(),
            status,
            message,
            weight: 15,
            recommendation,
        }
    }
}

/// Validates connectivity to public network endpoints.
pub struct InternetCheck;

impl DoctorCheck for InternetCheck {
    fn name(&self) -> &str {
        "Internet Connection"
    }

    fn group(&self) -> CheckGroup {
        CheckGroup::Network
    }

    fn run(&self) -> CheckResult {
        let test_addr = "8.8.8.8:53";
        let is_connected = if let Ok(addr) = test_addr.parse::<SocketAddr>() {
            TcpStream::connect_timeout(&addr, Duration::from_secs(2)).is_ok()
        } else {
            false
        };

        let (status, message, recommendation) = if is_connected {
            (
                CheckStatus::Installed,
                "Connected to network services".to_string(),
                None,
            )
        } else {
            (
                CheckStatus::Warning,
                "Offline or connection timed out".to_string(),
                Some("Verify network hardware connections, Wi-Fi passwords, or system gateway routes.".to_string())
            )
        };

        CheckResult {
            name: self.name().to_string(),
            group: self.group(),
            status,
            message,
            weight: 15,
            recommendation,
        }
    }
}

/// Validates tool path availability.
pub struct ToolCheck {
    tool_name: &'static str,
    weight: u32,
    recommendation: &'static str,
}

impl DoctorCheck for ToolCheck {
    fn name(&self) -> &str {
        self.tool_name
    }

    fn group(&self) -> CheckGroup {
        CheckGroup::Development
    }

    fn run(&self) -> CheckResult {
        match check_tool(self.tool_name) {
            ToolStatus::Found(path) => CheckResult {
                name: self.tool_name.to_string(),
                group: self.group(),
                status: CheckStatus::Installed,
                message: format!("Installed at {}", path),
                weight: self.weight,
                recommendation: None,
            },
            ToolStatus::NotFound => CheckResult {
                name: self.tool_name.to_string(),
                group: self.group(),
                status: CheckStatus::Missing,
                message: "Not found in PATH".to_string(),
                weight: self.weight,
                recommendation: Some(self.recommendation.to_string()),
            },
        }
    }
}

// ==============================================================================
// Orchestration & Scoring Logic
// ==============================================================================

/// Executes all system checks and groups the results. Calculates health score.
pub fn run_diagnostics() -> (Vec<(CheckGroup, Vec<CheckResult>)>, u32) {
    let checks: Vec<Box<dyn DoctorCheck>> = vec![
        // 1. System checks
        Box::new(OsKernelCheck),
        Box::new(CpuArchCheck),
        Box::new(AvailableRamCheck),
        // 2. Storage checks
        Box::new(DiskFreeSpaceCheck),
        // 3. Network checks
        Box::new(InternetCheck),
        // 4. Development tools checks
        Box::new(ToolCheck {
            tool_name: "git",
            weight: 5,
            recommendation: "Install git using your package manager (e.g. `sudo apt install git` or `sudo pacman -S git`).",
        }),
        Box::new(ToolCheck {
            tool_name: "rustc",
            weight: 5,
            recommendation: "Install Rust compiler toolchain via rustup: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`",
        }),
        Box::new(ToolCheck {
            tool_name: "cargo",
            weight: 5,
            recommendation: "Install Cargo manager via rustup: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`",
        }),
        Box::new(ToolCheck {
            tool_name: "gcc",
            weight: 3,
            recommendation: "Install gcc compiler packages (e.g. `sudo apt install build-essential` or `sudo pacman -S base-devel`).",
        }),
        Box::new(ToolCheck {
            tool_name: "clang",
            weight: 3,
            recommendation: "Install clang compiler utility (e.g. `sudo apt install clang`).",
        }),
        Box::new(ToolCheck {
            tool_name: "cmake",
            weight: 3,
            recommendation: "Install cmake tool for builder setups (e.g. `sudo apt install cmake`).",
        }),
        Box::new(ToolCheck {
            tool_name: "ninja",
            weight: 3,
            recommendation: "Install ninja build package (e.g. `sudo apt install ninja-build`).",
        }),
        Box::new(ToolCheck {
            tool_name: "docker",
            weight: 3,
            recommendation: "Install docker runtime toolchain package or manage container permissions.",
        }),
        Box::new(ToolCheck {
            tool_name: "podman",
            weight: 3,
            recommendation: "Install podman container engine utility.",
        }),
        Box::new(ToolCheck {
            tool_name: "python3",
            weight: 2,
            recommendation: "Install python3 execution package (e.g. `sudo apt install python3`).",
        }),
        Box::new(ToolCheck {
            tool_name: "node",
            weight: 2,
            recommendation: "Install Node.js JavaScript environment runtimes.",
        }),
        Box::new(ToolCheck {
            tool_name: "npm",
            weight: 2,
            recommendation: "Install NPM package manager package alongside node.",
        }),
        Box::new(ToolCheck {
            tool_name: "java",
            weight: 2,
            recommendation: "Install OpenJDK JRE/JDK package runtime environment.",
        }),
    ];
    let mut results = Vec::new();
    let mut total_weight = 0;
    let mut earned_score: f64 = 0.0;

    for check in checks {
        let res = check.run();
        total_weight += res.weight;

        let multiplier = match res.status {
            CheckStatus::Installed => 1.0,
            CheckStatus::Warning => 0.5,
            CheckStatus::Missing => 0.0,
        };
        earned_score += multiplier * (res.weight as f64);

        results.push(res);
    }

    // Calculate final score Normalized 0 - 100
    let final_score = if total_weight > 0 {
        ((earned_score / (total_weight as f64)) * 100.0).round() as u32
    } else {
        100
    };

    // Group results by CheckGroup
    let groups = [
        CheckGroup::System,
        CheckGroup::Storage,
        CheckGroup::Network,
        CheckGroup::Development,
    ];

    let mut grouped_results = Vec::new();
    for group in groups {
        let group_checks: Vec<CheckResult> = results
            .iter()
            .filter(|r| r.group == group)
            .cloned()
            .collect();
        grouped_results.push((group, group_checks));
    }

    (grouped_results, final_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_diagnostics() {
        let (groups, score) = run_diagnostics();
        assert!(score <= 100);
        assert_eq!(groups.len(), 4);

        // Ensure groups are sorted as expected
        assert_eq!(groups[0].0, CheckGroup::System);
        assert_eq!(groups[1].0, CheckGroup::Storage);
        assert_eq!(groups[2].0, CheckGroup::Network);
        assert_eq!(groups[3].0, CheckGroup::Development);
    }

    #[test]
    fn test_os_kernel_check() {
        let check = OsKernelCheck;
        let res = check.run();
        assert_eq!(res.group, CheckGroup::System);
        assert!(!res.message.is_empty());
    }

    #[test]
    fn test_cpu_arch_check() {
        let check = CpuArchCheck;
        let res = check.run();
        assert_eq!(res.group, CheckGroup::System);
        assert!(!res.message.is_empty());
    }

    #[test]
    fn test_ram_check() {
        let check = AvailableRamCheck;
        let res = check.run();
        assert_eq!(res.group, CheckGroup::System);
        assert!(res.weight > 0);
    }

    #[test]
    fn test_disk_check() {
        let check = DiskFreeSpaceCheck;
        let res = check.run();
        assert_eq!(res.group, CheckGroup::Storage);
        assert!(res.weight > 0);
    }
}
