# Crate `webpki_roots`

A compiled-in copy of the root certificates trusted by Mozilla.

To use this library with rustls 0.22:

```rust
let root_store = rustls::RootCertStore {
  roots: webpki_roots::TLS_SERVER_ROOTS.to_vec(),
};
```

This library is suitable for use in applications that can always be recompiled and instantly deployed.
For applications that are deployed to end-users and cannot be recompiled, or which need certification
before deployment, consider a library that uses the platform native certificate verifier such as
[rustls-platform-verifier]. This has the additional benefit of supporting OS provided CA constraints
and revocation data.

[rustls-platform-verifier]: https://docs.rs/rustls-platform-verifier

## Constants

### `TLS_SERVER_ROOTS`

```rust
const TLS_SERVER_ROOTS: &[pki_types::TrustAnchor<'static>];
```

