*[rustls](../../../index.md) / [crypto](../../index.md) / [ring](../index.md) / [sign](index.md)*

---

# Module `sign`

Using software keys for authentication.

## Functions

### `any_supported_type`

```rust
fn any_supported_type(der: &pki_types::PrivateKeyDer<'_>) -> Result<alloc::sync::Arc<dyn SigningKey>, crate::error::Error>
```

Parse `der` as any supported key encoding/type, returning
the first which works.

### `any_ecdsa_type`

```rust
fn any_ecdsa_type(der: &pki_types::PrivateKeyDer<'_>) -> Result<alloc::sync::Arc<dyn SigningKey>, crate::error::Error>
```

Parse `der` as any ECDSA key type, returning the first which works.

Both SEC1 (PEM section starting with 'BEGIN EC PRIVATE KEY') and PKCS8
(PEM section starting with 'BEGIN PRIVATE KEY') encodings are supported.

### `any_eddsa_type`

```rust
fn any_eddsa_type(der: &pki_types::PrivatePkcs8KeyDer<'_>) -> Result<alloc::sync::Arc<dyn SigningKey>, crate::error::Error>
```

Parse `der` as any EdDSA key type, returning the first which works.

Note that, at the time of writing, Ed25519 does not have wide support
in browsers.  It is also not supported by the WebPKI, because the
CA/Browser Forum Baseline Requirements do not support it for publicly
trusted certificates.

