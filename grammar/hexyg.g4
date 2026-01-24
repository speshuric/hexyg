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
 * - Comments (single-line // and multi-line /* *_/)
 * 
 * See specs/01_grammar_spec.md for detailed specification.
 */
grammar hexyg;

// ============================================================================
// LEXER RULES
// ============================================================================

// Comments (must be first to have highest priority)
LINE_COMMENT: '//' ~[\r\n]* -> skip;
BLOCK_COMMENT: '/*' .*? '*/' -> skip;

// Whitespace (ignored)
WHITESPACE: [ \t]+ -> skip;
NEWLINE: '\r'? '\n' -> skip;

// Directives
DIRECTIVE_OPTION: '#option';
DIRECTIVE_STRUCT: '#struct';

// ASCII Preview Literal (must be before LBRACKET to match first)
// Matches [ followed by anything until ]
PREVIEW_LITERAL: '[' ~[\r\n\]]* ']';

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

// String Literal (with escape sequences)
STRING_LITERAL: '"' ('\\' . | ~["\\])* '"';

// Hex sequence (any length) - used for addresses, hex bytes, and hex values
// This is a universal token, parser will determine usage from context
HEX_SEQUENCE: [0-9A-Fa-f]+;

// Identifier (variable names, option names, struct names, type names)
// Must start with letter or underscore (not digit!)
IDENTIFIER: [a-zA-Z_][a-zA-Z0-9_]*;

// Unquoted Value (for option values that are not pure identifiers or hex)
// Must start with letter to avoid conflict with HEX_SEQUENCE
// Can contain dots, slashes for filenames/paths, but NOT colon (reserved for syntax)
UNQUOTED_VALUE: [a-zA-Z][a-zA-Z0-9_+\-./]*;

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
// Option value can be: quoted string, identifier, hex sequence, or composite value
option_value: STRING_LITERAL | UNQUOTED_VALUE | IDENTIFIER | HEX_SEQUENCE;

struct_directive: DIRECTIVE_STRUCT IDENTIFIER LBRACE struct_fields RBRACE;
struct_fields: struct_field (COMMA struct_field)* (COMMA)?;
struct_field: IDENTIFIER COLON type_specifier;

// Data Lines
// Note: hex_data can span multiple lines after address declaration
data_line: (address COLON)? hex_data preview? annotations?;

// Address is a hex sequence in address position (before colon)
address: HEX_SEQUENCE;

// Hex data: one or more hex bytes OR one long hex sequence without spaces
// A hex byte is exactly 2 hex digits
// Parser accepts any HEX_SEQUENCE, semantic validation must check length is even and divide by 2
hex_data: HEX_SEQUENCE+;

// Preview: ASCII representation in square brackets
// PREVIEW_LITERAL already includes the brackets
preview: PREVIEW_LITERAL;

// Annotations
annotations: PIPE annotation_list;
annotation_list: annotation+; // Whitespace between annotations is already skipped
annotation: offset_annotation | sequential_annotation;

offset_annotation: (PLUS offset)? (LPAREN IDENTIFIER RPAREN)? COLON type_specifier EQUALS value;
sequential_annotation: type_specifier EQUALS value;

// Offset from current address (hex digits)
offset: HEX_SEQUENCE;

// Value can be: decimal integer (HEX_SEQUENCE that looks like decimal), string, or struct
value: HEX_SEQUENCE | STRING_LITERAL | struct_value;

struct_value: LBRACE struct_field_values RBRACE;
struct_field_values: struct_field_value (COMMA struct_field_value)*;
struct_field_value: IDENTIFIER EQUALS value;

// Type System
// Types are defined at semantic level, not in grammar
type_specifier: IDENTIFIER;
