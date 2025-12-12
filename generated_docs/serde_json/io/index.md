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

### `ErrorKind`

```rust
fn ErrorKind(&self) -> impl Iterator<Item = gimli::Range> + '_
```

*Defined in [`addr2line-0.25.1/src/line.rs:121-126`](../../../.source_1765210505/addr2line-0.25.1/src/line.rs#L121-L126)*

### `Bytes`

```rust
fn Bytes(self) -> u16
```

*Defined in [`aho-corasick-1.1.4/src/util/int.rs:83-85`](../../../.source_1765210505/aho-corasick-1.1.4/src/util/int.rs#L83-L85)*

### `Read`

```rust
fn Read(&self) -> &T
```

