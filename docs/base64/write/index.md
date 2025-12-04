*[base64](../index.md) / [write](index.md)*

---

# Module `write`

Implementations of `io::Write` to transparently handle base64.

## Structs

### `EncoderWriter<'e, E: Engine, W: io::Write>`

```rust
struct EncoderWriter<'e, E: Engine, W: io::Write> {
}
```

A `Write` implementation that base64 encodes data before delegating to the wrapped writer.

Because base64 has special handling for the end of the input data (padding, etc), there's a
`finish()` method on this type that encodes any leftover input bytes and adds padding if
appropriate. It's called automatically when deallocated (see the `Drop` implementation), but
any error that occurs when invoking the underlying writer will be suppressed. If you want to
handle such errors, call `finish()` yourself.

# Examples

```
use std::io::Write;
use base64::engine::general_purpose;

// use a vec as the simplest possible `Write` -- in real code this is probably a file, etc.
let mut enc = base64::write::EncoderWriter::new(Vec::new(), &general_purpose::STANDARD);

// handle errors as you normally would
enc.write_all(b"asdf").unwrap();

// could leave this out to be called by Drop, if you don't care
// about handling errors or getting the delegate writer back
let delegate = enc.finish().unwrap();

// base64 was written to the writer
assert_eq!(b"YXNkZg==", &delegate[..]);

```

# Panics

Calling `write()` (or related methods) or `finish()` after `finish()` has completed without
error is invalid and will panic.

# Errors

Base64 encoding itself does not generate errors, but errors from the wrapped writer will be
returned as per the contract of `Write`.

# Performance

It has some minor performance loss compared to encoding slices (a couple percent).
It does not do any heap allocation.

# Limitations

Owing to the specification of the `write` and `flush` methods on the `Write` trait and their
implications for a buffering implementation, these methods may not behave as expected. In
particular, calling `write_all` on this interface may fail with `io::ErrorKind::WriteZero`.
See the documentation of the `Write` trait implementation for further details.

#### Implementations

- `fn new(delegate: W, engine: &'e E) -> EncoderWriter<'e, E, W>`
  Create a new encoder that will write to the provided delegate writer.

- `fn finish(self: &mut Self) -> Result<W>`
  Encode all remaining buffered data and write it, including any trailing incomplete input

- `fn into_inner(self: Self) -> W`
  Unwraps this `EncoderWriter`, returning the base writer it writes base64 encoded output

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

##### `impl Drop<'e, E: Engine, W: io::Write>`

- `fn drop(self: &mut Self)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write<'e, E: Engine, W: io::Write>`

- `fn write(self: &mut Self, input: &[u8]) -> Result<usize>`
  Encode input and then write to the delegate writer.

- `fn flush(self: &mut Self) -> Result<()>`
  Because this is usually treated as OK to call multiple times, it will *not* flush any

##### `impl Debug<'e, E: Engine, W: io::Write>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `EncoderStringWriter<'e, E: Engine, S: StrConsumer>`

```rust
struct EncoderStringWriter<'e, E: Engine, S: StrConsumer> {
}
```

A `Write` implementation that base64-encodes data using the provided config and accumulates the
resulting base64 utf8 `&str` in a [StrConsumer] implementation (typically `String`), which is
then exposed via `into_inner()`.

# Examples

Buffer base64 in a new String:

```
use std::io::Write;
use base64::engine::general_purpose;

let mut enc = base64::write::EncoderStringWriter::new(&general_purpose::STANDARD);

enc.write_all(b"asdf").unwrap();

// get the resulting String
let b64_string = enc.into_inner();

assert_eq!("YXNkZg==", &b64_string);
```

Or, append to an existing `String`, which implements `StrConsumer`:

```
use std::io::Write;
use base64::engine::general_purpose;

let mut buf = String::from("base64: ");

let mut enc = base64::write::EncoderStringWriter::from_consumer(
    &mut buf,
    &general_purpose::STANDARD);

enc.write_all(b"asdf").unwrap();

// release the &mut reference on buf
let _ = enc.into_inner();

assert_eq!("base64: YXNkZg==", &buf);
```

# Performance

Because it has to validate that the base64 is UTF-8, it is about 80% as fast as writing plain
bytes to a `io::Write`.

#### Implementations

- `fn from_consumer(str_consumer: S, engine: &'e E) -> Self`
  Create a EncoderStringWriter that will append to the provided `StrConsumer`.

- `fn into_inner(self: Self) -> S`
  Encode all remaining buffered data, including any trailing incomplete input triples and

- `fn new(engine: &'e E) -> Self`
  Create a EncoderStringWriter that will encode into a new `String` with the provided config.

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

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write<'e, E: Engine, S: StrConsumer>`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

## Traits

