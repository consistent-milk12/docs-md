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

*Defined in [`anstream-0.6.21/src/auto.rs:19-21`](../../../.source_1765633015/anstream-0.6.21/src/auto.rs#L19-L21)*

[`std::io::Write`](../../fs_err/index.md) that adapts ANSI escape codes to the underlying `Write`s capabilities

This includes
- Stripping colors for non-terminals
- Respecting env variables like [NO_COLOR](https://no-color.org/) or [CLICOLOR](https://bixense.com/clicolors/)
- *(windows)* Falling back to the wincon API where [ENABLE_VIRTUAL_TERMINAL_PROCESSING](https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#output-sequences) is unsupported

You can customize auto-detection by calling into
[anstyle_query](https://docs.rs/anstyle-query/latest/anstyle_query/)
to get a [`ColorChoice`](../index.md) and then calling `AutoStream::new(stream, choice)`.

#### Implementations

- <span id="autostream-new"></span>`fn new(raw: S, choice: ColorChoice) -> Self` — [`ColorChoice`](../index.md#colorchoice)

  Runtime control over styling behavior

  

  # Example

  

  ```rust

  #[cfg(feature = "auto")] {

  use std::io::IsTerminal as _;

  // Like `AutoStream::choice` but without `NO_COLOR`, `CLICOLOR_FORCE`, `CI`

  fn choice(raw: &dyn anstream::stream::RawStream) -> anstream::ColorChoice {

      let choice = anstream::ColorChoice::global();

      if choice == anstream::ColorChoice::Auto {

          if raw.is_terminal() && anstyle_query::term_supports_color() {

              anstream::ColorChoice::Always

          } else {

              anstream::ColorChoice::Never

          }

      } else {

          choice

      }

  }

  

  let stream = std::io::stdout();

  let choice = choice(&stream);

  let auto = anstream::AutoStream::new(stream, choice);

  }

  ```

- <span id="autostream-auto"></span>`fn auto(raw: S) -> Self`

  Auto-adapt for the stream's capabilities

- <span id="autostream-choice"></span>`fn choice(raw: &S) -> ColorChoice` — [`ColorChoice`](../index.md#colorchoice)

  Report the desired choice for the given stream

- <span id="autostream-always-ansi"></span>`fn always_ansi(raw: S) -> Self`

  Force ANSI escape codes to be passed through as-is, no matter what the inner `Write`

  supports.

- <span id="autostream-always-ansi"></span>`fn always_ansi_(raw: S) -> Self`

- <span id="autostream-always"></span>`fn always(raw: S) -> Self`

  Force color, no matter what the inner `Write` supports.

- <span id="autostream-never"></span>`fn never(raw: S) -> Self`

  Only pass printable data to the inner `Write`.

- <span id="autostream-wincon"></span>`fn wincon(raw: S) -> Result<Self, S>`

- <span id="autostream-into-inner"></span>`fn into_inner(self) -> S`

  Get the wrapped [`RawStream`](../stream/index.md)

- <span id="autostream-as-inner"></span>`fn as_inner(&self) -> &S`

  Get the wrapped [`RawStream`](../stream/index.md)

- <span id="autostream-is-terminal"></span>`fn is_terminal(&self) -> bool`

  Returns `true` if the descriptor/handle refers to a terminal/tty.

- <span id="autostream-current-choice"></span>`fn current_choice(&self) -> ColorChoice` — [`ColorChoice`](../index.md#colorchoice)

  Prefer `AutoStream::choice`

  

  This doesn't report what is requested but what is currently active.

#### Trait Implementations

##### `impl Any for AutoStream<S>`

- <span id="autostream-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AutoStream<S>`

- <span id="autostream-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AutoStream<S>`

- <span id="autostream-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<S: fmt::Debug + RawStream> Debug for AutoStream<S>`

- <span id="autostream-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AutoStream<S>`

- <span id="autostream-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AutoStream<S>`

- <span id="autostream-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for AutoStream<S>`

- <span id="autostream-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="autostream-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AutoStream<S>`

- <span id="autostream-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="autostream-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<S> Write for AutoStream<S>`

- <span id="autostream-write"></span>`fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>`

- <span id="autostream-write-write-vectored"></span>`fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize>`

- <span id="autostream-write-flush"></span>`fn flush(&mut self) -> std::io::Result<()>`

- <span id="autostream-write-write-all"></span>`fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()>`

- <span id="autostream-write-write-fmt"></span>`fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::io::Result<()>`

## Enums

### `StreamInner<S: RawStream>`

```rust
enum StreamInner<S: RawStream> {
    PassThrough(S),
    Strip(crate::StripStream<S>),
}
```

*Defined in [`anstream-0.6.21/src/auto.rs:24-29`](../../../.source_1765633015/anstream-0.6.21/src/auto.rs#L24-L29)*

#### Trait Implementations

##### `impl Any for StreamInner<S>`

- <span id="streaminner-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StreamInner<S>`

- <span id="streaminner-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StreamInner<S>`

- <span id="streaminner-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<S: fmt::Debug + RawStream> Debug for StreamInner<S>`

- <span id="streaminner-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for StreamInner<S>`

- <span id="streaminner-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StreamInner<S>`

- <span id="streaminner-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for StreamInner<S>`

- <span id="streaminner-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="streaminner-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StreamInner<S>`

- <span id="streaminner-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="streaminner-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `choice`

```rust
fn choice(raw: &dyn RawStream) -> crate::ColorChoice
```

*Defined in [`anstream-0.6.21/src/auto.rs:198-223`](../../../.source_1765633015/anstream-0.6.21/src/auto.rs#L198-L223)*

