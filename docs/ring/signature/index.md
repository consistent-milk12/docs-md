*[ring](../index.md) / [signature](index.md)*

---

# Module `signature`

Public key signatures: signing and verification.

Use the `verify` function to verify signatures, passing a reference to the
algorithm that identifies the algorithm. See the documentation for `verify`
for examples.

For signature verification, this API treats each combination of parameters
as a separate algorithm. For example, instead of having a single "RSA"
algorithm with a verification function that takes a bunch of parameters,
there are `RSA_PKCS1_2048_8192_SHA256`, `RSA_PKCS1_2048_8192_SHA384`, etc.,
which encode sets of parameter choices into objects. This is designed to
reduce the risks of algorithm agility and to provide consistency with ECDSA
and EdDSA.

Currently this module does not support digesting the message to be signed
separately from the public key operation, as it is currently being
optimized for Ed25519 and for the implementation of protocols that do not
requiring signing large messages. An interface for efficiently supporting
larger messages may be added later.


# Algorithm Details

## `ECDSA_*_ASN1` Details: ASN.1-encoded ECDSA Signatures

The signature is a ASN.1 DER-encoded `Ecdsa-Sig-Value` as described in
[RFC 3279 Section 2.2.3]. This is the form of ECDSA signature used in
X.509-related structures and in TLS's `ServerKeyExchange` messages.

The public key is encoding in uncompressed form using the
Octet-String-to-Elliptic-Curve-Point algorithm in
[SEC 1: Elliptic Curve Cryptography, Version 2.0].

During verification, the public key is validated using the ECC Partial
Public-Key Validation Routine from Section 5.6.2.3.3 of
[NIST Special Publication 800-56A, revision 2] and Appendix A.3 of the
NSA's [Suite B implementer's guide to FIPS 186-3]. Note that, as explained
in the NSA guide, ECC Partial Public-Key Validation is equivalent to ECC
Full Public-Key Validation for prime-order curves like this one.

## `ECDSA_*_FIXED` Details: Fixed-length (PKCS#11-style) ECDSA Signatures

The signature is *r*||*s*, where || denotes concatenation, and where both
*r* and *s* are both big-endian-encoded values that are left-padded to the
maximum length. A P-256 signature will be 64 bytes long (two 32-byte
components) and a P-384 signature will be 96 bytes long (two 48-byte
components). This is the form of ECDSA signature used PKCS#11 and DNSSEC.

The public key is encoding in uncompressed form using the
Octet-String-to-Elliptic-Curve-Point algorithm in
[SEC 1: Elliptic Curve Cryptography, Version 2.0].

During verification, the public key is validated using the ECC Partial
Public-Key Validation Routine from Section 5.6.2.3.3 of
[NIST Special Publication 800-56A, revision 2] and Appendix A.3 of the
NSA's [Suite B implementer's guide to FIPS 186-3]. Note that, as explained
in the NSA guide, ECC Partial Public-Key Validation is equivalent to ECC
Full Public-Key Validation for prime-order curves like this one.

## `RSA_PKCS1_*` Details: RSA PKCS#1 1.5 Signatures

The signature is an RSASSA-PKCS1-v1_5 signature as described in
[RFC 3447 Section 8.2].

The public key is encoded as an ASN.1 `RSAPublicKey` as described in
[RFC 3447 Appendix-A.1.1]. The public key modulus length, rounded *up* to
the nearest (larger) multiple of 8 bits, must be in the range given in the
name of the algorithm. The public exponent must be an odd integer of 2-33
bits, inclusive.


## `RSA_PSS_*` Details: RSA PSS Signatures

The signature is an RSASSA-PSS signature as described in
[RFC 3447 Section 8.1].

The public key is encoded as an ASN.1 `RSAPublicKey` as described in
[RFC 3447 Appendix-A.1.1]. The public key modulus length, rounded *up* to
the nearest (larger) multiple of 8 bits, must be in the range given in the
name of the algorithm. The public exponent must be an odd integer of 2-33
bits, inclusive.

During verification, signatures will only be accepted if the MGF1 digest
algorithm is the same as the message digest algorithm and if the salt
length is the same length as the message digest. This matches the
requirements in TLS 1.3 and other recent specifications.

