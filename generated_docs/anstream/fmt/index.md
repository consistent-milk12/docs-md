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

*Defined in [`anstream-0.6.21/src/fmt.rs:4-10`](../../../.source_1765633015/anstream-0.6.21/src/fmt.rs#L4-L10)*

A shim which allows a [`std::io::Write`](../../fs_err/index.md) to be implemented in terms of a [`std::fmt::Write`](../../fs_err/index.md)

This saves off I/O errors. instead of discarding them

#### Implementations

- <span id="adapter-new"></span>`fn new(writer: W) -> Self`

- <span id="adapter-write-fmt"></span>`fn write_fmt(self, fmt: std::fmt::Arguments<'_>) -> std::io::Result<()>`

#### Trait Implementations

##### `impl Any for Adapter<W>`

- <span id="adapter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Adapter<W>`

- <span id="adapter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Adapter<W>`

- <span id="adapter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Adapter<W>`

- <span id="adapter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Adapter<W>`

- <span id="adapter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Adapter<W>`

- <span id="adapter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="adapter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Adapter<W>`

- <span id="adapter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="adapter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<W> Write for Adapter<W>`

- <span id="adapter-write-write-str"></span>`fn write_str(&mut self, s: &str) -> std::fmt::Result`

