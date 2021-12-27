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
const UTF8_2 = '11000000'.charCodeAt(0); // n & UTF8_2 === UTF8_2, Skip next byte
const UTF8_3 = '11100000'.charCodeAt(0); // n & UTF8_3 === UTF8_3, Skip next 2 bytes
const UTF8_4 = '11110000'.charCodeAt(0); // n & UTF8_4 === UTF8_4, Skip next 3 bytes

enum State {
  AtStart,
  InKey,
  PostKey,
  InValue,
  PostValue,
  InStream,
  AtEnd,
}

export const jsonparse = <T>(pathToArray?: ReadonlyArray<string>) => {
  const utf8Decoder = new TextDecoder('utf-8');

  const state = State.AtStart;
  const currentPath: string[] = [];
  const depthStack: number[] = [];

  let currentIndex = 0;
  let stream = new Uint8Array();

  const push = (arr: Uint8Array) => {
    const joinedStream = new Uint8Array(stream.length + arr.length);
    joinedStream.set(stream);
    joinedStream.set(arr);
  };

  const poll = (): [T] | 'done' | 'pending' => {
    // First, find array start in path.

    // throw any error.
    // Find next array item.

    return 'pending';
  };

  return {
    push,
    poll,
  };
};
