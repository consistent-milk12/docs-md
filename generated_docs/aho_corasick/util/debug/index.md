*[aho_corasick](../../index.md) / [util](../index.md) / [debug](index.md)*

---

# Module `debug`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugByte`](#debugbyte) | struct | A type that wraps a single byte with a convenient fmt::Debug impl that escapes the byte. |

## Structs

### `DebugByte`

```rust
struct DebugByte(u8);
```

*Defined in [`aho-corasick-1.1.4/src/util/debug.rs:3`](../../../../.source_1765633015/aho-corasick-1.1.4/src/util/debug.rs#L3)*

A type that wraps a single byte with a convenient fmt::Debug impl that
escapes the byte.

#### Trait Implementations

##### `impl Any for DebugByte`

- <span id="debugbyte-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugByte`

- <span id="debugbyte-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugByte`

- <span id="debugbyte-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for DebugByte`

- <span id="debugbyte-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for DebugByte`

- <span id="debugbyte-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugByte`

- <span id="debugbyte-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DebugByte`

- <span id="debugbyte-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugbyte-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugByte`

- <span id="debugbyte-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugbyte-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

