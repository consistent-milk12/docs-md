*[serde_core](../index.md) / [format](index.md)*

---

# Module `format`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Buf`](#buf) | struct |  |

## Structs

### `Buf<'a>`

```rust
struct Buf<'a> {
    bytes: &'a mut [u8],
    offset: usize,
}
```

*Defined in [`serde_core-1.0.228/src/format.rs:4-7`](../../../.source_1765633015/serde_core-1.0.228/src/format.rs#L4-L7)*

#### Implementations

- <span id="buf-new"></span>`fn new(bytes: &'a mut [u8]) -> Self`

- <span id="buf-as-str"></span>`fn as_str(&self) -> &str`

#### Trait Implementations

##### `impl Any for Buf<'a>`

- <span id="buf-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Buf<'a>`

- <span id="buf-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Buf<'a>`

- <span id="buf-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Buf<'a>`

- <span id="buf-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Buf<'a>`

- <span id="buf-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Buf<'a>`

- <span id="buf-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="buf-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Buf<'a>`

- <span id="buf-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="buf-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write for Buf<'a>`

- <span id="buf-write-write-str"></span>`fn write_str(&mut self, s: &str) -> fmt::Result`

