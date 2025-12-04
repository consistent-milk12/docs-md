*[rustls](../index.md) / [server](index.md)*

---

# Module `server`

Items for use in a server.

## Modules

- [`danger`](danger/index.md) - Dangerous configuration that should be audited and used with extreme care.

## Structs

### `WantsServerCert`

```rust
struct WantsServerCert {
    // [REDACTED: Private Fields]
}
```

A config builder state where the caller must supply how to provide a server certificate to
the connecting peer.

For more information, see the [`ConfigBuilder`](../index.md) documentation.

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

- `fn clone(self: &Self) -> WantsServerCert`

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

### `ResolvesServerCertUsingSni`

```rust
struct ResolvesServerCertUsingSni {
    // [REDACTED: Private Fields]
}
```

Something that resolves do different cert chains/keys based
on client-supplied server name (via SNI).

#### Implementations

- `fn new() -> Self`
  Create a new and empty (i.e., knows no certificates) resolver.

- `fn add(self: &mut Self, name: &str, ck: sign::CertifiedKey) -> Result<(), Error>`
  Add a new `sign::CertifiedKey` to be used for the given SNI `name`.

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

##### `impl ResolvesServerCert`

- `fn resolve(self: &Self, client_hello: ClientHello<'_>) -> Option<alloc::sync::Arc<sign::CertifiedKey>>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ServerSessionMemoryCache`

```rust
struct ServerSessionMemoryCache {
    // [REDACTED: Private Fields]
}
```

An implementer of `StoresServerSessions` that stores everything
in memory.  If enforces a limit on the number of stored sessions
to bound memory usage.

#### Implementations

- `fn new(size: usize) -> alloc::sync::Arc<Self>`
  Make a new ServerSessionMemoryCache.  `size` is the maximum

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

##### `impl StoresServerSessions`

- `fn put(self: &Self, key: Vec<u8>, value: Vec<u8>) -> bool`

- `fn get(self: &Self, key: &[u8]) -> Option<Vec<u8>>`

- `fn take(self: &Self, key: &[u8]) -> Option<Vec<u8>>`

- `fn can_cache(self: &Self) -> bool`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

### `AlwaysResolvesServerRawPublicKeys`

```rust
struct AlwaysResolvesServerRawPublicKeys();
```

An exemplar `ResolvesServerCert` implementation that always resolves to a single
[RFC 7250] raw public key.

[RFC 7250]: https://tools.ietf.org/html/rfc7250

#### Implementations

- `fn new(certified_key: alloc::sync::Arc<sign::CertifiedKey>) -> Self`
  Create a new `AlwaysResolvesServerRawPublicKeys` instance.

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

