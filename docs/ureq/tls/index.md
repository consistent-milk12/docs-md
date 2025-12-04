*[ureq](../index.md) / [tls](index.md)*

---

# Module `tls`

TLS for handling `https`.

## Structs

### `Certificate<'a>`

```rust
struct Certificate<'a> {
    // [REDACTED: Private Fields]
}
```

An X509 certificate for a server or a client.

These are either used as trust roots, or client authentication.

The internal representation is DER form. The provided helpers for PEM
translates to DER.

#### Implementations

- `fn from_der(der: &'a [u8]) -> Self`
  Read an X509 certificate in DER form.

- `fn from_pem(pem: &'a [u8]) -> Result<Certificate<'static>, Error>`
  Read an X509 certificate in PEM form.

- `fn der(self: &Self) -> &[u8]`
  This certificate in DER (the internal) format.

- `fn to_owned(self: &Self) -> Certificate<'static>`
  Clones (allocates) to produce a static copy.

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

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> Certificate<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Hash<'a>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

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

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `PrivateKey<'a>`

```rust
struct PrivateKey<'a> {
    // [REDACTED: Private Fields]
}
```

A private key used in client certificate auth.

The internal representation is DER form. The provided helpers for PEM
translates to DER.

Deliberately not `Clone` to avoid accidental copies in memory.

#### Implementations

- `fn from_der(kind: KeyKind, der: &'a [u8]) -> Self`
  Read private key in DER form.

- `fn from_pem(pem: &'a [u8]) -> Result<PrivateKey<'static>, Error>`
  Read a private key in PEM form.

- `fn kind(self: &Self) -> KeyKind`
  The key kind

- `fn der(self: &Self) -> &[u8]`
  This private key in DER (the internal) format.

- `fn to_owned(self: &Self) -> PrivateKey<'static>`
  Clones (allocates) to produce a static copy.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsRef<'a>`

- `fn as_ref(self: &Self) -> &[u8]`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Hash<'a>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `TlsConfig`

```rust
struct TlsConfig {
    // [REDACTED: Private Fields]
}
```

Configuration of TLS.

This configuration is in common for both the different TLS mechanisms (available through
feature flags **rustls** and **native-tls**).

#### Implementations

- `fn builder() -> TlsConfigBuilder`
  Builder to make a bespoke config.

- `fn provider(self: &Self) -> TlsProvider`
  The provider to use.

- `fn client_cert(self: &Self) -> Option<&ClientCert>`
  Client certificate chain with corresponding private key.

- `fn root_certs(self: &Self) -> &RootCerts`
  The set of trusted root certificates to use to validate server certificates.

- `fn use_sni(self: &Self) -> bool`
  Whether to send SNI (Server Name Indication) to the remote server.

- `fn disable_verification(self: &Self) -> bool`
  **WARNING** Disable all server certificate verification.

- `fn unversioned_rustls_crypto_provider(self: &Self) -> &Option<Arc<::rustls::crypto::CryptoProvider>>`
  Specific `CryptoProvider` to use for `rustls`.

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

- `fn clone(self: &Self) -> TlsConfig`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Hash`

- `fn hash<H: std::hash::Hasher>(self: &Self, state: &mut H)`

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

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default`

- `fn default() -> Self`

### `TlsConfigBuilder`

```rust
struct TlsConfigBuilder {
    // [REDACTED: Private Fields]
}
```

Builder of [`TlsConfig`](tls/index.md)

#### Implementations

- `fn provider(self: Self, v: TlsProvider) -> Self`
  The provider to use.

- `fn client_cert(self: Self, v: Option<ClientCert>) -> Self`
  Client certificate chain with corresponding private key.

- `fn root_certs(self: Self, v: RootCerts) -> Self`
  The set of trusted root certificates to use to validate server certificates.

- `fn use_sni(self: Self, v: bool) -> Self`
  Whether to send SNI (Server Name Indication) to the remote server.

- `fn disable_verification(self: Self, v: bool) -> Self`
  **WARNING** Disable all server certificate verification.

- `fn unversioned_rustls_crypto_provider(self: Self, v: Arc<::rustls::crypto::CryptoProvider>) -> Self`
  Specific `CryptoProvider` to use for `rustls`.

- `fn build(self: Self) -> TlsConfig`
  Finalize the config

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

### `ClientCert`

```rust
struct ClientCert();
```

A client certificate.

#### Implementations

- `fn new_with_certs(chain: &[Certificate<'static>], key: PrivateKey<'static>) -> Self`
  Creates a new client certificate from a chain and a private key.

- `fn certs(self: &Self) -> &[Certificate<'static>]`
  Client certificate chain.

- `fn private_key(self: &Self) -> &PrivateKey<'static>`
  Client certificate private key.

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

- `fn clone(self: &Self) -> ClientCert`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

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

## Enums

### `PemItem<'a>`

```rust
enum PemItem<'a> {
    Certificate(Certificate<'a>),
    PrivateKey(PrivateKey<'a>),
}
```

Kinds of PEM data found by [`parse_pem`](index.md)

#### Variants

- **`Certificate`**

  An X509 certificate

- **`PrivateKey`**

  A private key

#### Trait Implementations

##### `impl From<'a>`

- `fn from(value: Certificate<'a>) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<'a>`

- `fn from(value: PrivateKey<'a>) -> Self`

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

### `TlsProvider`

```rust
enum TlsProvider {
    Rustls,
    NativeTls,
}
```

Setting for which TLS provider to use.

Defaults to `Rustls` because this has the highest chance
to compile and "just work" straight out of the box without installing additional
development dependencies.

#### Variants

- **`Rustls`**

  [Rustls](https://crates.io/crates/rustls) with the
  [process-wide default cryptographic backend](https://docs.rs/rustls/latest/rustls/crypto/struct.CryptoProvider.html#method.install_default),
  or [Ring](https://crates.io/crates/ring) if no process-wide default is set.
  
  Requires the feature flag **rustls**.
  
  This is the default.

- **`NativeTls`**

  [Native-TLS](https://crates.io/crates/native-tls) for cases where it's important to
  use the TLS libraries installed on the host running ureq.
  
  Requires the feature flag **native-tls** and that using an [`Agent`](crate::Agent) with
  this config option set in the [`TlsConfig`](tls/index.md).
  
  The setting is never picked up automatically.

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

- `fn clone(self: &Self) -> TlsProvider`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &TlsProvider) -> bool`

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

##### `impl Default`

- `fn default() -> TlsProvider`

### `RootCerts`

```rust
enum RootCerts {
    Specific(std::sync::Arc<Vec<Certificate<'static>>>),
    PlatformVerifier,
    WebPki,
}
```

Configuration setting for root certs.

#### Variants

- **`Specific`**

  Use these specific certificates as root certs.

- **`PlatformVerifier`**

  Use the platform's verifier.
  
  * For **rustls**, this uses the `rustls-platform-verifier` crate. It requires
    the feature **platform-verifier**.
  * For **native-tls**, this uses the roots that native-tls loads by default.

- **`WebPki`**

  Use Mozilla's root certificates instead of the platform.
  
  This is useful when you can't trust the system roots, such as in
  environments where TLS is intercepted and decrypted by a proxy (MITM attack).
  
  This is the default value.

#### Implementations

- `fn new_with_certs(certs: &[Certificate<'static>]) -> Self`
  Use these specific root certificates

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<I: IntoIterator<Item = Certificate<'static>>>`

- `fn from(value: I) -> Self`

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

- `fn clone(self: &Self) -> RootCerts`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

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

## Functions

