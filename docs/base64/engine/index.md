*[base64](../index.md) / [engine](index.md)*

---

# Module `engine`

Provides the [Engine] abstraction and out of the box implementations.

## Modules

- [`general_purpose`](general_purpose/index.md) - Provides the [GeneralPurpose] engine and associated config types.

## Structs

### `GeneralPurpose`

```rust
struct GeneralPurpose {
    // [REDACTED: Private Fields]
}
```

A general-purpose base64 engine.

- It uses no vector CPU instructions, so it will work on any system.
- It is reasonably fast (~2-3GiB/s).
- It is not constant-time, though, so it is vulnerable to timing side-channel attacks. For loading cryptographic keys, etc, it is suggested to use the forthcoming constant-time implementation.

#### Implementations

- `const fn new(alphabet: &Alphabet, config: GeneralPurposeConfig) -> Self`
  Create a `GeneralPurpose` engine from an [Alphabet].

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

- `fn clone(self: &Self) -> GeneralPurpose`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Engine`

- `type Config = GeneralPurposeConfig`

- `type DecodeEstimate = GeneralPurposeEstimate`

- `fn config(self: &Self) -> &<Self as >::Config`

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

### `GeneralPurposeConfig`

```rust
struct GeneralPurposeConfig {
    // [REDACTED: Private Fields]
}
```

Contains configuration parameters for base64 encoding and decoding.

```rust
use base64::engine::GeneralPurposeConfig;
let config = GeneralPurposeConfig::new()
    .with_encode_padding(false);
    // further customize using `.with_*` methods as needed
```

The constants [PAD] and [NO_PAD] cover most use cases.

To specify the characters used, see [Alphabet].

#### Implementations

- `const fn new() -> Self`
  Create a new config with `padding` = `true`, `decode_allow_trailing_bits` = `false`, and

- `const fn with_encode_padding(self: Self, padding: bool) -> Self`
  Create a new config based on `self` with an updated `padding` setting.

- `const fn with_decode_allow_trailing_bits(self: Self, allow: bool) -> Self`
  Create a new config based on `self` with an updated `decode_allow_trailing_bits` setting.

- `const fn with_decode_padding_mode(self: Self, mode: DecodePaddingMode) -> Self`
  Create a new config based on `self` with an updated `decode_padding_mode` setting.

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

- `fn clone(self: &Self) -> GeneralPurposeConfig`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Config`

- `fn encode_padding(self: &Self) -> bool`

##### `impl Copy`

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

- `fn default() -> Self`
  Delegates to [GeneralPurposeConfig::new].

### `DecodeMetadata`

```rust
struct DecodeMetadata {
    // [REDACTED: Private Fields]
}
```

Metadata about the result of a decode operation

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

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &DecodeMetadata) -> bool`

##### `impl StructuralPartialEq`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Enums

### `DecodePaddingMode`

```rust
enum DecodePaddingMode {
    Indifferent,
    RequireCanonical,
    RequireNone,
}
```

Controls how pad bytes are handled when decoding.

Each [Engine] must support at least the behavior indicated by
[DecodePaddingMode::RequireCanonical], and may support other modes.

#### Variants

- **`Indifferent`**

  Canonical padding is allowed, but any fewer padding bytes than that is also allowed.

- **`RequireCanonical`**

  Padding must be canonical (0, 1, or 2 `=` as needed to produce a 4 byte suffix).

- **`RequireNone`**

  Padding must be absent -- for when you want predictable padding, without any wasted bytes.

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

- `fn clone(self: &Self) -> DecodePaddingMode`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &DecodePaddingMode) -> bool`

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

## Traits

### `Engine`

```rust
trait Engine: Send + Sync { ... }
```

An `Engine` provides low-level encoding and decoding operations that all other higher-level parts of the API use. Users of the library will generally not need to implement this.

Different implementations offer different characteristics. The library currently ships with
[GeneralPurpose] that offers good speed and works on any CPU, with more choices
coming later, like a constant-time one when side channel resistance is called for, and vendor-specific vectorized ones for more speed.

See [general_purpose::STANDARD_NO_PAD] if you just want standard base64. Otherwise, when possible, it's
recommended to store the engine in a `const` so that references to it won't pose any lifetime
issues, and to avoid repeating the cost of engine setup.

Since almost nobody will need to implement `Engine`, docs for internal methods are hidden.

#### Required Methods

- `type Config: 1`

- `type DecodeEstimate: 1`

- `fn config(self: &Self) -> &<Self as >::Config`

  Returns the config for this engine.

- `fn encode<T: AsRef<[u8]>>(self: &Self, input: T) -> String`

  Encode arbitrary octets as base64 using the provided `Engine`.

- `fn encode_string<T: AsRef<[u8]>>(self: &Self, input: T, output_buf: &mut String)`

  Encode arbitrary octets as base64 into a supplied `String`.

- `fn encode_slice<T: AsRef<[u8]>>(self: &Self, input: T, output_buf: &mut [u8]) -> Result<usize, EncodeSliceError>`

  Encode arbitrary octets as base64 into a supplied slice.

- `fn decode<T: AsRef<[u8]>>(self: &Self, input: T) -> Result<Vec<u8>, DecodeError>`

  Decode the input into a new `Vec`.

- `fn decode_vec<T: AsRef<[u8]>>(self: &Self, input: T, buffer: &mut Vec<u8>) -> Result<(), DecodeError>`

  Decode the `input` into the supplied `buffer`.

- `fn decode_slice<T: AsRef<[u8]>>(self: &Self, input: T, output: &mut [u8]) -> Result<usize, DecodeSliceError>`

  Decode the input into the provided output slice.

- `fn decode_slice_unchecked<T: AsRef<[u8]>>(self: &Self, input: T, output: &mut [u8]) -> Result<usize, DecodeError>`

  Decode the input into the provided output slice.

### `Config`

```rust
trait Config { ... }
```

The minimal level of configuration that engines must support.

#### Required Methods

- `fn encode_padding(self: &Self) -> bool`

  Returns `true` if padding should be added after the encoded output.

### `DecodeEstimate`

```rust
trait DecodeEstimate { ... }
```

The decode estimate used by an engine implementation. Users do not need to interact with this;
it is only for engine implementors.

Implementors may store relevant data here when constructing this to avoid having to calculate
them again during actual decoding.

#### Required Methods

- `fn decoded_len_estimate(self: &Self) -> usize`

  Returns a conservative (err on the side of too big) estimate of the decoded length to use