- `fn clone(self: &Self) -> AlwaysResolvesServerRawPublicKeys`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ResolvesServerCert`

- `fn resolve(self: &Self, _client_hello: ClientHello<'_>) -> Option<alloc::sync::Arc<sign::CertifiedKey>>`

- `fn only_raw_public_keys(self: &Self) -> bool`

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

### `NoServerSessionStorage`

```rust
struct NoServerSessionStorage {
}
```

Something which never stores sessions.

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

##### `impl StoresServerSessions`

- `fn put(self: &Self, _id: Vec<u8>, _sec: Vec<u8>) -> bool`

- `fn get(self: &Self, _id: &[u8]) -> Option<Vec<u8>>`

- `fn take(self: &Self, _id: &[u8]) -> Option<Vec<u8>>`

- `fn can_cache(self: &Self) -> bool`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Accepted`

```rust
struct Accepted {
    // [REDACTED: Private Fields]
}
```

Represents a `ClientHello` message received through the [`Acceptor`](../index.md).

Contains the state required to resume the connection through `Accepted::into_connection()`.

#### Implementations

- `fn client_hello(self: &Self) -> ClientHello<'_>`
  Get the [`ClientHello`] for this connection.

- `fn into_connection(self: Self, config: alloc::sync::Arc<ServerConfig>) -> Result<ServerConnection, (Error, AcceptedAlert)>`
  Convert the [`Accepted`] into a [`ServerConnection`].

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

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> fmt::Result`

### `ClientHello<'a>`

```rust
struct ClientHello<'a> {
    // [REDACTED: Private Fields]
}
```

A struct representing the received Client Hello

#### Implementations

- `fn server_name(self: &Self) -> Option<&str>`
  Get the server name indicator.

- `fn signature_schemes(self: &Self) -> &[SignatureScheme]`
  Get the compatible signature schemes.

- `fn alpn(self: &Self) -> Option<impl Iterator<Item = &'a [u8]>>`
  Get the ALPN protocol identifiers submitted by the client.

- `fn cipher_suites(self: &Self) -> &[CipherSuite]`
  Get cipher suites.

- `fn server_cert_types(self: &Self) -> Option<&'a [CertificateType]>`
  Get the server certificate types offered in the ClientHello.

- `fn client_cert_types(self: &Self) -> Option<&'a [CertificateType]>`
  Get the client certificate types offered in the ClientHello.

- `fn certificate_authorities(self: &Self) -> Option<&'a [DistinguishedName]>`
  Get the [certificate_authorities] extension sent by the client.

- `fn named_groups(self: &Self) -> Option<&'a [NamedGroup]>`
  Get the [`named_groups`] extension sent by the client.

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

### `ServerConfig`

```rust
struct ServerConfig {
    pub ignore_client_order: bool,
    pub max_fragment_size: Option<usize>,
    pub session_storage: alloc::sync::Arc<dyn StoresServerSessions>,
    pub ticketer: alloc::sync::Arc<dyn ProducesTickets>,
    pub cert_resolver: alloc::sync::Arc<dyn ResolvesServerCert>,
    pub alpn_protocols: alloc::vec::Vec<alloc::vec::Vec<u8>>,
    pub key_log: alloc::sync::Arc<dyn KeyLog>,
    pub enable_secret_extraction: bool,
    pub max_early_data_size: u32,
    pub send_half_rtt_data: bool,
    pub send_tls13_tickets: usize,
    pub require_ems: bool,
    pub time_provider: alloc::sync::Arc<dyn TimeProvider>,
    pub cert_compressors: alloc::vec::Vec<&'static dyn compress::CertCompressor>,
    pub cert_compression_cache: alloc::sync::Arc<compress::CompressionCache>,
    pub cert_decompressors: alloc::vec::Vec<&'static dyn compress::CertDecompressor>,
    // [REDACTED: Private Fields]
}
```

Common configuration for a set of server sessions.

Making one of these is cheap, though one of the inputs may be expensive: gathering trust roots
from the operating system to add to the [`RootCertStore`](../index.md) passed to a `ClientCertVerifier`
builder may take on the order of a few hundred milliseconds.

These must be created via the `ServerConfig::builder()` or `ServerConfig::builder_with_provider()`
function.

# Defaults

* `ServerConfig::max_fragment_size`: the default is `None` (meaning 16kB).
* `ServerConfig::session_storage`: if the `std` feature is enabled, the default stores 256
  sessions in memory. If the `std` feature is not enabled, the default is to not store any
  sessions. In a no-std context, by enabling the `hashbrown` feature you may provide your
  own `session_storage` using [`ServerSessionMemoryCache`](../index.md) and a `crate::lock::MakeMutex`
  implementation.
* `ServerConfig::alpn_protocols`: the default is empty -- no ALPN protocol is negotiated.
* `ServerConfig::key_log`: key material is not logged.
* `ServerConfig::send_tls13_tickets`: 2 tickets are sent.
* `ServerConfig::cert_compressors`: depends on the crate features, see `compress::default_cert_compressors()`.
* `ServerConfig::cert_compression_cache`: caches the most recently used 4 compressions
* `ServerConfig::cert_decompressors`: depends on the crate features, see `compress::default_cert_decompressors()`.

# Sharing resumption storage between `ServerConfig`s

In a program using many `ServerConfig`s it may improve resumption rates
(which has a significant impact on connection performance) if those
configs share `ServerConfig::session_storage` or `ServerConfig::ticketer`.

However, caution is needed: other fields influence the security of a session
and resumption between them can be surprising.  If sharing
`ServerConfig::session_storage` or `ServerConfig::ticketer` between two
`ServerConfig`s, you should also evaluate the following fields and ensure
they are equivalent:

* `ServerConfig::verifier` -- client authentication requirements,
* `ServerConfig::cert_resolver` -- server identities.

To illustrate, imagine two `ServerConfig`s `A` and `B`.  `A` requires
client authentication, `B` does not.  If `A` and `B` shared a resumption store,
it would be possible for a session originated by `B` (that is, an unauthenticated client)
to be inserted into the store, and then resumed by `A`.  This would give a false
impression to the user of `A` that the client was authenticated.  This is possible
whether the resumption is performed statefully (via `ServerConfig::session_storage`)
or statelessly (via `ServerConfig::ticketer`).

_Unlike_ `ClientConfig`, rustls does not enforce any policy here.



#### Fields

- **`ignore_client_order`**: `bool`

  Ignore the client's ciphersuite order. Instead,
  choose the top ciphersuite in the server list
  which is supported by the client.

- **`max_fragment_size`**: `Option<usize>`

  The maximum size of plaintext input to be emitted in a single TLS record.
  A value of None is equivalent to the [TLS maximum] of 16 kB.
  
  rustls enforces an arbitrary minimum of 32 bytes for this field.
  Out of range values are reported as errors from [ServerConnection::new].
  
  Setting this value to a little less than the TCP MSS may improve latency
  for stream-y workloads.
  
  [TLS maximum]: https://datatracker.ietf.org/doc/html/rfc8446#section-5.1
  [ServerConnection::new]: crate::server::ServerConnection::new

- **`session_storage`**: `alloc::sync::Arc<dyn StoresServerSessions>`

  How to store client sessions.
  
  See [ServerConfig#sharing-resumption-storage-between-serverconfigs]
  for a warning related to this field.

- **`ticketer`**: `alloc::sync::Arc<dyn ProducesTickets>`

  How to produce tickets.
  
  See [ServerConfig#sharing-resumption-storage-between-serverconfigs]
  for a warning related to this field.

- **`cert_resolver`**: `alloc::sync::Arc<dyn ResolvesServerCert>`

  How to choose a server cert and key. This is usually set by
  [ConfigBuilder::with_single_cert] or [ConfigBuilder::with_cert_resolver].
  For async applications, see also [Acceptor].

- **`alpn_protocols`**: `alloc::vec::Vec<alloc::vec::Vec<u8>>`

  Protocol names we support, most preferred first.
  If empty we don't do ALPN at all.

- **`key_log`**: `alloc::sync::Arc<dyn KeyLog>`

  How to output key material for debugging.  The default
  does nothing.

- **`enable_secret_extraction`**: `bool`

  Allows traffic secrets to be extracted after the handshake,
  e.g. for kTLS setup.

- **`max_early_data_size`**: `u32`

  Amount of early data to accept for sessions created by
  this config.  Specify 0 to disable early data.  The
  default is 0.
  
  Read the early data via `ServerConnection::early_data`.
  
  The units for this are _both_ plaintext bytes, _and_ ciphertext
  bytes, depending on whether the server accepts a client's early_data
  or not.  It is therefore recommended to include some slop in
  this value to account for the unknown amount of ciphertext
  expansion in the latter case.

- **`send_half_rtt_data`**: `bool`

  Whether the server should send "0.5RTT" data.  This means the server
  sends data after its first flight of handshake messages, without
  waiting for the client to complete the handshake.
  
  This can improve TTFB latency for either server-speaks-first protocols,
  or client-speaks-first protocols when paired with "0RTT" data.  This
  comes at the cost of a subtle weakening of the normal handshake
  integrity guarantees that TLS provides.  Note that the initial
  `ClientHello` is indirectly authenticated because it is included
  in the transcript used to derive the keys used to encrypt the data.
  
  This only applies to TLS1.3 connections.  TLS1.2 connections cannot
  do this optimisation and this setting is ignored for them.  It is
  also ignored for TLS1.3 connections that even attempt client
  authentication.
  
  This defaults to false.  This means the first application data
  sent by the server comes after receiving and validating the client's
  handshake up to the `Finished` message.  This is the safest option.

- **`send_tls13_tickets`**: `usize`

  How many TLS1.3 tickets to send immediately after a successful
  handshake.
  
  Because TLS1.3 tickets are single-use, this allows
  a client to perform multiple resumptions.
  
  The default is 2.
  
  If this is 0, no tickets are sent and clients will not be able to
  do any resumption.

- **`require_ems`**: `bool`

  If set to `true`, requires the client to support the extended
  master secret extraction method defined in [RFC 7627].
  
  The default is `true` if the "fips" crate feature is enabled,
  `false` otherwise.
  
  It must be set to `true` to meet FIPS requirement mentioned in section
  **D.Q Transition of the TLS 1.2 KDF to Support the Extended Master
  Secret** from [FIPS 140-3 IG.pdf].
  
  [RFC 7627]: https://datatracker.ietf.org/doc/html/rfc7627
  [FIPS 140-3 IG.pdf]: https://csrc.nist.gov/csrc/media/Projects/cryptographic-module-validation-program/documents/fips%20140-3/FIPS%20140-3%20IG.pdf

- **`time_provider`**: `alloc::sync::Arc<dyn TimeProvider>`

  Provides the current system time

- **`cert_compressors`**: `alloc::vec::Vec<&'static dyn compress::CertCompressor>`

  How to compress the server's certificate chain.
  
  If a client supports this extension, and advertises support
  for one of the compression algorithms included here, the
  server certificate will be compressed according to [RFC8779].
  
  This only applies to TLS1.3 connections.  It is ignored for
  TLS1.2 connections.
  
  [RFC8779]: https://datatracker.ietf.org/doc/rfc8879/

- **`cert_compression_cache`**: `alloc::sync::Arc<compress::CompressionCache>`

  Caching for compressed certificates.
  
  This is optional: `compress::CompressionCache::Disabled` gives
  a cache that does no caching.

- **`cert_decompressors`**: `alloc::vec::Vec<&'static dyn compress::CertDecompressor>`

  How to decompress the clients's certificate chain.
  
  If this is non-empty, the [RFC8779] certificate compression
  extension is offered when requesting client authentication,
  and any compressed certificates are transparently decompressed
  during the handshake.
  
  This only applies to TLS1.3 connections.  It is ignored for
  TLS1.2 connections.
  
  [RFC8779]: https://datatracker.ietf.org/doc/rfc8879/

#### Implementations

- `fn builder() -> ConfigBuilder<Self, WantsVerifier>`
  Create a builder for a server configuration with

- `fn builder_with_protocol_versions(versions: &[&'static versions::SupportedProtocolVersion]) -> ConfigBuilder<Self, WantsVerifier>`
  Create a builder for a server configuration with

- `fn builder_with_provider(provider: alloc::sync::Arc<CryptoProvider>) -> ConfigBuilder<Self, WantsVersions>`
  Create a builder for a server configuration with a specific [`CryptoProvider`].

- `fn builder_with_details(provider: alloc::sync::Arc<CryptoProvider>, time_provider: alloc::sync::Arc<dyn TimeProvider>) -> ConfigBuilder<Self, WantsVersions>`
  Create a builder for a server configuration with no default implementation details.

- `fn fips(self: &Self) -> bool`
  Return `true` if connections made with this `ServerConfig` will

- `fn crypto_provider(self: &Self) -> &alloc::sync::Arc<CryptoProvider>`
  Return the crypto provider used to construct this client configuration.

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

- `fn clone(self: &Self) -> ServerConfig`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ConfigSide`

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

### `ServerConnectionData`

```rust
struct ServerConnectionData {
    // [REDACTED: Private Fields]
}
```

State associated with a server connection.

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

##### `impl SideData`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> ServerConnectionData`

### `UnbufferedServerConnection`

```rust
struct UnbufferedServerConnection {
    // [REDACTED: Private Fields]
}
```

Unbuffered version of `ServerConnection`

See the `crate::unbuffered` module docs for more details

#### Implementations

- `fn new(config: alloc::sync::Arc<ServerConfig>) -> Result<Self, Error>`
  Make a new ServerConnection. `config` controls how we behave in the TLS protocol.

- `fn dangerous_extract_secrets(self: Self) -> Result<ExtractedSecrets, Error>`
  Extract secrets, so they can be used when configuring kTLS, for example.

- `fn dangerous_into_kernel_connection(self: Self) -> Result<(ExtractedSecrets, KernelConnection<ServerConnectionData>), Error>`
  Extract secrets and an [`KernelConnection`] object.

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

- `type Target = UnbufferedConnectionCommon<ServerConnectionData>`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

### `AcceptedAlert`

```rust
struct AcceptedAlert();
```

Represents a TLS alert resulting from handling the client's `ClientHello` message.

When `Acceptor::accept()` returns an error, it yields an `AcceptedAlert` such that the
application can communicate failure to the client via `AcceptedAlert::write()`.

#### Implementations

- `fn write(self: &mut Self, wr: &mut dyn io::Write) -> Result<usize, io::Error>`
  Send the alert to the client.

- `fn write_all(self: &mut Self, wr: &mut dyn io::Write) -> Result<(), io::Error>`
  Send the alert to the client.

#### Trait Implementations

##### `impl From`

- `fn from(conn: ConnectionCommon<ServerConnectionData>) -> Self`

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

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> fmt::Result`

### `Acceptor`

```rust
struct Acceptor {
    // [REDACTED: Private Fields]
}
```

Handle a server-side connection before configuration is available.

`Acceptor` allows the caller to choose a [`ServerConfig`](../index.md) after reading
the `super::ClientHello` of an incoming connection. This is useful for servers
that choose different certificates or cipher suites based on the
characteristics of the `ClientHello`. In particular it is useful for
servers that need to do some I/O to load a certificate and its private key
and don't want to use the blocking interface provided by
`super::ResolvesServerCert`.

Create an Acceptor with `Acceptor::default()`.

# Example

```no_run
# #[cfg(feature = "aws_lc_rs")] {
# fn choose_server_config(
#     _: rustls::server::ClientHello,
# ) -> std::sync::Arc<rustls::ServerConfig> {
#     unimplemented!();
# }
# #[allow(unused_variables)]
# fn main() {
use rustls::server::{Acceptor, ServerConfig};
let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
for stream in listener.incoming() {
    let mut stream = stream.unwrap();
    let mut acceptor = Acceptor::default();
    let accepted = loop {
        acceptor.read_tls(&mut stream).unwrap();
        if let Some(accepted) = acceptor.accept().unwrap() {
            break accepted;
        }
    };

    // For some user-defined choose_server_config:
    let config = choose_server_config(accepted.client_hello());
    let conn = accepted
        .into_connection(config)
        .unwrap();

    // Proceed with handling the ServerConnection.
}
# }
# }
```

#### Implementations

- `fn read_tls(self: &mut Self, rd: &mut dyn io::Read) -> Result<usize, io::Error>`
  Read TLS content from `rd`.

- `fn accept(self: &mut Self) -> Result<Option<Accepted>, (Error, AcceptedAlert)>`
  Check if a `ClientHello` message has been received.

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

##### `impl Default`

- `fn default() -> Self`
  Return an empty Acceptor, ready to receive bytes from a new client connection.

### `ReadEarlyData<'a>`

```rust
struct ReadEarlyData<'a> {
    // [REDACTED: Private Fields]
}
```

Allows reading of early data in resumed TLS1.3 connections.

"Early data" is also known as "0-RTT data".

This structure implements `std::io::Read`.

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

##### `impl Read`

- `fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ServerConnection`

```rust
struct ServerConnection {
    // [REDACTED: Private Fields]
}
```

This represents a single TLS server connection.

Send TLS-protected data to the peer using the `io::Write` trait implementation.
Read data from the peer using the `io::Read` trait implementation.

#### Implementations

- `fn new(config: alloc::sync::Arc<ServerConfig>) -> Result<Self, Error>`
  Make a new ServerConnection.  `config` controls how

- `fn server_name(self: &Self) -> Option<&str>`
  Retrieves the server name, if any, used to select the certificate and

- `fn received_resumption_data(self: &Self) -> Option<&[u8]>`
  Application-controlled portion of the resumption ticket supplied by the client, if any.

- `fn set_resumption_data(self: &mut Self, data: &[u8])`
  Set the resumption data to embed in future resumption tickets supplied to the client.

- `fn reject_early_data(self: &mut Self)`
  Explicitly discard early data, notifying the client

- `fn early_data(self: &mut Self) -> Option<ReadEarlyData<'_>>`
  Returns an `io::Read` implementer you can read bytes from that are

- `fn fips(self: &Self) -> bool`
  Return true if the connection was made with a `ServerConfig` that is FIPS compatible.

- `fn dangerous_extract_secrets(self: Self) -> Result<ExtractedSecrets, Error>`
  Extract secrets, so they can be used when configuring kTLS, for example.

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

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl Deref`

- `type Target = ConnectionCommon<ServerConnectionData>`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

### `NoClientAuth`

```rust
struct NoClientAuth;
```

Turns off client authentication.

In contrast to using
`WebPkiClientVerifier::builder(roots).allow_unauthenticated().build()`, the `NoClientAuth`
`ClientCertVerifier` will not offer client authentication at all, vs offering but not
requiring it.

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

##### `impl ClientCertVerifier`

- `fn offer_client_auth(self: &Self) -> bool`

- `fn root_hint_subjects(self: &Self) -> &[DistinguishedName]`

- `fn verify_client_cert(self: &Self, _end_entity: &CertificateDer<'_>, _intermediates: &[CertificateDer<'_>], _now: UnixTime) -> Result<ClientCertVerified, Error>`

- `fn verify_tls12_signature(self: &Self, _message: &[u8], _cert: &CertificateDer<'_>, _dss: &DigitallySignedStruct) -> Result<HandshakeSignatureValid, Error>`

- `fn verify_tls13_signature(self: &Self, _message: &[u8], _cert: &CertificateDer<'_>, _dss: &DigitallySignedStruct) -> Result<HandshakeSignatureValid, Error>`

- `fn supported_verify_schemes(self: &Self) -> Vec<SignatureScheme>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ClientCertVerifierBuilder`

```rust
struct ClientCertVerifierBuilder {
    // [REDACTED: Private Fields]
}
```

A builder for configuring a `webpki` client certificate verifier.

For more information, see the [`WebPkiClientVerifier`](../index.md) documentation.

#### Implementations

- `fn clear_root_hint_subjects(self: Self) -> Self`
  Clear the list of trust anchor hint subjects.

- `fn add_root_hint_subjects(self: Self, subjects: impl IntoIterator<Item = DistinguishedName>) -> Self`
  Add additional [`DistinguishedName`]s to the list of trust anchor hint subjects.

- `fn with_crls(self: Self, crls: impl IntoIterator<Item = CertificateRevocationListDer<'static>>) -> Self`
  Verify the revocation state of presented client certificates against the provided

- `fn only_check_end_entity_revocation(self: Self) -> Self`
  Only check the end entity certificate revocation status when using CRLs.

- `fn allow_unauthenticated(self: Self) -> Self`
  Allow unauthenticated clients to connect.

- `fn allow_unknown_revocation_status(self: Self) -> Self`
  Allow unknown certificate revocation status when using CRLs.

- `fn enforce_revocation_expiration(self: Self) -> Self`
  Enforce the CRL nextUpdate field (i.e. expiration)

- `fn build(self: Self) -> Result<alloc::sync::Arc<dyn ClientCertVerifier>, VerifierBuilderError>`
  Build a client certificate verifier. The built verifier will be used for the server to offer

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

- `fn clone(self: &Self) -> ClientCertVerifierBuilder`

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

### `ParsedCertificate<'a>`

```rust
struct ParsedCertificate<'a>();
```

Wrapper around internal representation of a parsed certificate.

This is used in order to avoid parsing twice when specifying custom verification

#### Implementations

- `fn subject_public_key_info(self: &Self) -> SubjectPublicKeyInfoDer<'static>`
  Get the parsed certificate's SubjectPublicKeyInfo (SPKI)

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

##### `impl TryFrom<'a>`

- `type Error = Error`

- `fn try_from(value: &'a CertificateDer<'a>) -> Result<Self, <Self as >::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `WebPkiClientVerifier`

```rust
struct WebPkiClientVerifier {
    // [REDACTED: Private Fields]
}
```

A client certificate verifier that uses the `webpki` crate[^1] to perform client certificate
validation.

It must be created via the `WebPkiClientVerifier::builder()` or
`WebPkiClientVerifier::builder_with_provider()` functions.

Once built, the provided `Arc<dyn ClientCertVerifier>` can be used with a Rustls [`ServerConfig`](../index.md)
to configure client certificate validation using [`with_client_cert_verifier`][ConfigBuilder<ClientConfig, WantsVerifier>::with_client_cert_verifier].

Example:

To require all clients present a client certificate issued by a trusted CA:
```no_run
# #[cfg(any(feature = "ring", feature = "aws_lc_rs"))] {
# use rustls::RootCertStore;
# use rustls::server::WebPkiClientVerifier;
# let roots = RootCertStore::empty();
let client_verifier = WebPkiClientVerifier::builder(roots.into())
  .build()
  .unwrap();
# }
```

Or, to allow clients presenting a client certificate authenticated by a trusted CA, or
anonymous clients that present no client certificate:
```no_run
# #[cfg(any(feature = "ring", feature = "aws_lc_rs"))] {
# use rustls::RootCertStore;
# use rustls::server::WebPkiClientVerifier;
# let roots = RootCertStore::empty();
let client_verifier = WebPkiClientVerifier::builder(roots.into())
  .allow_unauthenticated()
  .build()
  .unwrap();
# }
```

If you wish to disable advertising client authentication:
```no_run
# use rustls::RootCertStore;
# use rustls::server::WebPkiClientVerifier;
# let roots = RootCertStore::empty();
let client_verifier = WebPkiClientVerifier::no_client_auth();
```

You can also configure the client verifier to check for certificate revocation with
client certificate revocation lists (CRLs):
```no_run
# #[cfg(any(feature = "ring", feature = "aws_lc_rs"))] {
# use rustls::RootCertStore;
# use rustls::server::{WebPkiClientVerifier};
# let roots = RootCertStore::empty();
# let crls = Vec::new();
let client_verifier = WebPkiClientVerifier::builder(roots.into())
  .with_crls(crls)
  .build()
  .unwrap();
# }
```

[^1]: <https://github.com/rustls/webpki>

#### Implementations

- `fn builder(roots: alloc::sync::Arc<RootCertStore>) -> ClientCertVerifierBuilder`
  Create a builder for the `webpki` client certificate verifier configuration using

- `fn builder_with_provider(roots: alloc::sync::Arc<RootCertStore>, provider: alloc::sync::Arc<CryptoProvider>) -> ClientCertVerifierBuilder`
  Create a builder for the `webpki` client certificate verifier configuration using

- `fn no_client_auth() -> alloc::sync::Arc<dyn ClientCertVerifier>`
  Create a new `WebPkiClientVerifier` that disables client authentication. The server will

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

##### `impl ClientCertVerifier`

- `fn offer_client_auth(self: &Self) -> bool`

- `fn client_auth_mandatory(self: &Self) -> bool`

- `fn root_hint_subjects(self: &Self) -> &[DistinguishedName]`

- `fn verify_client_cert(self: &Self, end_entity: &CertificateDer<'_>, intermediates: &[CertificateDer<'_>], now: UnixTime) -> Result<ClientCertVerified, Error>`

- `fn verify_tls12_signature(self: &Self, message: &[u8], cert: &CertificateDer<'_>, dss: &DigitallySignedStruct) -> Result<HandshakeSignatureValid, Error>`

- `fn verify_tls13_signature(self: &Self, message: &[u8], cert: &CertificateDer<'_>, dss: &DigitallySignedStruct) -> Result<HandshakeSignatureValid, Error>`

- `fn supported_verify_schemes(self: &Self) -> Vec<SignatureScheme>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Enums

### `CertificateType`

```rust
enum CertificateType {
    X509,
    RawPublicKey,
    Unknown(u8),
}
```

The `CertificateType` enum sent in the cert_type extensions.
Values in this enum are taken from the various RFCs covering TLS, and are listed by IANA.

[RFC 6091 Section 5]: <https://datatracker.ietf.org/doc/html/rfc6091#section-5>
[RFC 7250 Section 7]: <https://datatracker.ietf.org/doc/html/rfc7250#section-7>

#### Implementations

- `fn to_array(self: Self) -> [u8; 1]`

- `fn as_str(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl From`

- `fn from(x: u8) -> Self`

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

- `fn clone(self: &Self) -> CertificateType`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Codec`

- `fn encode(self: &Self, bytes: &mut alloc::vec::Vec<u8>)`

- `fn read(r: &mut Reader<'_>) -> Result<Self, crate::error::InvalidMessage>`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &CertificateType) -> bool`

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

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default`

- `fn default() -> CertificateType`

### `VerifierBuilderError`

```rust
enum VerifierBuilderError {
    NoRootAnchors,
    InvalidCrl(crate::error::CertRevocationListError),
}
```

An error that can occur when building a certificate verifier.

#### Variants

- **`NoRootAnchors`**

  No root trust anchors were provided.

- **`InvalidCrl`**

  A provided CRL could not be parsed.

#### Trait Implementations

##### `impl From`

- `fn from(value: CertRevocationListError) -> Self`

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

- `fn clone(self: &Self) -> VerifierBuilderError`

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

## Traits

