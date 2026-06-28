//! Command-line interface parser definition.

use clap::{Parser, Subcommand};

/// The main command line interface parser.
#[derive(Parser, Debug)]
#[command(
    name = "daemon",
    author = "DaemonOS Authors",
    version = "0.1.0",
    about = "DaemonOS CLI Foundation"
)]
pub struct Cli {
    /// Enable detailed debug logs
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Subcommands to execute
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Supported subcommands for system diagnostics.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Print version details
    Version,
    /// Inspect CPU, RAM, and Operating System details
    Info,
    /// Audit presence of typical developer tools in PATH
    Doctor,
    /// Run diagnostic hardware and compiler benchmark tests
    Benchmark,
}
