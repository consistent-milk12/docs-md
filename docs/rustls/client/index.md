*[rustls](../index.md) / [client](index.md)*

---

# Module `client`

Items for use in a client.

## Modules

- [`danger`](danger/index.md) - Dangerous configuration that should be audited and used with extreme care.

## Structs

### `WantsClientCert`

```rust
struct WantsClientCert {
    // [REDACTED: Private Fields]
}
```

A config builder state where the caller needs to supply whether and how to provide a client
certificate.

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

- `fn clone(self: &Self) -> WantsClientCert`

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

### `ClientConfig`

```rust
struct ClientConfig {
    pub alpn_protocols: alloc::vec::Vec<alloc::vec::Vec<u8>>,
    pub resumption: Resumption,
    pub max_fragment_size: Option<usize>,
    pub client_auth_cert_resolver: alloc::sync::Arc<dyn ResolvesClientCert>,
    pub enable_sni: bool,
    pub key_log: alloc::sync::Arc<dyn KeyLog>,
    pub enable_secret_extraction: bool,
    pub enable_early_data: bool,
    pub require_ems: bool,
    pub time_provider: alloc::sync::Arc<dyn TimeProvider>,
    pub cert_decompressors: alloc::vec::Vec<&'static dyn compress::CertDecompressor>,
    pub cert_compressors: alloc::vec::Vec<&'static dyn compress::CertCompressor>,
    pub cert_compression_cache: alloc::sync::Arc<compress::CompressionCache>,
    // [REDACTED: Private Fields]
}
```

Common configuration for (typically) all connections made by a program.

Making one of these is cheap, though one of the inputs may be expensive: gathering trust roots
from the operating system to add to the [`RootCertStore`](../index.md) passed to `with_root_certificates()`
(the rustls-native-certs crate is often used for this) may take on the order of a few hundred
milliseconds.

These must be created via the `ClientConfig::builder()` or `ClientConfig::builder_with_provider()`
function.

Note that using `ConfigBuilder<ClientConfig, WantsVersions>::with_ech()` will produce a common
configuration specific to the provided `crate::client::EchConfig` that may not be appropriate
for all connections made by the program. In this case the configuration should only be shared
by connections intended for domains that offer the provided `crate::client::EchConfig` in
their DNS zone.

# Defaults

* `ClientConfig::max_fragment_size`: the default is `None` (meaning 16kB).
* `ClientConfig::resumption`: supports resumption with up to 256 server names, using session
  ids or tickets, with a max of eight tickets per server.
* `ClientConfig::alpn_protocols`: the default is empty -- no ALPN protocol is negotiated.
* `ClientConfig::key_log`: key material is not logged.
* `ClientConfig::cert_decompressors`: depends on the crate features, see `compress::default_cert_decompressors()`.
* `ClientConfig::cert_compressors`: depends on the crate features, see `compress::default_cert_compressors()`.
* `ClientConfig::cert_compression_cache`: caches the most recently used 4 compressions


#### Fields

- **`alpn_protocols`**: `alloc::vec::Vec<alloc::vec::Vec<u8>>`

  Which ALPN protocols we include in our client hello.
  If empty, no ALPN extension is sent.

- **`resumption`**: `Resumption`

  How and when the client can resume a previous session.
  
  # Sharing `resumption` between `ClientConfig`s
  In a program using many `ClientConfig`s it may improve resumption rates
  (which has a significant impact on connection performance) if those
  configs share a single `Resumption`.
  
  However, resumption is only allowed between two `ClientConfig`s if their
  `client_auth_cert_resolver` (ie, potential client authentication credentials)
  and `verifier` (ie, server certificate verification settings) are
  the same (according to `Arc::ptr_eq`).
  
  To illustrate, imagine two `ClientConfig`s `A` and `B`.  `A` fully validates
  the server certificate, `B` does not.  If `A` and `B` shared a resumption store,
  it would be possible for a session originated by `B` to be inserted into the
  store, and then resumed by `A`.  This would give a false impression to the user
  of `A` that the server certificate is fully validated.

- **`max_fragment_size`**: `Option<usize>`

  The maximum size of plaintext input to be emitted in a single TLS record.
  A value of None is equivalent to the [TLS maximum] of 16 kB.
  
  rustls enforces an arbitrary minimum of 32 bytes for this field.
  Out of range values are reported as errors from [ClientConnection::new].
  
  Setting this value to a little less than the TCP MSS may improve latency
  for stream-y workloads.
  
  [TLS maximum]: https://datatracker.ietf.org/doc/html/rfc8446#section-5.1
  [ClientConnection::new]: crate::client::ClientConnection::new

