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

## Functions

### `ErrorKind`

```rust
fn ErrorKind(&self, row: &LineRow) -> Location<'_>
```

*Defined in [`addr2line-0.25.1/src/line.rs:128-140`](../../../.source_1765521767/addr2line-0.25.1/src/line.rs#L128-L140)*

### `Bytes`

```rust
fn Bytes(self) -> usize
```

*Defined in [`aho-corasick-1.1.4/src/util/int.rs:89`](../../../.source_1765521767/aho-corasick-1.1.4/src/util/int.rs#L89)*

