//! Conversion functions between binary and hex formats

use crate::{Config, Error, Result};
use std::io::{BufRead, BufReader, Read, Write};

/// Convert binary data to hex text format
///
/// # Arguments
/// * `reader` - Input binary data
/// * `writer` - Output hex text
/// * `config` - Configuration options
pub fn bin_to_hex<R: Read, W: Write>(
    reader: R,
    mut writer: W,
    config: &Config,
) -> Result<()> {
    let mut reader = BufReader::new(reader);
    let mut address: u64 = 0;
    let mut buffer = vec![0u8; config.line_length];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        let line_data = &buffer[0..bytes_read];

        // Write address if needed
        if config.repeat_address || address == 0 {
            write_address(&mut writer, address, config)?;
            write!(writer, ": ")?;
        } else {
            // Indent continuation lines
            write!(writer, "          ")?;
        }

        // Write hex bytes
        write_hex_bytes(&mut writer, line_data, config)?;

        // Write ASCII preview if enabled
        if config.show_preview {
            write!(writer, " ")?;
            write_ascii_preview(&mut writer, line_data)?;
        }

        writeln!(writer)?;

        address += bytes_read as u64;
    }

    Ok(())
}

/// Convert hex text to binary data
///
/// # Arguments
/// * `reader` - Input hex text
/// * `writer` - Output binary data
pub fn hex_to_bin<R: Read, W: Write>(reader: R, mut writer: W) -> Result<()> {
    let reader = BufReader::new(reader);

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        // Skip block comments (simple handling)
        if line.starts_with("/*") || line.ends_with("*/") {
            continue;
        }

        // Skip directives
        if line.starts_with("#") {
            continue;
        }

        // Extract hex data from line
        let hex_data = extract_hex_from_line(line)?;
        if !hex_data.is_empty() {
            let bytes = parse_hex_string(&hex_data)?;
            writer.write_all(&bytes)?;
        }
    }

    Ok(())
}

/// Write address in appropriate format based on config
fn write_address<W: Write>(writer: &mut W, address: u64, config: &Config) -> Result<()> {
    use crate::config::AddressSize;

    let width = match config.address_size {
        AddressSize::U8 => 2,
        AddressSize::U16 => 4,
        AddressSize::U24 => 6,
        AddressSize::U32 => 8,
        AddressSize::U40 => 10,
        AddressSize::U48 => 12,
        AddressSize::U64 => 16,
        AddressSize::Stretch => {
            // Calculate minimum width needed
            if address < 0x100 {
                2
            } else if address < 0x10000 {
                4
            } else if address < 0x1000000 {
                6
            } else if address < 0x100000000 {
                8
            } else {
                16
            }
        }
    };

    write!(writer, "{:0width$X}", address, width = width)?;
    Ok(())
}

/// Write hex bytes with appropriate spacing
fn write_hex_bytes<W: Write>(writer: &mut W, data: &[u8], config: &Config) -> Result<()> {
    for (i, byte) in data.iter().enumerate() {
        // Add space before byte if:
        // - It's not the first byte AND
        // - block_length > 0 (spacing enabled) AND
        // - We're starting a new block
        if i > 0 && config.block_length > 0 && i % config.block_length == 0 {
            write!(writer, " ")?;
        }

        // Write the byte
        write!(writer, "{:02X}", byte)?;

        // Add space after byte if:
        // - block_length > 0 (spacing enabled) AND
        // - It's not the last byte AND
        // - We're not at the end of a block
        if config.block_length > 0
            && (i + 1) < data.len()
            && (i + 1) % config.block_length != 0
        {
            write!(writer, " ")?;
        }
    }
    Ok(())
}

/// Write ASCII preview in square brackets
fn write_ascii_preview<W: Write>(writer: &mut W, data: &[u8]) -> Result<()> {
    write!(writer, "[")?;
    for &byte in data {
        if byte >= 32 && byte < 127 {
            write!(writer, "{}", byte as char)?;
        } else {
            write!(writer, ".")?;
        }
    }
    write!(writer, "]")?;
    Ok(())
}