- **`client_auth_cert_resolver`**: `alloc::sync::Arc<dyn ResolvesClientCert>`

  How to decide what client auth certificate/keys to use.

- **`enable_sni`**: `bool`

  Whether to send the Server Name Indication (SNI) extension
  during the client handshake.
  
  The default is true.

- **`key_log`**: `alloc::sync::Arc<dyn KeyLog>`

  How to output key material for debugging.  The default
  does nothing.

- **`enable_secret_extraction`**: `bool`

  Allows traffic secrets to be extracted after the handshake,
  e.g. for kTLS setup.

- **`enable_early_data`**: `bool`

  Whether to send data on the first flight ("early data") in
  TLS 1.3 handshakes.
  
  The default is false.

- **`require_ems`**: `bool`

  If set to `true`, requires the server to support the extended
  master secret extraction method defined in [RFC 7627].
  
  The default is `true` if the `fips` crate feature is enabled,
  `false` otherwise.
  
  It must be set to `true` to meet FIPS requirement mentioned in section
  **D.Q Transition of the TLS 1.2 KDF to Support the Extended Master
  Secret** from [FIPS 140-3 IG.pdf].
  
  [RFC 7627]: https://datatracker.ietf.org/doc/html/rfc7627
  [FIPS 140-3 IG.pdf]: https://csrc.nist.gov/csrc/media/Projects/cryptographic-module-validation-program/documents/fips%20140-3/FIPS%20140-3%20IG.pdf

- **`time_provider`**: `alloc::sync::Arc<dyn TimeProvider>`

  Provides the current system time

- **`cert_decompressors`**: `alloc::vec::Vec<&'static dyn compress::CertDecompressor>`

  How to decompress the server's certificate chain.
  
  If this is non-empty, the [RFC8779] certificate compression
  extension is offered, and any compressed certificates are
  transparently decompressed during the handshake.
  
  This only applies to TLS1.3 connections.  It is ignored for
  TLS1.2 connections.
  
  [RFC8779]: https://datatracker.ietf.org/doc/rfc8879/

- **`cert_compressors`**: `alloc::vec::Vec<&'static dyn compress::CertCompressor>`

  How to compress the client's certificate chain.
  
  If a server supports this extension, and advertises support
  for one of the compression algorithms included here, the
  client certificate will be compressed according to [RFC8779].
  
  This only applies to TLS1.3 connections.  It is ignored for
  TLS1.2 connections.
  
  [RFC8779]: https://datatracker.ietf.org/doc/rfc8879/

- **`cert_compression_cache`**: `alloc::sync::Arc<compress::CompressionCache>`

  Caching for compressed certificates.
  
  This is optional: `compress::CompressionCache::Disabled` gives
  a cache that does no caching.

#### Implementations

- `fn builder() -> ConfigBuilder<Self, WantsVerifier>`
  Create a builder for a client configuration with

- `fn builder_with_protocol_versions(versions: &[&'static versions::SupportedProtocolVersion]) -> ConfigBuilder<Self, WantsVerifier>`
  Create a builder for a client configuration with

- `fn builder_with_provider(provider: alloc::sync::Arc<CryptoProvider>) -> ConfigBuilder<Self, WantsVersions>`
  Create a builder for a client configuration with a specific [`CryptoProvider`].

- `fn builder_with_details(provider: alloc::sync::Arc<CryptoProvider>, time_provider: alloc::sync::Arc<dyn TimeProvider>) -> ConfigBuilder<Self, WantsVersions>`
  Create a builder for a client configuration with no default implementation details.

- `fn fips(self: &Self) -> bool`
  Return true if connections made with this `ClientConfig` will

- `fn crypto_provider(self: &Self) -> &alloc::sync::Arc<CryptoProvider>`
  Return the crypto provider used to construct this client configuration.

- `fn dangerous(self: &mut Self) -> danger::DangerousClientConfig<'_>`
  Access configuration options whose use is dangerous and requires

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

- `fn clone(self: &Self) -> ClientConfig`

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

### `ClientConnectionData`

```rust
struct ClientConnectionData {
    // [REDACTED: Private Fields]
}
```

State associated with a client connection.

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

### `Resumption`

```rust
struct Resumption {
    // [REDACTED: Private Fields]
}
```

Configuration for how/when a client is allowed to resume a previous session.

#### Implementations

