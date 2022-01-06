const BACK_SLASH = '\\'.charCodeAt(0); // Skip next byte.
const BRACKET_OPEN = '['.charCodeAt(0);
const BRACKET_CLOSE = ']'.charCodeAt(0);
const COLON = ':'.charCodeAt(0);
const COMMA = ','.charCodeAt(0);
const CURLY_BRACKET_OPEN = '{'.charCodeAt(0);
const CURLY_BRACKET_CLOSE = '}'.charCodeAt(0);
const DOUBLE_QUOTE = '"'.charCodeAt(0);
const WHITE_SPACE = ' '.charCodeAt(0);

// Test in reverse order
const UTF8_2 = Number.parseInt('11000000', 2); // n & UTF8_2 === UTF8_2, Skip next byte
const UTF8_3 = Number.parseInt('11100000', 2); // n & UTF8_3 === UTF8_3, Skip next 2 bytes
const UTF8_4 = Number.parseInt('11110000', 2); // n & UTF8_4 === UTF8_4, Skip next 3 bytes

enum State {
  AtStart,
  InKey,
  PostKey,
  InValue,
  PostValue,
  InStream,
  AtEnd,
}
