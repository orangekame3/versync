use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

use versync::commands;
use versync::config::Config;
use versync::error::exit_code;

#[derive(Parser)]
#[command(name = "versync")]
#[command(about = "Synchronizes version numbers and git tags from a single source of truth")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to the configuration file
    #[arg(long, default_value = "version.toml", global = true)]
    config: PathBuf,

    /// Suppress output
    #[arg(long, global = true)]
    quiet: bool,

    /// Enable verbose output
    #[arg(long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Check if all version numbers match the source of truth
    Check,
    /// Apply the version from source of truth to all target files
    Apply,
    /// Create a git tag based on the current version
    Tag,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    // Load configuration
    let config = match Config::load(&cli.config) {
        Ok(config) => config,
        Err(e) => {
            if !cli.quiet {
                eprintln!("Error: {}", e);
            }
            return ExitCode::from(exit_code::ERROR as u8);
        }
    };

    if cli.verbose && !cli.quiet {
        eprintln!("Using config: {}", cli.config.display());
        eprintln!("Version: {}", config.version);
        eprintln!("Targets: {}", config.targets.len());
    }

    // Execute command
    match cli.command {
        Commands::Check => match commands::check(&config, cli.quiet) {
            Ok(true) => ExitCode::from(exit_code::SUCCESS as u8),
            Ok(false) => ExitCode::from(exit_code::MISMATCH as u8),
            Err(e) => {
                if !cli.quiet {
                    eprintln!("Error: {}", e);
                }
                ExitCode::from(exit_code::ERROR as u8)
            }
        },
        Commands::Apply => match commands::apply(&config, cli.quiet) {
            Ok(()) => ExitCode::from(exit_code::SUCCESS as u8),
            Err(e) => {
                if !cli.quiet {
                    eprintln!("Error: {}", e);
                }
                ExitCode::from(exit_code::ERROR as u8)
            }
        },
        Commands::Tag => match commands::tag(&config, cli.quiet) {
            Ok(()) => ExitCode::from(exit_code::SUCCESS as u8),
            Err(e) => {
                if !cli.quiet {
                    eprintln!("Error: {}", e);
                }
                ExitCode::from(exit_code::ERROR as u8)
            }
        },
    }
}
