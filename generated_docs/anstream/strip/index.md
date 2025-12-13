*[anstream](../index.md) / [strip](index.md)*

---

# Module `strip`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`StripStream`](#stripstream) | struct | Only pass printable data to the inner `Write` |
| [`write`](#write) | fn |  |
| [`write_all`](#write-all) | fn |  |
| [`write_fmt`](#write-fmt) | fn |  |
| [`offset_to`](#offset-to) | fn |  |

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

*Defined in [`anstream-0.6.21/src/strip.rs:7-13`](../../../.source_1765521767/anstream-0.6.21/src/strip.rs#L7-L13)*

Only pass printable data to the inner `Write`

#### Implementations

- <span id="stripstream-new"></span>`fn new(raw: S) -> Self`

  Only pass printable data to the inner `Write`

- <span id="stripstream-into-inner"></span>`fn into_inner(self) -> S`

  Get the wrapped [`std::io::Write`](../../fs_err/index.md)

- <span id="stripstream-as-inner"></span>`fn as_inner(&self) -> &S`

  Get the wrapped [`std::io::Write`](../../fs_err/index.md)

#### Trait Implementations

##### `impl Any for StripStream<S>`

- <span id="stripstream-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StripStream<S>`

- <span id="stripstream-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StripStream<S>`

- <span id="stripstream-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<S> Debug for StripStream<S>`

- <span id="stripstream-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for StripStream<S>`

- <span id="stripstream-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StripStream<S>`

- <span id="stripstream-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for StripStream<S>`

- <span id="stripstream-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stripstream-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StripStream<S>`

- <span id="stripstream-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stripstream-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<S> Write for StripStream<S>`

- <span id="stripstream-write"></span>`fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>`

- <span id="stripstream-write-write-vectored"></span>`fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize>`

- <span id="stripstream-write-flush"></span>`fn flush(&mut self) -> std::io::Result<()>`

- <span id="stripstream-write-write-all"></span>`fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()>`

- <span id="stripstream-write-write-fmt"></span>`fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::io::Result<()>`

## Functions

### `write`

```rust
fn write(raw: &mut dyn std::io::Write, state: &mut crate::adapter::StripBytes, buf: &[u8]) -> std::io::Result<usize>
```

*Defined in [`anstream-0.6.21/src/strip.rs:118-138`](../../../.source_1765521767/anstream-0.6.21/src/strip.rs#L118-L138)*

### `write_all`

```rust
fn write_all(raw: &mut dyn std::io::Write, state: &mut crate::adapter::StripBytes, buf: &[u8]) -> std::io::Result<()>
```

*Defined in [`anstream-0.6.21/src/strip.rs:140-149`](../../../.source_1765521767/anstream-0.6.21/src/strip.rs#L140-L149)*

### `write_fmt`

```rust
fn write_fmt(raw: &mut dyn std::io::Write, state: &mut crate::adapter::StripBytes, args: std::fmt::Arguments<'_>) -> std::io::Result<()>
```

*Defined in [`anstream-0.6.21/src/strip.rs:151-158`](../../../.source_1765521767/anstream-0.6.21/src/strip.rs#L151-L158)*

### `offset_to`

```rust
fn offset_to(total: &[u8], subslice: &[u8]) -> usize
```

*Defined in [`anstream-0.6.21/src/strip.rs:161-170`](../../../.source_1765521767/anstream-0.6.21/src/strip.rs#L161-L170)*

