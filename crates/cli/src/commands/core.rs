use std::str::FromStr;

use clap::{Args, Subcommand};
use smplx_sdk::program::logging::LogLevel;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Initializes Simplex project
    Init {
        #[command(flatten)]
        additional_flags: InitFlags,
    },
    /// Prints current Simplex config in use
    Config,
    /// Spins up local Electrs + Elements regtest
    Regtest,
    /// Runs Simplex tests
    Test {
        /// Name or a substring of the tests to run
        #[arg()]
        name: Option<String>,

        #[command(flatten)]
        additional_flags: TestFlags,
    },
    /// Generates the simplicity contracts artifacts
    Build,
    /// Clean Simplex artifacts in the current directory
    Clean,
}

#[derive(Debug, Args, Copy, Clone)]
pub struct InitFlags {
    /// Generate a draft Rust library instead of just `Simplex.toml`
    #[arg(long)]
    pub lib: bool,
}

#[derive(Debug, Args, Clone)]
pub struct TestFlags {
    /// Show output from successful tests
    #[arg(long)]
    pub nocapture: bool,
    /// Show grouped output after the test completion
    #[arg(long = "show-output")]
    pub show_output: bool,
    /// Run ignored tests
    #[arg(long)]
    pub ignored: bool,
    /// Enable trace log level
    #[arg(short = 'v', long, conflicts_with = "log_level")]
    pub verbose: bool,
    /// Set log level explicitly (none | debug | warning | trace)
    #[arg(long = "log-level", value_name = "LEVEL", conflicts_with = "verbose", value_parser = parse_log_level_arg)]
    pub log_level: Option<String>,
}

fn parse_log_level_arg(s: &str) -> Result<String, String> {
    LogLevel::from_str(s).map_err(|e| e.to_string())?;
    Ok(s.to_string())
}
