*[anstream](../index.md) / [fmt](index.md)*

---

# Module `fmt`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Adapter`](#adapter) | struct | A shim which allows a [`std::io::Write`] to be implemented in terms of a [`std::fmt::Write`] |

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

- <span id="adapter-new"></span>`fn new(writer: W) -> Self`

- <span id="adapter-write-fmt"></span>`fn write_fmt(self, fmt: std::fmt::Arguments<'_>) -> std::io::Result<()>`

#### Trait Implementations

##### `impl<W> Write for Adapter<W>`

- <span id="adapter-write-str"></span>`fn write_str(&mut self, s: &str) -> std::fmt::Result`

