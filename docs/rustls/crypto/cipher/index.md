*[rustls](../../index.md) / [crypto](../index.md) / [cipher](index.md)*

---

# Module `cipher`

TLS message encryption/decryption interfaces.

## Structs

### `BorrowedPayload<'a>`

```rust
struct BorrowedPayload<'a>();
```

#### Implementations

- `fn truncate(self: &mut Self, len: usize)`

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

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Deref`

- `type Target = [u8]`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

### `InboundOpaqueMessage<'a>`

```rust
struct InboundOpaqueMessage<'a> {
    pub typ: crate::enums::ContentType,
    pub version: crate::enums::ProtocolVersion,
    pub payload: BorrowedPayload<'a>,
}
```

A TLS frame, named TLSPlaintext in the standard.

This inbound type borrows its encrypted payload from a buffer elsewhere.
It is used for joining and is consumed by decryption.

#### Implementations

- `fn new(typ: ContentType, version: ProtocolVersion, payload: &'a mut [u8]) -> Self`
  Construct a new `InboundOpaqueMessage` from constituent fields.

- `fn into_plain_message(self: Self) -> InboundPlainMessage<'a>`
  Force conversion into a plaintext message.

- `fn into_plain_message_range(self: Self, range: Range<usize>) -> InboundPlainMessage<'a>`
  Force conversion into a plaintext message.

- `fn into_tls13_unpadded_message(self: Self) -> Result<InboundPlainMessage<'a>, Error>`
  For TLS1.3 (only), checks the length msg.payload is valid and removes the padding.

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

### `InboundPlainMessage<'a>`

```rust
struct InboundPlainMessage<'a> {
    pub typ: crate::enums::ContentType,
    pub version: crate::enums::ProtocolVersion,
    pub payload: &'a [u8],
}
```

A TLS frame, named `TLSPlaintext` in the standard.

This inbound type borrows its decrypted payload from the original buffer.
It results from decryption.

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

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `OutboundOpaqueMessage`

```rust
struct OutboundOpaqueMessage {
    pub typ: crate::enums::ContentType,
    pub version: crate::enums::ProtocolVersion,
    pub payload: PrefixedPayload,
}
```

A TLS frame, named `TLSPlaintext` in the standard.

This outbound type owns all memory for its interior parts.
It results from encryption and is used for io write.

#### Implementations

- `fn new(typ: ContentType, version: ProtocolVersion, payload: PrefixedPayload) -> Self`
  Construct a new `OpaqueMessage` from constituent fields.

- `fn read(r: &mut Reader<'_>) -> Result<Self, MessageError>`
  Construct by decoding from a [`Reader`].

- `fn encode(self: Self) -> Vec<u8>`

- `fn into_plain_message(self: Self) -> PlainMessage`
  Force conversion into a plaintext message.

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

- `fn clone(self: &Self) -> OutboundOpaqueMessage`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

### `OutboundPlainMessage<'a>`

```rust
struct OutboundPlainMessage<'a> {
    pub typ: crate::enums::ContentType,
    pub version: crate::enums::ProtocolVersion,
    pub payload: OutboundChunks<'a>,
}
```

A TLS frame, named `TLSPlaintext` in the standard.

This outbound type borrows its "to be encrypted" payload from the "user".
It is used for fragmenting and is consumed by encryption.

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

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `PlainMessage`

```rust
struct PlainMessage {
    pub typ: crate::enums::ContentType,
    pub version: crate::enums::ProtocolVersion,
    pub payload: crate::msgs::base::Payload<'static>,
}
```

A decrypted TLS frame

This type owns all memory for its interior parts. It can be decrypted from an OpaqueMessage
or encrypted into an OpaqueMessage, and it is also used for joining and fragmenting.

#### Implementations

- `fn into_unencrypted_opaque(self: Self) -> OutboundOpaqueMessage`

- `fn borrow_inbound(self: &Self) -> InboundPlainMessage<'_>`