- `fn in_memory_sessions(num: usize) -> Self`
  Create a new `Resumption` that stores data for the given number of sessions in memory.

- `fn store(store: alloc::sync::Arc<dyn ClientSessionStore>) -> Self`
  Use a custom [`ClientSessionStore`] implementation to store sessions.

- `fn disabled() -> Self`
  Disable all use of session resumption.

- `fn tls12_resumption(self: Self, tls12: Tls12Resumption) -> Self`
  Configure whether TLS 1.2 sessions may be resumed, and by what mechanism.

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

- `fn clone(self: &Self) -> Resumption`

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

##### `impl Default`

- `fn default() -> Self`
  Create an in-memory session store resumption with up to 256 server names, allowing

### `UnbufferedClientConnection`

```rust
struct UnbufferedClientConnection {
    // [REDACTED: Private Fields]
}
```

Unbuffered version of `ClientConnection`

See the `crate::unbuffered` module docs for more details

#### Implementations

- `fn new(config: alloc::sync::Arc<ClientConfig>, name: ServerName<'static>) -> Result<Self, Error>`
  Make a new ClientConnection. `config` controls how we behave in the TLS protocol, `name` is

- `fn new_with_alpn(config: alloc::sync::Arc<ClientConfig>, name: ServerName<'static>, alpn_protocols: Vec<Vec<u8>>) -> Result<Self, Error>`
  Make a new UnbufferedClientConnection with custom ALPN protocols.

- `fn dangerous_extract_secrets(self: Self) -> Result<ExtractedSecrets, Error>`
  Extract secrets, so they can be used when configuring kTLS, for example.

- `fn dangerous_into_kernel_connection(self: Self) -> Result<(ExtractedSecrets, KernelConnection<ClientConnectionData>), Error>`
  Extract secrets and a [`KernelConnection`] object.

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

##### `impl Deref`

- `type Target = UnbufferedConnectionCommon<ClientConnectionData>`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

### `ClientConnection`

```rust
struct ClientConnection {
    // [REDACTED: Private Fields]
}
```

This represents a single TLS client connection.

#### Implementations

- `fn new(config: alloc::sync::Arc<ClientConfig>, name: ServerName<'static>) -> Result<Self, Error>`
  Make a new ClientConnection.  `config` controls how

- `fn new_with_alpn(config: alloc::sync::Arc<ClientConfig>, name: ServerName<'static>, alpn_protocols: Vec<Vec<u8>>) -> Result<Self, Error>`
  Make a new ClientConnection with custom ALPN protocols.

- `fn early_data(self: &mut Self) -> Option<WriteEarlyData<'_>>`
  Returns an `io::Write` implementer you can write bytes to

- `fn is_early_data_accepted(self: &Self) -> bool`
  Returns True if the server signalled it will process early data.

- `fn dangerous_extract_secrets(self: Self) -> Result<ExtractedSecrets, Error>`
  Extract secrets, so they can be used when configuring kTLS, for example.

- `fn ech_status(self: &Self) -> EchStatus`
  Return the connection's Encrypted Client Hello (ECH) status.

- `fn tls13_tickets_received(self: &Self) -> u32`
  Returns the number of TLS1.3 tickets that have been received.

- `fn fips(self: &Self) -> bool`
  Return true if the connection was made with a `ClientConfig` that is FIPS compatible.

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

### `WriteEarlyData<'a>`

```rust
struct WriteEarlyData<'a> {
    // [REDACTED: Private Fields]
}
```

Stub that implements io::Write and dispatches to `write_early_data`.

#### Implementations

- `fn bytes_left(self: &Self) -> usize`
  How many bytes you may send.  Writes will become short

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

##### `impl Write`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

### `EchConfig`

```rust
struct EchConfig {
    // [REDACTED: Private Fields]
}
```

Configuration for performing encrypted client hello.

Note: differs from the protocol-encoded EchConfig (`EchConfigMsg`).

#### Implementations

- `fn new(ech_config_list: EchConfigListBytes<'_>, hpke_suites: &[&'static dyn Hpke]) -> Result<Self, Error>`
  Construct an EchConfig by selecting a ECH config from the provided bytes that is compatible

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

- `fn clone(self: &Self) -> EchConfig`

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

### `EchGreaseConfig`

```rust
struct EchGreaseConfig {
    // [REDACTED: Private Fields]
}
```

Configuration for GREASE Encrypted Client Hello.

#### Implementations

- `fn new(suite: &'static dyn Hpke, placeholder_key: HpkePublicKey) -> Self`
  Construct a GREASE ECH configuration.

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

