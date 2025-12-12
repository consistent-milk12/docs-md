*[rustix](../index.md) / [not_implemented](index.md)*

---

# Module `not_implemented`

Documentation about unimplemented functions.

This module contains documentation for several functions that rustix does
not implement, either because they are out of scope, or because they are
could probably be implemented but are not yet.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`memory_allocation`](#memory-allocation) | mod | Memory-allocation functions are out of scope for rustix. |
| [`libc_internals`](#libc-internals) | mod | Functions which need access to libc internals are out of scope for rustix. |
| [`higher_level`](#higher-level) | mod | Functions which provide higher-level functionality are out of scope for rustix. |
| [`impossible`](#impossible) | mod | Functions which don't seem possible to even call from Rust with current language features, even with `unsafe`. |
| [`yet`](#yet) | mod | These functions are not yet implemented in rustix, but probably could be. |
| [`quite_yet`](#quite-yet) | mod | These functions are not quite yet finished in rustix. |
| [`not_implemented!`](#not-implemented) | macro |  |

## Modules

- [`memory_allocation`](memory_allocation/index.md) — Memory-allocation functions are out of scope for rustix.
- [`libc_internals`](libc_internals/index.md) — Functions which need access to libc internals are out of scope for rustix.
- [`higher_level`](higher_level/index.md) — Functions which provide higher-level functionality are out of scope for
- [`impossible`](impossible/index.md) — Functions which don't seem possible to even call from Rust with current
- [`yet`](yet/index.md) — These functions are not yet implemented in rustix, but probably could be.
- [`quite_yet`](quite_yet/index.md) — These functions are not quite yet finished in rustix.

## Macros

### `not_implemented!`

*Defined in [`rustix-1.1.2/src/not_implemented.rs:7-14`](../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L7-L14)*

