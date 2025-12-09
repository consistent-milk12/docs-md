*[rustix](../index.md) / [maybe_polyfill](index.md)*

---

# Module `maybe_polyfill`

Imports from `std` that would be polyfilled for `no_std` builds (see
`src/polyfill/no_std`).

This implementation is used when `std` is available and just imports the
necessary items from `std`. For `no_std` builds, the file
`src/polyfill/no_std` is used instead, which doesn't depend on the standard
library.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`io`](#io) | mod |  |
| [`os`](#os) | mod |  |

## Modules

- [`io`](io/index.md)
- [`os`](os/index.md)

