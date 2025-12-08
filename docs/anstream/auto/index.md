*[anstream](../index.md) / [auto](index.md)*

---

# Module `auto`

## Structs

### `AutoStream<S: RawStream>`

```rust
struct AutoStream<S: RawStream> {
    inner: StreamInner<S>,
}
```

[`std::io::Write`](../../fs_err/index.md) that adapts ANSI escape codes to the underlying `Write`s capabilities

This includes
- Stripping colors for non-terminals
- Respecting env variables like [NO_COLOR](https://no-color.org/) or [CLICOLOR](https://bixense.com/clicolors/)
- *(windows)* Falling back to the wincon API where [ENABLE_VIRTUAL_TERMINAL_PROCESSING](https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#output-sequences) is unsupported

You can customize auto-detection by calling into
[anstyle_query](https://docs.rs/anstyle-query/latest/anstyle_query/)
to get a [`ColorChoice`](../index.md) and then calling `AutoStream::new(stream, choice)`.

#### Implementations

- `fn lock(self: Self) -> AutoStream<std::io::StdoutLock<'static>>` — [`AutoStream`](../index.md)

#### Trait Implementations

##### `impl<S: $crate::fmt::Debug + RawStream> Debug for AutoStream<S>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<S> Write for AutoStream<S>`

- `fn write(self: &mut Self, buf: &[u8]) -> std::io::Result<usize>`

- `fn write_vectored(self: &mut Self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize>`

- `fn flush(self: &mut Self) -> std::io::Result<()>`

- `fn write_all(self: &mut Self, buf: &[u8]) -> std::io::Result<()>`

- `fn write_fmt(self: &mut Self, args: std::fmt::Arguments<'_>) -> std::io::Result<()>`

## Enums

### `StreamInner<S: RawStream>`

```rust
enum StreamInner<S: RawStream> {
    PassThrough(S),
    Strip(crate::StripStream<S>),
}
```

#### Trait Implementations

##### `impl<S: $crate::fmt::Debug + RawStream> Debug for StreamInner<S>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Functions

### `choice`

```rust
fn choice(raw: &dyn RawStream) -> crate::ColorChoice
```

