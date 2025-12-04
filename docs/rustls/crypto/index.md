*[rustls](../index.md) / [crypto](index.md)*

---

# Module `crypto`

Crypto provider interface.

## Modules

- [`ring`](ring/index.md) - *ring* based CryptoProvider.
- [`cipher`](cipher/index.md) - TLS message encryption/decryption interfaces.
- [`hash`](hash/index.md) - Hashing interfaces.
- [`hmac`](hmac/index.md) - HMAC interfaces.
- [`tls12`](tls12/index.md) - Cryptography specific to TLS1.2.
- [`tls13`](tls13/index.md) - Cryptography specific to TLS1.3.
- [`hpke`](hpke/index.md) - Hybrid public key encryption (RFC 9180).

## Structs

### `WebPkiSupportedAlgorithms`

```rust
struct WebPkiSupportedAlgorithms {
    pub all: &'static [&'static dyn SignatureVerificationAlgorithm],
    pub mapping: &'static [(crate::enums::SignatureScheme, &'static [&'static dyn SignatureVerificationAlgorithm])],
}
```

Describes which `webpki` signature verification algorithms are supported and
how they map to TLS [`SignatureScheme`](#signaturescheme)s.

#### Fields

- **`all`**: `&'static [&'static dyn SignatureVerificationAlgorithm]`

  A list of all supported signature verification algorithms.
  
  Used for verifying certificate chains.
  
  The order of this list is not significant.

- **`mapping`**: `&'static [(crate::enums::SignatureScheme, &'static [&'static dyn SignatureVerificationAlgorithm])]`

  A mapping from TLS `SignatureScheme`s to matching webpki signature verification algorithms.
  
  This is one (`SignatureScheme`) to many ([`SignatureVerificationAlgorithm`](../../rustls_pki_types/rustls_pki_types/index.md)) because
  (depending on the protocol version) there is not necessary a 1-to-1 mapping.
  
  For TLS1.2, all `SignatureVerificationAlgorithm`s are tried in sequence.
  
  For TLS1.3, only the first is tried.
  
  The supported schemes in this mapping is communicated to the peer and the order is significant.
  The first mapping is our highest preference.

#### Implementations

- `fn supported_schemes(self: &Self) -> Vec<SignatureScheme>`
  Return all the `scheme` items in `mapping`, maintaining order.

- `fn fips(self: &Self) -> bool`
  Return `true` if all cryptography is FIPS-approved.

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

- `fn clone(self: &Self) -> WebPkiSupportedAlgorithms`

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

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `GetRandomFailed`

```rust
struct GetRandomFailed;
```

Random material generation failed.

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

### `CipherSuiteCommon`

```rust
struct CipherSuiteCommon {
    pub suite: crate::enums::CipherSuite,
    pub hash_provider: &'static dyn crypto::hash::Hash,
    pub confidentiality_limit: u64,
}
```

Common state for cipher suites (both for TLS 1.2 and TLS 1.3)

#### Fields

- **`suite`**: `crate::enums::CipherSuite`

  The TLS enumeration naming this cipher suite.

- **`hash_provider`**: `&'static dyn crypto::hash::Hash`

  Which hash function the suite uses.

- **`confidentiality_limit`**: `u64`

  Number of TCP-TLS messages that can be safely encrypted with a single key of this type
  
  Once a `MessageEncrypter` produced for this suite has encrypted more than
  `confidentiality_limit` messages, an attacker gains an advantage in distinguishing it
  from an ideal pseudorandom permutation (PRP).
  
  This is to be set on the assumption that messages are maximally sized --
  each is 2<sup>14</sup> bytes. It **does not** consider confidentiality limits for
  QUIC connections - see the [`quic::PacketKey::confidentiality_limit`](#confidentiality-limit) field for
  this context.
  
  For AES-GCM implementations, this should be set to 2<sup>24</sup> to limit attack
  probability to one in 2<sup>60</sup>.  See [AEBounds] (Table 1) and [draft-irtf-aead-limits-08]:
  
  ```python
  >>> p = 2 ** -60
  >>> L = (2 ** 14 // 16) + 1
  >>> qlim = (math.sqrt(p) * (2 ** (129 // 2)) - 1) / (L + 1)
  >>> print(int(qlim).bit_length())
  24
  ```
  [AEBounds]: https://eprint.iacr.org/2024/051.pdf
  [draft-irtf-aead-limits-08]: https://www.ietf.org/archive/id/draft-irtf-cfrg-aead-limits-08.html#section-5.1.1
  
  For chacha20-poly1305 implementations, this should be set to `u64::MAX`:
  see <https://www.ietf.org/archive/id/draft-irtf-cfrg-aead-limits-08.html#section-5.2.1>

#### Implementations

- `fn fips(self: &Self) -> bool`
  Return `true` if this is backed by a FIPS-approved implementation.

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

### `CryptoProvider`

```rust
struct CryptoProvider {
    pub cipher_suites: alloc::vec::Vec<suites::SupportedCipherSuite>,
    pub kx_groups: alloc::vec::Vec<&'static dyn SupportedKxGroup>,
    pub signature_verification_algorithms: WebPkiSupportedAlgorithms,
    pub secure_random: &'static dyn SecureRandom,
    pub key_provider: &'static dyn KeyProvider,
}
```

Controls core cryptography used by rustls.

This crate comes with two built-in options, provided as
`CryptoProvider` structures:

- [`crypto::aws_lc_rs::default_provider`](#default-provider): (behind the `aws_lc_rs` crate feature,
  which is enabled by default).  This provider uses the [aws-lc-rs](https://github.com/aws/aws-lc-rs)
  crate.  The `fips` crate feature makes this option use FIPS140-3-approved cryptography.
- [`crypto::ring::default_provider`](#default-provider): (behind the `ring` crate feature, which
  is optional).  This provider uses the [*ring*](https://github.com/briansmith/ring)
  crate.

This structure provides defaults. Everything in it can be overridden at
runtime by replacing field values as needed.

# Using the per-process default `CryptoProvider`

There is the concept of an implicit default provider, configured at run-time once in
a given process.

It is used for functions like [`ClientConfig::builder()`](#builder) and [`ServerConfig::builder()`](#builder).

The intention is that an application can specify the [`CryptoProvider`](rustls/crypto/index.md) they wish to use
once, and have that apply to the variety of places where their application does TLS
(which may be wrapped inside other libraries).
They should do this by calling [`CryptoProvider::install_default()`](#install-default) early on.

To achieve this goal:

- _libraries_ should use [`ClientConfig::builder()`](#builder)/[`ServerConfig::builder()`](#builder)
  or otherwise rely on the [`CryptoProvider::get_default()`](#get-default) provider.
- _applications_ should call [`CryptoProvider::install_default()`](#install-default) early
  in their `fn main()`. If _applications_ uses a custom provider based on the one built-in,
  they can activate the `custom-provider` feature to ensure its usage.

# Using a specific `CryptoProvider`

Supply the provider when constructing your [`ClientConfig`](#clientconfig) or [`ServerConfig`](#serverconfig):

- [`ClientConfig::builder_with_provider()`](#builder-with-provider)
- [`ServerConfig::builder_with_provider()`](#builder-with-provider)

When creating and configuring a webpki-backed client or server certificate verifier, a choice of
provider is also needed to start the configuration process:

- [`client::WebPkiServerVerifier::builder_with_provider()`](#builder-with-provider)
- [`server::WebPkiClientVerifier::builder_with_provider()`](#builder-with-provider)

If you install a custom provider and want to avoid any accidental use of a built-in provider, the feature
`custom-provider` can be activated to ensure your custom provider is used everywhere
and not a built-in one. This will disable any implicit use of a built-in provider.

# Making a custom `CryptoProvider`

Your goal will be to populate an instance of this `CryptoProvider` struct.

## Which elements are required?

There is no requirement that the individual elements ([`SupportedCipherSuite`](#supportedciphersuite), [`SupportedKxGroup`](rustls/crypto/index.md),
[`SigningKey`](#signingkey), etc.) come from the same crate.  It is allowed and expected that uninteresting
elements would be delegated back to one of the default providers (statically) or a parent
provider (dynamically).

For example, if we want to make a provider that just overrides key loading in the config builder
API (with [`ConfigBuilder::with_single_cert`](#with-single-cert), etc.), it might look like this:

```
# #[cfg(feature = "aws_lc_rs")] {
# use std::sync::Arc;
# mod fictious_hsm_api { pub fn load_private_key(key_der: pki_types::PrivateKeyDer<'static>) -> ! { unreachable!(); } }
use rustls::crypto::aws_lc_rs;

pub fn provider() -> rustls::crypto::CryptoProvider {
  rustls::crypto::CryptoProvider{
    key_provider: &HsmKeyLoader,
    ..aws_lc_rs::default_provider()
  }
}

#[derive(Debug)]
struct HsmKeyLoader;

impl rustls::crypto::KeyProvider for HsmKeyLoader {
    fn load_private_key(&self, key_der: pki_types::PrivateKeyDer<'static>) -> Result<Arc<dyn rustls::sign::SigningKey>, rustls::Error> {
         fictious_hsm_api::load_private_key(key_der)
    }
}
# }
```

## References to the individual elements

The elements are documented separately:

- **Random** - see [`crypto::SecureRandom::fill()`](#fill).
- **Cipher suites** - see [`SupportedCipherSuite`](#supportedciphersuite), [`Tls12CipherSuite`](#tls12ciphersuite), and
  [`Tls13CipherSuite`](#tls13ciphersuite).
- **Key exchange groups** - see [`crypto::SupportedKxGroup`](#supportedkxgroup).
- **Signature verification algorithms** - see [`crypto::WebPkiSupportedAlgorithms`](#webpkisupportedalgorithms).
- **Authentication key loading** - see [`crypto::KeyProvider::load_private_key()`](#load-private-key) and
  [`sign::SigningKey`](#signingkey).

# Example code

See custom [`provider-example/`](#provider-example) for a full client and server example that uses
cryptography from the [`RustCrypto`](#rustcrypto) and [`dalek-cryptography`](#dalek-cryptography) projects.

```shell
$ cargo run --example client | head -3
Current ciphersuite: TLS13_CHACHA20_POLY1305_SHA256
HTTP/1.1 200 OK
Content-Type: text/html; charset=utf-8
Content-Length: 19899
```



# FIPS-approved cryptography
The `fips` crate feature enables use of the `aws-lc-rs` crate in FIPS mode.

You can verify the configuration at runtime by checking
[`ServerConfig::fips()`](#fips)/[`ClientConfig::fips()`](#fips) return `true`.

#### Fields

- **`cipher_suites`**: `alloc::vec::Vec<suites::SupportedCipherSuite>`

  List of supported ciphersuites, in preference order -- the first element
  is the highest priority.
  
  The `SupportedCipherSuite` type carries both configuration and implementation.
  
  A valid `CryptoProvider` must ensure that all cipher suites are accompanied by at least
  one matching key exchange group in [`CryptoProvider::kx_groups`](#kx-groups).

- **`kx_groups`**: `alloc::vec::Vec<&'static dyn SupportedKxGroup>`

  List of supported key exchange groups, in preference order -- the
  first element is the highest priority.
  
  The first element in this list is the _default key share algorithm_,
  and in TLS1.3 a key share for it is sent in the client hello.
  
  The `SupportedKxGroup` type carries both configuration and implementation.

- **`signature_verification_algorithms`**: `WebPkiSupportedAlgorithms`

  List of signature verification algorithms for use with webpki.
  
  These are used for both certificate chain verification and handshake signature verification.
  
  This is called by [`ConfigBuilder::with_root_certificates()`](#with-root-certificates),
  [`server::WebPkiClientVerifier::builder_with_provider()`](#builder-with-provider) and
  [`client::WebPkiServerVerifier::builder_with_provider()`](#builder-with-provider).

- **`secure_random`**: `&'static dyn SecureRandom`

  Source of cryptographically secure random numbers.

- **`key_provider`**: `&'static dyn KeyProvider`

  Provider for loading private [`SigningKey`](#signingkey)s from [`PrivateKeyDer`](../../rustls_pki_types/rustls_pki_types/index.md).

#### Implementations

- `fn install_default(self: Self) -> Result<(), alloc::sync::Arc<Self>>`
  Sets this `CryptoProvider` as the default for this process.

- `fn get_default() -> Option<&'static alloc::sync::Arc<Self>>`
  Returns the default `CryptoProvider` for this process.

- `fn fips(self: &Self) -> bool`
  Returns `true` if this `CryptoProvider` is operating in FIPS mode.

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

- `fn clone(self: &Self) -> CryptoProvider`

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

### `CompletedKeyExchange`

```rust
struct CompletedKeyExchange {
    pub group: crate::NamedGroup,
    pub pub_key: alloc::vec::Vec<u8>,
    pub secret: SharedSecret,
}
```

The result from [`SupportedKxGroup::start_and_complete()`](#start-and-complete).

#### Fields

- **`group`**: `crate::NamedGroup`

  Which group was used.

- **`pub_key`**: `alloc::vec::Vec<u8>`

  Our key share (sometimes a public key).

- **`secret`**: `SharedSecret`

  The computed shared secret.

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

### `SharedSecret`

```rust
struct SharedSecret {
}
```

The result from [`ActiveKeyExchange::complete`](#complete) or [`ActiveKeyExchange::complete_hybrid_component`](#complete-hybrid-component).

#### Implementations

- `fn secret_bytes(self: &Self) -> &[u8]`
  Returns the shared secret as a slice of bytes.

#### Trait Implementations

##### `impl From`

- `fn from(buf: Vec<u8>) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(source: &[u8]) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

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

### `KeyExchangeAlgorithm`

```rust
enum KeyExchangeAlgorithm {
    DHE,
    ECDHE,
}
```

Describes supported key exchange mechanisms.

#### Variants

- **`DHE`**

  Diffie-Hellman Key exchange (with only known parameters as defined in [RFC 7919]).
  
  [RFC 7919]: https://datatracker.ietf.org/doc/html/rfc7919

- **`ECDHE`**

  Key exchange performed via elliptic curve Diffie-Hellman.

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

- `fn clone(self: &Self) -> KeyExchangeAlgorithm`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &KeyExchangeAlgorithm) -> bool`

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

### `SecureRandom`

```rust
trait SecureRandom: Send + Sync + Debug { ... }
```

A source of cryptographically secure randomness.

#### Required Methods

- `fn fill(self: &Self, buf: &mut [u8]) -> Result<(), GetRandomFailed>`

  Fill the given buffer with random bytes.

- `fn fips(self: &Self) -> bool`

  Return `true` if this is backed by a FIPS-approved implementation.

### `KeyProvider`

```rust
trait KeyProvider: Send + Sync + Debug { ... }
```

A mechanism for loading private [`SigningKey`](#signingkey)s from [`PrivateKeyDer`](../../rustls_pki_types/rustls_pki_types/index.md).

This trait is intended to be used with private key material that is sourced from DER,
such as a private-key that may be present on-disk. It is not intended to be used with
keys held in hardware security modules (HSMs) or physical tokens. For these use-cases
see the Rustls manual section on [customizing private key usage].

[customizing private key usage]: <https://docs.rs/rustls/latest/rustls/manual/_03_howto/index.html#customising-private-key-usage>

#### Required Methods

- `fn load_private_key(self: &Self, key_der: PrivateKeyDer<'static>) -> Result<alloc::sync::Arc<dyn SigningKey>, Error>`

  Decode and validate a private signing key from `key_der`.

- `fn fips(self: &Self) -> bool`

  Return `true` if this is backed by a FIPS-approved implementation.

### `SupportedKxGroup`

```rust
trait SupportedKxGroup: Send + Sync + Debug { ... }
```

A supported key exchange group.

This type carries both configuration and implementation. Specifically,
it has a TLS-level name expressed using the [`NamedGroup`](#namedgroup) enum, and
a function which produces a [`ActiveKeyExchange`](rustls/crypto/index.md).

Compare with [`NamedGroup`](#namedgroup), which carries solely a protocol identifier.

#### Required Methods

- `fn start(self: &Self) -> Result<Box<dyn ActiveKeyExchange>, Error>`

  Start a key exchange.

- `fn start_and_complete(self: &Self, peer_pub_key: &[u8]) -> Result<CompletedKeyExchange, Error>`

  Start and complete a key exchange, in one operation.

- `fn ffdhe_group(self: &Self) -> Option<FfdheGroup<'static>>`

  FFDHE group the `SupportedKxGroup` operates in.

- `fn name(self: &Self) -> NamedGroup`

  Named group the SupportedKxGroup operates in.

- `fn fips(self: &Self) -> bool`

  Return `true` if this is backed by a FIPS-approved implementation.

- `fn usable_for_version(self: &Self, _version: ProtocolVersion) -> bool`

  Return `true` if this should be offered/selected with the given version.

### `ActiveKeyExchange`

```rust
trait ActiveKeyExchange: Send + Sync { ... }
```

An in-progress key exchange originating from a [`SupportedKxGroup`](rustls/crypto/index.md).

#### Required Methods

- `fn complete(self: Box<Self>, peer_pub_key: &[u8]) -> Result<SharedSecret, Error>`

  Completes the key exchange, given the peer's public key.

- `fn complete_for_tls_version(self: Box<Self>, peer_pub_key: &[u8], tls_version: &SupportedProtocolVersion) -> Result<SharedSecret, Error>`

  Completes the key exchange for the given TLS version, given the peer's public key.

- `fn hybrid_component(self: &Self) -> Option<(NamedGroup, &[u8])>`

  For hybrid key exchanges, returns the [`NamedGroup`](#namedgroup) and key share

- `fn complete_hybrid_component(self: Box<Self>, _peer_pub_key: &[u8]) -> Result<SharedSecret, Error>`

  Completes the classical component of the key exchange, given the peer's public key.

- `fn pub_key(self: &Self) -> &[u8]`

  Return the public key being used.

- `fn ffdhe_group(self: &Self) -> Option<FfdheGroup<'static>>`

  FFDHE group the `ActiveKeyExchange` is operating in.

- `fn group(self: &Self) -> NamedGroup`

  Return the group being used.

## Functions

