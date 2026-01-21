# Hexator

[Русский](README.ru.md) | English

**Hexator** is a bidirectional converter and editor for hexadecimal (hex) text representations of binary data. It provides a powerful CLI tool, LSP server, and editor plugins for working with hex-encoded binary files in a human-readable, version-control-friendly format.

## Overview

Hexator allows you to:
- Convert binary files to structured hex text with formatting, comments, and metadata
- Convert hex text back to binary files with validation
- Edit hex files in your favorite editor with syntax highlighting and LSP support
- Store binary data in git repositories as text files, enabling diff analysis in pull requests

### Why Hexator?

Traditional binary files are opaque in version control systems. Hexator solves this by:
- Converting binaries to readable hex text that can be versioned and diffed
- Supporting comments and structured annotations for better documentation
- Providing editor integration for comfortable editing
- Enabling binary file generation during build processes

## Features

- **Bidirectional Conversion**: Convert between binary and hex text formats
- **Rich Formatting**: Customize line length, separators, address display, and more
- **Comments**: Single-line (`//`) and multi-line (`/* */`) comments in C/Java style
- **Structured Data**: Annotate hex regions with names, types, and values
- **LSP Support**: Language Server Protocol for editor integration
- **CLI Tool**: Command-line interface with pipe support
- **Cross-Platform**: Linux, Windows, macOS support
- **Multi-Language Core**: Reference implementation in Rust, with ports planned for TypeScript, Kotlin, and more

## Status

🚧 **Project Status**: Early Development

This project is currently in active development. The core functionality is being implemented according to the [master plan](specs/00_master_plan.md).

## Quick Start

### Installation

*(Installation instructions will be available once the project reaches a release state)*

### Basic Usage

```bash
# Convert binary file to hex text
hexator --bin-to-hex --input input.binary --output text.hex

# Convert hex text to binary file
hexator --hex-to-bin --input text.hex --output output.binary

# Use with pipes
echo "GOOGLE" | hexator --bin-to-hex
# Output: 474F4F474C45

echo "474F4F474C45" | hexator --hex-to-bin
# Output: GOOGLE
```

## Hex File Format

Hexator uses a structured text format for representing binary data:

```hex
#option endian=LITTLE_ENDIAN
#option padding=00
#option line_length=16

// Comments are supported
00000000: 4C 6F 72 65 6D 20 69 70 73 75 6D 20 64 6F 6C 6F [Lorem ipsum dolo]
00000010: 72 20 73 69 74 20 61 6D 65 74 2C 20 63 6F 6E 73 [r sit amet, cons]

// Address gaps are allowed with padding
00000100: 00 01 02 03
          04 05 06 07

// Structured annotations
00000200: FF 00 00 00 EE EE 00 00 | +0:u32=255 +03:u32=61166
```

See [Basic Idea](specs/00.02_basic_idea.md) for detailed grammar specification.

## Project Structure

- `/specs` - Project specifications and documentation
- `/grammar` - Grammar definitions (planned)
- `/crates` - Rust code (CLI, LSP, core)
- `/packages` - TypeScript/JavaScript code (planned)
- `/platforms` - IDE plugin implementations (planned)
- `/tests/corpus` - Shared test corpus for cross-language validation

## Documentation

- [Master Plan](specs/00_master_plan.md) - Overall development roadmap
- [Basic Idea](specs/00.02_basic_idea.md) - Core concepts and grammar
- [Architecture](specs/03_architecture.md) - System architecture
- [Grammar Specification](specs/01_grammar_spec.md) - Detailed grammar rules
- [Options](specs/02_options.md) - Configuration options

## Contributing

Contributions are welcome! Please refer to the project specifications in the `/specs` directory for implementation guidelines.

## License

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

## Roadmap

- [x] Project initialization and structure
- [ ] Grammar specification and reference implementation
- [ ] Rust core (CLI, parser, converter)
- [ ] LSP server implementation
- [ ] VS Code extension
- [ ] IntelliJ plugin
- [ ] Ports to other languages (TypeScript, Kotlin, etc.)

See [Master Plan](specs/00_master_plan.md) for detailed roadmap.

---

**Note**: This project is in early development. API and file formats may change.
