*[flate2](../index.md) / [write](index.md)*

---

# Module `write`

Types which operate over [`Write`](#write) streams, both encoders and decoders for
various formats.


## Structs

### `DeflateDecoder<W: Write>`

```rust
struct DeflateDecoder<W: Write> {
    // [REDACTED: Private Fields]
}
```

A DEFLATE decoder, or decompressor.

This structure implements a [`Write`](#write) and will emit a stream of decompressed
data when fed a stream of compressed data.

After decoding a single member of the DEFLATE data this writer will return the number of bytes up to
to the end of the DEFLATE member and subsequent writes will return Ok(0) allowing the caller to
handle any data following the DEFLATE member.

# Examples

```rust
use std::io::prelude::*;
use std::io;
use flate2::Compression;
use flate2::write::DeflateEncoder;
use flate2::write::DeflateDecoder;

fn main() {
   let mut e = DeflateEncoder::new(Vec::new(), Compression::default());
   e.write_all(b"Hello World").unwrap();
   let bytes = e.finish().unwrap();
   println!("{}", decode_writer(bytes).unwrap());
}
// Uncompresses a Deflate Encoded vector of bytes and returns a string or error
// Here Vec<u8> implements Write
fn decode_writer(bytes: Vec<u8>) -> io::Result<String> {
   let mut writer = Vec::new();
   let mut deflater = DeflateDecoder::new(writer);
   deflater.write_all(&bytes[..])?;
   writer = deflater.finish()?;
   let return_string = String::from_utf8(writer).expect("String parsing error");
   Ok(return_string)
}
```

#### Implementations

- `fn new(w: W) -> DeflateDecoder<W>`
  Creates a new decoder which will write uncompressed data to the stream.

- `fn get_ref(self: &Self) -> &W`
  Acquires a reference to the underlying writer.

- `fn get_mut(self: &mut Self) -> &mut W`
  Acquires a mutable reference to the underlying writer.

- `fn reset(self: &mut Self, w: W) -> io::Result<W>`
  Resets the state of this decoder entirely, swapping out the output

- `fn try_finish(self: &mut Self) -> io::Result<()>`
  Attempt to finish this output stream, writing out final chunks of data.

- `fn finish(self: Self) -> io::Result<W>`
  Consumes this encoder, flushing the output stream.

- `fn total_in(self: &Self) -> u64`
  Returns the number of bytes that the decompressor has consumed for

- `fn total_out(self: &Self) -> u64`
  Returns the number of bytes that the decompressor has written to its

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

##### `impl Read<W: Read + Write>`

- `fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write<W: Write>`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

##### `impl Debug<W: $crate::fmt::Debug + Write>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `DeflateEncoder<W: Write>`

```rust
struct DeflateEncoder<W: Write> {
    // [REDACTED: Private Fields]
}
```

A DEFLATE encoder, or compressor.

This structure implements a [`Write`](#write) interface and takes a stream of
uncompressed data, writing the compressed data to the wrapped writer.

# Examples

```rust
use std::io::prelude::*;
use flate2::Compression;
use flate2::write::DeflateEncoder;

// Vec<u8> implements Write to print the compressed bytes of sample string
fn main() {

let mut e = DeflateEncoder::new(Vec::new(), Compression::default());
e.write_all(b"Hello World").unwrap();
println!("{:?}", e.finish().unwrap());
}
```

#### Implementations

- `fn new(w: W, level: crate::Compression) -> DeflateEncoder<W>`
  Creates a new encoder which will write compressed data to the stream

- `fn get_ref(self: &Self) -> &W`
  Acquires a reference to the underlying writer.

- `fn get_mut(self: &mut Self) -> &mut W`
  Acquires a mutable reference to the underlying writer.

- `fn reset(self: &mut Self, w: W) -> io::Result<W>`
  Resets the state of this encoder entirely, swapping out the output

- `fn try_finish(self: &mut Self) -> io::Result<()>`
  Attempt to finish this output stream, writing out final chunks of data.

- `fn finish(self: Self) -> io::Result<W>`
  Consumes this encoder, flushing the output stream.

- `fn flush_finish(self: Self) -> io::Result<W>`
  Consumes this encoder, flushing the output stream.

- `fn total_in(self: &Self) -> u64`
  Returns the number of bytes that have been written to this compressor.

- `fn total_out(self: &Self) -> u64`
  Returns the number of bytes that the compressor has produced.

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

##### `impl Read<W: Read + Write>`

- `fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write<W: Write>`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

##### `impl Debug<W: $crate::fmt::Debug + Write>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `GzDecoder<W: Write>`

```rust
struct GzDecoder<W: Write> {
    // [REDACTED: Private Fields]
}
```

A decoder for a single member of a [gzip file].

This structure exposes a [`Write`](#write) interface, receiving compressed data and
writing uncompressed data to the underlying writer.

After decoding a single member of the gzip data this writer will return the number of bytes up to
to the end of the gzip member and subsequent writes will return Ok(0) allowing the caller to
handle any data following the gzip member.

To handle gzip files that may have multiple members, see [`MultiGzDecoder`](../index.md)
or read more
[in the introduction](../index.html#about-multi-member-gzip-files).

[gzip file]: https://www.rfc-editor.org/rfc/rfc1952#page-5

# Examples

```rust
use std::io::prelude::*;
use std::io;
use flate2::Compression;
use flate2::write::{GzEncoder, GzDecoder};

fn main() {
   let mut e = GzEncoder::new(Vec::new(), Compression::default());
   e.write(b"Hello World").unwrap();
   let bytes = e.finish().unwrap();
   assert_eq!("Hello World", decode_writer(bytes).unwrap());
}
// Uncompresses a gzip encoded vector of bytes and returns a string or error
// Here Vec<u8> implements Write
fn decode_writer(bytes: Vec<u8>) -> io::Result<String> {
   let mut writer = Vec::new();
   let mut decoder = GzDecoder::new(writer);
   decoder.write_all(&bytes[..])?;
   writer = decoder.finish()?;
   let return_string = String::from_utf8(writer).expect("String parsing error");
   Ok(return_string)
}
```

#### Implementations

- `fn new(w: W) -> GzDecoder<W>`
  Creates a new decoder which will write uncompressed data to the stream.

- `fn header(self: &Self) -> Option<&GzHeader>`
  Returns the header associated with this stream.

- `fn get_ref(self: &Self) -> &W`
  Acquires a reference to the underlying writer.

- `fn get_mut(self: &mut Self) -> &mut W`
  Acquires a mutable reference to the underlying writer.

- `fn try_finish(self: &mut Self) -> io::Result<()>`
  Attempt to finish this output stream, writing out final chunks of data.

- `fn finish(self: Self) -> io::Result<W>`
  Consumes this decoder, flushing the output stream.

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

##### `impl Read<W: Read + Write>`

- `fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write<W: Write>`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

##### `impl Debug<W: $crate::fmt::Debug + Write>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `GzEncoder<W: Write>`

```rust
struct GzEncoder<W: Write> {
    // [REDACTED: Private Fields]
}
```

A gzip streaming encoder

This structure exposes a [`Write`](#write) interface that will emit compressed data
to the underlying writer `W`.

# Examples

```rust
use std::io::prelude::*;
use flate2::Compression;
use flate2::write::GzEncoder;

// Vec<u8> implements Write to print the compressed bytes of sample string
fn main() {

let mut e = GzEncoder::new(Vec::new(), Compression::default());
e.write_all(b"Hello World").unwrap();
println!("{:?}", e.finish().unwrap());
}
```

#### Implementations

- `fn new(w: W, level: Compression) -> GzEncoder<W>`
  Creates a new encoder which will use the given compression level.

- `fn get_ref(self: &Self) -> &W`
  Acquires a reference to the underlying writer.

- `fn get_mut(self: &mut Self) -> &mut W`
  Acquires a mutable reference to the underlying writer.

- `fn try_finish(self: &mut Self) -> io::Result<()>`
  Attempt to finish this output stream, writing out final chunks of data.

- `fn finish(self: Self) -> io::Result<W>`
  Finish encoding this stream, returning the underlying writer once the

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

##### `impl Drop<W: Write>`

- `fn drop(self: &mut Self)`

##### `impl Read<R: Read + Write>`

- `fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write<W: Write>`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

##### `impl Debug<W: $crate::fmt::Debug + Write>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `MultiGzDecoder<W: Write>`

```rust
struct MultiGzDecoder<W: Write> {
    // [REDACTED: Private Fields]
}
```

A gzip streaming decoder that decodes a [gzip file] with multiple members.

This structure exposes a [`Write`](#write) interface that will consume compressed data and
write uncompressed data to the underlying writer.

A gzip file consists of a series of *members* concatenated one after another.
`MultiGzDecoder` decodes all members of a file and writes them to the
underlying writer one after another.

To handle members separately, see [GzDecoder] or read more
[in the introduction](../index.html#about-multi-member-gzip-files).

[gzip file]: https://www.rfc-editor.org/rfc/rfc1952#page-5

#### Implementations

- `fn new(w: W) -> MultiGzDecoder<W>`
  Creates a new decoder which will write uncompressed data to the stream.

- `fn header(self: &Self) -> Option<&GzHeader>`
  Returns the header associated with the current member.

- `fn get_ref(self: &Self) -> &W`
  Acquires a reference to the underlying writer.

- `fn get_mut(self: &mut Self) -> &mut W`
  Acquires a mutable reference to the underlying writer.

- `fn try_finish(self: &mut Self) -> io::Result<()>`
  Attempt to finish this output stream, writing out final chunks of data.

- `fn finish(self: Self) -> io::Result<W>`
  Consumes this decoder, flushing the output stream.

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

##### `impl Write<W: Write>`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

##### `impl Debug<W: $crate::fmt::Debug + Write>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ZlibDecoder<W: Write>`

```rust
struct ZlibDecoder<W: Write> {
    // [REDACTED: Private Fields]
}
```

A ZLIB decoder, or decompressor.

This structure implements a [`Write`](#write) and will emit a stream of decompressed
data when fed a stream of compressed data.

After decoding a single member of the ZLIB data this writer will return the number of bytes up
to the end of the ZLIB member and subsequent writes will return Ok(0) allowing the caller to
handle any data following the ZLIB member.

# Examples

```rust
use std::io::prelude::*;
use std::io;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::write::ZlibDecoder;

fn main() {
   let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
   e.write_all(b"Hello World").unwrap();
   let bytes = e.finish().unwrap();
   println!("{}", decode_reader(bytes).unwrap());
}

// Uncompresses a Zlib Encoded vector of bytes and returns a string or error
// Here Vec<u8> implements Write

fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
   let mut writer = Vec::new();
   let mut z = ZlibDecoder::new(writer);
   z.write_all(&bytes[..])?;
   writer = z.finish()?;
   let return_string = String::from_utf8(writer).expect("String parsing error");
   Ok(return_string)
}
```

#### Implementations

- `fn new(w: W) -> ZlibDecoder<W>`
  Creates a new decoder which will write uncompressed data to the stream.

- `fn new_with_decompress(w: W, decompression: Decompress) -> ZlibDecoder<W>`
  Creates a new decoder which will write uncompressed data to the stream `w`

- `fn get_ref(self: &Self) -> &W`
  Acquires a reference to the underlying writer.

- `fn get_mut(self: &mut Self) -> &mut W`
  Acquires a mutable reference to the underlying writer.

- `fn reset(self: &mut Self, w: W) -> io::Result<W>`
  Resets the state of this decoder entirely, swapping out the output

- `fn try_finish(self: &mut Self) -> io::Result<()>`
  Attempt to finish this output stream, writing out final chunks of data.

- `fn finish(self: Self) -> io::Result<W>`
  Consumes this encoder, flushing the output stream.

- `fn total_in(self: &Self) -> u64`
  Returns the number of bytes that the decompressor has consumed for

- `fn total_out(self: &Self) -> u64`
  Returns the number of bytes that the decompressor has written to its

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

##### `impl Read<W: Read + Write>`

- `fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write<W: Write>`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

##### `impl Debug<W: $crate::fmt::Debug + Write>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ZlibEncoder<W: Write>`

```rust
struct ZlibEncoder<W: Write> {
    // [REDACTED: Private Fields]
}
```

A ZLIB encoder, or compressor.

This structure implements a [`Write`](#write) interface and takes a stream of
uncompressed data, writing the compressed data to the wrapped writer.

# Examples

```rust
use std::io::prelude::*;
use flate2::Compression;
use flate2::write::ZlibEncoder;

// Vec<u8> implements Write, assigning the compressed bytes of sample string

fn zlib_encoding() -> std::io::Result<()> {
let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
e.write_all(b"Hello World")?;
let compressed = e.finish()?;
Ok(())
}
```

#### Implementations

- `fn new(w: W, level: crate::Compression) -> ZlibEncoder<W>`
  Creates a new encoder which will write compressed data to the stream

- `fn new_with_compress(w: W, compression: Compress) -> ZlibEncoder<W>`
  Creates a new encoder which will write compressed data to the stream

- `fn get_ref(self: &Self) -> &W`
  Acquires a reference to the underlying writer.

- `fn get_mut(self: &mut Self) -> &mut W`
  Acquires a mutable reference to the underlying writer.

- `fn reset(self: &mut Self, w: W) -> io::Result<W>`
  Resets the state of this encoder entirely, swapping out the output

- `fn try_finish(self: &mut Self) -> io::Result<()>`
  Attempt to finish this output stream, writing out final chunks of data.

- `fn finish(self: Self) -> io::Result<W>`
  Consumes this encoder, flushing the output stream.

- `fn flush_finish(self: Self) -> io::Result<W>`
  Consumes this encoder, flushing the output stream.

- `fn total_in(self: &Self) -> u64`
  Returns the number of bytes that have been written to this compressor.

- `fn total_out(self: &Self) -> u64`
  Returns the number of bytes that the compressor has produced.

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

##### `impl Read<W: Read + Write>`

- `fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write<W: Write>`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

##### `impl Debug<W: $crate::fmt::Debug + Write>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

