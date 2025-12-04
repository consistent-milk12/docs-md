*[rustls](../index.md) / [manual](index.md)*

---

# Module `manual`

 This is the rustls manual.

This documentation primarily aims to explain design decisions taken in rustls.

It does this from a few aspects: how rustls attempts to avoid construction errors
that occurred in other TLS libraries, how rustls attempts to avoid past TLS
protocol vulnerabilities, and assorted advice for achieving common tasks with rustls.

## Modules

- [`_01_impl_vulnerabilities`](_01_impl_vulnerabilities/index.md) -  This section discusses vulnerabilities in other TLS implementations, theorising their
- [`_02_tls_vulnerabilities`](_02_tls_vulnerabilities/index.md) -  This section discusses vulnerabilities and design errors in the TLS protocol.
- [`_03_howto`](_03_howto/index.md) -  This section collects together goal-oriented documentation.
- [`_04_features`](_04_features/index.md) -  This section documents rustls itself: what protocol features are and are not implemented.
- [`_05_defaults`](_05_defaults/index.md) -  This section provides rationale for the defaults in rustls.
- [`_06_fips`](_06_fips/index.md) -  This section provides guidance on using rustls with FIPS-approved cryptography.

