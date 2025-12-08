*[anstream](../index.md) / [fmt](index.md)*

---

# Module `fmt`

## Structs

### `Adapter<W>`

```rust
struct Adapter<W>
where
    W: FnMut(&[u8]) -> std::io::Result<()> {
    writer: W,
    error: std::io::Result<()>,
}
```

A shim which allows a [`std::io::Write`](../../fs_err/index.md) to be implemented in terms of a [`std::fmt::Write`](../../fs_err/index.md)

This saves off I/O errors. instead of discarding them

#### Implementations

- `fn new(writer: W) -> Self`

- `fn write_fmt(self: Self, fmt: std::fmt::Arguments<'_>) -> std::io::Result<()>`

#### Trait Implementations

##### `impl<W> Write for Adapter<W>`

- `fn write_str(self: &mut Self, s: &str) -> std::fmt::Result`

