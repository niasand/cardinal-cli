mod cli;
mod output;
mod search;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands, OutputFormat};
use search::SearchRunner;
use std::process::ExitCode;
use tracing_subscriber::EnvFilter;

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(err) => {
            eprintln!("Error: {err:#}");
            ExitCode::from(3)
        }
    }
}

fn run() -> Result<ExitCode> {
    let cli = Cli::parse();
    init_tracing(&cli);

    match cli.command {
        Commands::Search {
            query,
            format,
            limit,
            case_sensitive,
            path,
            refresh,
        } => exec_search(&query, format, limit, case_sensitive, &path, refresh),
        Commands::Index { path, refresh } => exec_index(&path, refresh),
    }
}

fn exec_search(
    query: &str,
    format: OutputFormat,
    limit: usize,
    case_sensitive: bool,
    root: &std::path::Path,
    refresh: bool,
) -> Result<ExitCode> {
    let mut runner = SearchRunner::new(root, refresh)?;
    let output = runner.search(query, limit, case_sensitive)?;

    match format {
        OutputFormat::Json => {
            let json = output::format_json(&output)?;
            println!("{json}");
        }
        OutputFormat::Text => {
            println!("{}", output::format_text(&output));
        }
    }

    runner.save_cache()?;

    if output.results.is_empty() {
        Ok(ExitCode::from(1))
    } else {
        Ok(ExitCode::SUCCESS)
    }
}

fn exec_index(root: &std::path::Path, refresh: bool) -> Result<ExitCode> {
    let cache = search::build_index(root, refresh)?;
    println!("Index built: {cache:?}");
    Ok(ExitCode::SUCCESS)
}

fn init_tracing(cli: &Cli) {
    let builder = tracing_subscriber::fmt();
    if let Ok(filter) = EnvFilter::try_from_default_env() {
        builder.with_env_filter(filter).init();
    } else {
        builder
            .with_max_level(cli.verbosity.tracing_level())
            .init();
    }
}
