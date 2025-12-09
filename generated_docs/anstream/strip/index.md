*[anstream](../index.md) / [strip](index.md)*

---

# Module `strip`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`StripStream`](#stripstream) | struct | Only pass printable data to the inner `Write` |
| [`write`](#write) | fn |  |
| [`write_all`](#write_all) | fn |  |
| [`write_fmt`](#write_fmt) | fn |  |
| [`offset_to`](#offset_to) | fn |  |

## Structs

### `StripStream<S>`

```rust
struct StripStream<S>
where
    S: std::io::Write {
    raw: S,
    state: crate::adapter::StripBytes,
}
```

Only pass printable data to the inner `Write`

#### Implementations

- <span id="stripstream-lock"></span>`fn lock(self) -> StripStream<std::io::StdoutLock<'static>>` — [`StripStream`](../index.md)

#### Trait Implementations

##### `impl<S> Debug for StripStream<S>`

- <span id="stripstream-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S> Write for StripStream<S>`

- <span id="stripstream-write"></span>`fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>`

- <span id="stripstream-write-vectored"></span>`fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize>`

- <span id="stripstream-flush"></span>`fn flush(&mut self) -> std::io::Result<()>`

- <span id="stripstream-write-all"></span>`fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()>`

- <span id="stripstream-write-fmt"></span>`fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::io::Result<()>`

## Functions

### `write`

```rust
fn write(raw: &mut dyn std::io::Write, state: &mut crate::adapter::StripBytes, buf: &[u8]) -> std::io::Result<usize>
```

### `write_all`

```rust
fn write_all(raw: &mut dyn std::io::Write, state: &mut crate::adapter::StripBytes, buf: &[u8]) -> std::io::Result<()>
```

### `write_fmt`

```rust
fn write_fmt(raw: &mut dyn std::io::Write, state: &mut crate::adapter::StripBytes, args: std::fmt::Arguments<'_>) -> std::io::Result<()>
```

### `offset_to`

```rust
fn offset_to(total: &[u8], subslice: &[u8]) -> usize
```

