# Crate `flate2`

A DEFLATE-based stream compression/decompression library

This library provides support for compression and decompression of
DEFLATE-based streams:

* the DEFLATE format itself
* the zlib format
* gzip

These three formats are all closely related and largely only differ in their
headers/footers. This crate has three types in each submodule for dealing
with these three formats.

# Implementation

In addition to supporting three formats, this crate supports several different
backends, controlled through this crate's features:

* `default`, or `rust_backend` - this implementation uses the `miniz_oxide`
  crate which is a port of `miniz.c` to Rust. This feature does not
  require a C compiler, and only uses safe Rust code.

* `zlib-rs` - this implementation utilizes the `zlib-rs` crate, a Rust rewrite of zlib.
  This backend is the fastest, at the cost of some `unsafe` Rust code.

Several backends implemented in C are also available.
These are useful in case you are already using a specific C implementation
and need the result of compression to be bit-identical.
See the crate's README for details on the available C backends.

The `zlib-rs` backend typically outperforms all the C implementations.

# Organization

This crate consists mainly of three modules, [`read`](flate2/read/index.md), [`write`](flate2/write/index.md), and
[`bufread`](flate2/bufread/index.md). Each module contains a number of types used to encode and
decode various streams of data.

All types in the [`write`](flate2/write/index.md) module work on instances of [`Write`][write](#write),
whereas all types in the [`read`](flate2/read/index.md) module work on instances of
[`Read`][read](#read) and [`bufread`](flate2/bufread/index.md) works with [`BufRead`][bufread](#bufread). If you
are decoding directly from a `&[u8](#u8)`, use the [`bufread`](flate2/bufread/index.md) types.

```
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io;
use std::io::prelude::*;

# fn main() { let _ = run(); }
# fn run() -> io::Result<()> {
let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
encoder.write_all(b"Example")?;
# Ok(())
# }
```


Other various types are provided at the top-level of the crate for
management and dealing with encoders/decoders. Also note that types which
operate over a specific trait often implement the mirroring trait as well.
For example a `flate2::read::DeflateDecoder<T>` *also* implements the
`Write` trait if `T: Write`. That is, the "dual trait" is forwarded directly
to the underlying object if available.

# About multi-member Gzip files

While most `gzip` files one encounters will have a single *member* that can be read
with the [`GzDecoder`](#gzdecoder), there may be some files which have multiple members.

A [`GzDecoder`](#gzdecoder) will only read the first member of gzip data, which may unexpectedly
provide partial results when a multi-member gzip file is encountered. `GzDecoder` is appropriate
for data that is designed to be read as single members from a multi-member file. `bufread::GzDecoder`
and `write::GzDecoder` also allow non-gzip data following gzip data to be handled.

The [`MultiGzDecoder`](#multigzdecoder) on the other hand will decode all members of a `gzip` file
into one consecutive stream of bytes, which hides the underlying *members* entirely.
If a file contains non-gzip data after the gzip data, MultiGzDecoder will
emit an error after decoding the gzip data. This behavior matches the `gzip`,
`gunzip`, and `zcat` command line tools.



[read](#read): https://doc.rust-lang.org/std/io/trait.Read.html
[write](#write): https://doc.rust-lang.org/std/io/trait.Write.html
[bufread](#bufread): https://doc.rust-lang.org/std/io/trait.BufRead.html



## Modules

- [`read`](read/index.md) - Types which operate over [`Read`] streams, both encoders and decoders for
- [`write`](write/index.md) - Types which operate over [`Write`] streams, both encoders and decoders for
- [`bufread`](bufread/index.md) - Types which operate over [`BufRead`] streams, both encoders and decoders for

## Structs

### `Crc`

```rust
struct Crc {
}
```

The CRC calculated by a [`CrcReader`](#crcreader).


#### Implementations

- `fn new() -> Self`
  Create a new CRC.

- `fn sum(self: &Self) -> u32`
  Returns the current crc32 checksum.

- `fn amount(self: &Self) -> u32`
  The number of bytes that have been used to calculate the CRC.

- `fn update(self: &mut Self, data: &[u8])`
  Update the CRC with the bytes in `data`.

- `fn reset(self: &mut Self)`
  Reset the CRC.

- `fn combine(self: &mut Self, additional_crc: &Crc)`
  Combine the CRC with the CRC for the subsequent block of bytes.

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

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Crc`

### `CrcReader<R>`

```rust
struct CrcReader<R> {
}
```

A wrapper around a [`Read`](#read) that calculates the CRC.


#### Implementations

- `fn new(r: R) -> CrcReader<R>`
  Create a new `CrcReader`.

- `fn crc(self: &Self) -> &Crc`
  Get the Crc for this `CrcReader`.

- `fn into_inner(self: Self) -> R`
  Get the reader that is wrapped by this `CrcReader`.

- `fn get_ref(self: &Self) -> &R`
  Get the reader that is wrapped by this `CrcReader` by reference.

- `fn get_mut(self: &mut Self) -> &mut R`
  Get a mutable reference to the reader that is wrapped by this `CrcReader`.

- `fn reset(self: &mut Self)`
  Reset the Crc in this `CrcReader`.

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

##### `impl BufRead<R: BufRead>`

- `fn fill_buf(self: &mut Self) -> io::Result<&[u8]>`

- `fn consume(self: &mut Self, amt: usize)`

##### `impl Read<R: Read>`

- `fn read(self: &mut Self, into: &mut [u8]) -> io::Result<usize>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<R: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `CrcWriter<W>`

```rust
struct CrcWriter<W> {
}
```

A wrapper around a [`Write`](#write) that calculates the CRC.


#### Implementations

- `fn crc(self: &Self) -> &Crc`
  Get the Crc for this `CrcWriter`.

- `fn into_inner(self: Self) -> W`
  Get the writer that is wrapped by this `CrcWriter`.

- `fn get_ref(self: &Self) -> &W`
  Get the writer that is wrapped by this `CrcWriter` by reference.

- `fn get_mut(self: &mut Self) -> &mut W`
  Get a mutable reference to the writer that is wrapped by this `CrcWriter`.

- `fn reset(self: &mut Self)`
  Reset the Crc in this `CrcWriter`.

- `fn new(w: W) -> CrcWriter<W>`
  Create a new `CrcWriter`.

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

##### `impl Debug<W: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `GzBuilder`

```rust
struct GzBuilder {
}
```

A builder structure to create a new gzip Encoder.

This structure controls header configuration options such as the filename.

# Examples

```
use std::io::prelude::*;
# use std::io;
use std::fs::File;
use flate2::GzBuilder;
use flate2::Compression;

// GzBuilder opens a file and writes a sample string using GzBuilder pattern

# fn sample_builder() -> Result<(), io::Error> {
let f = File::create("examples/hello_world.gz")?;
let mut gz = GzBuilder::new()
                .filename("hello_world.txt")
                .comment("test file, please delete")
                .write(f, Compression::default());
gz.write_all(b"hello world")?;
gz.finish()?;
# Ok(())
# }
```

#### Implementations

- `fn new() -> GzBuilder`
  Create a new blank builder with no header by default.

- `fn mtime(self: Self, mtime: u32) -> GzBuilder`
  Configure the `mtime` field in the gzip header.

- `fn operating_system(self: Self, os: u8) -> GzBuilder`
  Configure the `operating_system` field in the gzip header.

- `fn extra<T: Into<Vec<u8>>>(self: Self, extra: T) -> GzBuilder`
  Configure the `extra` field in the gzip header.

- `fn filename<T: Into<Vec<u8>>>(self: Self, filename: T) -> GzBuilder`
  Configure the `filename` field in the gzip header.

- `fn comment<T: Into<Vec<u8>>>(self: Self, comment: T) -> GzBuilder`
  Configure the `comment` field in the gzip header.

- `fn write<W: Write>(self: Self, w: W, lvl: Compression) -> write::GzEncoder<W>`
  Consume this builder, creating a writer encoder in the process.

- `fn read<R: Read>(self: Self, r: R, lvl: Compression) -> read::GzEncoder<R>`
  Consume this builder, creating a reader encoder in the process.

- `fn buf_read<R>(self: Self, r: R, lvl: Compression) -> bufread::GzEncoder<R>`
  Consume this builder, creating a reader encoder in the process.

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

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> GzBuilder`

### `GzHeader`

```rust
struct GzHeader {
}
```

A structure representing the header of a gzip stream.

The header can contain metadata about the file that was compressed, if
present.

#### Implementations

- `fn filename(self: &Self) -> Option<&[u8]>`
  Returns the `filename` field of this gzip stream's header, if present.

- `fn extra(self: &Self) -> Option<&[u8]>`
  Returns the `extra` field of this gzip stream's header, if present.

- `fn comment(self: &Self) -> Option<&[u8]>`
  Returns the `comment` field of this gzip stream's header, if present.

- `fn operating_system(self: &Self) -> u8`
  Returns the `operating_system` field of this gzip stream's header.

- `fn mtime(self: &Self) -> u32`
  This gives the most recent modification time of the original file being compressed.

- `fn mtime_as_datetime(self: &Self) -> Option<time::SystemTime>`
  Returns the most recent modification time represented by a date-time type.

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

##### `impl Clone`

- `fn clone(self: &Self) -> GzHeader`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &GzHeader) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> GzHeader`

### `Compress`

```rust
struct Compress {
}
```

Raw in-memory compression stream for blocks of data.

This type is the building block for the I/O streams in the rest of this
crate. It requires more management than the [`Read`](#read)/[`Write`](#write) API but is
maximally flexible in terms of accepting input from any source and being
able to produce output to any memory location.

It is recommended to use the I/O stream adaptors over this type as they're
easier to use.



#### Implementations

- `fn new(level: Compression, zlib_header: bool) -> Compress`
  Creates a new object ready for compressing data that it's given.

- `fn total_in(self: &Self) -> u64`
  Returns the total number of input bytes which have been processed by

- `fn total_out(self: &Self) -> u64`
  Returns the total number of output bytes which have been produced by

- `fn reset(self: &mut Self)`
  Quickly resets this compressor without having to reallocate anything.

- `fn compress(self: &mut Self, input: &[u8], output: &mut [u8], flush: FlushCompress) -> Result<Status, CompressError>`
  Compresses the input data into the output, consuming only as much

- `fn compress_uninit(self: &mut Self, input: &[u8], output: &mut [MaybeUninit<u8>], flush: FlushCompress) -> Result<Status, CompressError>`
  Similar to [`Self::compress`] but accepts uninitialized buffer.

- `fn compress_vec(self: &mut Self, input: &[u8], output: &mut Vec<u8>, flush: FlushCompress) -> Result<Status, CompressError>`
  Compresses the input data into the extra space of the output, consuming

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

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `CompressError`

```rust
struct CompressError {
}
```

Error returned when a compression object is used incorrectly or otherwise
generates an error.

#### Implementations

- `fn message(self: &Self) -> Option<&str>`
  Retrieve the implementation's message about why the operation failed, if one exists.

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

##### `impl Clone`

- `fn clone(self: &Self) -> CompressError`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Decompress`

```rust
struct Decompress {
}
```

Raw in-memory decompression stream for blocks of data.

This type is the building block for the I/O streams in the rest of this
crate. It requires more management than the [`Read`](#read)/[`Write`](#write) API but is
maximally flexible in terms of accepting input from any source and being
able to produce output to any memory location.

It is recommended to use the I/O stream adaptors over this type as they're
easier to use.



#### Implementations

- `fn new(zlib_header: bool) -> Decompress`
  Creates a new object ready for decompressing data that it's given.

- `fn total_in(self: &Self) -> u64`
  Returns the total number of input bytes which have been processed by

- `fn total_out(self: &Self) -> u64`
  Returns the total number of output bytes which have been produced by

- `fn decompress(self: &mut Self, input: &[u8], output: &mut [u8], flush: FlushDecompress) -> Result<Status, DecompressError>`
  Decompresses the input data into the output, consuming only as much

- `fn decompress_uninit(self: &mut Self, input: &[u8], output: &mut [MaybeUninit<u8>], flush: FlushDecompress) -> Result<Status, DecompressError>`
  Similar to [`Self::decompress`] but accepts uninitialized buffer

- `fn decompress_vec(self: &mut Self, input: &[u8], output: &mut Vec<u8>, flush: FlushDecompress) -> Result<Status, DecompressError>`
  Decompresses the input data into the extra space in the output vector

- `fn reset(self: &mut Self, zlib_header: bool)`
  Performs the equivalent of replacing this decompression state with a

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

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `DecompressError`

```rust
struct DecompressError();
```

Error returned when a decompression object finds that the input stream of
bytes was not a valid input stream of bytes.

#### Implementations

- `fn needs_dictionary(self: &Self) -> Option<u32>`
  Indicates whether decompression failed due to requiring a dictionary.

- `fn message(self: &Self) -> Option<&str>`
  Retrieve the implementation's message about why the operation failed, if one exists.

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

##### `impl Clone`

- `fn clone(self: &Self) -> DecompressError`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Compression`

```rust
struct Compression();
```

When compressing data, the compression level can be specified by a value in
this struct.

#### Implementations

- `const fn new(level: u32) -> Compression`
  Creates a new description of the compression level with an explicitly

- `const fn none() -> Compression`
  No compression is to be performed, this may actually inflate data

- `const fn fast() -> Compression`
  Optimize for the best speed of encoding.

- `const fn best() -> Compression`
  Optimize for the size of data being encoded.

- `fn level(self: &Self) -> u32`
  Returns an integer representing the compression level, typically on a

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

##### `impl Clone`

- `fn clone(self: &Self) -> Compression`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Compression) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Compression`

## Enums

### `Status`

```rust
enum Status {
    Ok,
    BufError,
    StreamEnd,
}
```

Possible status results of compressing some data or successfully
decompressing a block of data.

#### Variants

- **`Ok`**

  Indicates success.
  
  Means that more input may be needed but isn't available
  and/or there's more output to be written but the output buffer is full.

- **`BufError`**

  Indicates that forward progress is not possible due to input or output
  buffers being empty.
  
  For compression it means the input buffer needs some more data or the
  output buffer needs to be freed up before trying again.
  
  For decompression this means that more input is needed to continue or
  the output buffer isn't large enough to contain the result. The function
  can be called again after fixing both.

- **`StreamEnd`**

  Indicates that all input has been consumed and all output bytes have
  been written. Decompression/compression should not be called again.
  
  For decompression with zlib streams the adler-32 of the decompressed
  data has also been verified.

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

##### `impl Clone`

- `fn clone(self: &Self) -> Status`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Status) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `FlushCompress`

```rust
enum FlushCompress {
    None,
    Partial,
    Sync,
    Full,
    Finish,
}
```

Values which indicate the form of flushing to be used when compressing
in-memory data.

#### Variants

- **`None`**

  A typical parameter for passing to compression/decompression functions,
  this indicates that the underlying stream to decide how much data to
  accumulate before producing output in order to maximize compression.

- **`Partial`**

  All pending output is flushed to the output buffer, but the output is
  not aligned to a byte boundary.
  
  All input data so far will be available to the decompressor (as with
  `Flush::Sync`). This completes the current deflate block and follows it
  with an empty fixed codes block that is 10 bits long, and it assures
  that enough bytes are output in order for the decompressor to finish the
  block before the empty fixed code block.

- **`Sync`**

  All pending output is flushed to the output buffer and the output is
  aligned on a byte boundary so that the decompressor can get all input
  data available so far.
  
  Flushing may degrade compression for some compression algorithms and so
  it should only be used when necessary. This will complete the current
  deflate block and follow it with an empty stored block.

- **`Full`**

  All output is flushed as with `Flush::Sync` and the compression state is
  reset so decompression can restart from this point if previous
  compressed data has been damaged or if random access is desired.
  
  Using this option too often can seriously degrade compression.

- **`Finish`**

  Pending input is processed and pending output is flushed.
  
  The return value may indicate that the stream is not yet done and more
  data has yet to be processed.

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

##### `impl Clone`

- `fn clone(self: &Self) -> FlushCompress`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &FlushCompress) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `FlushDecompress`

```rust
enum FlushDecompress {
    None,
    Sync,
    Finish,
}
```

Values which indicate the form of flushing to be used when
decompressing in-memory data.

#### Variants

- **`None`**

  A typical parameter for passing to compression/decompression functions,
  this indicates that the underlying stream to decide how much data to
  accumulate before producing output in order to maximize compression.

- **`Sync`**

  All pending output is flushed to the output buffer and the output is
  aligned on a byte boundary so that the decompressor can get all input
  data available so far.
  
  Flushing may degrade compression for some compression algorithms and so
  it should only be used when necessary. This will complete the current
  deflate block and follow it with an empty stored block.

- **`Finish`**

  Pending input is processed and pending output is flushed.
  
  The return value may indicate that the stream is not yet done and more
  data has yet to be processed.

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

##### `impl Clone`

- `fn clone(self: &Self) -> FlushDecompress`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &FlushDecompress) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

