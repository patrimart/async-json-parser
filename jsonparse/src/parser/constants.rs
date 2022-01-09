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

#[derive(Clone, Debug)]
pub enum ParseState {
    AtStart,
    InKey,
    PostKey,
    InValue,
    PostValue,
    InStream,
    AtEnd,
}

#[cfg(test)]
mod tests {
    use crate::parser::constants::*;

    #[test]
    fn verify_constants() {
        let json = b"{ \"foo\": [1, 2, 3], \"bar\": true }";
        let mut back_slash_count = 0;
        let mut bracket_open_count = 0;
        let mut bracket_close_count = 0;
        let mut colon_count = 0;
        let mut comma_count = 0;
        let mut curly_bracket_open_count = 0;
        let mut curly_bracket_close_count = 0;
        let mut double_quote_count = 0;
        let mut white_space_count = 0;
        let mut utf8_2_count = 0;
        let mut utf8_3_count = 0;
        let mut utf8_4_count = 0;
        for c in json.iter() {
            match *c {
                BACK_SLASH => {
                    back_slash_count = back_slash_count + 1;
                }
                BRACKET_OPEN => {
                    bracket_open_count = bracket_open_count + 1;
                }
                BRACKET_CLOSE => {
                    bracket_close_count = bracket_close_count + 1;
                }
                COLON => {
                    colon_count = colon_count + 1;
                }
                COMMA => {
                    comma_count = comma_count + 1;
                }
                CURLY_BRACKET_OPEN => {
                    curly_bracket_open_count = curly_bracket_open_count + 1;
                }
                CURLY_BRACKET_CLOSE => {
                    curly_bracket_close_count = curly_bracket_close_count + 1;
                }
                DOUBLE_QUOTE => {
                    double_quote_count = double_quote_count + 1;
                }
                WHITE_SPACE => {
                    white_space_count = white_space_count + 1;
                }
                UTF8_2 => {
                    utf8_2_count = utf8_2_count + 1;
                }
                UTF8_3 => {
                    utf8_3_count = utf8_3_count + 1;
                }
                UTF8_4 => {
                    utf8_4_count = utf8_4_count + 1;
                }
                _ => {}
            }
        }
        assert_eq!(back_slash_count, 0);
        assert_eq!(bracket_open_count, 1);
        assert_eq!(bracket_close_count, 1);
        assert_eq!(colon_count, 2);
        assert_eq!(comma_count, 3);
        assert_eq!(curly_bracket_open_count, 1);
        assert_eq!(curly_bracket_close_count, 1);
        assert_eq!(double_quote_count, 4);
        assert_eq!(white_space_count, 7);
        assert_eq!(utf8_2_count, 0);
        assert_eq!(utf8_3_count, 0);
        assert_eq!(utf8_4_count, 0);
    }
}
