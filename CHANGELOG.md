# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-06-28

### Added
- Created multi-crate Cargo workspace structure with resolver version 2.
- Registered core modules: `daemon-cli`, `daemon-core`, `daemon-config`, `daemon-ipc`, `daemon-desktop`, `daemon-panel`, `daemon-launcher`, `daemon-settings`, `daemon-notify`, `daemon-lock`, `daemon-installer`.
- Configured workspace-wide lints to forbid `unsafe_code` and deny `unwrap()`.
- Implemented CI workflows checking formatting, clippy rules, and compile status.
- Added repository health starter document files: README, ROADMAP, CHANGELOG, CONTRIBUTING, SECURITY, and CODE_OF_CONDUCT.
