*[aho_corasick](../../index.md) / [util](../index.md) / [debug](index.md)*

---

# Module `debug`

## Structs

### `DebugByte`

```rust
struct DebugByte(u8);
```

A type that wraps a single byte with a convenient fmt::Debug impl that
escapes the byte.

#### Trait Implementations

##### `impl Debug for DebugByte`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

