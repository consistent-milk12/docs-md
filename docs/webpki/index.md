# Crate `webpki`

webpki: Web PKI X.509 Certificate Validation.

See `EndEntityCert`'s documentation for a description of the certificate
processing steps necessary for a TLS connection.

# Features

| Feature | Description |
| ------- | ----------- |
| `alloc` | Enable features that require use of the heap. Currently all RSA signature algorithms require this feature. |
| `std` | Enable features that require libstd. Implies `alloc`. |
| `ring` | Enable use of the *ring* crate for cryptography. |
| `aws-lc-rs` | Enable use of the aws-lc-rs crate for cryptography. Previously this feature was named `aws_lc_rs`. |

## Modules

- [`ring`](ring/index.md) - Signature verification algorithm implementations using the *ring* crypto library.

## Structs

### `Cert<'a>`

```rust
struct Cert<'a> {
    // [REDACTED: Private Fields]
}
```

A parsed X509 certificate.

#### Implementations

- `fn valid_dns_names(self: &Self) -> impl Iterator<Item = &str>`
  Returns a list of valid DNS names provided in the subject alternative names extension

- `fn valid_uri_names(self: &Self) -> impl Iterator<Item = &str>`
  Returns a list of valid URI names provided in the subject alternative names extension

- `fn serial(self: &Self) -> &[u8]`
  Raw certificate serial number.

- `fn issuer(self: &Self) -> &[u8]`
  Raw DER-encoded certificate issuer.

- `fn subject(self: &Self) -> &[u8]`
  Raw DER encoded certificate subject.

- `fn subject_public_key_info(self: &Self) -> SubjectPublicKeyInfoDer<'static>`
  Get the RFC 5280-compliant [`SubjectPublicKeyInfoDer`] (SPKI) of this [`Cert`].

- `fn der(self: &Self) -> CertificateDer<'a>`
  Raw DER-encoded representation of the certificate.

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

### `BorrowedCertRevocationList<'a>`

```rust
struct BorrowedCertRevocationList<'a> {
    // [REDACTED: Private Fields]
}
```

Borrowed representation of a RFC 5280[^1] profile Certificate Revocation List (CRL).

[^1]: <https://www.rfc-editor.org/rfc/rfc5280#section-5>

#### Implementations

- `fn from_der(crl_der: &'a [u8]) -> Result<Self, Error>`
  Try to parse the given bytes as a RFC 5280[^1] profile Certificate Revocation List (CRL).

- `fn to_owned(self: &Self) -> Result<OwnedCertRevocationList, Error>`
  Convert the CRL to an [`OwnedCertRevocationList`]. This may error if any of the revoked

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

### `BorrowedRevokedCert<'a>`

```rust
struct BorrowedRevokedCert<'a> {
    pub serial_number: &'a [u8],
    pub revocation_date: pki_types::UnixTime,
    pub reason_code: Option<RevocationReason>,
    pub invalidity_date: Option<pki_types::UnixTime>,
}
```

Borrowed representation of a RFC 5280[^1] profile Certificate Revocation List (CRL) revoked
certificate entry.

[^1]: <https://www.rfc-editor.org/rfc/rfc5280#section-5>

#### Fields

- **`serial_number`**: `&'a [u8]`

  Serial number of the revoked certificate.

- **`revocation_date`**: `pki_types::UnixTime`

  The date at which the CA processed the revocation.

- **`reason_code`**: `Option<RevocationReason>`

  Identifies the reason for the certificate revocation. When absent, the revocation reason
  is assumed to be RevocationReason::Unspecified. For consistency with other extensions
  and to ensure only one revocation reason extension may be present we maintain this field
  as optional instead of defaulting to unspecified.

- **`invalidity_date`**: `Option<pki_types::UnixTime>`

  Provides the date on which it is known or suspected that the private key was compromised or
  that the certificate otherwise became invalid. This date may be earlier than the revocation
  date which is the date at which the CA processed the revocation.

#### Implementations

- `fn to_owned(self: &Self) -> OwnedRevokedCert`
  Construct an owned representation of the revoked certificate.

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

### `CrlsRequired`

```rust
struct CrlsRequired();
```

An opaque error indicating the caller must provide at least one CRL when building a
[RevocationOptions] instance.

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

- `fn clone(self: &Self) -> CrlsRequired`

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

