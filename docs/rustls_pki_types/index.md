# Crate `rustls_pki_types`

This crate provides types for representing X.509 certificates, keys and other types as
commonly used in the rustls ecosystem. It is intended to be used by crates that need to work
with such X.509 types, such as [rustls](https://crates.io/crates/rustls),
[rustls-webpki](https://crates.io/crates/rustls-webpki),
[rustls-pemfile](https://crates.io/crates/rustls-pemfile), and others.

Some of these crates used to define their own trivial wrappers around DER-encoded bytes.
However, in order to avoid inconvenient dependency edges, these were all disconnected. By
using a common low-level crate of types with long-term stable API, we hope to avoid the
downsides of unnecessary dependency edges while providing good interoperability between crates.

## DER and PEM

Many of the types defined in this crate represent DER-encoded data. DER is a binary encoding of
the ASN.1 format commonly used in web PKI specifications. It is a binary encoding, so it is
relatively compact when stored in memory. However, as a binary format, it is not very easy to
work with for humans and in contexts where binary data is inconvenient. For this reason,
many tools and protocols use a ASCII-based encoding of DER, called PEM. In addition to the
base64-encoded DER, PEM objects are delimited by header and footer lines which indicate the type
of object contained in the PEM blob.

Types here can be created from:

- DER using (for example) `PrivatePkcs8KeyDer::from()`.
- PEM using (for example) `pem::PemObject::from_pem_slice()`.

The `pem::PemObject` trait contains the full selection of ways to construct
these types from PEM encodings.  That includes ways to open and read from a file,
from a slice, or from an `std::io` stream.

There is also a lower-level API that allows a given PEM file to be fully consumed
in one pass, even if it contains different data types: see the implementation of
the `pem::PemObject` trait on the `(pem::SectionKind, Vec<u8>)` tuple.

## Creating new certificates and keys

This crate does not provide any functionality for creating new certificates or keys. However,
the [rcgen](https://docs.rs/rcgen) crate can be used to create new certificates and keys.

## Cloning private keys

This crate intentionally **does not** implement `Clone` on private key types in
order to minimize the exposure of private key data in memory.

If you want to extend the lifetime of a `PrivateKeyDer<'_>`, consider `PrivateKeyDer::clone_key()`.
Alternatively  since these types are immutable, consider wrapping the `PrivateKeyDer<'_>` in a [`Rc`](#rc)
or an [`Arc`](#arc).



## Target `wasm32-unknown-unknown` with the `web` feature

[`std::time::SystemTime`](https://doc.rust-lang.org/std/time/struct.SystemTime.html)
is unavailable in `wasm32-unknown-unknown` targets, so calls to
[`UnixTime::now()`](https://docs.rs/rustls-pki-types/latest/rustls_pki_types/struct.UnixTime.html#method.now),
otherwise enabled by the [`std`](https://docs.rs/crate/rustls-pki-types/latest/features#std) feature,
require building instead with the [`web`](https://docs.rs/crate/rustls-pki-types/latest/features#web)
feature. It gets time by calling [`Date.now()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/now)
in the browser.

## Modules

- [`alg_id`](alg_id/index.md) - The PKIX [`AlgorithmIdentifier`] type, and common values.
- [`pem`](pem/index.md) - Low-level PEM decoding APIs.

## Structs

### `AlgorithmIdentifier`

```rust
struct AlgorithmIdentifier();
```

A DER encoding of the PKIX AlgorithmIdentifier type:

```ASN.1
AlgorithmIdentifier  ::=  SEQUENCE  {
    algorithm               OBJECT IDENTIFIER,
    parameters              ANY DEFINED BY algorithm OPTIONAL  }
                               -- contains a value of the type
                               -- registered for use with the
                               -- algorithm object identifier value
```
(from <https://www.rfc-editor.org/rfc/rfc5280#section-4.1.1.2>)

The outer sequence encoding is *not included*, so this is the DER encoding
of an OID for `algorithm` plus the `parameters` value.

For example, this is the `rsaEncryption` algorithm (but prefer to use the constant
[`RSA_ENCRYPTION`](alg_id/index.md) instead):

```rust
let rsa_encryption = rustls_pki_types::AlgorithmIdentifier::from_slice(
    &[
        // algorithm: 1.2.840.113549.1.1.1
        0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x01,
        // parameters: NULL
        0x05, 0x00
    ]
);
assert_eq!(rustls_pki_types::alg_id::RSA_ENCRYPTION, rsa_encryption);
```

Common values for this type are provided in this module.

#### Implementations

- `const fn from_slice(bytes: &'static [u8]) -> Self`
  Makes a new `AlgorithmIdentifier` from a static octet slice.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

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

##### `impl Clone`

- `fn clone(self: &Self) -> AlgorithmIdentifier`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &AlgorithmIdentifier) -> bool`

##### `impl Receiver<P, T>`

- `type Target = T`

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

##### `impl Deref`

- `type Target = [u8]`

- `fn deref(self: &Self) -> &<Self as >::Target`

### `AddrParseError`

```rust
struct AddrParseError();
```

Failure to parse an IP address

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

- `fn clone(self: &Self) -> AddrParseError`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq`

##### `impl Error`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &AddrParseError) -> bool`

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

### `DnsName<'a>`

```rust
struct DnsName<'a>();
```

A type which encapsulates a string (borrowed or owned) that is a syntactically valid DNS name.

#### Implementations

- `fn borrow(self: &'a Self) -> Self`
  Produce a borrowed `DnsName` from this owned `DnsName`.

- `fn to_lowercase_owned(self: &Self) -> DnsName<'static>`
  Copy this object to produce an owned `DnsName`, smashing the case to lowercase

- `fn to_owned(self: &Self) -> DnsName<'static>`
  Produce an owned `DnsName` from this (potentially borrowed) `DnsName`.

- `const fn try_from_str(s: &str) -> Result<DnsName<'_>, InvalidDnsNameError>`
  Produces a borrowed [`DnsName`] from a borrowed [`str`].

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &str`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> DnsName<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq<'a>`

##### `impl Hash<'a>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &DnsName<'a>) -> bool`

##### `impl StructuralPartialEq<'a>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryFrom<'a>`

- `type Error = InvalidDnsNameError`

- `fn try_from(value: &'a [u8]) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<'a>`

- `type Error = InvalidDnsNameError`

- `fn try_from(value: &'a str) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom`

- `type Error = InvalidDnsNameError`

- `fn try_from(value: String) -> Result<Self, <Self as >::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `InvalidDnsNameError`

```rust
struct InvalidDnsNameError;
```

The provided input could not be parsed because
it is not a syntactically-valid DNS Name.

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

### `Ipv4Addr`

```rust
struct Ipv4Addr();
```

`no_std` implementation of `std::net::Ipv4Addr`.

Note: because we intend to replace this type with `core::net::Ipv4Addr` as soon as it is
stabilized, the identity of this type should not be considered semver-stable. However, the
attached interfaces are stable; they form a subset of those provided by `core::net::Ipv4Addr`.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(value: [u8; 4]) -> Self`

##### `impl From`

- `fn from(addr: std::net::Ipv4Addr) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &[u8; 4]`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Ipv4Addr`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Ipv4Addr) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryFrom`

- `type Error = AddrParseError`

- `fn try_from(value: &str) -> Result<Self, <Self as >::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Ipv6Addr`

```rust
struct Ipv6Addr();
```

`no_std` implementation of `std::net::Ipv6Addr`.

Note: because we intend to replace this type with `core::net::Ipv6Addr` as soon as it is
stabilized, the identity of this type should not be considered semver-stable. However, the
attached interfaces are stable; they form a subset of those provided by `core::net::Ipv6Addr`.

#### Trait Implementations

##### `impl From`

- `fn from(addr: std::net::Ipv6Addr) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(value: [u16; 8]) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &[u8; 16]`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Ipv6Addr`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Ipv6Addr) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryFrom`

- `type Error = AddrParseError`

- `fn try_from(value: &str) -> Result<Self, <Self as >::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `PrivatePkcs1KeyDer<'a>`

```rust
struct PrivatePkcs1KeyDer<'a>();
```

A DER-encoded plaintext RSA private key; as specified in PKCS#1/RFC 3447

RSA private keys are identified in PEM context as `RSA PRIVATE KEY` and when stored in a
file usually use a `.pem` or `.key` extension.

```rust
#[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{PrivatePkcs1KeyDer, pem::PemObject};

// load from a PEM file
PrivatePkcs1KeyDer::from_pem_file("tests/data/rsa1024.pkcs1.pem").unwrap();

// or from a PEM byte slice...
let byte_slice = include_bytes!("../tests/data/rsa1024.pkcs1.pem");
PrivatePkcs1KeyDer::from_pem_slice(byte_slice).unwrap();
}
```

#### Implementations

- `fn clone_key(self: &Self) -> PrivatePkcs1KeyDer<'static>`
  Clone the private key to a `'static` value

- `fn secret_pkcs1_der(self: &Self) -> &[u8]`
  Yield the DER-encoded bytes of the private key

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(vec: Vec<u8>) -> Self`

##### `impl From<'a>`

- `fn from(slice: &'a [u8]) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Eq<'a>`

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &PrivatePkcs1KeyDer<'a>) -> bool`

##### `impl PemObject<T>`

- `fn from_pem(kind: SectionKind, der: Vec<u8>) -> Option<T>`

##### `impl StructuralPartialEq<'a>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Zeroize`

- `fn zeroize(self: &mut Self)`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `PrivateSec1KeyDer<'a>`

```rust
struct PrivateSec1KeyDer<'a>();
```

A Sec1-encoded plaintext private key; as specified in RFC 5915

Sec1 private keys are identified in PEM context as `EC PRIVATE KEY` and when stored in a
file usually use a `.pem` or `.key` extension. For more on PEM files, refer to the crate
documentation.

```rust
#[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{PrivateSec1KeyDer, pem::PemObject};

// load from a PEM file
PrivateSec1KeyDer::from_pem_file("tests/data/nistp256key.pem").unwrap();

// or from a PEM byte slice...
let byte_slice = include_bytes!("../tests/data/nistp256key.pem");
PrivateSec1KeyDer::from_pem_slice(byte_slice).unwrap();
}
```

#### Implementations

- `fn clone_key(self: &Self) -> PrivateSec1KeyDer<'static>`
  Clone the private key to a `'static` value

- `fn secret_sec1_der(self: &Self) -> &[u8]`
  Yield the DER-encoded bytes of the private key

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<'a>`

- `fn from(slice: &'a [u8]) -> Self`

##### `impl From`

- `fn from(vec: Vec<u8>) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Eq<'a>`

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &PrivateSec1KeyDer<'a>) -> bool`

##### `impl PemObject<T>`

- `fn from_pem(kind: SectionKind, der: Vec<u8>) -> Option<T>`

##### `impl StructuralPartialEq<'a>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Zeroize`

- `fn zeroize(self: &mut Self)`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `PrivatePkcs8KeyDer<'a>`

```rust
struct PrivatePkcs8KeyDer<'a>();
```

A DER-encoded plaintext private key; as specified in PKCS#8/RFC 5958

PKCS#8 private keys are identified in PEM context as `PRIVATE KEY` and when stored in a
file usually use a `.pem` or `.key` extension. For more on PEM files, refer to the crate
documentation.

```rust
#[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{PrivatePkcs8KeyDer, pem::PemObject};

// load from a PEM file
PrivatePkcs8KeyDer::from_pem_file("tests/data/nistp256key.pkcs8.pem").unwrap();
PrivatePkcs8KeyDer::from_pem_file("tests/data/rsa1024.pkcs8.pem").unwrap();

// or from a PEM byte slice...
let byte_slice = include_bytes!("../tests/data/nistp256key.pkcs8.pem");
PrivatePkcs8KeyDer::from_pem_slice(byte_slice).unwrap();
}
```

#### Implementations

- `fn clone_key(self: &Self) -> PrivatePkcs8KeyDer<'static>`
  Clone the private key to a `'static` value

- `fn secret_pkcs8_der(self: &Self) -> &[u8]`
  Yield the DER-encoded bytes of the private key

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<'a>`

- `fn from(slice: &'a [u8]) -> Self`

##### `impl From`

- `fn from(vec: Vec<u8>) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Eq<'a>`

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &PrivatePkcs8KeyDer<'a>) -> bool`

##### `impl PemObject<T>`

- `fn from_pem(kind: SectionKind, der: Vec<u8>) -> Option<T>`

##### `impl StructuralPartialEq<'a>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Zeroize`

- `fn zeroize(self: &mut Self)`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `TrustAnchor<'a>`

```rust
struct TrustAnchor<'a> {
    pub subject: Der<'a>,
    pub subject_public_key_info: Der<'a>,
    pub name_constraints: Option<Der<'a>>,
}
```

A trust anchor (a.k.a. root CA)

Traditionally, certificate verification libraries have represented trust anchors as full X.509
root certificates. However, those certificates contain a lot more data than is needed for
verifying certificates. The [`TrustAnchor`](#trustanchor) representation allows an application to store
just the essential elements of trust anchors.

The most common way to get one of these is to call `rustls_webpki::anchor_from_trusted_cert()`.


#### Fields

- **`subject`**: `Der<'a>`

  Value of the `subject` field of the trust anchor

- **`subject_public_key_info`**: `Der<'a>`

  Value of the `subjectPublicKeyInfo` field of the trust anchor

- **`name_constraints`**: `Option<Der<'a>>`

  Value of DER-encoded `NameConstraints`, containing name constraints to the trust anchor, if any

#### Implementations

- `fn to_owned(self: &Self) -> TrustAnchor<'static>`
  Yield a `'static` lifetime of the `TrustAnchor` by allocating owned `Der` variants

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

- `fn clone(self: &Self) -> TrustAnchor<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq<'a>`

##### `impl Hash<'a>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &TrustAnchor<'a>) -> bool`

##### `impl StructuralPartialEq<'a>`

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

### `CertificateRevocationListDer<'a>`

```rust
struct CertificateRevocationListDer<'a>();
```

A Certificate Revocation List; as specified in RFC 5280

Certificate revocation lists are identified in PEM context as `X509 CRL` and when stored in a
file usually use a `.crl` extension. For more on PEM files, refer to the crate documentation.

```rust
#[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{CertificateRevocationListDer, pem::PemObject};

// load several from a PEM file
let crls: Vec<_> = CertificateRevocationListDer::pem_file_iter("tests/data/crl.pem")
    .unwrap()
    .collect();
assert!(crls.len() >= 1);

// or one from a PEM byte slice...
let byte_slice = include_bytes!("../tests/data/crl.pem");
CertificateRevocationListDer::from_pem_slice(byte_slice).unwrap();

// or several from a PEM byte slice
let crls: Vec<_> = CertificateRevocationListDer::pem_slice_iter(byte_slice)
    .collect();
assert!(crls.len() >= 1);
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(vec: Vec<u8>) -> Self`

##### `impl From<'a>`

- `fn from(slice: &'a [u8]) -> Self`

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

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> CertificateRevocationListDer<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq<'a>`

##### `impl Hash<'a>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &CertificateRevocationListDer<'a>) -> bool`

##### `impl PemObject<T>`

- `fn from_pem(kind: SectionKind, der: Vec<u8>) -> Option<T>`

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl StructuralPartialEq<'a>`

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

##### `impl Deref`

- `type Target = [u8]`

- `fn deref(self: &Self) -> &<Self as >::Target`

### `CertificateSigningRequestDer<'a>`

```rust
struct CertificateSigningRequestDer<'a>();
```

A Certificate Signing Request; as specified in RFC 2986

Certificate signing requests are identified in PEM context as `CERTIFICATE REQUEST` and when stored in a
file usually use a `.csr` extension. For more on PEM files, refer to the crate documentation.

```rust
#[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{CertificateSigningRequestDer, pem::PemObject};

// load from a PEM file
CertificateSigningRequestDer::from_pem_file("tests/data/csr.pem").unwrap();

// or from a PEM byte slice...
let byte_slice = include_bytes!("../tests/data/csr.pem");
CertificateSigningRequestDer::from_pem_slice(byte_slice).unwrap();
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(vec: Vec<u8>) -> Self`

##### `impl From<'a>`

- `fn from(slice: &'a [u8]) -> Self`

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

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> CertificateSigningRequestDer<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq<'a>`

##### `impl Hash<'a>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &CertificateSigningRequestDer<'a>) -> bool`

##### `impl PemObject<T>`

- `fn from_pem(kind: SectionKind, der: Vec<u8>) -> Option<T>`

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl StructuralPartialEq<'a>`

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

##### `impl Deref`

- `type Target = [u8]`

- `fn deref(self: &Self) -> &<Self as >::Target`

### `CertificateDer<'a>`

```rust
struct CertificateDer<'a>();
```

A DER-encoded X.509 certificate; as specified in RFC 5280

Certificates are identified in PEM context as `CERTIFICATE` and when stored in a
file usually use a `.pem`, `.cer` or `.crt` extension. For more on PEM files, refer to the
crate documentation.

```rust
#[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{CertificateDer, pem::PemObject};

// load several from a PEM file
let certs: Vec<_> = CertificateDer::pem_file_iter("tests/data/certificate.chain.pem")
    .unwrap()
    .collect();
assert_eq!(certs.len(), 3);

// or one from a PEM byte slice...
let byte_slice = include_bytes!("../tests/data/certificate.chain.pem");
CertificateDer::from_pem_slice(byte_slice).unwrap();

// or several from a PEM byte slice
let certs: Vec<_> = CertificateDer::pem_slice_iter(byte_slice)
    .collect();
assert_eq!(certs.len(), 3);
}
```

#### Implementations

- `const fn from_slice(bytes: &'a [u8]) -> Self`
  A const constructor to create a `CertificateDer` from a slice of DER.

- `fn into_owned(self: Self) -> CertificateDer<'static>`
  Converts this certificate into its owned variant, unfreezing borrowed content (if any)

#### Trait Implementations

##### `impl From<'a>`

- `fn from(slice: &'a [u8]) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(vec: Vec<u8>) -> Self`

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

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> CertificateDer<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq<'a>`

##### `impl Hash<'a>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &CertificateDer<'a>) -> bool`

##### `impl PemObject<T>`

- `fn from_pem(kind: SectionKind, der: Vec<u8>) -> Option<T>`

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl StructuralPartialEq<'a>`

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

##### `impl Deref`

- `type Target = [u8]`

- `fn deref(self: &Self) -> &<Self as >::Target`

### `SubjectPublicKeyInfoDer<'a>`

```rust
struct SubjectPublicKeyInfoDer<'a>();
```

A DER-encoded SubjectPublicKeyInfo (SPKI), as specified in RFC 5280.

Public keys are identified in PEM context as a `PUBLIC KEY`.

```rust
#[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{SubjectPublicKeyInfoDer, pem::PemObject};

// load from a PEM file
SubjectPublicKeyInfoDer::from_pem_file("tests/data/spki.pem").unwrap();

// or from a PEM byte slice...
let byte_slice = include_bytes!("../tests/data/spki.pem");
SubjectPublicKeyInfoDer::from_pem_slice(byte_slice).unwrap();
}
```

#### Implementations

- `fn into_owned(self: Self) -> SubjectPublicKeyInfoDer<'static>`
  Converts this SubjectPublicKeyInfo into its owned variant, unfreezing borrowed content (if any)

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(vec: Vec<u8>) -> Self`

##### `impl From<'a>`

- `fn from(slice: &'a [u8]) -> Self`

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

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> SubjectPublicKeyInfoDer<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq<'a>`

##### `impl Hash<'a>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &SubjectPublicKeyInfoDer<'a>) -> bool`

##### `impl PemObject<T>`

- `fn from_pem(kind: SectionKind, der: Vec<u8>) -> Option<T>`

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl StructuralPartialEq<'a>`

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

##### `impl Deref`

- `type Target = [u8]`

- `fn deref(self: &Self) -> &<Self as >::Target`

### `EchConfigListBytes<'a>`

```rust
struct EchConfigListBytes<'a>();
```

A TLS-encoded Encrypted Client Hello (ECH) configuration list (`ECHConfigList`); as specified in
[draft-ietf-tls-esni-18 §4](https://datatracker.ietf.org/doc/html/draft-ietf-tls-esni-18#section-4)

#### Implementations

- `fn into_owned(self: Self) -> EchConfigListBytes<'static>`
  Converts this config into its owned variant, unfreezing borrowed content (if any)

- `fn config_and_key_from_iter(iter: impl Iterator<Item = Result<(SectionKind, Vec<u8>), pem::Error>>) -> Result<(Self, PrivatePkcs8KeyDer<'static>), pem::Error>`
  Convert an iterator over PEM items into an `EchConfigListBytes` and private key.

#### Trait Implementations

##### `impl From`

- `fn from(vec: Vec<u8>) -> Self`

##### `impl From<'a>`

- `fn from(slice: &'a [u8]) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

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

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> EchConfigListBytes<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq<'a>`

##### `impl Hash<'a>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &EchConfigListBytes<'a>) -> bool`

##### `impl PemObject<T>`

- `fn from_pem(kind: SectionKind, der: Vec<u8>) -> Option<T>`

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl StructuralPartialEq<'a>`

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

##### `impl Deref`

- `type Target = [u8]`

- `fn deref(self: &Self) -> &<Self as >::Target`

### `InvalidSignature`

```rust
struct InvalidSignature;
```

A detail-less error when a signature is not valid.

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

- `fn clone(self: &Self) -> InvalidSignature`

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

### `UnixTime`

```rust
struct UnixTime();
```

A timestamp, tracking the number of non-leap seconds since the Unix epoch.

The Unix epoch is defined January 1, 1970 00:00:00 UTC.

#### Implementations

- `fn now() -> Self`
  The current time, as a `UnixTime`

- `const fn since_unix_epoch(duration: Duration) -> Self`
  Convert a `Duration` since the start of 1970 to a `UnixTime`

- `const fn as_secs(self: &Self) -> u64`
  Number of seconds since the Unix epoch

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

- `fn clone(self: &Self) -> UnixTime`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord`

- `fn cmp(self: &Self, other: &UnixTime) -> $crate::cmp::Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &UnixTime) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &UnixTime) -> $crate::option::Option<$crate::cmp::Ordering>`

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

### `Der<'a>`

```rust
struct Der<'a>();
```

DER-encoded data, either owned or borrowed

This wrapper type is used to represent DER-encoded data in a way that is agnostic to whether
the data is owned (by a `Vec<u8>`) or borrowed (by a `&[u8]`). Support for the owned
variant is only available when the `alloc` feature is enabled.

#### Implementations

- `const fn from_slice(der: &'a [u8]) -> Self`
  A const constructor to create a `Der` from a borrowed slice

#### Trait Implementations

##### `impl From`

- `fn from(vec: Vec<u8>) -> Self`

##### `impl From<'a>`

- `fn from(slice: &'a [u8]) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

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

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> Der<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq<'a>`

##### `impl Hash<'a>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &Der<'a>) -> bool`

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl StructuralPartialEq<'a>`

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

##### `impl Deref`

- `type Target = [u8]`

- `fn deref(self: &Self) -> &<Self as >::Target`

## Enums

### `IpAddr`

```rust
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

`no_std` implementation of `std::net::IpAddr`.

Note: because we intend to replace this type with `core::net::IpAddr` as soon as it is
stabilized, the identity of this type should not be considered semver-stable. However, the
attached interfaces are stable; they form a subset of those provided by `core::net::IpAddr`.

#### Variants

- **`V4`**

  An Ipv4 address.

- **`V6`**

  An Ipv6 address.

#### Trait Implementations

##### `impl From`

- `fn from(v6: std::net::Ipv6Addr) -> Self`

##### `impl From`

- `fn from(v4: std::net::Ipv4Addr) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(addr: std::net::IpAddr) -> Self`

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

- `fn clone(self: &Self) -> IpAddr`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &IpAddr) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom`

- `type Error = AddrParseError`

- `fn try_from(value: &str) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ServerName<'a>`

```rust
enum ServerName<'a> {
    DnsName(DnsName<'a>),
    IpAddress(IpAddr),
}
```

Encodes ways a client can know the expected name of the server.

This currently covers knowing the DNS name of the server, but
will be extended in the future to supporting privacy-preserving names
for the server ("ECH").  For this reason this enum is `non_exhaustive`.

# Making one

If you have a DNS name as a `&str`, this type implements `TryFrom<&str>`,
so you can do:

```rust
use rustls_pki_types::ServerName;
ServerName::try_from("example.com").expect("invalid DNS name");
```

If you have an owned `String`, you can use `TryFrom` directly:

```rust
use rustls_pki_types::ServerName;
let name = "example.com".to_string();
#[cfg(feature = "alloc")]
ServerName::try_from(name).expect("invalid DNS name");
```

which will yield a `ServerName<'static>` if successful.

or, alternatively...

```rust
use rustls_pki_types::ServerName;
let x: ServerName = "example.com".try_into().expect("invalid DNS name");
```

#### Variants

- **`DnsName`**

  The server is identified by a DNS name.  The name
  is sent in the TLS Server Name Indication (SNI)
  extension.

- **`IpAddress`**

  The server is identified by an IP address. SNI is not
  done.

#### Implementations

- `fn to_owned(self: &Self) -> ServerName<'static>`
  Produce an owned `ServerName` from this (potentially borrowed) `ServerName`.

- `fn to_str(self: &Self) -> Cow<'_, str>`
  Return the string representation of this `ServerName`.

#### Trait Implementations

##### `impl From<'a>`

- `fn from(dns_name: DnsName<'a>) -> Self`

##### `impl From`

- `fn from(v4: std::net::Ipv4Addr) -> Self`

##### `impl From`

- `fn from(v4: Ipv4Addr) -> Self`

##### `impl From`

- `fn from(v6: Ipv6Addr) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(addr: IpAddr) -> Self`

##### `impl From`

- `fn from(v6: std::net::Ipv6Addr) -> Self`

##### `impl From`

- `fn from(addr: std::net::IpAddr) -> Self`

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

- `fn clone(self: &Self) -> ServerName<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq<'a>`

##### `impl Hash<'a>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &ServerName<'a>) -> bool`

##### `impl StructuralPartialEq<'a>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryFrom<'a>`

- `type Error = InvalidDnsNameError`

- `fn try_from(value: &'a [u8]) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom`

- `type Error = InvalidDnsNameError`

- `fn try_from(value: String) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<'a>`

- `type Error = InvalidDnsNameError`

- `fn try_from(s: &'a str) -> Result<Self, <Self as >::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `PrivateKeyDer<'a>`

```rust
enum PrivateKeyDer<'a> {
    Pkcs1(PrivatePkcs1KeyDer<'a>),
    Sec1(PrivateSec1KeyDer<'a>),
    Pkcs8(PrivatePkcs8KeyDer<'a>),
}
```

A DER-encoded X.509 private key, in one of several formats

See variant inner types for more detailed information.

This can load several types of PEM-encoded private key, and then reveal
which types were found:

```rust
#[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{PrivateKeyDer, pem::PemObject};

// load from a PEM file
let pkcs8 = PrivateKeyDer::from_pem_file("tests/data/nistp256key.pkcs8.pem").unwrap();
let pkcs1 = PrivateKeyDer::from_pem_file("tests/data/rsa1024.pkcs1.pem").unwrap();
let sec1 = PrivateKeyDer::from_pem_file("tests/data/nistp256key.pem").unwrap();
assert!(matches!(pkcs8, PrivateKeyDer::Pkcs8(_)));
assert!(matches!(pkcs1, PrivateKeyDer::Pkcs1(_)));
assert!(matches!(sec1, PrivateKeyDer::Sec1(_)));
}
```

#### Variants

- **`Pkcs1`**

  An RSA private key

- **`Sec1`**

  A Sec1 private key

- **`Pkcs8`**

  A PKCS#8 private key

#### Implementations

- `fn clone_key(self: &Self) -> PrivateKeyDer<'static>`
  Clone the private key to a `'static` value

- `fn secret_der(self: &Self) -> &[u8]`
  Yield the DER-encoded bytes of the private key

#### Trait Implementations

##### `impl From<'a>`

- `fn from(key: PrivateSec1KeyDer<'a>) -> Self`

##### `impl From<'a>`

- `fn from(key: PrivatePkcs8KeyDer<'a>) -> Self`

##### `impl From<'a>`

- `fn from(key: PrivatePkcs1KeyDer<'a>) -> Self`

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

##### `impl Eq<'a>`

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &PrivateKeyDer<'a>) -> bool`

##### `impl PemObject`

- `fn from_pem(kind: SectionKind, value: Vec<u8>) -> Option<Self>`

##### `impl StructuralPartialEq<'a>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryFrom`

- `type Error = &'static str`

- `fn try_from(key: Vec<u8>) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<'a>`

- `type Error = &'static str`

- `fn try_from(key: &'a [u8]) -> Result<Self, <Self as >::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Zeroize`

- `fn zeroize(self: &mut Self)`

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Traits

### `SignatureVerificationAlgorithm`

```rust
trait SignatureVerificationAlgorithm: Send + Sync + fmt::Debug { ... }
```

An abstract signature verification algorithm.

One of these is needed per supported pair of public key type (identified
with `public_key_alg_id()`) and `signatureAlgorithm` (identified with
`signature_alg_id()`).  Note that both of these `AlgorithmIdentifier`s include
the parameters encoding, so separate `SignatureVerificationAlgorithm`s are needed
for each possible public key or signature parameters.

Debug implementations should list the public key algorithm identifier and
signature algorithm identifier in human friendly form (i.e. not encoded bytes),
along with the name of the implementing library (to distinguish different
implementations of the same algorithms).

#### Required Methods

- `fn verify_signature(self: &Self, public_key: &[u8], message: &[u8], signature: &[u8]) -> Result<(), InvalidSignature>`

  Verify a signature.

- `fn public_key_alg_id(self: &Self) -> AlgorithmIdentifier`

  Return the `AlgorithmIdentifier` that must equal a public key's

- `fn signature_alg_id(self: &Self) -> AlgorithmIdentifier`

  Return the `AlgorithmIdentifier` that must equal the `signatureAlgorithm` value

- `fn fips(self: &Self) -> bool`

  Return `true` if this is backed by a FIPS-approved implementation.

## Type Aliases

### `SubjectPublicKeyInfo<'a>`

```rust
type SubjectPublicKeyInfo<'a> = SubjectPublicKeyInfoDer<'a>;
```

A DER-encoded SubjectPublicKeyInfo (SPKI), as specified in RFC 5280.

