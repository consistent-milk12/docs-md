# Crate `percent_encoding`

URLs use special characters to indicate the parts of the request.
For example, a `?` question mark marks the end of a path and the start of a query string.
In order for that character to exist inside a path, it needs to be encoded differently.

Percent encoding replaces reserved characters with the `%` escape character
followed by a byte value as two hexadecimal digits.
For example, an ASCII space is replaced with `%20`.

When encoding, the set of characters that can (and should, for readability) be left alone
depends on the context.
The `?` question mark mentioned above is not a separator when used literally
inside of a query string, and therefore does not need to be encoded.
The [`AsciiSet`](#asciiset) parameter of [`percent_encode`](percent_encoding/index.md) and [`utf8_percent_encode`](percent_encoding/index.md)
lets callers configure this.

This crate deliberately does not provide many different sets.
Users should consider in what context the encoded string will be used,
read relevant specifications, and define their own set.
This is done by using the `add` method of an existing set.

# Examples

```
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

/// https://url.spec.whatwg.org/#fragment-percent-encode-set
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

assert_eq!(utf8_percent_encode("foo <bar>", FRAGMENT).to_string(), "foo%20%3Cbar%3E");
```

## Structs

### `AsciiSet`

```rust
struct AsciiSet {
}
```

Represents a set of characters or bytes in the ASCII range.

This is used in [`percent_encode`](percent_encoding/index.md) and [`utf8_percent_encode`](percent_encoding/index.md).
This is similar to [percent-encode sets](https://url.spec.whatwg.org/#percent-encoded-bytes).

Use the `add` method of an existing set to define a new set. For example:


```
use percent_encoding::{AsciiSet, CONTROLS};

/// https://url.spec.whatwg.org/#fragment-percent-encode-set
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');
```

#### Implementations

- `const EMPTY: Self`

- `const fn add(self: &Self, byte: u8) -> Self`

- `const fn remove(self: &Self, byte: u8) -> Self`

- `const fn union(self: &Self, other: Self) -> Self`
  Return the union of two sets.

- `const fn complement(self: &Self) -> Self`
  Return the negation of the set.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Add`

- `type Output = AsciiSet`

- `fn add(self: Self, other: Self) -> Self`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Eq`

##### `impl Not`

- `type Output = AsciiSet`

- `fn not(self: Self) -> Self`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &AsciiSet) -> bool`

##### `impl StructuralPartialEq`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `PercentEncode<'a>`

```rust
struct PercentEncode<'a> {
}
```

The return type of [`percent_encode`](percent_encoding/index.md) and [`utf8_percent_encode`](percent_encoding/index.md).

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> PercentEncode<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq<'a>`

##### `impl Iterator<'a>`

- `type Item = &'a str`

- `fn next(self: &mut Self) -> Option<&'a str>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &PercentEncode<'a>) -> bool`

##### `impl StructuralPartialEq<'a>`

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

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `PercentDecode<'a>`

```rust
struct PercentDecode<'a> {
}
```

The return type of [`percent_decode`](percent_encoding/index.md).

#### Implementations

- `fn decode_utf8(self: Self) -> Result<Cow<'a, str>, str::Utf8Error>`
  Decode the result of percent-decoding as UTF-8.

- `fn decode_utf8_lossy(self: Self) -> Cow<'a, str>`
  Decode the result of percent-decoding as UTF-8, lossily.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> PercentDecode<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Iterator`

- `type Item = u8`

- `fn next(self: &mut Self) -> Option<u8>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

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

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Functions

### `percent_encode_byte`

```rust
fn percent_encode_byte(byte: u8) -> &'static str
```

Return the percent-encoding of the given byte.

This is unconditional, unlike `percent_encode()` which has an `AsciiSet` parameter.

# Examples

```
use percent_encoding::percent_encode_byte;

assert_eq!("foo bar".bytes().map(percent_encode_byte).collect::<String>(),
           "%66%6F%6F%20%62%61%72");
```

### `percent_encode`

```rust
fn percent_encode<'a>(input: &'a [u8], ascii_set: &'static AsciiSet) -> PercentEncode<'a>
```

Percent-encode the given bytes with the given set.

Non-ASCII bytes and bytes in `ascii_set` are encoded.

The return type:

* Implements `Iterator<Item = &str>` and therefore has a `.collect::<String>()` method,
* Implements `Display` and therefore has a `.to_string()` method,
* Implements `Into<Cow<str>>` borrowing `input` when none of its bytes are encoded.

# Examples

```
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

assert_eq!(percent_encode(b"foo bar?", NON_ALPHANUMERIC).to_string(), "foo%20bar%3F");
```

### `utf8_percent_encode`

```rust
fn utf8_percent_encode<'a>(input: &'a str, ascii_set: &'static AsciiSet) -> PercentEncode<'a>
```

Percent-encode the UTF-8 encoding of the given string.

See [`percent_encode`](percent_encoding/index.md) regarding the return type.

# Examples

```
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

assert_eq!(utf8_percent_encode("foo bar?", NON_ALPHANUMERIC).to_string(), "foo%20bar%3F");
```

### `percent_decode_str`

```rust
fn percent_decode_str(input: &str) -> PercentDecode<'_>
```

Percent-decode the given string.

<https://url.spec.whatwg.org/#string-percent-decode>

See [`percent_decode`](percent_encoding/index.md) regarding the return type.

### `percent_decode`

```rust
fn percent_decode(input: &[u8]) -> PercentDecode<'_>
```

Percent-decode the given bytes.

<https://url.spec.whatwg.org/#percent-decode>

Any sequence of `%` followed by two hexadecimal digits is decoded.
The return type:

* Implements `Into<Cow<u8>>` borrowing `input` when it contains no percent-encoded sequence,
* Implements `Iterator<Item = u8>` and therefore has a `.collect::<Vec<u8>>()` method,
* Has `decode_utf8()` and `decode_utf8_lossy()` methods.

# Examples

```
use percent_encoding::percent_decode;

assert_eq!(percent_decode(b"foo%20bar%3f").decode_utf8().unwrap(), "foo bar?");
```

## Constants

