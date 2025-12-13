*[regex_syntax](../index.md) / [debug](index.md)*

---

# Module `debug`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Byte`](#byte) | struct | A type that wraps a single byte with a convenient fmt::Debug impl that escapes the byte. |
| [`Bytes`](#bytes) | struct | A type that provides a human readable debug impl for arbitrary bytes. |
| [`utf8_decode`](#utf8-decode) | fn | Decodes the next UTF-8 encoded codepoint from the given byte slice. |

## Structs

### `Byte`

```rust
struct Byte(u8);
```

*Defined in [`regex-syntax-0.8.8/src/debug.rs:3`](../../../.source_1765521767/regex-syntax-0.8.8/src/debug.rs#L3)*

A type that wraps a single byte with a convenient fmt::Debug impl that
escapes the byte.

#### Trait Implementations

##### `impl Any for Byte`

- <span id="byte-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Byte`

- <span id="byte-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Byte`

- <span id="byte-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Byte`

- <span id="byte-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for Byte`

- <span id="byte-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Byte`

- <span id="byte-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Byte`

- <span id="byte-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="byte-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Byte`

- <span id="byte-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="byte-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Bytes<'a>`

```rust
struct Bytes<'a>(&'a [u8]);
```

*Defined in [`regex-syntax-0.8.8/src/debug.rs:34`](../../../.source_1765521767/regex-syntax-0.8.8/src/debug.rs#L34)*

A type that provides a human readable debug impl for arbitrary bytes.

This generally works best when the bytes are presumed to be mostly UTF-8,
but will work for anything.

N.B. This is copied nearly verbatim from regex-automata. Sigh.

#### Trait Implementations

##### `impl Any for Bytes<'a>`

- <span id="bytes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Bytes<'a>`

- <span id="bytes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Bytes<'a>`

- <span id="bytes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Bytes<'a>`

- <span id="bytes-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for Bytes<'a>`

- <span id="bytes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Bytes<'a>`

- <span id="bytes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Bytes<'a>`

- <span id="bytes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bytes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Bytes<'a>`

- <span id="bytes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bytes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `utf8_decode`

```rust
fn utf8_decode(bytes: &[u8]) -> Option<Result<char, u8>>
```

*Defined in [`regex-syntax-0.8.8/src/debug.rs:77-107`](../../../.source_1765521767/regex-syntax-0.8.8/src/debug.rs#L77-L107)*

Decodes the next UTF-8 encoded codepoint from the given byte slice.

If no valid encoding of a codepoint exists at the beginning of the given
byte slice, then the first byte is returned instead.

This returns `None` if and only if `bytes` is empty.

