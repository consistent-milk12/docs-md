*[base64](../../index.md) / [engine](../index.md) / [general_purpose](index.md)*

---

# Module `general_purpose`

Provides the [GeneralPurpose] engine and associated config types.

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

## Constants

### `STANDARD`

```rust
const STANDARD: GeneralPurpose;
```

A [GeneralPurpose] engine using the [alphabet::STANDARD] base64 alphabet and [PAD] config.

### `STANDARD_NO_PAD`

```rust
const STANDARD_NO_PAD: GeneralPurpose;
```

A [GeneralPurpose] engine using the [alphabet::STANDARD] base64 alphabet and [NO_PAD] config.

### `URL_SAFE`

```rust
const URL_SAFE: GeneralPurpose;
```

A [GeneralPurpose] engine using the [alphabet::URL_SAFE] base64 alphabet and [PAD] config.

### `URL_SAFE_NO_PAD`

```rust
const URL_SAFE_NO_PAD: GeneralPurpose;
```

A [GeneralPurpose] engine using the [alphabet::URL_SAFE] base64 alphabet and [NO_PAD] config.

### `PAD`

```rust
const PAD: GeneralPurposeConfig;
```

Include padding bytes when encoding, and require that they be present when decoding.

This is the standard per the base64 RFC, but consider using [NO_PAD] instead as padding serves
little purpose in practice.

### `NO_PAD`

```rust
const NO_PAD: GeneralPurposeConfig;
```

Don't add padding when encoding, and require no padding when decoding.