### `RevocationOptions<'a>`

```rust
struct RevocationOptions<'a> {
    // [REDACTED: Private Fields]
}
```

Describes how revocation checking is performed, if at all. Can be constructed with a
[RevocationOptionsBuilder] instance.

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

- `fn clone(self: &Self) -> RevocationOptions<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<'a>`

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

### `RevocationOptionsBuilder<'a>`

```rust
struct RevocationOptionsBuilder<'a> {
    // [REDACTED: Private Fields]
}
```

Builds a RevocationOptions instance to control how revocation checking is performed.

#### Implementations

- `fn new(crls: &'a [&'a CertRevocationList<'a>]) -> Result<Self, CrlsRequired>`
  Create a builder that will perform revocation checking using the provided certificate

- `fn with_depth(self: Self, depth: RevocationCheckDepth) -> Self`
  Customize the depth at which revocation checking will be performed, controlling

- `fn with_status_policy(self: Self, policy: UnknownStatusPolicy) -> Self`
  Customize whether unknown revocation status is an error, or permitted.

- `fn with_expiration_policy(self: Self, policy: ExpirationPolicy) -> Self`
  Customize whether the CRL nextUpdate field (i.e. expiration) is enforced.

- `fn build(self: Self) -> RevocationOptions<'a>`
  Construct a [RevocationOptions] instance based on the builder's configuration.

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

- `fn clone(self: &Self) -> RevocationOptionsBuilder<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<'a>`

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

### `DerIterator<'a, T>`

```rust
struct DerIterator<'a, T> {
    // [REDACTED: Private Fields]
}
```

Iterator to parse a sequence of DER-encoded values of type `T`.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Iterator<'a, T: FromDer<'a>>`

- `type Item = Result<T, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `EndEntityCert<'a>`

```rust
struct EndEntityCert<'a> {
    // [REDACTED: Private Fields]
}
```

An end-entity certificate.

Server certificate processing in a TLS connection consists of several
steps. All of these steps are necessary:

* `EndEntityCert::verify_for_usage()`: Verify that the peer's certificate
  is valid for the current usage scenario. For server authentication, use
  `crate::KeyUsage::server_auth()`.
* `EndEntityCert::verify_is_valid_for_subject_name()`: Verify that the server's
  certificate is valid for the host or IP address that is being connected to.
* `EndEntityCert::verify_signature()`: Verify that the signature of server's
  `ServerKeyExchange` message is valid for the server's certificate.

Client certificate processing in a TLS connection consists of analogous
steps. All of these steps are necessary:

* `EndEntityCert::verify_for_usage()`: Verify that the peer's certificate
  is valid for the current usage scenario. For client authentication, use
  `crate::KeyUsage::client_auth()`.
* `EndEntityCert::verify_signature()`: Verify that the signature of client's
  `CertificateVerify` message is valid using the public key from the
  client's certificate.

