# Command line options

## USAGE:

```
hexyg [COMMAND] [COMMAND_ARGUMENT COMMAND_ARGUMENT ...] [COMMAND_OPTION COMMAND_OPTION ...]
```

### COMMANDS:
    --help, --?, -?, -h - вывести справку (команда по умолчанию)
    --bin-to-hex - конвертировать двоичные данные в hex
    --hex-to-bin - конвертировать hex в двоичные данные

### COMMAND ARGUMENTS:
    --input {filename}, -i {filename} - входной файл, если не задан, то stdio
    --output {filename}, -o {filename} - выходной файл, если не задан, то stdio
    
## COMMAND OPTIONS:
    --endian={ENDIAN}, -e {ENDIAN} - ENDIAN: LITTLE_ENDIAN, LE, BIG_ENDIAN, BE
    --address_size={SIZE} - SIZE: u8, u16, u24, u32, u40, u48, u64, stretch; по умолчанию u32
    --padding={PADDING} - PADDING: 00..FF, forbidden; по умолчанию 00; forbidden для запрета пропусков
    --line_length={N} - N = 2..256 (степени 2), по умолчанию 16
    --block_length={N} - N = 1..256 (степени 2) или no - сколько байт блок в строке, no - без разрывов
    --repeat_adress=every_line

