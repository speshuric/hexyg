/**
 * Hexyg Grammar (ANTLR4)
 * 
 * This grammar defines the syntax for Hexyg hex text format.
 * It is primarily used as documentation for LLM-assisted development,
 * but can also be used to generate parsers if needed.
 * 
 * Key features:
 * - Directives (#option, #struct)
 * - Hex data with addresses
 * - ASCII preview in square brackets
 * - Annotations with type information
 * - Comments (single-line // and multi-line /* */)
 * 
 * See specs/01_grammar_spec.md for detailed specification.
 */
grammar hexyg;

// ============================================================================
// LEXER RULES
// ============================================================================

// Directives
DIRECTIVE_OPTION: '#option';
DIRECTIVE_STRUCT: '#struct';

// Operators and Punctuation
COLON: ':';
EQUALS: '=';
PLUS: '+';
PIPE: '|';
COMMA: ',';
LBRACE: '{';
RBRACE: '}';
LBRACKET: '[';
RBRACKET: ']';
LPAREN: '(';
RPAREN: ')';

// Comments (must be before other rules to match first)
LINE_COMMENT: '//' ~[\r\n]* -> skip;
BLOCK_COMMENT: '/*' .*? '*/' -> skip;

// Whitespace (ignored)
WHITESPACE: [ \t]+ -> skip;
NEWLINE: '\r'? '\n' -> skip;

// String Literal (with escape sequences)
STRING_LITERAL: '"' ('\\' . | ~["\\])* '"';

// Hex Digits and Bytes
HEX_DIGIT: [0-9A-Fa-f];
HEX_BYTE: HEX_DIGIT HEX_DIGIT;

// Address (sequence of hex digits, must be before INTEGER)
ADDRESS: HEX_DIGIT+;

// Integer (decimal)
INTEGER: [0-9]+;

// Identifier (variable names, option names, struct names, type names)
IDENTIFIER: [a-zA-Z_][a-zA-Z0-9_]*;

// Unquoted Value (for option values)
// Matches alphanumeric, underscores, plus, minus, dots, slashes
// This is a flexible token that can represent various option values
UNQUOTED_VALUE: [a-zA-Z0-9_+\-./]+;

// ============================================================================
// PARSER RULES
// ============================================================================

// Top-level structure
// Comments and whitespace are already skipped by lexer
hex_file: (directive | data_line)* EOF;

// Directives
directive: option_directive | struct_directive;

option_directive: DIRECTIVE_OPTION option_name EQUALS option_value;
option_name: IDENTIFIER;
// Option value can be quoted string or unquoted value
// Unquoted value can be identifier, number, hex value, or composite value
// Note: IDENTIFIER, INTEGER, ADDRESS, and UNQUOTED_VALUE can all match,
// but parser will accept any of them as option_value
option_value: STRING_LITERAL | UNQUOTED_VALUE | IDENTIFIER | INTEGER | ADDRESS;

struct_directive: DIRECTIVE_STRUCT IDENTIFIER LBRACE struct_fields RBRACE;
struct_fields: struct_field (COMMA struct_field)* (COMMA)?;
struct_field: IDENTIFIER COLON type_specifier;

// Data Lines
// Note: hex_data can span multiple lines after address declaration
data_line: (address COLON)? hex_data? preview? annotations?;

address: ADDRESS;

// Hex data: sequence of hex bytes (whitespace between bytes is ignored by lexer)
hex_data: hex_byte+;
hex_byte: HEX_BYTE;

// Preview: ASCII representation in square brackets
preview: LBRACKET preview_chars RBRACKET;
preview_chars: PREVIEW_CHAR*;
fragment PREVIEW_CHAR: ~[\[\]\r\n]; // Any character except brackets and line breaks

// Annotations
annotations: PIPE annotation_list;
annotation_list: annotation+; // Whitespace between annotations is already skipped
annotation: offset_annotation | sequential_annotation;

offset_annotation: (PLUS offset)? (LPAREN IDENTIFIER RPAREN)? COLON type_specifier EQUALS value;
sequential_annotation: type_specifier EQUALS value;

offset: ADDRESS; // Offset from current address (hex digits)

value: INTEGER | STRING_LITERAL | struct_value;

struct_value: LBRACE struct_field_values RBRACE;
struct_field_values: struct_field_value (COMMA struct_field_value)*;
struct_field_value: IDENTIFIER EQUALS value;

// Type System
// Types are defined at semantic level, not in grammar
type_specifier: IDENTIFIER;