Although it would be less error-prone to combine all these steps into a
single function call, some significant optimizations are possible if the
three steps are processed separately (in parallel). It does not matter much
which order the steps are done in, but **all of these steps must completed
before application data is sent and before received application data is
processed**. The [`TryFrom`](#tryfrom) conversion from `&CertificateDer<'_>` is an
inexpensive operation and is deterministic, so if these tasks are done in
multiple threads, it is probably best to just create multiple [`EndEntityCert`](#endentitycert)
instances for the same DER-encoded ASN.1 certificate bytes.

#### Implementations

- `fn verify_for_usage<'p>(self: &'p Self, supported_sig_algs: &[&dyn SignatureVerificationAlgorithm], trust_anchors: &'p [TrustAnchor<'_>], intermediate_certs: &'p [CertificateDer<'p>], time: UnixTime, usage: impl ExtendedKeyUsageValidator, revocation: Option<RevocationOptions<'_>>, verify_path: Option<&dyn Fn(&VerifiedPath<'_>) -> Result<(), Error>>) -> Result<VerifiedPath<'p>, Error>`
  Verifies that the end-entity certificate is valid for use against the

- `fn verify_is_valid_for_subject_name(self: &Self, server_name: &ServerName<'_>) -> Result<(), Error>`
  Verifies that the certificate is valid for the given Subject Name.

- `fn verify_signature(self: &Self, signature_alg: &dyn SignatureVerificationAlgorithm, msg: &[u8], signature: &[u8]) -> Result<(), Error>`
  Verifies the signature `signature` of message `msg` using the

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

##### `impl TryFrom<'a>`

- `type Error = Error`

- `fn try_from(cert: &'a CertificateDer<'a>) -> Result<Self, <Self as >::Error>`
  Parse the ASN.1 DER-encoded X.509 encoding of the certificate

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Deref<'a>`

- `type Target = Cert<'a>`

- `fn deref(self: &Self) -> &<Self as >::Target`

### `InvalidNameContext`

```rust
struct InvalidNameContext {
    pub expected: pki_types::ServerName<'static>,
    pub presented: alloc::vec::Vec<alloc::string::String>,
}
```

Additional context for the `CertNotValidForName` error variant.

The contents of this type depend on whether the `alloc` feature is enabled.

#### Fields

- **`expected`**: `pki_types::ServerName<'static>`

  Expected server name.

- **`presented`**: `alloc::vec::Vec<alloc::string::String>`

  The names presented in the end entity certificate.
  
  These are the subject names as present in the leaf certificate and may contain DNS names
  with or without a wildcard label as well as IP address names.

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

- `fn clone(self: &Self) -> InvalidNameContext`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &InvalidNameContext) -> bool`

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

### `UnsupportedSignatureAlgorithmContext`

```rust
struct UnsupportedSignatureAlgorithmContext {
    pub signature_algorithm_id: alloc::vec::Vec<u8>,
    pub supported_algorithms: alloc::vec::Vec<pki_types::AlgorithmIdentifier>,
}
```

Additional context for the `UnsupportedSignatureAlgorithm` error variant.

The contents of this type depend on whether the `alloc` feature is enabled.

#### Fields

- **`signature_algorithm_id`**: `alloc::vec::Vec<u8>`

  The signature algorithm OID that was unsupported.

- **`supported_algorithms`**: `alloc::vec::Vec<pki_types::AlgorithmIdentifier>`

  Supported algorithms that were available for signature verification.

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

- `fn clone(self: &Self) -> UnsupportedSignatureAlgorithmContext`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &UnsupportedSignatureAlgorithmContext) -> bool`

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

### `UnsupportedSignatureAlgorithmForPublicKeyContext`

```rust
struct UnsupportedSignatureAlgorithmForPublicKeyContext {
    pub signature_algorithm_id: alloc::vec::Vec<u8>,
    pub public_key_algorithm_id: alloc::vec::Vec<u8>,
}
```

Additional context for the `UnsupportedSignatureAlgorithmForPublicKey` error variant.

The contents of this type depend on whether the `alloc` feature is enabled.

#### Fields

- **`signature_algorithm_id`**: `alloc::vec::Vec<u8>`

  The signature algorithm OID.

- **`public_key_algorithm_id`**: `alloc::vec::Vec<u8>`

  The public key algorithm OID.

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

- `fn clone(self: &Self) -> UnsupportedSignatureAlgorithmForPublicKeyContext`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &UnsupportedSignatureAlgorithmForPublicKeyContext) -> bool`

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

### `RawPublicKeyEntity<'a>`

```rust
struct RawPublicKeyEntity<'a> {
    // [REDACTED: Private Fields]
}
```