- `fn borrow_outbound(self: &Self) -> OutboundPlainMessage<'_>`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(msg: Message<'_>) -> Self`

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

- `fn clone(self: &Self) -> PlainMessage`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

### `PrefixedPayload`

```rust
struct PrefixedPayload();
```

#### Implementations

- `fn with_capacity(capacity: usize) -> Self`

- `fn extend_from_slice(self: &mut Self, slice: &[u8])`

- `fn extend_from_chunks(self: &mut Self, chunks: &OutboundChunks<'_>)`

- `fn truncate(self: &mut Self, len: usize)`

#### Trait Implementations

##### `impl From`

- `fn from(content: &[u8]) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<const N: usize>`

- `fn from(content: &[u8; N]) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsMut`

- `fn as_mut(self: &mut Self) -> &mut [u8]`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &[u8]`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> PrefixedPayload`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Extend<'a>`

- `fn extend<T: IntoIterator<Item = &'a u8>>(self: &mut Self, iter: T)`

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

### `UnsupportedOperationError`

```rust
struct UnsupportedOperationError;
```

An error indicating that the AEAD algorithm does not support the requested operation.

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

- `fn clone(self: &Self) -> UnsupportedOperationError`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Error`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &UnsupportedOperationError) -> bool`

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

### `KeyBlockShape`

```rust
struct KeyBlockShape {
    pub enc_key_len: usize,
    pub fixed_iv_len: usize,
    pub explicit_nonce_len: usize,
}
```

How a TLS1.2 `key_block` is partitioned.

Note: ciphersuites with non-zero `mac_key_length` are  not currently supported.

#### Fields

- **`enc_key_len`**: `usize`

  How long keys are.
  
  `enc_key_length` terminology is from the standard ([RFC5246 A.6]).
  
  [RFC5246 A.6]: <https://www.rfc-editor.org/rfc/rfc5246#appendix-A.6>

- **`fixed_iv_len`**: `usize`

  How long the fixed part of the 'IV' is.
  
  `fixed_iv_length` terminology is from the standard ([RFC5246 A.6]).
  
  This isn't usually an IV, but we continue the
  terminology misuse to match the standard.
  
  [RFC5246 A.6]: <https://www.rfc-editor.org/rfc/rfc5246#appendix-A.6>

- **`explicit_nonce_len`**: `usize`

  This is a non-standard extension which extends the
  key block to provide an initial explicit nonce offset,
  in a deterministic and safe way.  GCM needs this,
  chacha20poly1305 works this way by design.

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

### `Iv`

```rust
struct Iv();
```

A write or read IV.

#### Implementations

- `fn new(value: [u8; 12]) -> Self`
  Create a new `Iv` from a byte array, of precisely `NONCE_LEN` bytes.

- `fn copy(value: &[u8]) -> Self`
  Create a new `Iv` from a byte slice, of precisely `NONCE_LEN` bytes.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(bytes: [u8; 12]) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &[u8]`

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

##### `impl Default`

- `fn default() -> Iv`

### `Nonce`

```rust
struct Nonce([u8; 12]);
```

A nonce.  This is unique for all messages on a connection.

#### Implementations

- `fn new(iv: &Iv, seq: u64) -> Self`
  Combine an `Iv` and sequence number to produce a unique nonce.

- `fn for_path(path_id: u32, iv: &Iv, pn: u64) -> Self`
  Creates a unique nonce based on the `iv`, the packet number `pn` and multipath `path_id`.

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

### `AeadKey`

```rust
struct AeadKey {
    // [REDACTED: Private Fields]
}
```

A key for an AEAD algorithm.

This is a value type for a byte string up to `AeadKey::MAX_LEN` bytes in length.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(bytes: [u8; 32]) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &[u8]`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Drop`

- `fn drop(self: &mut Self)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `OutboundChunks<'a>`

```rust
enum OutboundChunks<'a> {
    Single(&'a [u8]),
    Multiple {
        chunks: &'a [&'a [u8]],
        start: usize,
        end: usize,
    },
}
```

A collection of borrowed plaintext slices.

Warning: OutboundChunks does not guarantee that the simplest variant is used.
Multiple can hold non fragmented or empty payloads.

#### Variants

- **`Single`**

  A single byte slice. Contrary to `Multiple`, this uses a single pointer indirection

- **`Multiple`**

  A collection of chunks (byte slices)
  and cursors to single out a fragmented range of bytes.
  OutboundChunks assumes that start <= end

#### Implementations

- `fn new(chunks: &'a [&'a [u8]]) -> Self`
  Create a payload from a slice of byte slices.

- `fn new_empty() -> Self`
  Create a payload with a single empty slice

- `fn to_vec(self: &Self) -> Vec<u8>`
  Flatten the slice of byte slices to an owned vector of bytes

- `fn copy_to_vec(self: &Self, vec: &mut Vec<u8>)`
  Append all bytes to a vector

- `fn split_at(self: &Self, mid: usize) -> (Self, Self)`
  Split self in two, around an index

- `fn is_empty(self: &Self) -> bool`
  Returns true if the payload is empty

- `fn len(self: &Self) -> usize`
  Returns the cumulative length of all chunks

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<'a>`

- `fn from(payload: &'a [u8]) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> OutboundChunks<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

## Traits

### `Tls13AeadAlgorithm`

```rust
trait Tls13AeadAlgorithm: Send + Sync { ... }
```

Factory trait for building `MessageEncrypter` and `MessageDecrypter` for a TLS1.3 cipher suite.

#### Required Methods

- `fn encrypter(self: &Self, key: AeadKey, iv: Iv) -> Box<dyn MessageEncrypter>`

  Build a `MessageEncrypter` for the given key/iv.

- `fn decrypter(self: &Self, key: AeadKey, iv: Iv) -> Box<dyn MessageDecrypter>`

  Build a `MessageDecrypter` for the given key/iv.

- `fn key_len(self: &Self) -> usize`

  The length of key in bytes required by `encrypter()` and `decrypter()`.

- `fn extract_keys(self: &Self, key: AeadKey, iv: Iv) -> Result<ConnectionTrafficSecrets, UnsupportedOperationError>`

  Convert the key material from `key`/`iv`, into a `ConnectionTrafficSecrets` item.

- `fn fips(self: &Self) -> bool`

  Return `true` if this is backed by a FIPS-approved implementation.

### `Tls12AeadAlgorithm`

```rust
trait Tls12AeadAlgorithm: Send + Sync + 'static { ... }
```

Factory trait for building `MessageEncrypter` and `MessageDecrypter` for a TLS1.2 cipher suite.

#### Required Methods

- `fn encrypter(self: &Self, key: AeadKey, iv: &[u8], extra: &[u8]) -> Box<dyn MessageEncrypter>`

  Build a `MessageEncrypter` for the given key/iv and extra key block (which can be used for

- `fn decrypter(self: &Self, key: AeadKey, iv: &[u8]) -> Box<dyn MessageDecrypter>`

  Build a `MessageDecrypter` for the given key/iv.

- `fn key_block_shape(self: &Self) -> KeyBlockShape`

  Return a `KeyBlockShape` that defines how large the `key_block` is and how it

- `fn extract_keys(self: &Self, key: AeadKey, iv: &[u8], explicit: &[u8]) -> Result<ConnectionTrafficSecrets, UnsupportedOperationError>`

  Convert the key material from `key`/`iv`, into a `ConnectionTrafficSecrets` item.

- `fn fips(self: &Self) -> bool`

  Return `true` if this is backed by a FIPS-approved implementation.

### `MessageDecrypter`

```rust
trait MessageDecrypter: Send + Sync { ... }
```

Objects with this trait can decrypt TLS messages.

#### Required Methods

- `fn decrypt<'a>(self: &mut Self, msg: InboundOpaqueMessage<'a>, seq: u64) -> Result<InboundPlainMessage<'a>, Error>`

  Decrypt the given TLS message `msg`, using the sequence number

