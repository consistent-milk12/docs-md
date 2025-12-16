*[rustix](../index.md) / [fd](index.md)*

---

# Module `fd`

Export the `*Fd` types and traits that are used in rustix's public API.

This module exports the types and traits from `std::os::fd`, or polyills
on Rust < 1.66 or on Windows.

On Windows, the polyfill consists of aliases of the socket types and
traits, For example, `OwnedSocket` is aliased to `OwnedFd`, and so on,
and there are blanket impls for `AsFd` etc. that map to `AsSocket` impls.
These blanket impls suffice for using the traits, however not for
implementing them, so this module also exports `AsSocket` and the other
traits as-is so that users can implement them if needed.


