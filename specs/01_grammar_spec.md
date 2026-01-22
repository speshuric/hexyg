# Hexator Grammar Specification

## Informal Grammar Example

```
#option bin_filename=lorem.bin
#option encoding=utf8
#option endian=LITTLE_ENDIAN
#option address_size=u32
#option padding=00 // FF or other value or padding=forbidden to disallow gaps

// the following settings are mainly for parsing or reformatting, they don't interfere with hex-to-bin conversion
#option line_length=16 
#option block_length=1 // so the line would be "4C 6F 72 65 6D 20 69 70 73 75 6D 20 64 6F 6C 6F"
#option repeat_adress=every_line

// comments in C/Java style
// empty lines and spaces don't matter

00000000: 4C 6F 72 65 6D 20 69 70 73 75 6D 20 64 6F 6C 6F [Lorem ipsum dolo]
00000010: 72 20 73 69 74 20 61 6D 65 74 2C 20 63 6F 6E 73 [r sit amet, cons]
00000020: 65 63 74 65 74 75 72 20 61 64 69 70 69 73 63 69 [ectetur adipisci]
00000030: 6E 67 20 65 6C 69 74 2C 20 73 65 64 20 64 6F 20 [ng elit, sed do ]
00000040: 65 69 75 73 6D 6F 64 20 74 65 6D 70 6F 72 20 69 [eiusmod tempor i]
00000050: 6E 63 69 64 69 64 75 6E 74 20 75 74 20 6C 61 62 [ncididunt ut lab]
00000060: 6F 72 65 20 65 74 20 64 6F 6C 6F 72 65 20 6D 61 [ore et dolore ma]
00000070: 67 6E 61 20 61 6C 69 71 75 61 2E                [gna aliqua.]

// text representation in square brackets
// address must end with colon and start at the beginning of line
// default start address is 0
// if padding is allowed, address gaps are permitted

00000100: 00 01 02 03 // this representation is also possible
          04 05 06 07

00000108: 
08090A0B
0C0D0E0F 
// same as "00000108: 08090A0B0C0D0E0F"

// values can appear after "|"
00000200: FF 00 00 00 EE EE 00 00 | +0:u32=255 +03:u32=61166 // by addresses
00000208: 01 00 00 00 02 00 03 00 | u32=1 u16=2 u16=3 // sequentially without addresses
00000210: 01 00 00 00 02 00 03 00 | +00(name_x):u32=1 +03(name_y):u16=2 +03(name_z):u16=3 // with names
// structures will be in future versions, this is for syntax demonstration
#struct data {
    name_x:u32, 
    name_y:u16, 
    name_z:u16, 
}
00000218: 01 00 00 00 02 00 03 00 | +00:data={name_x=1, name_y=2, name_z=3} // json-like
00000220: 4C 6F 72 65 6D 20 69 70 | +00:utf8="Lorem ipsum dolo"
          73 75 6D 20 64 6F 6C 6F 
// Structures are simple, no recursion, of course. But nesting is possible.

// representations and values can be combined
00000300: 4C 6F 72 65 6D 20 69 70 73 75 6D 20 64 6F 6C 6F [Lorem ipsum dolo] | +00:utf8="Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
00000310: 72 20 73 69 74 20 61 6D 65 74 2C 20 63 6F 6E 73 [r sit amet, cons]
00000320: 65 63 74 65 74 75 72 20 61 64 69 70 69 73 63 69 [ectetur adipisci]
00000330: 6E 67 20 65 6C 69 74 2C 20 73 65 64 20 64 6F 20 [ng elit, sed do ]
00000340: 65 69 75 73 6D 6F 64 20 74 65 6D 70 6F 72 20 69 [eiusmod tempor i]
00000350: 6E 63 69 64 69 64 75 6E 74 20 75 74 20 6C 61 62 [ncididunt ut lab]
00000360: 6F 72 65 20 65 74 20 64 6F 6C 6F 72 65 20 6D 61 [ore et dolore ma]
00000370: 67 6E 61 20 61 6C 69 71 75 61 2E                [gna aliqua.]

```

## Formal Grammar Specification

### Lexer (Token Definitions)

#### Whitespace and Comments
- `WHITESPACE`: `[ \t]+` (spaces and tabs, ignored)
- `NEWLINE`: `\r?\n` (line breaks)
- `LINE_COMMENT`: `//.*` (single-line comment, C/Java style)
- `BLOCK_COMMENT_START`: `/\*`
- `BLOCK_COMMENT_END`: `\*/`
- `BLOCK_COMMENT`: `/\*([^*]|\*+[^*/])*\*+/` (multi-line comment)

#### Directives
- `DIRECTIVE_OPTION`: `#option`
- `DIRECTIVE_STRUCT`: `#struct`

#### Identifiers and Literals
- `IDENTIFIER`: `[a-zA-Z_][a-zA-Z0-9_]*` (variable names, option names, struct names)
- `HEX_DIGIT`: `[0-9A-Fa-f]`
- `HEX_BYTE`: `HEX_DIGIT HEX_DIGIT` (two hex digits representing one byte)
- `ADDRESS`: `HEX_DIGIT+` (hexadecimal address, variable length)
- `INTEGER`: `[0-9]+` (decimal integer)
- `STRING_LITERAL`: `"([^"\\]|\\.)*"` (quoted string with escape sequences)
- `UNQUOTED_VALUE`: `[a-zA-Z0-9_+\-./]+` (unquoted option value: alphanumeric, underscores, plus, minus, dots, slashes; extends until whitespace, comment, or end of line)

