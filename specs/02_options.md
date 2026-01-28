# hexyg Command Line Options

Usage:
```shell
hexyg [COMMAND] [OPTIONS]
```

## Commands

- `--help`, `-?`, `-h` or no command – print help and exit, no options
- `--hex-to-bin` – convert hex input to binary output
- `--bin-to-hex` – convert binary input to hex output

## Options

### Common options

- `--input {filename}`, `-i {filename}` – input filename, stdin if omitted
- `--output {filename}`, `-o {filename}` – output filename, stdout if omitted
- `--from={addr}` – start address can be decimal (such as 987) or hexadecimal with `0x` prefix (such as 0x4FE). Negative decimal works as the end-based address (as in slices in Python, for example). Addresses outside the file boundaries cause an error (non-zero return code). Default is 0.
- `--to={addr}` – end address can be decimal (such as 987) or hexadecimal with `0x` prefix (such as 0x4FE). Negative decimal works as the end-based address (as in slices in Python, for example). End address is exclusive. Addresses outside the file boundaries cause an error (non-zero return code). Default is the last address in the file.

### `--hex-to-bin` options

- `--check={none|text|values|all|text,values|values,text}` – check that hex values correspond to the text representation, explicit values or both. Default is `none`

### `--bin-to-hex` options

- `--option {optionname}={optionvalue}` – set `#option {optionname}={optionvalue}` in hex file. Note that some `#option` lines are generated from command line options. If such options are in conflict, the program terminates with an error (non-zero return code).
- `--start-address={addr}` – start address in hex file can be non-negative decimal (such as 987) or hexadecimal with `0x` prefix (such as 0x4FE). `from` address in binary become `start-address` in hex.
- `--address-size={u16|u24|u32|u64|u128|stretch}` – address size. Default is `stretch` (automatically adjusts based on data size). Set `address_size` option in file.
- `--address-size-step={N}` – alignment step for address width (number of digits). Default is 1. Set `address_size_step` option in file.
- `--line-length={N}` – line length in bytes. Default is 16. Set `line_length` option in file.
- `--block-length={N}` – split line into blocks of N bytes. Default is 0 (no blocking). Set `block_length` option in file.
- `--byte-separator=""` – empty or whitespace string to separate bytes
- `--block-separator=" "` – empty or whitespace string to separate blocks of bytes in one line
- `--repeat-address={never|once|every_line}` – print address on every line or only on discontinuity. Default is `every_line`. Set `repeat_address` option in file.

#### Future options (not implemented)

- `--format-line {formatline}`
- `--format-file {formatfile}`
- `--imhex-file {imhexfile}`
- `--include-file {includefile}` include file with options and structs

## Examples

```bash
# Convert binary to hex (stdout)
echo "GOOGLE" | hexyg --bin-to-hex

# Convert hex to binary (stdout)
echo "474F4F474C45" | hexyg --hex-to-bin

# Convert file binary to hex
hexyg --bin-to-hex --input input.bin --output output.hex

# Convert file hex to binary
hexyg --hex-to-bin --input input.hex --output output.bin

# Custom formatting
hexyg --bin-to-hex --input data.bin --line-length 8 --address-size u16

# Block formatting
hexyg --bin-to-hex --input data.bin --block-length 4

# Never repeat addresses
hexyg --bin-to-hex --input data.bin --repeat-address never
```

## Implementation Status

### Fully Implemented
- Commands: `--bin-to-hex`, `--hex-to-bin`
- Common: `--input`, `--output`
- bin-to-hex: `--address-size`, `--line-length`, `--block-length`, `--repeat-address`

### Parsed but Not Yet Used
- Common: `--from`, `--to`
- bin-to-hex: `--option`, `--start-address`, `--address-size-step`, `--byte-separator`, `--block-separator`
- hex-to-bin: `--check`

### Future Implementation
- Advanced formatting options
- File includes
- ImHex integration
