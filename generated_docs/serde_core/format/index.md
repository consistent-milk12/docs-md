*[serde_core](../index.md) / [format](index.md)*

---

# Module `format`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Buf`](#buf) | struct |  |

## Structs

### `Buf<'a>`

```rust
struct Buf<'a> {
    bytes: &'a mut [u8],
    offset: usize,
}
```

*Defined in [`serde_core-1.0.228/src/format.rs:4-7`](../../../.source_1765210505/serde_core-1.0.228/src/format.rs#L4-L7)*

#### Implementations

- <span id="buf-new"></span>`fn new(bytes: &'a mut [u8]) -> Self`

- <span id="buf-as-str"></span>`fn as_str(&self) -> &str`

#### Trait Implementations

##### `impl Write for Buf<'a>`

- <span id="buf-write-str"></span>`fn write_str(&mut self, s: &str) -> fmt::Result`

