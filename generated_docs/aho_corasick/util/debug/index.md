*[aho_corasick](../../index.md) / [util](../index.md) / [debug](index.md)*

---

# Module `debug`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugByte`](#debugbyte) | struct | A type that wraps a single byte with a convenient fmt::Debug impl that escapes the byte. |

## Structs

### `DebugByte`

```rust
struct DebugByte(u8);
```

*Defined in [`aho-corasick-1.1.4/src/util/debug.rs:3`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/debug.rs#L3)*

A type that wraps a single byte with a convenient fmt::Debug impl that
escapes the byte.

#### Trait Implementations

##### `impl Debug for DebugByte`

- <span id="debugbyte-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

