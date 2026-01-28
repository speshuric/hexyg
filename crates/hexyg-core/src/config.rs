//! Configuration for hexyg conversion operations

/// Byte order (endianness)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endian {
    /// Little endian byte order
    Little,
    /// Big endian byte order
    Big,
}

impl Default for Endian {
    fn default() -> Self {
        Endian::Little
    }
}

/// Address size in bytes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressSize {
    U8,
    U16,
    U24,
    U32,
    U40,
    U48,
    U64,
    /// Automatically adjust based on data size
    Stretch,
}

impl Default for AddressSize {
    fn default() -> Self {
        AddressSize::U32
    }
}

/// Padding behavior for address gaps
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Padding {
    /// Fill gaps with specified byte value
    Value(u8),
    /// Disallow gaps in addresses
    Forbidden,
}

impl Default for Padding {
    fn default() -> Self {
        Padding::Value(0x00)
    }
}

/// Configuration for hex conversion
#[derive(Debug, Clone)]
pub struct Config {
    /// Byte order
    pub endian: Endian,

    /// Address size
    pub address_size: AddressSize,

    /// Padding behavior
    pub padding: Padding,

    /// Number of bytes per line (default: 16)
    pub line_length: usize,

    /// Number of bytes per block within a line (default: 1, meaning space after each byte)
    /// If set to 0 or line_length, no intra-line spacing
    pub block_length: usize,

    /// Whether to repeat address on every line
    pub repeat_address: bool,

    /// Whether to include ASCII preview in square brackets
    pub show_preview: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            endian: Endian::default(),
            address_size: AddressSize::default(),
            padding: Padding::default(),
            line_length: 16,
            block_length: 1,
            repeat_address: true,
            show_preview: true,
        }
    }
}

impl Config {
    /// Create a new config with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder: set endianness
    pub fn with_endian(mut self, endian: Endian) -> Self {
        self.endian = endian;
        self
    }

    /// Builder: set address size
    pub fn with_address_size(mut self, address_size: AddressSize) -> Self {
        self.address_size = address_size;
        self
    }

    /// Builder: set padding
    pub fn with_padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    /// Builder: set line length
    pub fn with_line_length(mut self, line_length: usize) -> Self {
        self.line_length = line_length;
        self
    }

    /// Builder: set block length
    pub fn with_block_length(mut self, block_length: usize) -> Self {
        self.block_length = block_length;
        self
    }

    /// Builder: set whether to repeat address
    pub fn with_repeat_address(mut self, repeat: bool) -> Self {
        self.repeat_address = repeat;
        self
    }

    /// Builder: set whether to show ASCII preview
    pub fn with_preview(mut self, show: bool) -> Self {
        self.show_preview = show;
        self
    }
}