During signing, the message digest algorithm will be used as the MGF1
digest algorithm. The salt will be the same length as the message digest.
This matches the requirements in TLS 1.3 and other recent specifications.
Additionally, the entire salt is randomly generated separately for each
signature using the secure random number generator passed to `sign()`.


[SEC 1: Elliptic Curve Cryptography, Version 2.0]:
    http://www.secg.org/sec1-v2.pdf
[NIST Special Publication 800-56A, revision 2]:
    http://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-56Ar2.pdf
[Suite B implementer's guide to FIPS 186-3]:
    https://github.com/briansmith/ring/blob/main/doc/ecdsa.pdf
[RFC 3279 Section 2.2.3]:
    https://tools.ietf.org/html/rfc3279#section-2.2.3
[RFC 3447 Section 8.2]:
    https://tools.ietf.org/html/rfc3447#section-7.2
[RFC 3447 Section 8.1]:
    https://tools.ietf.org/html/rfc3447#section-8.1
[RFC 3447 Appendix-A.1.1]:
    https://tools.ietf.org/html/rfc3447#appendix-A.1.1


# Examples

## Signing and verifying with Ed25519

```
use ring::{
    rand,
    signature::{self, KeyPair},
};

# fn main() -> Result<(), ring::error::Unspecified> {
// Generate a key pair in PKCS#8 (v2) format.
let rng = rand::SystemRandom::new();
let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng)?;

// Normally the application would store the PKCS#8 file persistently. Later
// it would read the PKCS#8 file from persistent storage to use it.

let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())?;

// Sign the message "hello, world".
const MESSAGE: &[u8](#u8) = b"hello, world";
let sig = key_pair.sign(MESSAGE);

// Normally an application would extract the bytes of the signature and
// send them in a protocol message to the peer(s). Here we just get the
// public key key directly from the key pair.
let peer_public_key_bytes = key_pair.public_key().as_ref();

// Verify the signature of the message using the public key. Normally the
// verifier of the message would parse the inputs to this code out of the
// protocol message(s) sent by the signer.
let peer_public_key =
    signature::UnparsedPublicKey::new(&signature::ED25519, peer_public_key_bytes);
peer_public_key.verify(MESSAGE, sig.as_ref())?;

# Ok(())
# }
```

## Signing and verifying with RSA (PKCS#1 1.5 padding)

By default OpenSSL writes RSA public keys in SubjectPublicKeyInfo format,
not RSAPublicKey format, and Base64-encodes them (“PEM” format).

To convert the PEM SubjectPublicKeyInfo format (“BEGIN PUBLIC KEY”) to the
binary RSAPublicKey format needed by `verify()`, use:

```sh
openssl rsa -pubin \
            -in public_key.pem \
            -inform PEM \
            -RSAPublicKey_out \
            -outform DER \
            -out public_key.der
```

To extract the RSAPublicKey-formatted public key from an ASN.1 (binary)
DER-encoded RSAPrivateKey format private key file, use:

```sh
openssl rsa -in private_key.der \
            -inform DER \
            -RSAPublicKey_out \
            -outform DER \
            -out public_key.der
```

```
# #[cfg(feature = "std")]
use ring::{rand, rsa, signature};

# #[cfg(feature = "std")]
fn sign_and_verify_rsa(private_key_path: &std::path::Path,
                       public_key_path: &std::path::Path)
                       -> Result<(), MyError> {
// Create an RSA keypair from the DER-encoded bytes. This example uses
// a 2048-bit key, but larger keys are also supported.
let private_key_der = read_file(private_key_path)?;
let key_pair = rsa::KeyPair::from_der(&private_key_der)
    .map_err(|_| MyError::BadPrivateKey)?;

// Sign the message "hello, world", using PKCS#1 v1.5 padding and the
// SHA256 digest algorithm.
const MESSAGE: &'static [u8](#u8) = b"hello, world";
let rng = rand::SystemRandom::new();
let mut signature = vec![0; key_pair.public().modulus_len()];
key_pair.sign(&signature::RSA_PKCS1_SHA256, &rng, MESSAGE, &mut signature)
    .map_err(|_| MyError::OOM)?;

// Verify the signature.
let public_key =
    signature::UnparsedPublicKey::new(&signature::RSA_PKCS1_2048_8192_SHA256,
                                      read_file(public_key_path)?);
public_key.verify(MESSAGE, &signature)
    .map_err(|_| MyError::BadSignature)
}

#[derive(Debug)]
enum MyError {
#  #[cfg(feature = "std")]
   IO(std::io::Error),
   BadPrivateKey,
   OOM,
   BadSignature,
}

# #[cfg(feature = "std")]
fn read_file(path: &std::path::Path) -> Result<Vec<u8>, MyError> {
    use std::io::Read;

    let mut file = std::fs::File::open(path).map_err(|e| MyError::IO(e))?;
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents).map_err(|e| MyError::IO(e))?;
    Ok(contents)
}
#
# #[cfg(not(feature = "std"))]
# fn sign_and_verify_rsa(_private_key_path: &std::path::Path,
#                        _public_key_path: &std::path::Path)
#                        -> Result<(), ()> {
#     Ok(())
# }
#
# fn main() {
#     let private_key_path =
#         std::path::Path::new("src/rsa/signature_rsa_example_private_key.der");
#     let public_key_path =
#         std::path::Path::new("src/rsa/signature_rsa_example_public_key.der");
#     sign_and_verify_rsa(&private_key_path, &public_key_path).unwrap()
# }
```

## Structs

### `Ed25519KeyPair`

```rust
struct Ed25519KeyPair {
}
```

An Ed25519 key pair, for signing.

#### Implementations

- `fn generate_pkcs8(rng: &dyn rand::SecureRandom) -> Result<pkcs8::Document, error::Unspecified>`
  Generates a new key pair and returns the key pair serialized as a

- `fn from_pkcs8(pkcs8: &[u8]) -> Result<Self, error::KeyRejected>`
  Constructs an Ed25519 key pair by parsing an unencrypted PKCS#8 v2

- `fn from_pkcs8_maybe_unchecked(pkcs8: &[u8]) -> Result<Self, error::KeyRejected>`
  Constructs an Ed25519 key pair by parsing an unencrypted PKCS#8 v1 or v2

- `fn from_seed_and_public_key(seed: &[u8], public_key: &[u8]) -> Result<Self, error::KeyRejected>`
  Constructs an Ed25519 key pair from the private key seed `seed` and its

- `fn from_seed_unchecked(seed: &[u8]) -> Result<Self, error::KeyRejected>`
  Constructs a Ed25519 key pair from the private key seed `seed`.

- `fn sign(self: &Self, msg: &[u8]) -> signature::Signature`
  Returns the signature of the message `msg`.

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

##### `impl KeyPair`

- `type PublicKey = PublicKey`

- `fn public_key(self: &Self) -> &<Self as >::PublicKey`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error>`

### `EdDSAParameters`

```rust
struct EdDSAParameters;
```

Parameters for EdDSA signing and verification.

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

##### `impl VerificationAlgorithm`

- `fn verify(self: &Self, public_key: untrusted::Input<'_>, msg: untrusted::Input<'_>, signature: untrusted::Input<'_>) -> Result<(), error::Unspecified>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

### `EcdsaKeyPair`

```rust
struct EcdsaKeyPair {
}
```

An ECDSA key pair, used for signing.

#### Implementations

- `fn generate_pkcs8(alg: &'static EcdsaSigningAlgorithm, rng: &dyn rand::SecureRandom) -> Result<pkcs8::Document, error::Unspecified>`
  Generates a new key pair and returns the key pair serialized as a

- `fn from_pkcs8(alg: &'static EcdsaSigningAlgorithm, pkcs8: &[u8], rng: &dyn rand::SecureRandom) -> Result<Self, error::KeyRejected>`
  Constructs an ECDSA key pair by parsing an unencrypted PKCS#8 v1

- `fn from_private_key_and_public_key(alg: &'static EcdsaSigningAlgorithm, private_key: &[u8], public_key: &[u8], rng: &dyn rand::SecureRandom) -> Result<Self, error::KeyRejected>`
  Constructs an ECDSA key pair from the private key and public key bytes

- `fn sign(self: &Self, rng: &dyn rand::SecureRandom, message: &[u8]) -> Result<signature::Signature, error::Unspecified>`
  Returns the signature of the `message` using a random nonce generated by `rng`.

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

##### `impl KeyPair`

- `type PublicKey = PublicKey`

- `fn public_key(self: &Self) -> &<Self as >::PublicKey`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error>`

### `EcdsaSigningAlgorithm`

```rust
struct EcdsaSigningAlgorithm {
}
```

An ECDSA signing algorithm.

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

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error>`

### `EcdsaVerificationAlgorithm`

```rust
struct EcdsaVerificationAlgorithm {
}
```

An ECDSA verification algorithm.

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

##### `impl VerificationAlgorithm`

- `fn verify(self: &Self, public_key: untrusted::Input<'_>, msg: untrusted::Input<'_>, signature: untrusted::Input<'_>) -> Result<(), error::Unspecified>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error>`

### `RsaPublicKeyComponents<B>`

```rust
struct RsaPublicKeyComponents<B> {
    pub n: B,
    pub e: B,
}
```

RSA public key components.

`B` must implement `AsRef<[u8](#u8)>` like `&[u8](#u8)` or `Vec<u8>`.

#### Fields

- **`n`**: `B`

  The public modulus, encoded in big-endian bytes without leading zeros.

- **`e`**: `B`

  The public exponent, encoded in big-endian bytes without leading zeros.

#### Implementations

- `fn verify(self: &Self, params: &RsaParameters, message: &[u8], signature: &[u8]) -> Result<(), error::Unspecified>`
  Verifies that `signature` is a valid signature of `message` using `self`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<B>`

- `fn from(public_key: &PublicKey) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<B: $crate::clone::Clone>`

- `fn clone(self: &Self) -> PublicKeyComponents<B>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<B: $crate::marker::Copy>`

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

##### `impl Debug<B>`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

### `RsaParameters`

```rust
struct RsaParameters {
}
```

Parameters for RSA verification.

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

##### `impl VerificationAlgorithm`

- `fn verify(self: &Self, public_key: untrusted::Input<'_>, msg: untrusted::Input<'_>, signature: untrusted::Input<'_>) -> Result<(), error::Unspecified>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Signature`

```rust
struct Signature {
}
```

A public key signature returned from a signing operation.

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

- `fn clone(self: &Self) -> Signature`

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

### `UnparsedPublicKey<B>`

```rust
struct UnparsedPublicKey<B> {
}
```

An unparsed, possibly malformed, public key for signature verification.

#### Implementations

- `fn new(algorithm: &'static dyn VerificationAlgorithm, bytes: B) -> Self`
  Construct a new `UnparsedPublicKey`.

- `fn verify(self: &Self, message: &[u8], signature: &[u8]) -> Result<(), error::Unspecified>`
  Parses the public key and verifies `signature` is a valid signature of

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsRef<B>`

- `fn as_ref(self: &Self) -> &[u8]`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<B: $crate::clone::Clone>`

- `fn clone(self: &Self) -> UnparsedPublicKey<B>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<B: $crate::marker::Copy>`

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

##### `impl Debug<B>`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

## Traits

### `KeyPair`

```rust
trait KeyPair: core::fmt::Debug + Send + Sized + Sync { ... }
```

Key pairs for signing messages (private key and public key).

#### Required Methods

- `type PublicKey: 6`

- `fn public_key(self: &Self) -> &<Self as >::PublicKey`

  The public key for the key pair.

### `VerificationAlgorithm`

```rust
trait VerificationAlgorithm: core::fmt::Debug + Sync + sealed::Sealed { ... }
```

A signature verification algorithm.

#### Required Methods

- `fn verify(self: &Self, public_key: untrusted::Input<'_>, msg: untrusted::Input<'_>, signature: untrusted::Input<'_>) -> Result<(), error::Unspecified>`

  Verify the signature `signature` of message `msg` with the public key

## Type Aliases

### `RsaKeyPair`

```rust
type RsaKeyPair = crate::rsa::KeyPair;
```

An RSA key pair, used for signing.

## Constants

