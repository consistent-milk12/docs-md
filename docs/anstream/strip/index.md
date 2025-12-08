*[anstream](../index.md) / [strip](index.md)*

---

# Module `strip`

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

- `fn new(raw: S) -> Self`

- `fn into_inner(self: Self) -> S`

- `fn as_inner(self: &Self) -> &S`

#### Trait Implementations

##### `impl<S> Debug for StripStream<S>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<S> Write for StripStream<S>`

- `fn write(self: &mut Self, buf: &[u8]) -> std::io::Result<usize>`

- `fn write_vectored(self: &mut Self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize>`

- `fn flush(self: &mut Self) -> std::io::Result<()>`

- `fn write_all(self: &mut Self, buf: &[u8]) -> std::io::Result<()>`

- `fn write_fmt(self: &mut Self, args: std::fmt::Arguments<'_>) -> std::io::Result<()>`

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

