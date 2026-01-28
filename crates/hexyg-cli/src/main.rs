use anyhow::Result;
use clap::Parser;
use hexyg_core::{config::*, Config};
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "hexyg")]
#[command(about = "Bidirectional converter between binary data and hex text format")]
#[command(version)]
#[command(disable_help_flag = true)]
struct Cli {
    /// Print help
    #[arg(long = "help", short = '?', short_alias = 'h', action = clap::ArgAction::Help)]
    help: Option<bool>,

    /// Convert binary data to hex text format
    #[arg(long = "bin-to-hex", group = "mode")]
    bin_to_hex: bool,

    /// Convert hex text to binary data
    #[arg(long = "hex-to-bin", group = "mode")]
    hex_to_bin: bool,

    // Common options
    /// Input file (if not specified, reads from stdin)
    #[arg(long, short = 'i')]
    input: Option<PathBuf>,

    /// Output file (if not specified, writes to stdout)
    #[arg(long, short = 'o')]
    output: Option<PathBuf>,

    /// Start address (decimal or 0x-prefixed hex, negative for end-based)
    #[arg(long)]
    from: Option<String>,

    /// End address (exclusive, decimal or 0x-prefixed hex, negative for end-based)
    #[arg(long)]
    to: Option<String>,

    // --bin-to-hex options
    /// Set option in hex file: --option name=value
    #[arg(long = "option", value_name = "NAME=VALUE")]
    options: Vec<String>,

    /// Start address in hex file (decimal or 0x-prefixed hex)
    #[arg(long = "start-address")]
    start_address: Option<String>,

    /// Address size: u16, u24, u32, u64, u128, stretch
    #[arg(long = "address-size")]
    address_size: Option<String>,

    /// Alignment step for address width
    #[arg(long = "address-size-step")]
    address_size_step: Option<usize>,

    /// Line length in bytes (default: 16)
    #[arg(long = "line-length")]
    line_length: Option<usize>,

    /// Split line into blocks of N bytes (default: 0, no blocking)
    #[arg(long = "block-length")]
    block_length: Option<usize>,

    /// Byte separator (empty or whitespace)
    #[arg(long = "byte-separator", default_value = " ")]
    byte_separator: String,

    /// Block separator (empty or whitespace)
    #[arg(long = "block-separator", default_value = " ")]
    block_separator: String,

    /// Repeat address: never, once, every_line (default: every_line)
    #[arg(long = "repeat-address", default_value = "every_line")]
    repeat_address: String,

    // --hex-to-bin options
    /// Check consistency: none, text, values, all, text,values
    #[arg(long = "check", default_value = "none")]
    check: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Check that a mode is specified
    if !cli.bin_to_hex && !cli.hex_to_bin {
        eprintln!("Error: Must specify either --bin-to-hex or --hex-to-bin");
        eprintln!("Use --help for usage information");
        std::process::exit(1);
    }

    // Build configuration
    let mut config = Config::default();

    // Parse address_size
    if let Some(addr_size) = cli.address_size {
        config.address_size = match addr_size.to_lowercase().as_str() {
            "u16" => AddressSize::U16,
            "u24" => AddressSize::U24,
            "u32" => AddressSize::U32,
            "u40" => AddressSize::U40,
            "u48" => AddressSize::U48,
            "u64" => AddressSize::U64,
            "stretch" => AddressSize::Stretch,
            _ => {
                eprintln!("Invalid address-size: {}. Use u16, u24, u32, u40, u48, u64, or stretch", addr_size);
                std::process::exit(1);
            }
        };
    }

    if let Some(line_length) = cli.line_length {
        config.line_length = line_length;
    }

    if let Some(block_length) = cli.block_length {
        config.block_length = block_length;
    }

    // Parse repeat_address
    config.repeat_address = match cli.repeat_address.as_str() {
        "every_line" => true,
        "never" | "once" => false,
        _ => {
            eprintln!("Invalid repeat-address: {}. Use never, once, or every_line", cli.repeat_address);
            std::process::exit(1);
        }
    };

    // Note: byte_separator, block_separator, from, to, start_address, check are parsed but not yet used
    // These will be implemented in future iterations

    // Execute command
    if cli.bin_to_hex {
        let input: Box<dyn io::Read> = if let Some(path) = cli.input {
            Box::new(BufReader::new(File::open(path)?))
        } else {
            Box::new(io::stdin().lock())
        };

        let output: Box<dyn io::Write> = if let Some(path) = cli.output {
            Box::new(BufWriter::new(File::create(path)?))
        } else {
            Box::new(io::stdout().lock())
        };

        hexyg_core::convert::bin_to_hex(input, output, &config)?;
    } else if cli.hex_to_bin {
        let input: Box<dyn io::Read> = if let Some(path) = cli.input {
            Box::new(BufReader::new(File::open(path)?))
        } else {
            Box::new(io::stdin().lock())
        };

        let output: Box<dyn io::Write> = if let Some(path) = cli.output {
            Box::new(BufWriter::new(File::create(path)?))
        } else {
            Box::new(io::stdout().lock())
        };

        hexyg_core::convert::hex_to_bin(input, output)?;
    }

    Ok(())
}
