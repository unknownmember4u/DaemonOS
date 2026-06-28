//! Audit utility for typical developer command line binaries.

use which::which;

/// Represents resolution status of a targeted command path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolStatus {
    /// Command resolved successfully to a path
    Found(String),
    /// Command not present in any system PATH directory
    NotFound,
}

/// Audit if a single command binary exists in system environment paths.
pub fn check_tool(name: &str) -> ToolStatus {
    match which(name) {
        Ok(path) => ToolStatus::Found(path.to_string_lossy().to_string()),
        Err(_) => ToolStatus::NotFound,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_tool_found() {
        // 'cargo' must exist in our test environment
        let status = check_tool("cargo");
        assert!(matches!(status, ToolStatus::Found(_)));
    }

    #[test]
    fn test_check_tool_not_found() {
        let status = check_tool("nonexistent_binary_for_test_12345");
        assert_eq!(status, ToolStatus::NotFound);
    }
}
