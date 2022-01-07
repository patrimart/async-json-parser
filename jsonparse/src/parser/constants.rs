pub const BACK_SLASH: u8 = b'\\';
pub const BRACKET_OPEN: u8 = b'[';
pub const BRACKET_CLOSE: u8 = b']';
pub const COLON: u8 = b':';
pub const COMMA: u8 = b',';
pub const CURLY_BRACKET_OPEN: u8 = b'{';
pub const CURLY_BRACKET_CLOSE: u8 = b'}';
pub const DOUBLE_QUOTE: u8 = b'"';
pub const WHITE_SPACE: u8 = b' ';

pub const UTF8_2: u8 = 0b11000000; // n & UTF8_2 === UTF8_2, Skip next byte
pub const UTF8_3: u8 = 0b11100000; // n & UTF8_3 === UTF8_3, Skip next 2 bytes
pub const UTF8_4: u8 = 0b11110000; // n & UTF8_4 === UTF8_4, Skip next 3 bytes

pub enum ParseState {
    AtStart,
    InKey,
    PostKey,
    InValue,
    PostValue,
    InStream,
    AtEnd,
}
