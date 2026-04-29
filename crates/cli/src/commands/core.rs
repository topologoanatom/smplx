use clap::{Args, Subcommand, ValueEnum};

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Creates a new Simplex project in a new directory
    New {
        /// Name of the new project
        name: String,
    },
    /// Scaffolds an example Simplex project into a new directory
    Example {
        /// Name of the example to scaffold
        example: ExampleName,
    },
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

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum ExampleName {
    /// Basic p2pk example with contract scaffolding
    Basic,
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
    /// Log simplicity pruning stack trace
    #[arg(short = 'v', long)]
    pub verbose: bool,
}
