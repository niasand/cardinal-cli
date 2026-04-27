use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "cardinal-cli", version, about = "Fast file search powered by Cardinal")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[command(flatten)]
    pub verbosity: clap_verbosity_flag::Verbosity,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Search for files matching a query
    Search {
        /// The search query (Everything-compatible syntax)
        query: String,

        /// Output format: json or text
        #[arg(long, default_value = "json", value_name = "FORMAT")]
        format: OutputFormat,

        /// Maximum number of results to return
        #[arg(long, default_value = "50")]
        limit: usize,

        /// Enable case-sensitive search
        #[arg(long, default_value = "false")]
        case_sensitive: bool,

        /// Root path for the search index
        #[arg(long, default_value = "/")]
        path: PathBuf,

        /// Force a full filesystem rescan before searching
        #[arg(long, default_value = "false")]
        refresh: bool,
    },

    /// Build or refresh the search index
    Index {
        /// Root path to scan
        #[arg(long, default_value = "/")]
        path: PathBuf,

        /// Force a full rescan even if cache exists
        #[arg(long, default_value = "false")]
        refresh: bool,
    },
}

#[derive(Clone, clap::ValueEnum)]
pub enum OutputFormat {
    Json,
    Text,
}

#[derive(Clone, Copy, clap::ValueEnum)]
pub enum ExitCode {
    Success = 0,
    NoResults = 1,
    QueryError = 2,
    CacheError = 3,
}
