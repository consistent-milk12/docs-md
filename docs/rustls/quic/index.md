*[rustls](../index.md) / [quic](index.md)*

---

# Module `quic`

APIs for implementing QUIC TLS

## Structs

### `ClientConnection`

```rust
struct ClientConnection {
    // [REDACTED: Private Fields]
}
```

A QUIC client connection.

#### Implementations

- `fn new(config: alloc::sync::Arc<ClientConfig>, quic_version: Version, name: ServerName<'static>, params: Vec<u8>) -> Result<Self, Error>`
  Make a new QUIC ClientConnection.

- `fn new_with_alpn(config: alloc::sync::Arc<ClientConfig>, quic_version: Version, name: ServerName<'static>, params: Vec<u8>, alpn_protocols: Vec<Vec<u8>>) -> Result<Self, Error>`
  Make a new QUIC ClientConnection with custom ALPN protocols.

- `fn is_early_data_accepted(self: &Self) -> bool`
  Returns True if the server signalled it will process early data.

- `fn tls13_tickets_received(self: &Self) -> u32`
  Returns the number of TLS1.3 tickets that have been received.

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

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref`

- `type Target = ConnectionCommon<ClientConnectionData>`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

### `ConnectionCommon<Data>`

```rust
struct ConnectionCommon<Data> {
    // [REDACTED: Private Fields]
}
```

A shared interface for QUIC connections.

#### Implementations

- `fn quic_transport_parameters(self: &Self) -> Option<&[u8]>`
  Return the TLS-encoded transport parameters for the session's peer.

- `fn zero_rtt_keys(self: &Self) -> Option<DirectionalKeys>`
  Compute the keys for encrypting/decrypting 0-RTT packets, if available

- `fn read_hs(self: &mut Self, plaintext: &[u8]) -> Result<(), Error>`
  Consume unencrypted TLS handshake data.

- `fn write_hs(self: &mut Self, buf: &mut Vec<u8>) -> Option<KeyChange>`
  Emit unencrypted TLS handshake data.

- `fn alert(self: &Self) -> Option<AlertDescription>`
  Emit the TLS description code of a fatal alert, if one has arisen.

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

##### `impl Deref<Data>`

- `type Target = CommonState`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut<Data>`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

### `ServerConnection`

```rust
struct ServerConnection {
    // [REDACTED: Private Fields]
}
```

A QUIC server connection.

#### Implementations

- `fn new(config: alloc::sync::Arc<ServerConfig>, quic_version: Version, params: Vec<u8>) -> Result<Self, Error>`
  Make a new QUIC ServerConnection.

- `fn reject_early_data(self: &mut Self)`
  Explicitly discard early data, notifying the client

- `fn server_name(self: &Self) -> Option<&str>`
  Retrieves the server name, if any, used to select the certificate and

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

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref`

- `type Target = ConnectionCommon<ServerConnectionData>`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

### `Secrets`

```rust
struct Secrets {
    // [REDACTED: Private Fields]
}
```

Secrets used to encrypt/decrypt traffic

#### Implementations

- `fn next_packet_keys(self: &mut Self) -> PacketKeySet`
  Derive the next set of packet keys

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

- `fn clone(self: &Self) -> Secrets`

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

### `DirectionalKeys`

```rust
struct DirectionalKeys {
    pub header: alloc::boxed::Box<dyn HeaderProtectionKey>,
    pub packet: alloc::boxed::Box<dyn PacketKey>,
}
```

Keys used to communicate in a single direction

#### Fields

- **`header`**: `alloc::boxed::Box<dyn HeaderProtectionKey>`

  Encrypts or decrypts a packet's headers

- **`packet`**: `alloc::boxed::Box<dyn PacketKey>`

  Encrypts or decrypts the payload of a packet

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

### `Tag`

```rust
struct Tag();
```

Authentication tag from an AEAD seal operation.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(value: &[u8]) -> Self`

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

### `PacketKeySet`

```rust
struct PacketKeySet {
    pub local: alloc::boxed::Box<dyn PacketKey>,
    pub remote: alloc::boxed::Box<dyn PacketKey>,
}
```

Packet protection keys for bidirectional 1-RTT communication

#### Fields

- **`local`**: `alloc::boxed::Box<dyn PacketKey>`

  Encrypts outgoing packets

- **`remote`**: `alloc::boxed::Box<dyn PacketKey>`

  Decrypts incoming packets

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

### `Suite`

```rust
struct Suite {
    pub suite: &'static crate::tls13::Tls13CipherSuite,
    pub quic: &'static dyn Algorithm,
}
```

Produces QUIC initial keys from a TLS 1.3 ciphersuite and a QUIC key generation algorithm.

#### Fields

- **`suite`**: `&'static crate::tls13::Tls13CipherSuite`

  The TLS 1.3 ciphersuite used to derive keys.

- **`quic`**: `&'static dyn Algorithm`

  The QUIC key generation algorithm used to derive keys.

#### Implementations

- `fn keys(self: &Self, client_dst_connection_id: &[u8], side: Side, version: Version) -> Keys`
  Produce a set of initial keys given the connection ID, side and version

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

- `fn clone(self: &Self) -> Suite`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

### `Keys`

```rust
struct Keys {
    pub local: DirectionalKeys,
    pub remote: DirectionalKeys,
}
```

Complete set of keys used to communicate with the peer

#### Fields

- **`local`**: `DirectionalKeys`

  Encrypts outgoing packets

- **`remote`**: `DirectionalKeys`

  Decrypts incoming packets

#### Implementations

- `fn initial(version: Version, suite: &'static Tls13CipherSuite, quic: &'static dyn Algorithm, client_dst_connection_id: &[u8], side: Side) -> Self`
  Construct keys for use with initial packets

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