- `fn clone(self: &Self) -> EchGreaseConfig`

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

### `AlwaysResolvesClientRawPublicKeys`

```rust
struct AlwaysResolvesClientRawPublicKeys();
```

An exemplar `ResolvesClientCert` implementation that always resolves to a single
[RFC 7250] raw public key.

[RFC 7250]: https://tools.ietf.org/html/rfc7250

#### Implementations

- `fn new(certified_key: alloc::sync::Arc<sign::CertifiedKey>) -> Self`
  Create a new `AlwaysResolvesClientRawPublicKeys` instance.

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

- `fn clone(self: &Self) -> AlwaysResolvesClientRawPublicKeys`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ResolvesClientCert`

- `fn resolve(self: &Self, _root_hint_subjects: &[&[u8]], _sigschemes: &[SignatureScheme]) -> Option<alloc::sync::Arc<sign::CertifiedKey>>`

- `fn only_raw_public_keys(self: &Self) -> bool`

- `fn has_certs(self: &Self) -> bool`
  Returns true if the resolver is ready to present an identity.

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

### `ClientSessionMemoryCache`

```rust
struct ClientSessionMemoryCache {
    // [REDACTED: Private Fields]
}
```

An implementer of `ClientSessionStore` that stores everything
in memory.

It enforces a limit on the number of entries to bound memory usage.

#### Implementations

- `fn new(size: usize) -> Self`
  Make a new ClientSessionMemoryCache.  `size` is the

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

##### `impl ClientSessionStore`

- `fn set_kx_hint(self: &Self, server_name: ServerName<'static>, group: NamedGroup)`

- `fn kx_hint(self: &Self, server_name: &ServerName<'_>) -> Option<NamedGroup>`

- `fn set_tls12_session(self: &Self, _server_name: ServerName<'static>, _value: persist::Tls12ClientSessionValue)`

- `fn tls12_session(self: &Self, _server_name: &ServerName<'_>) -> Option<persist::Tls12ClientSessionValue>`

- `fn remove_tls12_session(self: &Self, _server_name: &ServerName<'static>)`

- `fn insert_tls13_ticket(self: &Self, server_name: ServerName<'static>, value: persist::Tls13ClientSessionValue)`

- `fn take_tls13_ticket(self: &Self, server_name: &ServerName<'static>) -> Option<persist::Tls13ClientSessionValue>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Tls12ClientSessionValue`

```rust
struct Tls12ClientSessionValue {
    // [REDACTED: Private Fields]
}
```

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

- `fn clone(self: &Self) -> Tls12ClientSessionValue`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Receiver<P, T>`

- `type Target = T`

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

##### `impl Deref`

- `type Target = ClientSessionCommon`

- `fn deref(self: &Self) -> &<Self as >::Target`

### `Tls13ClientSessionValue`

```rust
struct Tls13ClientSessionValue {
    // [REDACTED: Private Fields]
}
```

#### Implementations

- `fn max_early_data_size(self: &Self) -> u32`

- `fn suite(self: &Self) -> &'static Tls13CipherSuite`

- `fn set_quic_params(self: &mut Self, quic_params: &[u8])`

- `fn quic_params(self: &Self) -> Vec<u8>`

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

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deref`

- `type Target = ClientSessionCommon`

- `fn deref(self: &Self) -> &<Self as >::Target`

### `ServerCertVerifierBuilder`

```rust
struct ServerCertVerifierBuilder {
    // [REDACTED: Private Fields]
}
```

A builder for configuring a `webpki` server certificate verifier.

For more information, see the [`WebPkiServerVerifier`](../index.md) documentation.

#### Implementations

- `fn with_crls(self: Self, crls: impl IntoIterator<Item = CertificateRevocationListDer<'static>>) -> Self`
  Verify the revocation state of presented client certificates against the provided

- `fn only_check_end_entity_revocation(self: Self) -> Self`
  Only check the end entity certificate revocation status when using CRLs.

- `fn allow_unknown_revocation_status(self: Self) -> Self`
  Allow unknown certificate revocation status when using CRLs.

- `fn enforce_revocation_expiration(self: Self) -> Self`
  Enforce the CRL nextUpdate field (i.e. expiration)

- `fn build(self: Self) -> Result<alloc::sync::Arc<WebPkiServerVerifier>, VerifierBuilderError>`
  Build a server certificate verifier, allowing control over the root certificates to use as

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

- `fn clone(self: &Self) -> ServerCertVerifierBuilder`

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

### `WebPkiServerVerifier`

```rust
struct WebPkiServerVerifier {
    // [REDACTED: Private Fields]
}
```

Default `ServerCertVerifier`, see the trait impl for more information.

#### Implementations

- `fn builder(roots: alloc::sync::Arc<RootCertStore>) -> ServerCertVerifierBuilder`
  Create a builder for the `webpki` server certificate verifier configuration using

- `fn builder_with_provider(roots: alloc::sync::Arc<RootCertStore>, provider: alloc::sync::Arc<CryptoProvider>) -> ServerCertVerifierBuilder`
  Create a builder for the `webpki` server certificate verifier configuration using

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

##### `impl ServerCertVerifier`

- `fn verify_server_cert(self: &Self, end_entity: &CertificateDer<'_>, intermediates: &[CertificateDer<'_>], server_name: &ServerName<'_>, ocsp_response: &[u8], now: UnixTime) -> Result<ServerCertVerified, Error>`
  Will verify the certificate is valid in the following ways:

- `fn verify_tls12_signature(self: &Self, message: &[u8], cert: &CertificateDer<'_>, dss: &DigitallySignedStruct) -> Result<HandshakeSignatureValid, Error>`

- `fn verify_tls13_signature(self: &Self, message: &[u8], cert: &CertificateDer<'_>, dss: &DigitallySignedStruct) -> Result<HandshakeSignatureValid, Error>`

- `fn supported_verify_schemes(self: &Self) -> Vec<SignatureScheme>`

## Enums

### `EarlyDataError`

```rust
enum EarlyDataError {
    ExceededAllowedEarlyData,
    Encrypt(crate::unbuffered::EncryptError),
}
```

Errors that may arise when encrypting early (RTT-0) data

#### Variants

- **`ExceededAllowedEarlyData`**

  Cannot encrypt more early data due to imposed limits

- **`Encrypt`**

  Encryption error

#### Trait Implementations

##### `impl From`

- `fn from(v: EncryptError) -> Self`

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

##### `impl Error`

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

### `Tls12Resumption`

```rust
enum Tls12Resumption {
    Disabled,
    SessionIdOnly,
    SessionIdOrTickets,
}
```

What mechanisms to support for resuming a TLS 1.2 session.

#### Variants

- **`Disabled`**

  Disable 1.2 resumption.

- **`SessionIdOnly`**

  Support 1.2 resumption using session ids only.

- **`SessionIdOrTickets`**

  Support 1.2 resumption using session ids or RFC 5077 tickets.
  
  See[^1] for why you might like to disable RFC 5077 by instead choosing the `SessionIdOnly`
  option. Note that TLS 1.3 tickets do not have those issues.
  
  [^1]: <https://words.filippo.io/we-need-to-talk-about-session-tickets/>

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

- `fn clone(self: &Self) -> Tls12Resumption`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Tls12Resumption) -> bool`

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

### `EchMode`

```rust
enum EchMode {
    Enable(EchConfig),
    Grease(EchGreaseConfig),
}
```

Controls how Encrypted Client Hello (ECH) is used in a client handshake.

#### Variants

- **`Enable`**

  ECH is enabled and the ClientHello will be encrypted based on the provided
  configuration.

- **`Grease`**

  No ECH configuration is available but the client should act as though it were.
  
  This is an anti-ossification measure, sometimes referred to as "GREASE"[^0].
  [^0]: <https://www.rfc-editor.org/rfc/rfc8701>

#### Implementations

- `fn fips(self: &Self) -> bool`
  Returns true if the ECH mode will use a FIPS approved HPKE suite.

#### Trait Implementations

##### `impl From`

- `fn from(config: EchGreaseConfig) -> Self`

##### `impl From`

- `fn from(config: EchConfig) -> Self`

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

- `fn clone(self: &Self) -> EchMode`

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

### `EchStatus`

```rust
enum EchStatus {
    NotOffered,
    Grease,
    Offered,
    Accepted,
    Rejected,
}
```

An enum representing ECH offer status.

#### Variants

- **`NotOffered`**

  ECH was not offered - it is a normal TLS handshake.

- **`Grease`**

  GREASE ECH was sent. This is not considered offering ECH.

- **`Offered`**

  ECH was offered but we do not yet know whether the offer was accepted or rejected.

- **`Accepted`**

  ECH was offered and the server accepted.

- **`Rejected`**

  ECH was offered and the server rejected.

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

- `fn clone(self: &Self) -> EchStatus`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &EchStatus) -> bool`

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

## Functions

