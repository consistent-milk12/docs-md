*[flate2](../index.md) / [bufread](index.md)*

---

# Module `bufread`

Types which operate over [`BufRead`](#bufread) streams, both encoders and decoders for
various formats.


## Structs

### `DeflateDecoder<R>`

```rust
struct DeflateDecoder<R> {
    // [REDACTED: Private Fields]
}
```

A DEFLATE decoder, or decompressor.

This structure implements a [`Read`](#read) interface. When read from, it reads
compressed data from the underlying [`BufRead`](#bufread) and provides the uncompressed data.

After reading a single member of the DEFLATE data this reader will return
Ok(0) even if there are more bytes available in the underlying reader.
If you need the following bytes, call `into_inner()` after Ok(0) to
recover the underlying reader.


# Examples

```
use std::io::prelude::*;
use std::io;
# use flate2::Compression;
# use flate2::write::DeflateEncoder;
use flate2::bufread::DeflateDecoder;

# fn main() {
#    let mut e = DeflateEncoder::new(Vec::new(), Compression::default());
#    e.write_all(b"Hello World").unwrap();
#    let bytes = e.finish().unwrap();
#    println!("{}", decode_reader(bytes).unwrap());
# }
// Uncompresses a Deflate Encoded vector of bytes and returns a string or error
// Here &[u8](#u8)
 implements Read
fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
   let mut deflater = DeflateDecoder::new(&bytes[..]);
   let mut s = String::new();
   deflater.read_to_string(&mut s)?;
   Ok(s)
}
```

#### Implementations

- `fn new(r: R) -> DeflateDecoder<R>`
  Creates a new decoder which will decompress data read from the given

- `fn reset(self: &mut Self, r: R) -> R`
  Resets the state of this decoder entirely, swapping out the input

- `fn reset_data(self: &mut Self)`
  Resets the state of this decoder's data

- `fn get_ref(self: &Self) -> &R`
  Acquires a reference to the underlying stream

- `fn get_mut(self: &mut Self) -> &mut R`
  Acquires a mutable reference to the underlying stream

- `fn into_inner(self: Self) -> R`
  Consumes this decoder, returning the underlying reader.

- `fn total_in(self: &Self) -> u64`
  Returns the number of bytes that the decompressor has consumed.

- `fn total_out(self: &Self) -> u64`
  Returns the number of bytes that the decompressor has produced.

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

##### `impl Read<R: BufRead>`

- `fn read(self: &mut Self, into: &mut [u8]) -> io::Result<usize>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write<W: BufRead + Write>`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

##### `impl Debug<R: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `DeflateEncoder<R>`

```rust
struct DeflateEncoder<R> {
    // [REDACTED: Private Fields]
}
```

A DEFLATE encoder, or compressor.

This structure implements a [`Read`](#read) interface. When read from, it reads
uncompressed data from the underlying [`BufRead`](#bufread) and provides the compressed data.


# Examples

```
use std::io::prelude::*;
use std::io;
use flate2::Compression;
use flate2::bufread::DeflateEncoder;
use std::fs::File;
use std::io::BufReader;

# fn main() {
#    println!("{:?}", open_hello_world().unwrap());
# }
#
// Opens sample file, compresses the contents and returns a Vector
fn open_hello_world() -> io::Result<Vec<u8>> {
   let f = File::open("examples/hello_world.txt")?;
   let b = BufReader::new(f);
   let mut deflater = DeflateEncoder::new(b, Compression::fast());
   let mut buffer = Vec::new();
   deflater.read_to_end(&mut buffer)?;
   Ok(buffer)
}
```

#### Implementations

- `fn reset(self: &mut Self, r: R) -> R`
  Resets the state of this encoder entirely, swapping out the input

- `fn get_ref(self: &Self) -> &R`
  Acquires a reference to the underlying reader

- `fn get_mut(self: &mut Self) -> &mut R`
  Acquires a mutable reference to the underlying stream

- `fn into_inner(self: Self) -> R`
  Consumes this encoder, returning the underlying reader.

- `fn total_in(self: &Self) -> u64`
  Returns the number of bytes that have been read into this compressor.

- `fn total_out(self: &Self) -> u64`
  Returns the number of bytes that the compressor has produced.

- `fn new(r: R, level: crate::Compression) -> DeflateEncoder<R>`
  Creates a new encoder which will read uncompressed data from the given

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

##### `impl Read<R: BufRead>`

- `fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write<W: BufRead + Write>`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

##### `impl Debug<R: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `GzDecoder<R>`

```rust
struct GzDecoder<R> {
    // [REDACTED: Private Fields]
}
```

A decoder for a single member of a [gzip file].

This structure implements a [`Read`](#read) interface. When read from, it reads
compressed data from the underlying [`BufRead`](#bufread) and provides the uncompressed data.

After reading a single member of the gzip data this reader will return
Ok(0) even if there are more bytes available in the underlying reader.
If you need the following bytes, call `into_inner()` after Ok(0) to
recover the underlying reader.

To handle gzip files that may have multiple members, see [`MultiGzDecoder`](#multigzdecoder)
or read more
[in the introduction](../index.html#about-multi-member-gzip-files).

[gzip file]: https://www.rfc-editor.org/rfc/rfc1952#page-5


# Examples

```
use std::io::prelude::*;
use std::io;
# use flate2::Compression;
# use flate2::write::GzEncoder;
use flate2::bufread::GzDecoder;

# fn main() {
#   let mut e = GzEncoder::new(Vec::new(), Compression::default());
#   e.write_all(b"Hello World").unwrap();
#   let bytes = e.finish().unwrap();
#   println!("{}", decode_reader(bytes).unwrap());
# }
#
// Uncompresses a Gz Encoded vector of bytes and returns a string or error
// Here &[u8](#u8)
 implements BufRead

fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
   let mut gz = GzDecoder::new(&bytes[..]);
   let mut s = String::new();
   gz.read_to_string(&mut s)?;
   Ok(s)
}
```

#### Implementations

- `fn new(r: R) -> GzDecoder<R>`
  Creates a new decoder from the given reader, immediately parsing the

- `fn header(self: &Self) -> Option<&GzHeader>`
  Returns the header associated with this stream, if it was valid

- `fn get_ref(self: &Self) -> &R`
  Acquires a reference to the underlying reader.

- `fn get_mut(self: &mut Self) -> &mut R`
  Acquires a mutable reference to the underlying stream.

- `fn into_inner(self: Self) -> R`
  Consumes this decoder, returning the underlying reader.

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

##### `impl Read<R: BufRead>`

- `fn read(self: &mut Self, into: &mut [u8]) -> io::Result<usize>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write<R: BufRead + Write>`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

##### `impl Debug<R: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `GzEncoder<R>`

```rust
struct GzEncoder<R> {
    // [REDACTED: Private Fields]
}
```

A gzip streaming encoder

This structure implements a [`Read`](#read) interface. When read from, it reads
uncompressed data from the underlying [`BufRead`](#bufread) and provides the compressed data.


# Examples

```
use std::io::prelude::*;
use std::io;
use flate2::Compression;
use flate2::bufread::GzEncoder;
use std::fs::File;
use std::io::BufReader;

// Opens sample file, compresses the contents and returns a Vector or error
// File wrapped in a BufReader implements BufRead

fn open_hello_world() -> io::Result<Vec<u8>> {
    let f = File::open("examples/hello_world.txt")?;
    let b = BufReader::new(f);
    let mut gz = GzEncoder::new(b, Compression::fast());
    let mut buffer = Vec::new();
    gz.read_to_end(&mut buffer)?;
    Ok(buffer)
}
```

#### Implementations

- `fn new(r: R, level: Compression) -> GzEncoder<R>`
  Creates a new encoder which will use the given compression level.

- `fn get_ref(self: &Self) -> &R`
  Acquires a reference to the underlying reader.

- `fn get_mut(self: &mut Self) -> &mut R`
  Acquires a mutable reference to the underlying reader.

- `fn into_inner(self: Self) -> R`
  Returns the underlying stream, consuming this encoder

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

##### `impl Read<R: BufRead>`

- `fn read(self: &mut Self, into: &mut [u8]) -> io::Result<usize>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write<R: BufRead + Write>`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

##### `impl Debug<R: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `MultiGzDecoder<R>`

```rust
struct MultiGzDecoder<R>();
```

A gzip streaming decoder that decodes a [gzip file] that may have multiple members.

This structure implements a [`Read`](#read) interface. When read from, it reads
compressed data from the underlying [`BufRead`](#bufread) and provides the uncompressed data.

A gzip file consists of a series of *members* concatenated one after another.
MultiGzDecoder decodes all members from the data and only returns Ok(0) when the
underlying reader does. For a file, this reads to the end of the file.

To handle members separately, see [GzDecoder] or read more
[in the introduction](../index.html#about-multi-member-gzip-files).

[gzip file]: https://www.rfc-editor.org/rfc/rfc1952#page-5


# Examples

```
use std::io::prelude::*;
use std::io;
# use flate2::Compression;
# use flate2::write::GzEncoder;
use flate2::bufread::MultiGzDecoder;

# fn main() {
#   let mut e = GzEncoder::new(Vec::new(), Compression::default());
#   e.write_all(b"Hello World").unwrap();
#   let bytes = e.finish().unwrap();
#   println!("{}", decode_reader(bytes).unwrap());
# }
#
// Uncompresses a Gz Encoded vector of bytes and returns a string or error
// Here &[u8](#u8)
 implements BufRead

fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
   let mut gz = MultiGzDecoder::new(&bytes[..]);
   let mut s = String::new();
   gz.read_to_string(&mut s)?;
   Ok(s)
}
```

#### Implementations

- `fn new(r: R) -> MultiGzDecoder<R>`
  Creates a new decoder from the given reader, immediately parsing the

- `fn header(self: &Self) -> Option<&GzHeader>`
  Returns the current header associated with this stream, if it's valid

- `fn get_ref(self: &Self) -> &R`
  Acquires a reference to the underlying reader.

- `fn get_mut(self: &mut Self) -> &mut R`
  Acquires a mutable reference to the underlying stream.

- `fn into_inner(self: Self) -> R`
  Consumes this decoder, returning the underlying reader.

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

##### `impl Read<R: BufRead>`

- `fn read(self: &mut Self, into: &mut [u8]) -> io::Result<usize>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<R: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ZlibDecoder<R>`

```rust
struct ZlibDecoder<R> {
    // [REDACTED: Private Fields]
}
```

A ZLIB decoder, or decompressor.

This structure implements a [`Read`](#read) interface. When read from, it reads
compressed data from the underlying [`BufRead`](#bufread) and provides the uncompressed data.

After reading a single member of the ZLIB data this reader will return
Ok(0) even if there are more bytes available in the underlying reader.
If you need the following bytes, call `into_inner()` after Ok(0) to
recover the underlying reader.


# Examples

```
use std::io::prelude::*;
use std::io;
# use flate2::Compression;
# use flate2::write::ZlibEncoder;
use flate2::bufread::ZlibDecoder;

# fn main() {
# let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
# e.write_all(b"Hello World").unwrap();
# let bytes = e.finish().unwrap();
# println!("{}", decode_bufreader(bytes).unwrap());
# }
#
// Uncompresses a Zlib Encoded vector of bytes and returns a string or error
// Here &[u8](#u8)
 implements BufRead

fn decode_bufreader(bytes: Vec<u8>) -> io::Result<String> {
    let mut z = ZlibDecoder::new(&bytes[..]);
    let mut s = String::new();
    z.read_to_string(&mut s)?;
    Ok(s)
}
```

#### Implementations

- `fn reset(self: &mut Self, r: R) -> R`
  Resets the state of this decoder entirely, swapping out the input

- `fn get_ref(self: &Self) -> &R`
  Acquires a reference to the underlying stream

- `fn get_mut(self: &mut Self) -> &mut R`
  Acquires a mutable reference to the underlying stream

- `fn into_inner(self: Self) -> R`
  Consumes this decoder, returning the underlying reader.

- `fn total_in(self: &Self) -> u64`
  Returns the number of bytes that the decompressor has consumed.

- `fn total_out(self: &Self) -> u64`
  Returns the number of bytes that the decompressor has produced.

- `fn new(r: R) -> ZlibDecoder<R>`
  Creates a new decoder which will decompress data read from the given

- `fn new_with_decompress(r: R, decompression: Decompress) -> ZlibDecoder<R>`
  Creates a new decoder which will decompress data read from the given

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

##### `impl Read<R: BufRead>`

- `fn read(self: &mut Self, into: &mut [u8]) -> io::Result<usize>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write<R: BufRead + Write>`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

##### `impl Debug<R: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ZlibEncoder<R>`

```rust
struct ZlibEncoder<R> {
    // [REDACTED: Private Fields]
}
```

A ZLIB encoder, or compressor.

This structure implements a [`Read`](#read) interface. When read from, it reads
uncompressed data from the underlying [`BufRead`](#bufread) and provides the compressed data.


# Examples

```
use std::io::prelude::*;
use flate2::Compression;
use flate2::bufread::ZlibEncoder;
use std::fs::File;
use std::io::BufReader;

// Use a buffered file to compress contents into a Vec<u8>

# fn open_hello_world() -> std::io::Result<Vec<u8>> {
let f = File::open("examples/hello_world.txt")?;
let b = BufReader::new(f);
let mut z = ZlibEncoder::new(b, Compression::fast());
let mut buffer = Vec::new();
z.read_to_end(&mut buffer)?;
# Ok(buffer)
# }
```

#### Implementations

- `fn reset(self: &mut Self, r: R) -> R`
  Resets the state of this encoder entirely, swapping out the input

- `fn get_ref(self: &Self) -> &R`
  Acquires a reference to the underlying reader

- `fn get_mut(self: &mut Self) -> &mut R`
  Acquires a mutable reference to the underlying stream

- `fn into_inner(self: Self) -> R`
  Consumes this encoder, returning the underlying reader.

- `fn total_in(self: &Self) -> u64`
  Returns the number of bytes that have been read into this compressor.

- `fn total_out(self: &Self) -> u64`
  Returns the number of bytes that the compressor has produced.

- `fn new(r: R, level: crate::Compression) -> ZlibEncoder<R>`
  Creates a new encoder which will read uncompressed data from the given

- `fn new_with_compress(r: R, compression: Compress) -> ZlibEncoder<R>`
  Creates a new encoder with the given `compression` settings which will

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

##### `impl Read<R: BufRead>`

- `fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write<R: BufRead + Write>`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

##### `impl Debug<R: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