A Raw Public Key, used for connections using raw public keys as specified
in [RFC 7250](https://www.rfc-editor.org/rfc/rfc7250).

#### Implementations

- `fn verify_signature(self: &Self, signature_alg: &dyn SignatureVerificationAlgorithm, msg: &[u8], signature: &[u8]) -> Result<(), Error>`
  Verifies the signature `signature` of message `msg` using a raw public key,

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

##### `impl TryFrom<'a>`

- `type Error = Error`

- `fn try_from(spki: &'a SubjectPublicKeyInfoDer<'a>) -> Result<Self, <Self as >::Error>`
  Parse the ASN.1 DER-encoded SPKI encoding of the raw public key `spki`.

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `IntermediateIterator<'a>`

```rust
struct IntermediateIterator<'a> {
    // [REDACTED: Private Fields]
}
```

Iterator over a path's intermediate certificates.

Implements [`DoubleEndedIterator`](#doubleendediterator) so it can be traversed in both directions.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl DoubleEndedIterator`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl Iterator<'a>`

- `type Item = &'a Cert<'a>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `KeyPurposeId<'a>`

```rust
struct KeyPurposeId<'a> {
    // [REDACTED: Private Fields]
}
```

An OID value indicating an Extended Key Usage (EKU) key purpose.

#### Implementations

- `const fn new(oid: &'a [u8]) -> Self`
  Construct a new [`KeyPurposeId`].

- `fn to_decoded_oid(self: &Self) -> Vec<usize>`
  Yield the OID value as a sequence of `usize` components.

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

- `fn clone(self: &Self) -> KeyPurposeId<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<'a>`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Self) -> bool`

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

### `KeyPurposeIdIter<'a, 'r>`

```rust
struct KeyPurposeIdIter<'a, 'r> {
    // [REDACTED: Private Fields]
}
```

Iterator over [`KeyPurposeId`](#keypurposeid)s, for use in [`ExtendedKeyUsageValidator`](#extendedkeyusagevalidator).

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Drop`

- `fn drop(self: &mut Self)`

##### `impl Iterator<'a, 'r>`

- `type Item = Result<KeyPurposeId<'a>, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `KeyUsage`

```rust
struct KeyUsage {
    // [REDACTED: Private Fields]
}
```

The expected key usage of a certificate.

This type represents the expected key usage of an end entity certificate. Although for most
kinds of certificates the extended key usage extension is optional (and so certificates
not carrying a particular value in the EKU extension are acceptable). If the extension
is present, the certificate MUST only be used for one of the purposes indicated.

<https://www.rfc-editor.org/rfc/rfc5280#section-4.2.1.12>

#### Implementations

- `const fn server_auth() -> Self`
  Construct a new [`KeyUsage`] as appropriate for server certificate authentication.

- `const fn client_auth() -> Self`
  Construct a new [`KeyUsage`] as appropriate for client certificate authentication.

- `const fn required(oid: &'static [u8]) -> Self`
  Construct a new [`KeyUsage`] requiring a certificate to support the specified OID.

- `const fn required_if_present(oid: &'static [u8]) -> Self`
  Construct a new [`KeyUsage`] requiring a certificate to support the specified OID, if the certificate has EKUs.

- `fn oid_values(self: &Self) -> impl Iterator<Item = usize> + '_`
  Yield the OID values of the required extended key usage.

- `const SERVER_AUTH_REPR: &[usize]`

- `const CLIENT_AUTH_REPR: &[usize]`

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

- `fn clone(self: &Self) -> KeyUsage`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl ExtendedKeyUsageValidator`

- `fn validate(self: &Self, iter: KeyPurposeIdIter<'_, '_>) -> Result<(), Error>`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &KeyUsage) -> bool`

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

### `RequiredEkuNotFoundContext`

```rust
struct RequiredEkuNotFoundContext {
    pub required: KeyUsage,
    pub present: alloc::vec::Vec<alloc::vec::Vec<usize>>,
}
```

Additional context for the `RequiredEkuNotFoundContext` error variant.

The contents of this type depend on whether the `alloc` feature is enabled.

#### Fields

- **`required`**: `KeyUsage`

  The required ExtendedKeyUsage.

- **`present`**: `alloc::vec::Vec<alloc::vec::Vec<usize>>`

  The ExtendedKeyUsage OIDs present in the certificate.

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

- `fn clone(self: &Self) -> RequiredEkuNotFoundContext`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &RequiredEkuNotFoundContext) -> bool`

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

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `VerifiedPath<'p>`

```rust
struct VerifiedPath<'p> {
    // [REDACTED: Private Fields]
}
```

Path from end-entity certificate to trust anchor that's been verified.

See `EndEntityCert::verify_for_usage()` for more details on what verification entails.

#### Implementations

- `fn intermediate_certificates(self: &'p Self) -> IntermediateIterator<'p>`
  Yields a (double-ended) iterator over the intermediate certificates in this path.

- `fn end_entity(self: &Self) -> &'p EndEntityCert<'p>`
  Yields the end-entity certificate for this path.

- `fn anchor(self: &Self) -> &'p TrustAnchor<'p>`
  Yields the trust anchor for this path.

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

### `OwnedCertRevocationList`

```rust
struct OwnedCertRevocationList {
    // [REDACTED: Private Fields]
}
```

Owned representation of a RFC 5280[^1] profile Certificate Revocation List (CRL).

[^1]: <https://www.rfc-editor.org/rfc/rfc5280#section-5>

#### Implementations

- `fn from_der(crl_der: &[u8]) -> Result<Self, Error>`
  Try to parse the given bytes as a RFC 5280[^1] profile Certificate Revocation List (CRL).

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

- `fn clone(self: &Self) -> OwnedCertRevocationList`

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

### `OwnedRevokedCert`

```rust
struct OwnedRevokedCert {
    pub serial_number: alloc::vec::Vec<u8>,
    pub revocation_date: pki_types::UnixTime,
    pub reason_code: Option<RevocationReason>,
    pub invalidity_date: Option<pki_types::UnixTime>,
}
```

Owned representation of a RFC 5280[^1] profile Certificate Revocation List (CRL) revoked
certificate entry.

Only available when the "alloc" feature is enabled.

[^1]: <https://www.rfc-editor.org/rfc/rfc5280#section-5>

#### Fields

- **`serial_number`**: `alloc::vec::Vec<u8>`

  Serial number of the revoked certificate.

- **`revocation_date`**: `pki_types::UnixTime`

  The date at which the CA processed the revocation.

- **`reason_code`**: `Option<RevocationReason>`

  Identifies the reason for the certificate revocation. When absent, the revocation reason
  is assumed to be RevocationReason::Unspecified. For consistency with other extensions
  and to ensure only one revocation reason extension may be present we maintain this field
  as optional instead of defaulting to unspecified.

- **`invalidity_date`**: `Option<pki_types::UnixTime>`

  Provides the date on which it is known or suspected that the private key was compromised or
  that the certificate otherwise became invalid. This date may be earlier than the revocation
  date which is the date at which the CA processed the revocation.

#### Implementations

- `fn borrow(self: &Self) -> BorrowedRevokedCert<'_>`
  Convert the owned representation of this revoked cert to a borrowed version.

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

- `fn clone(self: &Self) -> OwnedRevokedCert`

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

## Enums

### `CertRevocationList<'a>`

```rust
enum CertRevocationList<'a> {
    Owned(OwnedCertRevocationList),
    Borrowed(BorrowedCertRevocationList<'a>),
}
```

A RFC 5280[^1] profile Certificate Revocation List (CRL).

May be either an owned, or a borrowed representation.

[^1]: <https://www.rfc-editor.org/rfc/rfc5280#section-5>

#### Variants

- **`Owned`**

  An owned representation of a CRL.

- **`Borrowed`**

  A borrowed representation of a CRL.

#### Implementations

- `fn issuer(self: &Self) -> &[u8]`
  Return the DER encoded issuer of the CRL.

- `fn issuing_distribution_point(self: &Self) -> Option<&[u8]>`
  Return the DER encoded issuing distribution point of the CRL, if any.

- `fn find_serial(self: &Self, serial: &[u8]) -> Result<Option<BorrowedRevokedCert<'_>>, Error>`
  Try to find a revoked certificate in the CRL by DER encoded serial number. This

#### Trait Implementations

##### `impl From`

- `fn from(crl: OwnedCertRevocationList) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<'a>`

- `fn from(crl: BorrowedCertRevocationList<'a>) -> Self`

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

### `ExpirationPolicy`

```rust
enum ExpirationPolicy {
    Enforce,
    Ignore,
}
```

Describes how to handle the nextUpdate field of the CRL (i.e. expiration).

#### Variants

- **`Enforce`**

  Enforce the verification time is before the time in the nextUpdate field.
  Treats an expired CRL as an error condition yielding [Error::CrlExpired].

- **`Ignore`**

  Ignore the CRL nextUpdate field.

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

- `fn clone(self: &Self) -> ExpirationPolicy`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ExpirationPolicy) -> bool`

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

### `RevocationCheckDepth`

```rust
enum RevocationCheckDepth {
    EndEntity,
    Chain,
}
```

Describes how much of a certificate chain is checked for revocation status.

#### Variants

- **`EndEntity`**

  Only check the end entity (leaf) certificate's revocation status.

- **`Chain`**

  Check the revocation status of the end entity (leaf) and all intermediates.

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

- `fn clone(self: &Self) -> RevocationCheckDepth`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &RevocationCheckDepth) -> bool`

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

