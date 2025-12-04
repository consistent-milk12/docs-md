*[rustls_pki_types](../index.md) / [alg_id](index.md)*

---

# Module `alg_id`

The PKIX [`AlgorithmIdentifier`](alg_id/index.md) type, and common values.

If you need to use an [`AlgorithmIdentifier`](alg_id/index.md) not defined here,
you can define it locally.

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

```
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

## Constants

### `ML_DSA_44`

```rust
const ML_DSA_44: AlgorithmIdentifier;
```

AlgorithmIdentifier for `id-ml-dsa-44`.

This is:

```text
OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.3.17 }
```

<https://www.ietf.org/archive/id/draft-ietf-lamps-dilithium-certificates-07.html#name-identifiers>

### `ML_DSA_65`

```rust
const ML_DSA_65: AlgorithmIdentifier;
```

AlgorithmIdentifier for `id-ml-dsa-65`.

This is:

```text
OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.3.18 }
```

<https://www.ietf.org/archive/id/draft-ietf-lamps-dilithium-certificates-07.html#name-identifiers>

### `ML_DSA_87`

```rust
const ML_DSA_87: AlgorithmIdentifier;
```

AlgorithmIdentifier for `id-ml-dsa-87`.

This is:

```text
OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.3.19 }
```

<https://www.ietf.org/archive/id/draft-ietf-lamps-dilithium-certificates-07.html#name-identifiers>

### `ECDSA_P256`

```rust
const ECDSA_P256: AlgorithmIdentifier;
```

AlgorithmIdentifier for `id-ecPublicKey` with named curve `secp256r1`.

This is:

```text
# ecPublicKey
OBJECT_IDENTIFIER { 1.2.840.10045.2.1 }
# secp256r1
OBJECT_IDENTIFIER { 1.2.840.10045.3.1.7 }
```

### `ECDSA_P384`

```rust
const ECDSA_P384: AlgorithmIdentifier;
```

AlgorithmIdentifier for `id-ecPublicKey` with named curve `secp384r1`.

This is:

```text
# ecPublicKey
OBJECT_IDENTIFIER { 1.2.840.10045.2.1 }
# secp384r1
OBJECT_IDENTIFIER { 1.3.132.0.34 }
```

### `ECDSA_P521`

```rust
const ECDSA_P521: AlgorithmIdentifier;
```

AlgorithmIdentifier for `id-ecPublicKey` with named curve `secp521r1`.

This is:

```text
# ecPublicKey
OBJECT_IDENTIFIER { 1.2.840.10045.2.1 }
# secp521r1
OBJECT_IDENTIFIER { 1.3.132.0.35 }
```

### `ECDSA_SHA256`

```rust
const ECDSA_SHA256: AlgorithmIdentifier;
```

AlgorithmIdentifier for `ecdsa-with-SHA256`.

This is:

```text
# ecdsa-with-SHA256
OBJECT_IDENTIFIER { 1.2.840.10045.4.3.2 }
```

### `ECDSA_SHA384`

```rust
const ECDSA_SHA384: AlgorithmIdentifier;
```

AlgorithmIdentifier for `ecdsa-with-SHA384`.

This is:

```text
# ecdsa-with-SHA384
OBJECT_IDENTIFIER { 1.2.840.10045.4.3.3 }
```

### `ECDSA_SHA512`

```rust
const ECDSA_SHA512: AlgorithmIdentifier;
```

AlgorithmIdentifier for `ecdsa-with-SHA512`.

This is:

```text
# ecdsa-with-SHA512
OBJECT_IDENTIFIER { 1.2.840.10045.4.3.4 }
```

### `RSA_ENCRYPTION`

```rust
const RSA_ENCRYPTION: AlgorithmIdentifier;
```

AlgorithmIdentifier for `rsaEncryption`.

This is:

```text
# rsaEncryption
OBJECT_IDENTIFIER { 1.2.840.113549.1.1.1 }
NULL {}
```

### `RSA_PKCS1_SHA256`

```rust
const RSA_PKCS1_SHA256: AlgorithmIdentifier;
```

AlgorithmIdentifier for `sha256WithRSAEncryption`.

This is:

```text
# sha256WithRSAEncryption
OBJECT_IDENTIFIER { 1.2.840.113549.1.1.11 }
NULL {}
```

### `RSA_PKCS1_SHA384`

```rust
const RSA_PKCS1_SHA384: AlgorithmIdentifier;
```

AlgorithmIdentifier for `sha384WithRSAEncryption`.

This is:

```text
# sha384WithRSAEncryption
OBJECT_IDENTIFIER { 1.2.840.113549.1.1.12 }
NULL {}
```

### `RSA_PKCS1_SHA512`

```rust
const RSA_PKCS1_SHA512: AlgorithmIdentifier;
```

AlgorithmIdentifier for `sha512WithRSAEncryption`.

This is:

```text
# sha512WithRSAEncryption
OBJECT_IDENTIFIER { 1.2.840.113549.1.1.13 }
NULL {}
```

### `RSA_PSS_SHA256`

```rust
const RSA_PSS_SHA256: AlgorithmIdentifier;
```

AlgorithmIdentifier for `rsassaPss` with:

- hashAlgorithm: sha256
- maskGenAlgorithm: mgf1 with sha256
- saltLength: 32

This is:

```text
# rsassa-pss
OBJECT_IDENTIFIER { 1.2.840.113549.1.1.10 }
SEQUENCE {
  # hashAlgorithm:
  [0] {
    SEQUENCE {
      # sha256
      OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.2.1 }
      NULL {}
    }
  }
  # maskGenAlgorithm:
  [1] {
    SEQUENCE {
      # mgf1
      OBJECT_IDENTIFIER { 1.2.840.113549.1.1.8 }
      SEQUENCE {
        # sha256
        OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.2.1 }
        NULL {}
      }
    }
  }
  # saltLength:
  [2] {
    INTEGER { 32 }
  }
}
```

See <https://datatracker.ietf.org/doc/html/rfc4055#section-3.1> for
the meaning of the context-specific tags.

### `RSA_PSS_SHA384`

```rust
const RSA_PSS_SHA384: AlgorithmIdentifier;
```

AlgorithmIdentifier for `rsassaPss` with:

- hashAlgorithm: sha384
- maskGenAlgorithm: mgf1 with sha384
- saltLength: 48

This is:

```text
# rsassa-pss
OBJECT_IDENTIFIER { 1.2.840.113549.1.1.10 }
SEQUENCE {
  # hashAlgorithm:
  [0] {
    SEQUENCE {
      # sha384
      OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.2.2 }
      NULL {}
    }
  }
  # maskGenAlgorithm:
  [1] {
    SEQUENCE {
      # mgf1
      OBJECT_IDENTIFIER { 1.2.840.113549.1.1.8 }
      SEQUENCE {
        # sha384
        OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.2.2 }
        NULL {}
      }
    }
  }
  # saltLength:
  [2] {
    INTEGER { 48 }
  }
}
```

See <https://datatracker.ietf.org/doc/html/rfc4055#section-3.1> for
the meaning of the context-specific tags.

### `RSA_PSS_SHA512`

```rust
const RSA_PSS_SHA512: AlgorithmIdentifier;
```

AlgorithmIdentifier for `rsassaPss` with:

- hashAlgorithm: sha512
- maskGenAlgorithm: mgf1 with sha512
- saltLength: 64

This is:

```text
# rsassa-pss
OBJECT_IDENTIFIER { 1.2.840.113549.1.1.10 }
SEQUENCE {
  # hashAlgorithm:
  [0] {
    SEQUENCE {
      # sha512
      OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.2.3 }
      NULL {}
    }
  }
  # maskGenAlgorithm:
  [1] {
    SEQUENCE {
      # mgf1
      OBJECT_IDENTIFIER { 1.2.840.113549.1.1.8 }
      SEQUENCE {
        # sha512
        OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.2.3 }
        NULL {}
      }
    }
  }
  # saltLength:
  [2] {
    INTEGER { 64 }
  }
}
```

See <https://datatracker.ietf.org/doc/html/rfc4055#section-3.1> for
the meaning of the context-specific tags.

### `ED25519`

```rust
const ED25519: AlgorithmIdentifier;
```

AlgorithmIdentifier for `ED25519`.

This is:

```text
# ed25519
OBJECT_IDENTIFIER { 1.3.101.112 }
```

### `ED448`

```rust
const ED448: AlgorithmIdentifier;
```

AlgorithmIdentifier for `ED448`.

This is:

```text
# ed448
OBJECT_IDENTIFIER { 1.3.101.113 }
```

