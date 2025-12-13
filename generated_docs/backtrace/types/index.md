*[backtrace](../index.md) / [types](index.md)*

---

# Module `types`

Platform dependent types.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BytesOrWideString`](#bytesorwidestring) | enum | A platform independent representation of a string. |

## Enums

### `BytesOrWideString<'a>`

```rust
enum BytesOrWideString<'a> {
    Bytes(&'a [u8]),
    Wide(&'a [u16]),
}
```

*Defined in [`backtrace-0.3.76/src/types.rs:17-22`](../../../.source_1765521767/backtrace-0.3.76/src/types.rs#L17-L22)*

A platform independent representation of a string. When working with `std`
enabled it is recommended to the convenience methods for providing
conversions to `std` types.

#### Variants

- **`Bytes`**

  A slice, typically provided on Unix platforms.

- **`Wide`**

  Wide strings typically from Windows.

#### Implementations

- <span id="bytesorwidestring-to-str-lossy"></span>`fn to_str_lossy(&self) -> Cow<'a, str>`

  Lossy converts to a `Cow<str>`, will allocate if `Bytes` is not valid

  UTF-8 or if `BytesOrWideString` is `Wide`.

  

  # Required features

  

  This function requires the `std` feature of the `backtrace` crate to be

  enabled, and the `std` feature is enabled by default.

- <span id="bytesorwidestring-into-path-buf"></span>`fn into_path_buf(self) -> PathBuf`

  Provides a `Path` representation of `BytesOrWideString`.

  

  # Required features

  

  This function requires the `std` feature of the `backtrace` crate to be

  enabled, and the `std` feature is enabled by default.

#### Trait Implementations

##### `impl Any for BytesOrWideString<'a>`

- <span id="bytesorwidestring-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BytesOrWideString<'a>`

- <span id="bytesorwidestring-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BytesOrWideString<'a>`

- <span id="bytesorwidestring-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for BytesOrWideString<'a>`

- <span id="bytesorwidestring-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BytesOrWideString<'a>`

- <span id="bytesorwidestring-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for BytesOrWideString<'a>`

- <span id="bytesorwidestring-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BytesOrWideString<'a>`

- <span id="bytesorwidestring-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for BytesOrWideString<'a>`

- <span id="bytesorwidestring-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for BytesOrWideString<'a>`

- <span id="bytesorwidestring-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bytesorwidestring-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BytesOrWideString<'a>`

- <span id="bytesorwidestring-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bytesorwidestring-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

