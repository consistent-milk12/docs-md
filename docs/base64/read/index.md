*[base64](../index.md) / [read](index.md)*

---

# Module `read`

Implementations of `io::Read` to transparently decode base64.

## Structs

### `DecoderReader<'e, E: Engine, R: io::Read>`

```rust
struct DecoderReader<'e, E: Engine, R: io::Read> {
    // [REDACTED: Private Fields]
}
```

A `Read` implementation that decodes base64 data read from an underlying reader.

# Examples

```
use std::io::Read;
use std::io::Cursor;
use base64::engine::general_purpose;

// use a cursor as the simplest possible `Read` -- in real code this is probably a file, etc.
let mut wrapped_reader = Cursor::new(b"YXNkZg==");
let mut decoder = base64::read::DecoderReader::new(
    &mut wrapped_reader,
    &general_purpose::STANDARD);

// handle errors as you normally would
let mut result = Vec::new();
decoder.read_to_end(&mut result).unwrap();

assert_eq!(b"asdf", &result[..]);

```

#### Implementations

- `fn new(reader: R, engine: &'e E) -> Self`
  Create a new decoder that will read from the provided reader `r`.

- `fn into_inner(self: Self) -> R`
  Unwraps this `DecoderReader`, returning the base reader which it reads base64 encoded

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Read<'e, E: Engine, R: io::Read>`

- `fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize>`
  Decode input from the wrapped reader.

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'e, E: Engine, R: io::Read>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

