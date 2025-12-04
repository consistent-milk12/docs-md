*[rustls_pki_types](../index.md) / [pem](index.md)*

---

# Module `pem`

Low-level PEM decoding APIs.

These APIs allow decoding PEM format in an iterator, which means you
can load multiple different types of PEM section from a file in a single
pass.

## Structs

### `ReadIter<R, T>`

```rust
struct ReadIter<R, T> {
    // [REDACTED: Private Fields]
}
```

Extract and return all PEM sections by reading `rd`.

#### Implementations

- `fn new(rd: R) -> Self`
  Create a new iterator.

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

##### `impl Iterator<R: io::BufRead, T: PemObject>`

- `type Item = Result<T, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `SliceIter<'a, T>`

```rust
struct SliceIter<'a, T> {
    // [REDACTED: Private Fields]
}
```

Iterator over all PEM sections in a `&[u8](#u8)
` slice.

#### Implementations

- `fn new(current: &'a [u8]) -> Self`
  Create a new iterator.

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

##### `impl Iterator<T: PemObject>`

- `type Item = Result<T, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `SectionKind`

```rust
enum SectionKind {
    Certificate,
    PublicKey,
    RsaPrivateKey,
    PrivateKey,
    EcPrivateKey,
    Crl,
    Csr,
    EchConfigList,
}
```

A single recognised section in a PEM file.

#### Variants

- **`Certificate`**

  A DER-encoded x509 certificate.
  
  Appears as "CERTIFICATE" in PEM files.

- **`PublicKey`**

  A DER-encoded Subject Public Key Info; as specified in RFC 7468.
  
  Appears as "PUBLIC KEY" in PEM files.

- **`RsaPrivateKey`**

  A DER-encoded plaintext RSA private key; as specified in PKCS #1/RFC 3447
  
  Appears as "RSA PRIVATE KEY" in PEM files.

- **`PrivateKey`**

  A DER-encoded plaintext private key; as specified in PKCS #8/RFC 5958
  
  Appears as "PRIVATE KEY" in PEM files.

- **`EcPrivateKey`**

  A Sec1-encoded plaintext private key; as specified in RFC 5915
  
  Appears as "EC PRIVATE KEY" in PEM files.

- **`Crl`**

  A Certificate Revocation List; as specified in RFC 5280
  
  Appears as "X509 CRL" in PEM files.

- **`Csr`**

  A Certificate Signing Request; as specified in RFC 2986
  
  Appears as "CERTIFICATE REQUEST" in PEM files.

- **`EchConfigList`**

  An EchConfigList structure, as specified in
  <https://www.ietf.org/archive/id/draft-farrell-tls-pemesni-05.html>.
  
  Appears as "ECHCONFIG" in PEM files.

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

- `fn clone(self: &Self) -> SectionKind`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &SectionKind) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom`

- `type Error = ()`

- `fn try_from(value: &[u8]) -> Result<Self, <Self as >::Error>`

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
    MissingSectionEnd {
        end_marker: alloc::vec::Vec<u8>,
    },
    IllegalSectionStart {
        line: alloc::vec::Vec<u8>,
    },
    Base64Decode(alloc::string::String),
    Io(io::Error),
    NoItemsFound,
}
```

Errors that may arise when parsing the contents of a PEM file

#### Variants

- **`MissingSectionEnd`**

  a section is missing its "END marker" line

- **`IllegalSectionStart`**

  syntax error found in the line that starts a new section

- **`Base64Decode`**

  base64 decode error

- **`Io`**

  I/O errors, from APIs that accept `std::io` types.

- **`NoItemsFound`**

  No items found of desired type

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

## Traits

### `PemObject`

```rust
trait PemObject: Sized { ... }
```

Items that can be decoded from PEM data.

#### Required Methods

- `fn from_pem_slice(pem: &[u8]) -> Result<Self, Error>`

  Decode the first section of this type from PEM contained in

- `fn pem_slice_iter(pem: &[u8]) -> SliceIter<'_, Self>`

  Iterate over all sections of this type from PEM contained in

- `fn from_pem_file(file_name: impl AsRef<std::path::Path>) -> Result<Self, Error>`

  Decode the first section of this type from the PEM contents of the named file.

- `fn pem_file_iter(file_name: impl AsRef<std::path::Path>) -> Result<ReadIter<io::BufReader<File>, Self>, Error>`

  Iterate over all sections of this type from the PEM contents of the named file.

- `fn from_pem_reader(rd: impl std::io::Read) -> Result<Self, Error>`

  Decode the first section of this type from PEM read from an `io::Read`.

- `fn pem_reader_iter<R: std::io::Read>(rd: R) -> ReadIter<io::BufReader<R>, Self>`

  Iterate over all sections of this type from PEM present in an `io::Read`.

- `fn from_pem(kind: SectionKind, der: Vec<u8>) -> Option<Self>`

  Conversion from a PEM [`SectionKind`](#sectionkind) and body data.

## Functions

### `from_buf`

```rust
fn from_buf(rd: &mut dyn io::BufRead) -> Result<Option<(SectionKind, alloc::vec::Vec<u8>)>, Error>
```

Extract and decode the next supported PEM section from `rd`.

- Ok(None) is returned if there is no PEM section read from `rd`.
- Underlying IO errors produce a `Err(...)`
- Otherwise each decoded section is returned with a `Ok(Some(...))`