### `RevocationReason`

```rust
enum RevocationReason {
    Unspecified,
    KeyCompromise,
    CaCompromise,
    AffiliationChanged,
    Superseded,
    CessationOfOperation,
    CertificateHold,
    RemoveFromCrl,
    PrivilegeWithdrawn,
    AaCompromise,
}
```

Identifies the reason a certificate was revoked.
See [RFC 5280 §5.3.1][1]

[1]: <https://www.rfc-editor.org/rfc/rfc5280#section-5.3.1>

#### Variants

- **`Unspecified`**

  Unspecified should not be used, and is instead assumed by the absence of a RevocationReason
  extension.

- **`RemoveFromCrl`**

  RemoveFromCrl only appears in delta CRLs that are unsupported.

#### Implementations

- `fn iter() -> impl Iterator<Item = Self>`
  Return an iterator over all possible [RevocationReason] variants.

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

- `fn clone(self: &Self) -> RevocationReason`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &RevocationReason) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom`

- `type Error = Error`

- `fn try_from(value: u8) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `UnknownStatusPolicy`

```rust
enum UnknownStatusPolicy {
    Allow,
    Deny,
}
```

Describes how to handle the case where a certificate's revocation status is unknown.

#### Variants

- **`Allow`**

  Treat unknown revocation status permissively, acting as if the certificate were
  not revoked.

