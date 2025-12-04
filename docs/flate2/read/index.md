*[flate2](../index.md) / [read](index.md)*

---

# Module `read`

Types which operate over [`Read`](#read) streams, both encoders and decoders for
various formats.

Note that the `read` decoder types may read past the end of the compressed
data while decoding. If the caller requires subsequent reads to start
immediately following the compressed data  wrap the `Read` type in a
[`BufReader`](#bufreader) and use the `BufReader` with the equivalent decoder from the
`bufread` module and also for the subsequent reads.



## Structs

### `DeflateDecoder<R>`

```rust
struct DeflateDecoder<R> {
}
```

A DEFLATE decoder, or decompressor.

This structure implements a [`Read`](#read) interface. When read from, it reads
compressed data from the underlying [`Read`](#read) and provides the uncompressed data.

After reading a single member of the DEFLATE data this reader will return
Ok(0) even if there are more bytes available in the underlying reader.
`DeflateDecoder` may have read additional bytes past the end of the DEFLATE data.
If you need the following bytes, wrap the `Reader` in a `std::io::BufReader`
and use `bufread::DeflateDecoder` instead.

# Examples

```
use std::io::prelude::*;
use std::io;
# use flate2::Compression;
# use flate2::write::DeflateEncoder;
use flate2::read::DeflateDecoder;

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

- `fn new(r: R) -> DeflateDecoder<R>`
  Creates a new decoder which will decompress data read from the given

- `fn new_with_buf(r: R, buf: Vec<u8>) -> DeflateDecoder<R>`
  Same as `new`, but the intermediate buffer for data is specified.

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

##### `impl Read<R: Read>`

- `fn read(self: &mut Self, into: &mut [u8]) -> io::Result<usize>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write<W: Read + Write>`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

##### `impl Debug<R: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `DeflateEncoder<R>`

```rust
struct DeflateEncoder<R> {
}
```

A DEFLATE encoder, or compressor.

This structure implements a [`Read`](#read) interface. When read from, it reads
uncompressed data from the underlying [`Read`](#read) and provides the compressed data.

# Examples

```
use std::io::prelude::*;
use std::io;
use flate2::Compression;
use flate2::read::DeflateEncoder;

# fn main() {
#    println!("{:?}", deflateencoder_read_hello_world().unwrap());
# }
#
// Return a vector containing the Deflate compressed version of hello world
fn deflateencoder_read_hello_world() -> io::Result<Vec<u8>> {
   let mut ret_vec = Vec::new();
   let c = b"hello world";
   let mut deflater = DeflateEncoder::new(&c[..], Compression::fast());
   deflater.read_to_end(&mut ret_vec)?;
   Ok(ret_vec)
}
```

#### Implementations

- `fn new(r: R, level: crate::Compression) -> DeflateEncoder<R>`
  Creates a new encoder which will read uncompressed data from the given

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

##### `impl Read<R: Read>`

- `fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write<W: Read + Write>`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

##### `impl Debug<R: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `GzDecoder<R>`

```rust
struct GzDecoder<R> {
}
```

A decoder for a single member of a [gzip file].

This structure implements a [`Read`](#read) interface. When read from, it reads
compressed data from the underlying [`Read`](#read) and provides the uncompressed data.

After reading a single member of the gzip data this reader will return
Ok(0) even if there are more bytes available in the underlying reader.
`GzDecoder` may have read additional bytes past the end of the gzip data.
If you need the following bytes, wrap the `Reader` in a `std::io::BufReader`
and use `bufread::GzDecoder` instead.

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
use flate2::read::GzDecoder;

# fn main() {
#    let mut e = GzEncoder::new(Vec::new(), Compression::default());
#    e.write_all(b"Hello World").unwrap();
#    let bytes = e.finish().unwrap();
#    println!("{}", decode_reader(bytes).unwrap());
# }
#
// Uncompresses a Gz Encoded vector of bytes and returns a string or error
// Here &[u8](#u8)
 implements Read

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
  Returns the header associated with this stream, if it was valid.

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

##### `impl Read<R: Read>`

- `fn read(self: &mut Self, into: &mut [u8]) -> io::Result<usize>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write<R: Read + Write>`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

##### `impl Debug<R: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `GzEncoder<R>`

```rust
struct GzEncoder<R> {
}
```

A gzip streaming encoder

This structure implements a [`Read`](#read) interface. When read from, it reads
uncompressed data from the underlying [`Read`](#read) and provides the compressed data.

# Examples

```
use std::io::prelude::*;
use std::io;
use flate2::Compression;
use flate2::read::GzEncoder;

// Return a vector containing the GZ compressed version of hello world

fn gzencode_hello_world() -> io::Result<Vec<u8>> {
    let mut ret_vec = Vec::new();
    let bytestring = b"hello world";
    let mut gz = GzEncoder::new(&bytestring[..], Compression::fast());
    gz.read_to_end(&mut ret_vec)?;
    Ok(ret_vec)
}
```

#### Implementations

- `fn get_ref(self: &Self) -> &R`
  Acquires a reference to the underlying reader.

- `fn get_mut(self: &mut Self) -> &mut R`
  Acquires a mutable reference to the underlying reader.

- `fn into_inner(self: Self) -> R`
  Returns the underlying stream, consuming this encoder

- `fn new(r: R, level: Compression) -> GzEncoder<R>`
  Creates a new encoder which will use the given compression level.

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

##### `impl Read<R: Read>`

- `fn read(self: &mut Self, into: &mut [u8]) -> io::Result<usize>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write<R: Read + Write>`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

##### `impl Debug<R: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `MultiGzDecoder<R>`

```rust
struct MultiGzDecoder<R> {
}
```

A gzip streaming decoder that decodes a [gzip file] that may have multiple members.

This structure implements a [`Read`](#read) interface. When read from, it reads
compressed data from the underlying [`Read`](#read) and provides the uncompressed
data.

A gzip file consists of a series of *members* concatenated one after another.
MultiGzDecoder decodes all members of a file and returns Ok(0) once the
underlying reader does.

To handle members separately, see [GzDecoder] or read more
[in the introduction](../index.html#about-multi-member-gzip-files).

[gzip file]: https://www.rfc-editor.org/rfc/rfc1952#page-5

# Examples

```
use std::io::prelude::*;
use std::io;
# use flate2::Compression;
# use flate2::write::GzEncoder;
use flate2::read::MultiGzDecoder;

# fn main() {
#    let mut e = GzEncoder::new(Vec::new(), Compression::default());
#    e.write_all(b"Hello World").unwrap();
#    let bytes = e.finish().unwrap();
#    println!("{}", decode_reader(bytes).unwrap());
# }
#
// Uncompresses a Gz Encoded vector of bytes and returns a string or error
// Here &[u8](#u8)
 implements Read

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
  Returns the current header associated with this stream, if it's valid.

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

##### `impl Read<R: Read>`

- `fn read(self: &mut Self, into: &mut [u8]) -> io::Result<usize>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write<R: Read + Write>`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

##### `impl Debug<R: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ZlibDecoder<R>`

```rust
struct ZlibDecoder<R> {
}
```

A ZLIB decoder, or decompressor.

This structure implements a [`Read`](#read) interface. When read from, it reads
compressed data from the underlying [`Read`](#read) and provides the uncompressed data.

After reading a single member of the ZLIB data this reader will return
Ok(0) even if there are more bytes available in the underlying reader.
`ZlibDecoder` may have read additional bytes past the end of the ZLIB data.
If you need the following bytes, wrap the `Reader` in a `std::io::BufReader`
and use `bufread::ZlibDecoder` instead.

# Examples

```
use std::io::prelude::*;
use std::io;
# use flate2::Compression;
# use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;

# fn main() {
# let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
# e.write_all(b"Hello World").unwrap();
# let bytes = e.finish().unwrap();
# println!("{}", decode_reader(bytes).unwrap());
# }
#
// Uncompresses a Zlib Encoded vector of bytes and returns a string or error
// Here &[u8](#u8)
 implements Read

fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
    let mut z = ZlibDecoder::new(&bytes[..]);
    let mut s = String::new();
    z.read_to_string(&mut s)?;
    Ok(s)
}
```

#### Implementations

- `fn new(r: R) -> ZlibDecoder<R>`
  Creates a new decoder which will decompress data read from the given

- `fn new_with_buf(r: R, buf: Vec<u8>) -> ZlibDecoder<R>`
  Creates a new decoder which will decompress data read from the given

- `fn new_with_decompress(r: R, decompression: Decompress) -> ZlibDecoder<R>`
  Creates a new decoder which will decompress data read from the given

- `fn new_with_decompress_and_buf(r: R, buf: Vec<u8>, decompression: Decompress) -> ZlibDecoder<R>`
  Creates a new decoder which will decompress data read from the given

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

##### `impl Read<R: Read>`

- `fn read(self: &mut Self, into: &mut [u8]) -> io::Result<usize>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write<R: Read + Write>`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

##### `impl Debug<R: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ZlibEncoder<R>`

```rust
struct ZlibEncoder<R> {
}
```

A ZLIB encoder, or compressor.

This structure implements a [`Read`](#read) interface. When read from, it reads
uncompressed data from the underlying [`Read`](#read) and provides the compressed data.

# Examples

```
use std::io::prelude::*;
use flate2::Compression;
use flate2::read::ZlibEncoder;
use std::fs::File;

// Open example file and compress the contents using Read interface

# fn open_hello_world() -> std::io::Result<Vec<u8>> {
let f = File::open("examples/hello_world.txt")?;
let mut z = ZlibEncoder::new(f, Compression::fast());
let mut buffer = Vec::new();
z.read_to_end(&mut buffer)?;
# Ok(buffer)
# }
```

#### Implementations

- `fn new(r: R, level: crate::Compression) -> ZlibEncoder<R>`
  Creates a new encoder which will read uncompressed data from the given

- `fn new_with_compress(r: R, compression: crate::Compress) -> ZlibEncoder<R>`
  Creates a new encoder with the given `compression` settings which will

- `fn reset(self: &mut Self, r: R) -> R`
  Resets the state of this encoder entirely, swapping out the input

- `fn get_ref(self: &Self) -> &R`
  Acquires a reference to the underlying stream

- `fn get_mut(self: &mut Self) -> &mut R`
  Acquires a mutable reference to the underlying stream

- `fn into_inner(self: Self) -> R`
  Consumes this encoder, returning the underlying reader.

- `fn total_in(self: &Self) -> u64`
  Returns the number of bytes that have been read into this compressor.

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

##### `impl Read<R: Read>`

- `fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write<W: Read + Write>`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

##### `impl Debug<R: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