## Enums

### `Connection`

```rust
enum Connection {
    Client(ClientConnection),
    Server(ServerConnection),
}
```

A QUIC client or server connection.

#### Variants

- **`Client`**

  A client connection

- **`Server`**

  A server connection

#### Implementations

- `fn quic_transport_parameters(self: &Self) -> Option<&[u8]>`
  Return the TLS-encoded transport parameters for the session's peer.

- `fn zero_rtt_keys(self: &Self) -> Option<DirectionalKeys>`
  Compute the keys for encrypting/decrypting 0-RTT packets, if available

- `fn read_hs(self: &mut Self, plaintext: &[u8]) -> Result<(), Error>`
  Consume unencrypted TLS handshake data.

- `fn write_hs(self: &mut Self, buf: &mut Vec<u8>) -> Option<KeyChange>`
  Emit unencrypted TLS handshake data.

- `fn alert(self: &Self) -> Option<AlertDescription>`
  Emit the TLS description code of a fatal alert, if one has arisen.

- `fn export_keying_material<T: AsMut<[u8]>>(self: &Self, output: T, label: &[u8], context: Option<&[u8]>) -> Result<T, Error>`
  Derives key material from the agreed connection secrets.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(c: ServerConnection) -> Self`

##### `impl From`

- `fn from(c: ClientConnection) -> Self`

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

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deref`

- `type Target = CommonState`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

### `KeyChange`

```rust
enum KeyChange {
    Handshake {
        keys: Keys,
    },
    OneRtt {
        keys: Keys,
        next: Secrets,
    },
}
```

Key material for use in QUIC packet spaces

QUIC uses 4 different sets of keys (and progressive key updates for long-running connections):

* Initial: these can be created from `Keys::initial()`
* 0-RTT keys: can be retrieved from `ConnectionCommon::zero_rtt_keys()`
* Handshake: these are returned from `ConnectionCommon::write_hs()` after `ClientHello` and
  `ServerHello` messages have been exchanged
* 1-RTT keys: these are returned from `ConnectionCommon::write_hs()` after the handshake is done

Once the 1-RTT keys have been exchanged, either side may initiate a key update. Progressive
update keys can be obtained from the [`Secrets`](#secrets) returned in `KeyChange::OneRtt`. Note that
only packet keys are updated by key updates; header protection keys remain the same.

#### Variants

- **`Handshake`**

  Keys for the handshake space

- **`OneRtt`**

  Keys for 1-RTT data

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

### `Version`

```rust
enum Version {
    V1Draft,
    V1,
    V2,
}
```

QUIC protocol version

Governs version-specific behavior in the TLS layer

#### Variants

- **`V1Draft`**

  Draft versions 29, 30, 31 and 32

- **`V1`**

  First stable RFC

- **`V2`**

  Anti-ossification variant of V1

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

- `fn clone(self: &Self) -> Version`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

- `fn default() -> Version`

## Traits

### `Algorithm`

```rust
trait Algorithm: Send + Sync { ... }
```

How a `Tls13CipherSuite` generates `PacketKey`s and `HeaderProtectionKey`s.

#### Required Methods

- `fn packet_key(self: &Self, key: AeadKey, iv: Iv) -> Box<dyn PacketKey>`

  Produce a `PacketKey` encrypter/decrypter for this suite.

- `fn header_protection_key(self: &Self, key: AeadKey) -> Box<dyn HeaderProtectionKey>`

  Produce a `HeaderProtectionKey` encrypter/decrypter for this suite.

- `fn aead_key_len(self: &Self) -> usize`

  The length in bytes of keys for this Algorithm.

- `fn fips(self: &Self) -> bool`

  Whether this algorithm is FIPS-approved.

### `HeaderProtectionKey`

```rust
trait HeaderProtectionKey: Send + Sync { ... }
```

A QUIC header protection key

#### Required Methods

- `fn encrypt_in_place(self: &Self, sample: &[u8], first: &mut u8, packet_number: &mut [u8]) -> Result<(), Error>`

  Adds QUIC Header Protection.

- `fn decrypt_in_place(self: &Self, sample: &[u8], first: &mut u8, packet_number: &mut [u8]) -> Result<(), Error>`

  Removes QUIC Header Protection.

- `fn sample_len(self: &Self) -> usize`

  Expected sample length for the key's algorithm

### `PacketKey`

```rust
trait PacketKey: Send + Sync { ... }
```

Keys to encrypt or decrypt the payload of a packet

#### Required Methods

- `fn encrypt_in_place(self: &Self, packet_number: u64, header: &[u8], payload: &mut [u8]) -> Result<Tag, Error>`

  Encrypt a QUIC packet

- `fn encrypt_in_place_for_path(self: &Self, _path_id: u32, _packet_number: u64, _header: &[u8], _payload: &mut [u8]) -> Result<Tag, Error>`

  Encrypts a multipath QUIC packet

- `fn decrypt_in_place<'a>(self: &Self, packet_number: u64, header: &[u8], payload: &'a mut [u8]) -> Result<&'a [u8], Error>`

  Decrypt a QUIC packet

- `fn decrypt_in_place_for_path<'a>(self: &Self, _path_id: u32, _packet_number: u64, _header: &[u8], _payload: &'a mut [u8]) -> Result<&'a [u8], Error>`

  Decrypt a multipath QUIC packet

- `fn tag_len(self: &Self) -> usize`

  Tag length for the underlying AEAD algorithm

- `fn confidentiality_limit(self: &Self) -> u64`

  Number of QUIC messages that can be safely encrypted with a single key of this type.

- `fn integrity_limit(self: &Self) -> u64`

  Number of QUIC messages that can be safely decrypted with a single key of this type