### `MessageEncrypter`

```rust
trait MessageEncrypter: Send + Sync { ... }
```

Objects with this trait can encrypt TLS messages.

#### Required Methods

- `fn encrypt(self: &mut Self, msg: OutboundPlainMessage<'_>, seq: u64) -> Result<OutboundOpaqueMessage, Error>`

  Encrypt the given TLS message `msg`, using the sequence number

- `fn encrypted_payload_len(self: &Self, payload_len: usize) -> usize`

  Return the length of the ciphertext that results from encrypting plaintext of

## Functions

### `make_tls13_aad`

```rust
fn make_tls13_aad(payload_len: usize) -> [u8; 5]
```

Returns a TLS1.3 `additional_data` encoding.

See RFC8446 s5.2 for the `additional_data` definition.

### `make_tls12_aad`

```rust
fn make_tls12_aad(seq: u64, typ: crate::enums::ContentType, vers: crate::enums::ProtocolVersion, len: usize) -> [u8; 13]
```

Returns a TLS1.2 `additional_data` encoding.

See RFC5246 s6.2.3.3 for the `additional_data` definition.

## Constants

### `NONCE_LEN`

```rust
const NONCE_LEN: usize = 12usize;
```

Size of TLS nonces (incorrectly termed "IV" in standard) for all supported ciphersuites
(AES-GCM, Chacha20Poly1305)

