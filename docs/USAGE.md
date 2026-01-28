# `hexyg` usage


Usage:
```shell
hexyg [COMMAND] [OPTIONS]
```

## Commands

- `--help`, `-?` or no command &ndash; print help and exit, no options
- `--hex-to-bin` &ndash; convert hex input to binary output
- `--bin-to-hex` &ndash; convert binary input to hex output

## Options

### Common options

- `--input {filename}` &ndash; input filename, stdin if omitted
- `--output {filename}` &ndash; output filename, stdout if omitted
- `--from={addr}` &ndash; start address can be decimal (such as 987) or hexadecimal with `0x` prefix (such as 0x4FE). Negative decimal works as the end-based address (as in slices in Python, for example). Addresses outside the file boundaries cause an error (non-zero return code). Default is 0.
- `--to={addr}` &ndash; end address can be decimal (such as 987) or hexadecimal with `0x` prefix (such as 0x4FE). Negative decimal works as the end-based address (as in slices in Python, for example). End address is exclusive. Addresses outside the file boundaries cause an error (non-zero return code). Default is the last address in the file.

### `--hex-to-bin` options

- `--check={none|text|values|all|text,values|values,text}` &ndash; check that hex values correspond to the text representation, explicit values or both. Default is `none`

### `--bin-to-hex` options

- `--option {optionname}={optionvalue}` &ndash; set `#option {optionname}={optionvalue}` in hex file. Note that some `#option` lines are generated from command line options. If such options are in conflict, the program terminates with an error (non-zero return code).
- `--start-address={addr}` &ndash; start address in hex file can be non-negative decimal (such as 987) or hexadecimal with `0x` prefix (such as 0x4FE). `from` address in binary become `start-address` in hex.
- `--address-size={u16|u24|u32|u64|u128|arbitrary}` &ndash; address size. Default is `arbitrary`. Set `address_size` option in file.
- `--address-size-step={N}` &ndash; alignment step for address width (number of digits). Default is 1. Set `address_size_step` option in file.
- `--line-length={N}` &ndash; line length in bytes. Default is 16. Set `line_length` option in file.
- `--block-length={N}` &ndash; split line into blocks of N bytes. Default is 0 (no blocking).
- `--byte-separator=""` &ndash; empty or whitespace string to separate bytes
- `--block-separator=" "` &ndash; empty or whitespace string to separate blocks of bytes in one line
- `--repeat-address={never|once|every_line}` &ndash; print address on every line or only on discontinuity. Default is `every_line`.

#### Future options (not implemented)

- `--format-line {formatline}`
- `--format-file {formatfile}`
- `--imhex-file {imhexfile}`
- `--include-file {includefile}` include file with options and structs 