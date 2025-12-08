*[serde_core](../index.md) / [format](index.md)*

---

# Module `format`

## Structs

### `Buf<'a>`

```rust
struct Buf<'a> {
    bytes: &'a mut [u8],
    offset: usize,
}
```

#### Implementations

- `fn new(bytes: &'a mut [u8]) -> Self`

- `fn as_str(self: &Self) -> &str`

#### Trait Implementations

##### `impl<'a> Write for Buf<'a>`

- `fn write_str(self: &mut Self, s: &str) -> fmt::Result`

