*[miette](../../index.md) / [eyreish](../index.md) / [context](index.md)*

---

# Module `context`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ext`](#ext) | mod |  |
| [`private`](#private) | mod |  |
| [`Quoted`](#quoted) | struct |  |

## Modules

- [`ext`](ext/index.md)
- [`private`](private/index.md)

## Structs

### `Quoted<D>`

```rust
struct Quoted<D>(D);
```

*Defined in [`miette-7.6.0/src/eyreish/context.rs:228`](../../../../.source_1765633015/miette-7.6.0/src/eyreish/context.rs#L228)*

#### Trait Implementations

##### `impl Any for Quoted<D>`

- <span id="quoted-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Quoted<D>`

- <span id="quoted-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Quoted<D>`

- <span id="quoted-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<D> Debug for Quoted<D>`

- <span id="quoted-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Quoted<D>`

- <span id="quoted-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Quoted<D>`

- <span id="quoted-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<D> OwoColorize for Quoted<D>`

##### `impl<U> TryFrom for Quoted<D>`

- <span id="quoted-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="quoted-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Quoted<D>`

- <span id="quoted-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="quoted-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write for Quoted<&mut fmt::Formatter<'_>>`

- <span id="quoted-write-write-str"></span>`fn write_str(&mut self, s: &str) -> fmt::Result`

