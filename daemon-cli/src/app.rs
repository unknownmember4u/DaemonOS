//! Command match runner and tracing system initialization.

use crate::cli::{Cli, Commands};
use crate::commands;
use anyhow::Result;
use clap::Parser;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

/// Run the command matching logic and handle options.
///
/// # Errors
///
/// Returns an error if any subcommand execution fails.
pub fn run() -> Result<()> {
    let args = Cli::parse();

    // Set standard filter depending on verbose flag
    let filter = if args.verbose {
        EnvFilter::new("daemon_cli=debug,info")
    } else {
        EnvFilter::new("daemon_cli=info")
    };

    tracing_subscriber::registry()
        .with(fmt::layer().compact())
        .with(filter)
        .init();

    match &args.command {
        Some(Commands::Version) => commands::version::handle()?,
        Some(Commands::Info) => commands::info::handle()?,
        Some(Commands::Doctor) => commands::doctor::handle()?,
        Some(Commands::Benchmark) => commands::benchmark::handle()?,
        None => {
            // Default to printing help if no command is passed
            use clap::CommandFactory;
            let mut cmd = Cli::command();
            cmd.print_help()?;
            println!();
        }
    }

    Ok(())
}