- **`Deny`**

  Treat unknown revocation status as an error condition, yielding
  [Error::UnknownRevocationStatus].

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

- `fn clone(self: &Self) -> UnknownStatusPolicy`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &UnknownStatusPolicy) -> bool`

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

### `DerTypeId`

```rust
enum DerTypeId {
    BitString,
    Bool,
    Certificate,
    CertificateExtensions,
    CertificateTbsCertificate,
    CertRevocationList,
    CertRevocationListExtension,
    CrlDistributionPoint,
    CommonNameInner,
    CommonNameOuter,
    DistributionPointName,
    Extension,
    GeneralName,
    RevocationReason,
    Signature,
    SignatureAlgorithm,
    SignedData,
    SubjectPublicKeyInfo,
    Time,
    TrustAnchorV1,
    TrustAnchorV1TbsCertificate,
    U8,
    RevokedCertificate,
    RevokedCertificateExtension,
    RevokedCertEntry,
    IssuingDistributionPoint,
}
```

Trailing data was found while parsing DER-encoded input for the named type.

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

- `fn clone(self: &Self) -> DerTypeId`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &DerTypeId) -> bool`

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

### `Error`

```rust
enum Error {
    BadDer,
    BadDerTime,
    CaUsedAsEndEntity,
    CertExpired {
        time: pki_types::UnixTime,
        not_after: pki_types::UnixTime,
    },
    CertNotValidForName(InvalidNameContext),
    CertNotValidYet {
        time: pki_types::UnixTime,
        not_before: pki_types::UnixTime,
    },
    CertRevoked,
    CrlExpired {
        time: pki_types::UnixTime,
        next_update: pki_types::UnixTime,
    },
    EmptyEkuExtension,
    EndEntityUsedAsCa,
    ExtensionValueInvalid,
    InvalidCertValidity,
    InvalidCrlNumber,
    InvalidNetworkMaskConstraint,
    InvalidSerialNumber,
    InvalidCrlSignatureForPublicKey,
    InvalidSignatureForPublicKey,
    IssuerNotCrlSigner,
    MalformedDnsIdentifier,
    MalformedExtensions,
    MalformedNameConstraint,
    MaximumNameConstraintComparisonsExceeded,
    MaximumPathBuildCallsExceeded,
    MaximumPathDepthExceeded,
    MaximumSignatureChecksExceeded,
    NameConstraintViolation,
    PathLenConstraintViolated,
    RequiredEkuNotFound,
    RequiredEkuNotFoundContext(crate::verify_cert::RequiredEkuNotFoundContext),
    SignatureAlgorithmMismatch,
    TrailingData(DerTypeId),
    UnknownIssuer,
    UnknownRevocationStatus,
    UnsupportedCertVersion,
    UnsupportedCriticalExtension,
    UnsupportedCrlIssuingDistributionPoint,
    UnsupportedCrlVersion,
    UnsupportedDeltaCrl,
    UnsupportedIndirectCrl,
    UnsupportedNameType,
    UnsupportedRevocationReason,
    UnsupportedRevocationReasonsPartitioning,
    UnsupportedCrlSignatureAlgorithm,
    UnsupportedCrlSignatureAlgorithmContext(UnsupportedSignatureAlgorithmContext),
    UnsupportedSignatureAlgorithm,
    UnsupportedSignatureAlgorithmContext(UnsupportedSignatureAlgorithmContext),
    UnsupportedCrlSignatureAlgorithmForPublicKey,
    UnsupportedCrlSignatureAlgorithmForPublicKeyContext(UnsupportedSignatureAlgorithmForPublicKeyContext),
    UnsupportedSignatureAlgorithmForPublicKey,
    UnsupportedSignatureAlgorithmForPublicKeyContext(UnsupportedSignatureAlgorithmForPublicKeyContext),
}
```

