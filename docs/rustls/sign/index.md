*[rustls](../index.md) / [sign](index.md)*

---

# Module `sign`

Message signing interfaces.

## Structs

### `CertifiedKey`

```rust
struct CertifiedKey {
    pub cert: alloc::vec::Vec<pki_types::CertificateDer<'static>>,
    pub key: alloc::sync::Arc<dyn SigningKey>,
    pub ocsp: Option<alloc::vec::Vec<u8>>,
}
```

A packaged-together certificate chain, matching `SigningKey` and
optional stapled OCSP response.

Note: this struct is also used to represent an [RFC 7250] raw public key,
when the client/server is configured to use raw public keys instead of
certificates.

[RFC 7250]: https://tools.ietf.org/html/rfc7250

#### Fields

- **`cert`**: `alloc::vec::Vec<pki_types::CertificateDer<'static>>`

  The certificate chain or raw public key.

- **`key`**: `alloc::sync::Arc<dyn SigningKey>`

  The certified key.

- **`ocsp`**: `Option<alloc::vec::Vec<u8>>`

  An optional OCSP response from the certificate issuer,
  attesting to its continued validity.

#### Implementations

- `fn from_der(cert_chain: Vec<CertificateDer<'static>>, key: PrivateKeyDer<'static>, provider: &CryptoProvider) -> Result<Self, Error>`
  Create a new `CertifiedKey` from a certificate chain and DER-encoded private key.

- `fn new(cert: Vec<CertificateDer<'static>>, key: alloc::sync::Arc<dyn SigningKey>) -> Self`
  Make a new CertifiedKey, with the given chain and key.

- `fn keys_match(self: &Self) -> Result<(), Error>`
  Verify the consistency of this [`CertifiedKey`]'s public and private keys.

- `fn end_entity_cert(self: &Self) -> Result<&CertificateDer<'_>, Error>`
  The end-entity certificate.

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

- `fn clone(self: &Self) -> CertifiedKey`

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

### `SingleCertAndKey`

```rust
struct SingleCertAndKey();
```

Server certificate resolver which always resolves to the same certificate and key.

For use with `ConfigBuilder::with_cert_resolver()`.


#### Trait Implementations

##### `impl From`

- `fn from(certified_key: alloc::sync::Arc<CertifiedKey>) -> Self`

##### `impl From`

- `fn from(certified_key: CertifiedKey) -> Self`

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

##### `impl ResolvesClientCert`

- `fn resolve(self: &Self, _root_hint_subjects: &[&[u8]], _sigschemes: &[SignatureScheme]) -> Option<alloc::sync::Arc<CertifiedKey>>`

- `fn has_certs(self: &Self) -> bool`

##### `impl ResolvesServerCert`

- `fn resolve(self: &Self, _client_hello: ClientHello<'_>) -> Option<alloc::sync::Arc<CertifiedKey>>`

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

