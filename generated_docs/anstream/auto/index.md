*[anstream](../index.md) / [auto](index.md)*

---

# Module `auto`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AutoStream`](#autostream) | struct | [`std::io::Write`] that adapts ANSI escape codes to the underlying `Write`s capabilities |
| [`StreamInner`](#streaminner) | enum |  |
| [`choice`](#choice) | fn |  |

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

- <span id="autostream-lock"></span>`fn lock(self) -> AutoStream<std::io::StderrLock<'static>>` — [`AutoStream`](../index.md)

#### Trait Implementations

##### `impl<S: fmt::Debug + RawStream> Debug for AutoStream<S>`

- <span id="autostream-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S> Write for AutoStream<S>`

- <span id="autostream-write"></span>`fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>`

- <span id="autostream-write-vectored"></span>`fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize>`

- <span id="autostream-flush"></span>`fn flush(&mut self) -> std::io::Result<()>`

- <span id="autostream-write-all"></span>`fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()>`

- <span id="autostream-write-fmt"></span>`fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::io::Result<()>`

## Enums

### `StreamInner<S: RawStream>`

```rust
enum StreamInner<S: RawStream> {
    PassThrough(S),
    Strip(crate::StripStream<S>),
}
```

#### Trait Implementations

##### `impl<S: fmt::Debug + RawStream> Debug for StreamInner<S>`

- <span id="streaminner-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `choice`

```rust
fn choice(raw: &dyn RawStream) -> crate::ColorChoice
```

