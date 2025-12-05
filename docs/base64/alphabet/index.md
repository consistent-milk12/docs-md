*[base64](../index.md) / [alphabet](index.md)*

---

# Module `alphabet`

Provides [Alphabet] and constants for alphabets commonly used in the wild.

## Structs

### `Alphabet`

```rust
struct Alphabet {
    // [REDACTED: Private Fields]
}
```

An alphabet defines the 64 ASCII characters (symbols) used for base64.

Common alphabets are provided as constants, and custom alphabets
can be made via `from_str` or the `TryFrom<str>` implementation.

# Examples

Building and using a custom Alphabet:

```rust
let custom = base64::alphabet::Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/").unwrap();

let engine = base64::engine::GeneralPurpose::new(
    &custom,
    base64::engine::general_purpose::PAD);
```

Building a const:

```rust
use base64::alphabet::Alphabet;

static CUSTOM: Alphabet = {
    // Result::unwrap() isn't const yet, but panic!() is OK
    match Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/") {
        Ok(x) => x,
        Err(_) => panic!("creation of alphabet failed"),
    }
};
```

Building lazily:

```rust
use base64::{
    alphabet::Alphabet,
    engine::{general_purpose::GeneralPurpose, GeneralPurposeConfig},
};
use once_cell::sync::Lazy;

static CUSTOM: Lazy<Alphabet> = Lazy::new(||
    Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/").unwrap()
);
```

#### Implementations

- `const fn new(alphabet: &str) -> Result<Self, ParseAlphabetError>`
  Create an `Alphabet` from a string of 64 unique printable ASCII bytes.

- `fn as_str(self: &Self) -> &str`
  Create a `&str` from the symbols in the `Alphabet`

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

- `fn clone(self: &Self) -> Alphabet`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Alphabet) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryFrom`

- `type Error = ParseAlphabetError`

- `fn try_from(value: &str) -> Result<Self, <Self as >::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Enums

### `ParseAlphabetError`

```rust
enum ParseAlphabetError {
    InvalidLength,
    DuplicatedByte(u8),
    UnprintableByte(u8),
    ReservedByte(u8),
}
```

Possible errors when constructing an [Alphabet] from a `str`.

#### Variants

- **`InvalidLength`**

  Alphabets must be 64 ASCII bytes

- **`DuplicatedByte`**

  All bytes must be unique

- **`UnprintableByte`**

  All bytes must be printable (in the range `[32, 126]`).

- **`ReservedByte`**

  `=` cannot be used

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

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Error`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ParseAlphabetError) -> bool`

##### `impl StructuralPartialEq`

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

## Constants

### `STANDARD`

```rust
const STANDARD: Alphabet;
```

The standard alphabet (with `+` and `/`) specified in [RFC 4648][].

[RFC 4648]: https://datatracker.ietf.org/doc/html/rfc4648#section-4

### `URL_SAFE`

```rust
const URL_SAFE: Alphabet;
```

The URL-safe alphabet (with `-` and `_`) specified in [RFC 4648][].

[RFC 4648]: https://datatracker.ietf.org/doc/html/rfc4648#section-5

### `CRYPT`

```rust
const CRYPT: Alphabet;
```

The `crypt(3)` alphabet (with `.` and `/` as the _first_ two characters).

Not standardized, but folk wisdom on the net asserts that this alphabet is what crypt uses.

### `BCRYPT`

```rust
const BCRYPT: Alphabet;
```

The bcrypt alphabet.

### `IMAP_MUTF7`

```rust
const IMAP_MUTF7: Alphabet;
```

The alphabet used in IMAP-modified UTF-7 (with `+` and `,`).

See [RFC 3501](https://tools.ietf.org/html/rfc3501#section-5.1.3)

### `BIN_HEX`

```rust
const BIN_HEX: Alphabet;
```

The alphabet used in BinHex 4.0 files.

See [BinHex 4.0 Definition](http://files.stairways.com/other/binhex-40-specs-info.txt)

