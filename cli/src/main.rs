use anyhow::Result;
use clap::{Parser, Subcommand};
use hashassin_core::{dump, hashing, password, Algorithm};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "hashassin",
    about = "CLI for password generation and hash computation",
    version,
    help_template = "\
{name} {version}
{author}
{about}

USAGE: {usage}
{all-args}
"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate random passwords
    GenPasswords {
        #[arg(long, default_value = "4")]
        chars: usize,

        #[arg(long)]
        out_file: Option<PathBuf>,

        #[arg(long, default_value = "1")]
        threads: usize,

        #[arg(long, default_value = "1")]
        num: usize,
    },

    /// Generate password hashes
    GenHashes {
        #[arg(long)]
        in_file: PathBuf,

        #[arg(long)]
        out_file: PathBuf,

        #[arg(long, default_value = "1")]
        threads: usize,

        #[arg(long, value_parser = clap::value_parser!(Algorithm))]
        algorithm: Algorithm,
    },

    /// Dump hash file contents
    DumpHashes {
        #[arg(long)]
        in_file: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenPasswords {
            chars,
            out_file,
            threads,
            num,
        } => password::generate_passwords(num, chars, threads, out_file).map_err(Into::into),
        Commands::GenHashes {
            in_file,
            out_file,
            threads,
            algorithm,
        } => hashing::generate_hashes(&in_file, &out_file, algorithm, threads).map_err(Into::into),
        Commands::DumpHashes { in_file } => dump::dump_hashes(&in_file).map_err(Into::into),
    }
}
