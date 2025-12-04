# Crate `base64`

Correct, fast, and configurable [base64][] decoding and encoding. Base64
transports binary data efficiently in contexts where only plain text is
allowed.

[base64](#base64): https://developer.mozilla.org/en-US/docs/Glossary/Base64

# Usage

Use an [`Engine`](base64/engine/index.md) to decode or encode base64, configured with the base64
alphabet and padding behavior best suited to your application.

## Engine setup

There is more than one way to encode a stream of bytes as “base64”.
Different applications use different encoding
[alphabets](#alphabet) and
[padding behaviors](#generalpurposeconfig).

### Encoding alphabet

Almost all base64 [alphabets](#alphabet) use `A-Z`, `a-z`, and
`0-9`, which gives nearly 64 characters (26 + 26 + 10 = 62), but they differ
in their choice of their final 2.

Most applications use the [standard](#standard) alphabet specified
in [RFC 4648][rfc-alphabet].  If that’s all you need, you can get started
quickly by using the pre-configured
[`STANDARD`](#standard) engine, which is also available
in the [`prelude`](base64/prelude/index.md) module as shown here, if you prefer a minimal `use`
footprint.

```
use base64::prelude::*;

# fn main() -> Result<(), base64::DecodeError> {
assert_eq!(BASE64_STANDARD.decode(b"+uwgVQA=")?, b"\xFA\xEC\x20\x55\0");
assert_eq!(BASE64_STANDARD.encode(b"\xFF\xEC\x20\x55\0"), "/+wgVQA=");
# Ok(())
# }
```

[rfc-alphabet]: https://datatracker.ietf.org/doc/html/rfc4648#section-4

Other common alphabets are available in the [`alphabet`](base64/alphabet/index.md) module.

#### URL-safe alphabet

The standard alphabet uses `+` and `/` as its two non-alphanumeric tokens,
which cannot be safely used in URL’s without encoding them as `%2B` and
`%2F`.

To avoid that, some applications use a [“URL-safe” alphabet](#url_safe),
which uses `-` and `_` instead. To use that alternative alphabet, use the
[`URL_SAFE`](#url_safe) engine. This example doesn't
use [`prelude`](base64/prelude/index.md) to show what a more explicit `use` would look like.

```
use base64::{engine::general_purpose::URL_SAFE, Engine as _};

# fn main() -> Result<(), base64::DecodeError> {
assert_eq!(URL_SAFE.decode(b"-uwgVQA=")?, b"\xFA\xEC\x20\x55\0");
assert_eq!(URL_SAFE.encode(b"\xFF\xEC\x20\x55\0"), "_-wgVQA=");
# Ok(())
# }
```

### Padding characters

Each base64 character represents 6 bits (2⁶ = 64) of the original binary
data, and every 3 bytes of input binary data will encode to 4 base64
characters (8 bits × 3 = 6 bits × 4 = 24 bits).

When the input is not an even multiple of 3 bytes in length, [canonical][]
base64 encoders insert padding characters at the end, so that the output
length is always a multiple of 4:

[canonical](#canonical): https://datatracker.ietf.org/doc/html/rfc4648#section-3.5

```
use base64::{engine::general_purpose::STANDARD, Engine as _};

assert_eq!(STANDARD.encode(b""),    "");
assert_eq!(STANDARD.encode(b"f"),   "Zg==");
assert_eq!(STANDARD.encode(b"fo"),  "Zm8=");
assert_eq!(STANDARD.encode(b"foo"), "Zm9v");
```

Canonical encoding ensures that base64 encodings will be exactly the same,
byte-for-byte, regardless of input length. But the `=` padding characters
aren’t necessary for decoding, and they may be omitted by using a
[`NO_PAD`](#no_pad) configuration:

```
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};

assert_eq!(STANDARD_NO_PAD.encode(b""),    "");
assert_eq!(STANDARD_NO_PAD.encode(b"f"),   "Zg");
assert_eq!(STANDARD_NO_PAD.encode(b"fo"),  "Zm8");
assert_eq!(STANDARD_NO_PAD.encode(b"foo"), "Zm9v");
```

The pre-configured `NO_PAD` engines will reject inputs containing padding
`=` characters. To encode without padding and still accept padding while
decoding, create an [engine](#generalpurpose) with
that [padding mode](#decodepaddingmode).

```
# use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
assert_eq!(STANDARD_NO_PAD.decode(b"Zm8="), Err(base64::DecodeError::InvalidPadding));
```

### Further customization

Decoding and encoding behavior can be customized by creating an
[engine](#generalpurpose) with an [alphabet](#alphabet) and
[padding configuration](#generalpurposeconfig):

```
use base64::{engine, alphabet, Engine as _};

// bizarro-world base64: +/ as the first symbols instead of the last
let alphabet =
    alphabet::Alphabet::new("+/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789")
    .unwrap();

// a very weird config that encodes with padding but requires no padding when decoding...?
let crazy_config = engine::GeneralPurposeConfig::new()
    .with_decode_allow_trailing_bits(true)
    .with_encode_padding(true)
    .with_decode_padding_mode(engine::DecodePaddingMode::RequireNone);

let crazy_engine = engine::GeneralPurpose::new(&alphabet, crazy_config);

let encoded = crazy_engine.encode(b"abc 123");

```

## Memory allocation

The [decode][Engine::decode()] and [encode][Engine::encode()] engine methods
allocate memory for their results – `decode` returns a `Vec<u8>` and
`encode` returns a `String`. To instead decode or encode into a buffer that
you allocated, use one of the alternative methods:

#### Decoding

| Method                     | Output                        | Allocates memory              |
| -------------------------- | ----------------------------- | ----------------------------- |
| [`Engine::decode`](#decode)         | returns a new `Vec<u8>`       | always                        |
| [`Engine::decode_vec`](#decode-vec)     | appends to provided `Vec<u8>` | if `Vec` lacks capacity       |
| [`Engine::decode_slice`](#decode-slice)   | writes to provided `&[u8](#u8)`    | never

#### Encoding

| Method                     | Output                       | Allocates memory               |
| -------------------------- | ---------------------------- | ------------------------------ |
| [`Engine::encode`](#encode)         | returns a new `String`       | always                         |
| [`Engine::encode_string`](#encode-string)  | appends to provided `String` | if `String` lacks capacity     |
| [`Engine::encode_slice`](#encode-slice)   | writes to provided `&[u8](#u8)`   | never                          |

## Input and output

The `base64` crate can [decode][Engine::decode()] and
[encode][Engine::encode()] values in memory, or
[`DecoderReader`](#decoderreader) and
[`EncoderWriter`](#encoderwriter) provide streaming decoding and
encoding for any [readable](#read) or [writable](#write)
byte stream.

#### Decoding

```
# use std::io;
use base64::{engine::general_purpose::STANDARD, read::DecoderReader};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let mut input = io::stdin();
let mut decoder = DecoderReader::new(&mut input, &STANDARD);
io::copy(&mut decoder, &mut io::stdout())?;
# Ok(())
# }
```

#### Encoding

```
# use std::io;
use base64::{engine::general_purpose::STANDARD, write::EncoderWriter};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let mut output = io::stdout();
let mut encoder = EncoderWriter::new(&mut output, &STANDARD);
io::copy(&mut io::stdin(), &mut encoder)?;
# Ok(())
# }
```

#### Display

If you only need a base64 representation for implementing the
[`Display`](#display) trait, use
[`Base64Display`](#base64display):

```
use base64::{display::Base64Display, engine::general_purpose::STANDARD};

let value = Base64Display::new(b"\0\x01\x02\x03", &STANDARD);
assert_eq!("base64: AAECAw==", format!("base64: {}", value));
```

# Panics

If length calculations result in overflowing `usize`, a panic will result.

## Modules

- [`display`](display/index.md) - Enables base64'd output anywhere you might use a `Display` implementation, like a format string.
- [`read`](read/index.md) - Implementations of `io::Read` to transparently decode base64.
- [`write`](write/index.md) - Implementations of `io::Write` to transparently handle base64.
- [`engine`](engine/index.md) - Provides the [Engine] abstraction and out of the box implementations.
- [`alphabet`](alphabet/index.md) - Provides [Alphabet] and constants for alphabets commonly used in the wild.
- [`prelude`](prelude/index.md) - Preconfigured engines for common use cases.

## Enums

### `EncodeSliceError`

```rust
enum EncodeSliceError {
    OutputSliceTooSmall,
}
```

Errors that can occur while encoding into a slice.

#### Variants

- **`OutputSliceTooSmall`**

  The provided slice is too small.

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

- `fn clone(self: &Self) -> EncodeSliceError`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Error`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &EncodeSliceError) -> bool`

##### `impl StructuralPartialEq`

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

### `DecodeError`

```rust
enum DecodeError {
    InvalidByte(usize, u8),
    InvalidLength(usize),
    InvalidLastSymbol(usize, u8),
    InvalidPadding,
}
```

Errors that can occur while decoding.

#### Variants

- **`InvalidByte`**

  An invalid byte was found in the input. The offset and offending byte are provided.
  
  Padding characters (`=`) interspersed in the encoded form are invalid, as they may only
  be present as the last 0-2 bytes of input.
  
  This error may also indicate that extraneous trailing input bytes are present, causing
  otherwise valid padding to no longer be the last bytes of input.

- **`InvalidLength`**

  The length of the input, as measured in valid base64 symbols, is invalid.
  There must be 2-4 symbols in the last input quad.

- **`InvalidLastSymbol`**

  The last non-padding input symbol's encoded 6 bits have nonzero bits that will be discarded.
  This is indicative of corrupted or truncated Base64.
  Unlike [DecodeError::InvalidByte], which reports symbols that aren't in the alphabet,
  this error is for symbols that are in the alphabet but represent nonsensical encodings.

- **`InvalidPadding`**

  The nature of the padding was not as configured: absent or incorrect when it must be
  canonical, or present when it must be absent, etc.

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

- `fn clone(self: &Self) -> DecodeError`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Error`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &DecodeError) -> bool`

##### `impl StructuralPartialEq`

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

### `DecodeSliceError`

```rust
enum DecodeSliceError {
    DecodeError(DecodeError),
    OutputSliceTooSmall,
}
```

Errors that can occur while decoding into a slice.

#### Variants

- **`DecodeError`**

  A [DecodeError] occurred

- **`OutputSliceTooSmall`**

  The provided slice is too small.

#### Trait Implementations

##### `impl From`

- `fn from(e: DecodeError) -> Self`

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

- `fn clone(self: &Self) -> DecodeSliceError`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Error`

- `fn source(self: &Self) -> Option<&dyn error::Error>`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &DecodeSliceError) -> bool`

##### `impl StructuralPartialEq`

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

## Traits

## Functions