An error that occurs during certificate validation or name validation.

#### Variants

- **`BadDer`**

  The encoding of some ASN.1 DER-encoded item is invalid.

- **`BadDerTime`**

  The encoding of an ASN.1 DER-encoded time is invalid.

- **`CaUsedAsEndEntity`**

  A CA certificate is being used as an end-entity certificate.

- **`CertExpired`**

  The certificate is expired; i.e. the time it is being validated for is
  later than the certificate's notAfter time.

- **`CertNotValidForName`**

  The certificate is not valid for the name it is being validated for.

- **`CertNotValidYet`**

  The certificate is not valid yet; i.e. the time it is being validated
  for is earlier than the certificate's notBefore time.

- **`CertRevoked`**

  The certificate, or one of its issuers, has been revoked.

- **`CrlExpired`**

  The CRL is expired; i.e. the verification time is not before the time
  in the CRL nextUpdate field.

- **`EmptyEkuExtension`**

  The certificate has an Extended Key Usage extension without any EKU values.

- **`EndEntityUsedAsCa`**

  An end-entity certificate is being used as a CA certificate.

- **`ExtensionValueInvalid`**

  An X.509 extension is invalid.

- **`InvalidCertValidity`**

  The certificate validity period (notBefore, notAfter) is invalid; e.g.
  the notAfter time is earlier than the notBefore time.

- **`InvalidCrlNumber`**

  A CRL number extension was invalid:
   - it was mis-encoded
   - it was negative
   - it was too long

- **`InvalidNetworkMaskConstraint`**

  A iPAddress name constraint was invalid:
  - it had a sparse network mask (ie, cannot be written in CIDR form).
  - it was too long or short

- **`InvalidSerialNumber`**

  A serial number was invalid:
   - it was misencoded
   - it was negative
   - it was too long

- **`InvalidCrlSignatureForPublicKey`**

  The CRL signature is invalid for the issuer's public key.

- **`InvalidSignatureForPublicKey`**

  The signature is invalid for the given public key.

- **`IssuerNotCrlSigner`**

  A CRL was signed by an issuer that has a KeyUsage bitstring that does not include
  the cRLSign key usage bit.

- **`MalformedDnsIdentifier`**

  A presented or reference DNS identifier was malformed, potentially
  containing invalid characters or invalid labels.

- **`MalformedExtensions`**

  The certificate extensions are malformed.
  
  In particular, webpki requires the DNS name(s) be in the subjectAltName
  extension as required by the CA/Browser Forum Baseline Requirements
  and as recommended by RFC6125.

- **`MalformedNameConstraint`**

  A name constraint was malformed, potentially containing invalid characters or
  invalid labels.

- **`MaximumNameConstraintComparisonsExceeded`**

  The maximum number of name constraint comparisons has been reached.

- **`MaximumPathBuildCallsExceeded`**

  The maximum number of internal path building calls has been reached. Path complexity is too great.

- **`MaximumPathDepthExceeded`**

  The path search was terminated because it became too deep.

- **`MaximumSignatureChecksExceeded`**

  The maximum number of signature checks has been reached. Path complexity is too great.

- **`NameConstraintViolation`**

  The certificate violates one or more name constraints.

- **`PathLenConstraintViolated`**

  The certificate violates one or more path length constraints.

- **`RequiredEkuNotFound`**

  The certificate is not valid for the Extended Key Usage for which it is
  being validated.

