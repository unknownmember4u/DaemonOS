# Contributing to DaemonOS

Thank you for your interest in contributing to DaemonOS! We welcome contributions of all types.

## Development Workflow

1.  **Fork and Branch**: Fork the repository and create a branch for your work:
    ```bash
    git checkout -b feature/my-new-feature
    ```
2.  **Make Changes**: Ensure your code meets our coding standards.
3.  **Validate**:
    *   Verify code formatting: `cargo fmt --check`
    *   Verify lints: `cargo clippy --all-targets`
    *   Ensure all tests pass: `cargo test`
4.  **Commit**: Use descriptive commit messages.
5.  **Submit PR**: Open a pull request against the `main` branch.

## Coding Standards

*   **No Unsafe**: Do not introduce `unsafe` code. The project has `#![forbid(unsafe_code)]` enabled.
*   **Panic-free**: Do not use `unwrap()` or `expect()`. Handle all potential errors gracefully using the `Result` type.
*   **Documentation**: Document public functions, structs, and modules using standard Rust doc comments (`///` and `//!`).
