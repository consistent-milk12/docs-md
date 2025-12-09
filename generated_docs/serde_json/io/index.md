*[serde_json](../index.md) / [io](index.md)*

---

# Module `io`

A tiny, `no_std`-friendly facade around `std::io`.
Reexports types from `std` when available; otherwise reimplements and
provides some of the core logic.

The main reason that `std::io` hasn't found itself reexported as part of
the `core` crate is the `std::io::{Read, Write}` traits' reliance on
`std::io::Error`, which may contain internally a heap-allocated `Box<Error>`
and/or now relying on OS-specific `std::backtrace::Backtrace`.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ErrorKind`](#errorkind) | fn |  |
| [`Bytes`](#bytes) | fn |  |
| [`Read`](#read) | fn |  |

## Functions

*Defined in [`serde_json-1.0.145/src/io/mod.rs:10`](../../../.source_1765210505/serde_json-1.0.145/src/io/mod.rs#L10)*

*Defined in [`serde_json-1.0.145/src/io/mod.rs:20`](../../../.source_1765210505/serde_json-1.0.145/src/io/mod.rs#L20)*

*Defined in [`serde_json-1.0.145/src/io/mod.rs:20`](../../../.source_1765210505/serde_json-1.0.145/src/io/mod.rs#L20)*