- **`RequiredEkuNotFoundContext`**

  The certificate is not valid for the Extended Key Usage for which it is
  being validated.

- **`SignatureAlgorithmMismatch`**

  The algorithm in the TBSCertificate "signature" field of a certificate
  does not match the algorithm in the signature of the certificate.

- **`TrailingData`**

  Trailing data was found while parsing DER-encoded input for the named type.

- **`UnknownIssuer`**

  A valid issuer for the certificate could not be found.

- **`UnknownRevocationStatus`**

  The certificate's revocation status could not be determined.

- **`UnsupportedCertVersion`**

  The certificate is not a v3 X.509 certificate.
  
  This error may be also reported if the certificate version field
  is malformed.

- **`UnsupportedCriticalExtension`**

  The certificate contains an unsupported critical extension.

- **`UnsupportedCrlIssuingDistributionPoint`**

  The CRL contains an issuing distribution point with no distribution point name,
  or a distribution point name relative to an issuer.

- **`UnsupportedCrlVersion`**

  The CRL is not a v2 X.509 CRL.
  
  The RFC 5280 web PKI profile mandates only version 2 be used. See section
  5.1.2.1 for more information.
  
  This error may also be reported if the CRL version field is malformed.

- **`UnsupportedDeltaCrl`**

  The CRL is an unsupported "delta" CRL.

- **`UnsupportedIndirectCrl`**

  The CRL contains unsupported "indirect" entries.

- **`UnsupportedNameType`**

  The `ServerName` contained an unsupported type of value.

- **`UnsupportedRevocationReason`**

  The revocation reason is not in the set of supported revocation reasons.

- **`UnsupportedRevocationReasonsPartitioning`**

  The CRL is partitioned by revocation reasons.

- **`UnsupportedCrlSignatureAlgorithm`**

  The signature algorithm for a signature over a CRL is not in the set of supported
  signature algorithms given.

- **`UnsupportedCrlSignatureAlgorithmContext`**

  The signature algorithm for a signature is not in the set of supported
  signature algorithms given.

- **`UnsupportedSignatureAlgorithm`**

  The signature algorithm for a signature is not in the set of supported
  signature algorithms given.

- **`UnsupportedSignatureAlgorithmContext`**

  The signature algorithm for a signature is not in the set of supported
  signature algorithms given.

- **`UnsupportedCrlSignatureAlgorithmForPublicKey`**

  The CRL signature's algorithm does not match the algorithm of the issuer
  public key it is being validated for. This may be because the public key
  algorithm's OID isn't recognized (e.g. DSA), or the public key
  algorithm's parameters don't match the supported parameters for that
  algorithm (e.g. ECC keys for unsupported curves), or the public key
  algorithm and the signature algorithm simply don't match (e.g.
  verifying an RSA signature with an ECC public key).

- **`UnsupportedCrlSignatureAlgorithmForPublicKeyContext`**

  The signature's algorithm does not match the algorithm of the public
  key it is being validated for. This may be because the public key
  algorithm's OID isn't recognized (e.g. DSA), or the public key
  algorithm's parameters don't match the supported parameters for that
  algorithm (e.g. ECC keys for unsupported curves), or the public key
  algorithm and the signature algorithm simply don't match (e.g.
  verifying an RSA signature with an ECC public key).

- **`UnsupportedSignatureAlgorithmForPublicKey`**

  The signature's algorithm does not match the algorithm of the public
  key it is being validated for. This may be because the public key
  algorithm's OID isn't recognized (e.g. DSA), or the public key
  algorithm's parameters don't match the supported parameters for that
  algorithm (e.g. ECC keys for unsupported curves), or the public key
  algorithm and the signature algorithm simply don't match (e.g.
  verifying an RSA signature with an ECC public key).

- **`UnsupportedSignatureAlgorithmForPublicKeyContext`**

  The signature's algorithm does not match the algorithm of the public
  key it is being validated for. This may be because the public key
  algorithm's OID isn't recognized (e.g. DSA), or the public key
  algorithm's parameters don't match the supported parameters for that
  algorithm (e.g. ECC keys for unsupported curves), or the public key
  algorithm and the signature algorithm simply don't match (e.g.
  verifying an RSA signature with an ECC public key).

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

- `fn clone(self: &Self) -> Error`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Error`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Error) -> bool`

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

## Traits

## Functions

