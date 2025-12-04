*[ring](../../index.md) / [aead](../index.md) / [chacha20_poly1305_openssh](index.md)*

---

# Module `chacha20_poly1305_openssh`

The [chacha20-poly1305@openssh.com] AEAD-ish construct.

This should only be used by SSH implementations. It has a similar, but
different API from `ring::aead` because the construct cannot use the same
API as `ring::aead` due to the way the construct handles the encrypted
packet length.

The concatenation of a and b is denoted `a||b`. `K_1` and `K_2` are defined
in the [chacha20-poly1305@openssh.com] specification. `packet_length`,
`padding_length`, `payload`, and `random padding` are defined in
[RFC 4253]. The term `plaintext` is used as a shorthand for
`padding_length||payload||random padding`.

[chacha20-poly1305@openssh.com]:
   http://cvsweb.openbsd.org/cgi-bin/cvsweb/src/usr.bin/ssh/PROTOCOL.chacha20poly1305?annotate=HEAD
[RFC 4253]: https://tools.ietf.org/html/rfc4253

## Structs

### `SealingKey`

```rust
struct SealingKey {
}
```

A key for sealing packets.

#### Implementations

- `fn new(key_material: &[u8; 64]) -> Self`
  Constructs a new `SealingKey`.

- `fn seal_in_place(self: &Self, sequence_number: u32, plaintext_in_ciphertext_out: &mut [u8], tag_out: &mut [u8; 16])`
  Seals (encrypts and signs) a packet.

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

### `OpeningKey`

```rust
struct OpeningKey {
}
```

A key for opening packets.

#### Implementations

- `fn new(key_material: &[u8; 64]) -> Self`
  Constructs a new `OpeningKey`.

- `fn decrypt_packet_length(self: &Self, sequence_number: u32, encrypted_packet_length: [u8; 4]) -> [u8; 4]`
  Returns the decrypted, but unauthenticated, packet length.

- `fn open_in_place<'a>(self: &Self, sequence_number: u32, ciphertext_in_plaintext_out: &'a mut [u8], tag: &[u8; 16]) -> Result<&'a [u8], error::Unspecified>`
  Opens (authenticates and decrypts) a packet.

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

## Constants

### `KEY_LEN`

```rust
const KEY_LEN: usize = 64usize;
```

The length of key.

### `PACKET_LENGTH_LEN`

```rust
const PACKET_LENGTH_LEN: usize = 4usize;
```

The length in bytes of the `packet_length` field in a SSH packet.

### `TAG_LEN`

```rust
const TAG_LEN: usize = 16usize;
```

The length in bytes of an authentication tag.