/// Extract hex data from a line, removing address, preview, annotations, and comments
fn extract_hex_from_line(line: &str) -> Result<String> {
    let mut result = String::new();

    // Remove inline comments first (everything after '//')
    let line = if let Some(pos) = line.find("//") {
        &line[..pos]
    } else {
        line
    };

    // Remove address (everything before first ':')
    let line = if let Some(pos) = line.find(':') {
        &line[pos + 1..]
    } else {
        line
    };

    // Remove ASCII preview (everything in square brackets)
    let line = if let Some(start) = line.find('[') {
        &line[..start]
    } else {
        line
    };

    // Remove annotations (everything after '|')
    let line = if let Some(pos) = line.find('|') {
        &line[..pos]
    } else {
        line
    };

    // Extract only hex characters (no spaces)
    for ch in line.chars() {
        if ch.is_ascii_hexdigit() {
            result.push(ch);
        }
        // Silently ignore all other characters (spaces, etc)
    }

    Ok(result)
}

/// Parse hex string to bytes
fn parse_hex_string(hex: &str) -> Result<Vec<u8>> {
    let hex = hex.trim();
    if hex.is_empty() {
        return Ok(Vec::new());
    }

    if hex.len() % 2 != 0 {
        return Err(Error::OddHexLength);
    }

    let mut bytes = Vec::with_capacity(hex.len() / 2);
    for i in (0..hex.len()).step_by(2) {
        let byte_str = &hex[i..i + 2];
        let byte = u8::from_str_radix(byte_str, 16).map_err(|_| {
            Error::InvalidHexChar(
                byte_str.chars().next().unwrap_or('?'),
                i,
            )
        })?;
        bytes.push(byte);
    }

    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_string() {
        assert_eq!(parse_hex_string("48656C6C6F").unwrap(), b"Hello");
        assert_eq!(parse_hex_string("DEADBEEF").unwrap(), vec![0xDE, 0xAD, 0xBE, 0xEF]);
    }

    #[test]
    fn test_parse_hex_string_lowercase() {
        assert_eq!(parse_hex_string("deadbeef").unwrap(), vec![0xDE, 0xAD, 0xBE, 0xEF]);
    }

    #[test]
    fn test_extract_hex_from_line() {
        assert_eq!(
            extract_hex_from_line("00000000: 48 65 6C 6C 6F [Hello]").unwrap(),
            "48656C6C6F"
        );
        assert_eq!(
            extract_hex_from_line("48 65 6C 6C 6F").unwrap(),
            "48656C6C6F"
        );
        assert_eq!(
            extract_hex_from_line("00000000: 48 65 6C 6C 6F [Hello] | +0:utf8=\"Hello\"").unwrap(),
            "48656C6C6F"
        );
    }

    #[test]
    fn test_bin_to_hex_basic() {
        let input = b"Hello";
        let mut output = Vec::new();
        let config = Config::default();

        bin_to_hex(&input[..], &mut output, &config).unwrap();

        let result = String::from_utf8(output).unwrap();
        assert!(result.contains("48 65 6C 6C 6F"));
        assert!(result.contains("[Hello]"));
    }

    #[test]
    fn test_hex_to_bin_basic() {
        let input = b"48 65 6C 6C 6F";
        let mut output = Vec::new();

        hex_to_bin(&input[..], &mut output).unwrap();

        assert_eq!(output, b"Hello");
    }

    #[test]
    fn test_round_trip() {
        let original = b"Hello, World!";
        let mut hex_output = Vec::new();
        let config = Config::default();

        // Binary to hex
        bin_to_hex(&original[..], &mut hex_output, &config).unwrap();

        // Hex back to binary
        let mut bin_output = Vec::new();
        hex_to_bin(&hex_output[..], &mut bin_output).unwrap();

        assert_eq!(bin_output, original);
    }
}
