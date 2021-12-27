import { jsonparse } from './jsonparse';

type Path<
  P1 extends string,
  P2 extends string | undefined = undefined,
  P3 extends string | undefined = undefined,
  P4 extends string | undefined = undefined,
  P5 extends string | undefined = undefined
> = P5 extends undefined
  ? P4 extends undefined
    ? P3 extends undefined
      ? P2 extends undefined
        ? [P1]
        : readonly [P1, P2]
      : readonly [P1, P2, P3]
    : readonly [P1, P2, P3, P4]
  : readonly [P1, P2, P3, P4, P5];

type ValueAtPath<
  Obj extends Readonly<Record<keyof Obj, unknown>>,
  Path extends ReadonlyArray<string>
> = Path extends readonly [keyof Obj]
  ? Obj[Path[0]] extends ReadonlyArray<any>
    ? Obj[Path[0]][0]
    : never
  : Path extends readonly [keyof Obj, ...infer R]
  ? R extends ReadonlyArray<string>
    ? ValueAtPath<Obj[Path[0]], R>
    : never
  : never;

type Response<W, T> = AsyncIterable<T> & {
  readonly write: (arr: W) => void;
  readonly cancel: () => void;
};

interface Obj {
  readonly a: {
    readonly b: {
      readonly c: number[];
    };
  };
}

const path: Path<'a', 'b', 'c'> = ['a', 'b', 'c'];

type Value = ValueAtPath<Obj, Path<'a', 'b', 'c'>>;

/**
 *
 * @param pathToArray - Assuming JSON is an object, path to the array. Omit if JSON is an array.
 * @throws
 */
export const createParser = <P extends string, T>(pathToArray?: Path<P>) => {
  /**
   *
   * @param stream
   */
  const withStream = (
    stream: ReadableStream<Uint8Array>
  ): Omit<Response<never, T>, 'write'> => {
    const cancel = () => {
      stream.cancel();
    };

    const asyncIterable = {
      async *[Symbol.asyncIterator]() {
        const reader = stream.getReader();
        const parser = jsonparse<T>(pathToArray);
        readMore: while (true) {
          const { value, done } = await reader.read();
          if (done || value === undefined) return;
          parser.push(value);
          while (true) {
            const output = parser.poll();
            switch (output) {
              case 'done':
                return;
              case 'pending':
                break readMore;
              default:
                yield output[0];
            }
          }
        }
      },
    };

    return { cancel, ...asyncIterable };
  };

  /**
   *
   * @param array
   */
  const withArray = (array: Uint8Array): Response<Uint8Array, T> => {
    let isDone = false;
    let callback: undefined | ((arr: Uint8Array) => void);

    const stream = new ReadableStream<Uint8Array>({
      type: 'bytes' as any,
      start(controller) {
        controller.enqueue(array);
        callback = arr => controller.enqueue(arr);
      },
    });

    const iterable = withStream(stream);

    const write = (arr: Uint8Array) => {
      if (!isDone) callback?.(arr);
    };

    const cancelPlus = () => {
      isDone = true;
      iterable.cancel();
    };

    return { ...iterable, write, cancel: cancelPlus };
  };

  /**
   *
   * @param json
   */
  const withString = (json: string): Response<string, T> => {
    const encoder = new TextEncoder();
    const {
      write: writeArray,
      cancel,
      ...iterable
    } = withArray(encoder.encode(json));
    const write = (json: string) => {
      writeArray(encoder.encode(json));
    };

    return { ...iterable, write, cancel };
  };

  return {
    withStream,
    withArray,
    withString,
  };
};