#### Operators and Punctuation
- `COLON`: `:`
- `EQUALS`: `=`
- `PLUS`: `+`
- `PIPE`: `|`
- `COMMA`: `,`
- `SEMICOLON`: `;`
- `LBRACE`: `{`
- `RBRACE`: `}`
- `LBRACKET`: `[`
- `RBRACKET`: `]`
- `LPAREN`: `(`
- `RPAREN`: `)`

#### Type Specifiers
- `TYPE_U8`: `u8`
- `TYPE_U16`: `u16`
- `TYPE_U24`: `u24`
- `TYPE_U32`: `u32`
- `TYPE_U40`: `u40`
- `TYPE_U48`: `u48`
- `TYPE_U64`: `u64`
- `TYPE_UTF8`: `utf8`
- `TYPE_STRUCT`: `IDENTIFIER` (struct type name)

### BNF Grammar Rules

#### Top-Level Structure

```
hex_file ::= (directive | data_line | comment | whitespace)*
```

#### Directives

```
directive ::= option_directive | struct_directive

option_directive ::= DIRECTIVE_OPTION option_name EQUALS option_value (comment)?

option_name ::= IDENTIFIER

option_value ::= STRING_LITERAL | UNQUOTED_VALUE

// UNQUOTED_VALUE can represent:
// - Identifiers (e.g., "forbidden", "LITTLE_ENDIAN", "utf8")
// - Hexadecimal numbers (e.g., "00", "FF", "474F4F474C45")
// - Decimal numbers (e.g., "16", "255")
// - Composite values (e.g., "lorem.bin", "every_line")
// Interpretation depends on the specific option name

struct_directive ::= DIRECTIVE_STRUCT IDENTIFIER LBRACE struct_fields RBRACE

struct_fields ::= struct_field (COMMA struct_field)* (COMMA)?

struct_field ::= IDENTIFIER COLON type_specifier
```

#### Data Lines

```
data_line ::= (address COLON)? (hex_data | empty) (preview)? (annotations)? (comment)?

address ::= HEX_DIGIT+  // variable length, typically 8 hex digits

hex_data ::= hex_byte (whitespace hex_byte)*

empty ::= ε  // empty hex data

preview ::= LBRACKET preview_chars RBRACKET

preview_chars ::= ([^\[\]\r\n])*  // any characters except brackets and line breaks
```

#### Annotations

```
annotations ::= PIPE annotation_list

annotation_list ::= annotation (whitespace annotation)*

annotation ::= offset_annotation | sequential_annotation

offset_annotation ::= (PLUS offset)? (LPAREN IDENTIFIER RPAREN)? COLON type_specifier EQUALS value

sequential_annotation ::= type_specifier EQUALS value

offset ::= HEX_DIGIT+  // offset from current address

value ::= INTEGER | STRING_LITERAL | struct_value

struct_value ::= LBRACE struct_field_values RBRACE

struct_field_values ::= struct_field_value (COMMA struct_field_value)*

struct_field_value ::= IDENTIFIER EQUALS value
```

#### Type System

```
type_specifier ::= TYPE_U8 | TYPE_U16 | TYPE_U24 | TYPE_U32 | TYPE_U40 | TYPE_U48 | TYPE_U64 | TYPE_UTF8 | TYPE_STRUCT
```

#### Comments

```
comment ::= LINE_COMMENT | BLOCK_COMMENT
```

### Lexical Rules

1. **Whitespace**: Spaces, tabs, and empty lines are ignored except within string literals and ASCII previews.
2. **Comments**: Single-line (`//`) and multi-line (`/* */`) comments are ignored by the parser.
3. **Case Sensitivity**: Hex digits are case-insensitive (`4C` = `4c`). Identifiers are case-sensitive.
4. **Address Format**: Addresses are hexadecimal numbers of variable length, typically 8 digits (32-bit).
5. **Hex Bytes**: Must be exactly two hex digits. Spaces between bytes are optional but recommended for readability.
6. **String Escapes**: String literals support standard escape sequences (`\n`, `\t`, `\\`, `\"`, etc.).
7. **Option Values**: 
   - If quoted (starts with `"`), parse as `STRING_LITERAL` until closing quote.
   - If unquoted, parse as `UNQUOTED_VALUE` until whitespace, comment start (`//` or `/*`), or end of line.
   - `UNQUOTED_VALUE` can contain alphanumeric characters, underscores, plus, minus, dots, and slashes.
   - Interpretation of `UNQUOTED_VALUE` (as identifier, hex number, decimal number, etc.) depends on the specific option name.

### Semantic Rules

1. **Address Continuity**: Addresses must be sequential or have gaps filled with padding (if `padding` option allows).
2. **Address Validation**: When `padding=forbidden`, address gaps are not allowed.
3. **Type Alignment**: Multi-byte types (u16, u32, etc.) must respect endianness setting.
4. **Annotation Consistency**: Annotations must match the actual hex data at specified offsets.
5. **Struct Definition**: Structs must be defined before use in annotations.

### Notes

- The grammar is designed to be flexible: hex data can span multiple lines after an address declaration.
- Preview (in square brackets) is optional and informational only. It can contain any characters except brackets and line breaks.
- Annotations are optional metadata and don't affect binary conversion.
- Struct definitions are forward-looking features and may be simplified in initial implementation.
