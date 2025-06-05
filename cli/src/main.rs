use anyhow::Result;
use clap::{Parser, Subcommand};
use hashassin_core::{dump, hashing, password, rainbow, Algorithm};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "hashassin",
    about = "CLI for password generation, hash computation, and rainbow table operations",
    version,
    help_template = "\
{name} {version}
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

    /// Generate rainbow table
    GenRainbowTable {
        #[arg(long, default_value = "5")]
        num_links: usize,

        #[arg(long, default_value = "1")]
        threads: usize,

        #[arg(long)]
        out_file: PathBuf,

        #[arg(long, value_parser = clap::value_parser!(Algorithm))]
        algorithm: Algorithm,

        #[arg(long)]
        in_file: PathBuf,

        #[arg(long, default_value_t = 95)]
        charset_size: u128,

        #[arg(long, default_value_t = 32)]
        unicode_offset: u128,
    },

    /// Dump rainbow table contents
    DumpRainbowTable {
        #[arg(long)]
        in_file: PathBuf,
    },

    /// Crack hashes using rainbow table
    Crack {
        #[arg(long)]
        in_file: PathBuf,

        #[arg(long)]
        out_file: Option<PathBuf>,

        #[arg(long, default_value = "1")]
        threads: usize,

        #[arg(long)]
        hashes: PathBuf,
    },
}

fn main() -> Result<()> {
    // Initialize structured logging
    tracing_subscriber::fmt::init();

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

        Commands::GenRainbowTable {
            num_links,
            threads,
            out_file,
            algorithm,
            in_file,
            charset_size,
            unicode_offset,
        } => rainbow::generate_rainbow_table(
            &in_file,
            &out_file,
            algorithm,
            num_links,
            threads,
            charset_size,
            unicode_offset,
        )
        .map_err(Into::into),

        Commands::DumpRainbowTable { in_file } => {
            rainbow::dump_rainbow_table(&in_file).map_err(Into::into)
        }

        Commands::Crack {
            in_file,
            out_file,
            threads,
            hashes,
        } => rainbow::crack(&in_file, &hashes, out_file.as_deref(), threads).map_err(Into::into),
    }
}
