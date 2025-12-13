*[regex_automata](../../index.md) / [util](../index.md) / [escape](index.md)*

---

# Module `escape`

Provides convenience routines for escaping raw bytes.

Since this crate tends to deal with `&[u8]` everywhere and the default
`Debug` implementation just shows decimal integers, it makes debugging those
representations quite difficult. This module provides types that show `&[u8]`
as if it were a string, with invalid UTF-8 escaped into its byte-by-byte hex
representation.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugByte`](#debugbyte) | struct | Provides a convenient `Debug` implementation for a `u8`. |
| [`DebugHaystack`](#debughaystack) | struct | Provides a convenient `Debug` implementation for `&[u8]`. |

## Structs

### `DebugByte`

```rust
struct DebugByte(u8);
```

*Defined in [`regex-automata-0.4.13/src/util/escape.rs:19`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/escape.rs#L19)*

Provides a convenient `Debug` implementation for a `u8`.

The `Debug` impl treats the byte as an ASCII, and emits a human readable
representation of it. If the byte isn't ASCII, then it's emitted as a hex
escape sequence.

#### Trait Implementations

##### `impl Any for DebugByte`

- <span id="debugbyte-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugByte`

- <span id="debugbyte-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugByte`

- <span id="debugbyte-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DebugByte`

- <span id="debugbyte-clone"></span>`fn clone(&self) -> DebugByte` — [`DebugByte`](#debugbyte)

##### `impl CloneToUninit for DebugByte`

- <span id="debugbyte-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for DebugByte`

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

##### `impl ToOwned for DebugByte`

- <span id="debugbyte-toowned-type-owned"></span>`type Owned = T`

- <span id="debugbyte-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugbyte-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugByte`

- <span id="debugbyte-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugbyte-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugByte`

- <span id="debugbyte-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugbyte-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugHaystack<'a>`

```rust
struct DebugHaystack<'a>(&'a [u8]);
```

*Defined in [`regex-automata-0.4.13/src/util/escape.rs:49`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/escape.rs#L49)*

Provides a convenient `Debug` implementation for `&[u8]`.

This generally works best when the bytes are presumed to be mostly UTF-8,
but will work for anything. For any bytes that aren't UTF-8, they are
emitted as hex escape sequences.

#### Trait Implementations

##### `impl Any for DebugHaystack<'a>`

- <span id="debughaystack-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugHaystack<'a>`

- <span id="debughaystack-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugHaystack<'a>`

- <span id="debughaystack-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for DebugHaystack<'a>`

- <span id="debughaystack-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for DebugHaystack<'a>`

- <span id="debughaystack-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugHaystack<'a>`

- <span id="debughaystack-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DebugHaystack<'a>`

- <span id="debughaystack-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debughaystack-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugHaystack<'a>`

- <span id="debughaystack-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debughaystack-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

