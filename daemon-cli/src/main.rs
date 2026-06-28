//! Entrypoint for the DaemonOS command-line utility.

use anyhow::Result;
use daemon_cli::app::run;

fn main() -> Result<()> {
    run()
}
