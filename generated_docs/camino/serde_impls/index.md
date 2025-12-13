*[camino](../index.md) / [serde_impls](index.md)*

---

# Module `serde_impls`

Serde implementations for the types in this crate.

* `Utf8Path` is an unsized type which the derive impls can't handle.
* `Utf8PathBuf` could be derived, but we don't depend on serde_derive to
  improve compile times. It's also very straightforward to implement.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Utf8PathBufVisitor`](#utf8pathbufvisitor) | struct |  |
| [`Utf8PathVisitor`](#utf8pathvisitor) | struct |  |

## Structs

### `Utf8PathBufVisitor`

```rust
struct Utf8PathBufVisitor;
```

*Defined in [`camino-1.2.1/src/serde_impls.rs:14`](../../../.source_1765633015/camino-1.2.1/src/serde_impls.rs#L14)*

#### Trait Implementations

##### `impl Any for Utf8PathBufVisitor`

- <span id="utf8pathbufvisitor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Utf8PathBufVisitor`

- <span id="utf8pathbufvisitor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Utf8PathBufVisitor`

- <span id="utf8pathbufvisitor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Expected for Utf8PathBufVisitor`

- <span id="utf8pathbufvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`

##### `impl<T> From for Utf8PathBufVisitor`

- <span id="utf8pathbufvisitor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Utf8PathBufVisitor`

- <span id="utf8pathbufvisitor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Utf8PathBufVisitor`

- <span id="utf8pathbufvisitor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="utf8pathbufvisitor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Utf8PathBufVisitor`

- <span id="utf8pathbufvisitor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="utf8pathbufvisitor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Visitor for Utf8PathBufVisitor`

- <span id="utf8pathbufvisitor-visitor-type-value"></span>`type Value = Utf8PathBuf`

- <span id="utf8pathbufvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="utf8pathbufvisitor-visitor-visit-str"></span>`fn visit_str<E>(self, v: &str) -> Result<<Self as >::Value, E>`

- <span id="utf8pathbufvisitor-visitor-visit-string"></span>`fn visit_string<E>(self, v: String) -> Result<<Self as >::Value, E>`

- <span id="utf8pathbufvisitor-visitor-visit-bytes"></span>`fn visit_bytes<E>(self, v: &[u8]) -> Result<<Self as >::Value, E>`

- <span id="utf8pathbufvisitor-visitor-visit-byte-buf"></span>`fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<<Self as >::Value, E>`

### `Utf8PathVisitor`

```rust
struct Utf8PathVisitor;
```

*Defined in [`camino-1.2.1/src/serde_impls.rs:74`](../../../.source_1765633015/camino-1.2.1/src/serde_impls.rs#L74)*

#### Trait Implementations

##### `impl Any for Utf8PathVisitor`

- <span id="utf8pathvisitor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Utf8PathVisitor`

- <span id="utf8pathvisitor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Utf8PathVisitor`

- <span id="utf8pathvisitor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Expected for Utf8PathVisitor`

- <span id="utf8pathvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`

##### `impl<T> From for Utf8PathVisitor`

- <span id="utf8pathvisitor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Utf8PathVisitor`

- <span id="utf8pathvisitor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Utf8PathVisitor`

- <span id="utf8pathvisitor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="utf8pathvisitor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Utf8PathVisitor`

- <span id="utf8pathvisitor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="utf8pathvisitor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Visitor for Utf8PathVisitor`

- <span id="utf8pathvisitor-visitor-type-value"></span>`type Value = &'a Utf8Path`

- <span id="utf8pathvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="utf8pathvisitor-visitor-visit-borrowed-str"></span>`fn visit_borrowed_str<E>(self, v: &'a str) -> Result<<Self as >::Value, E>`

- <span id="utf8pathvisitor-visitor-visit-borrowed-bytes"></span>`fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<<Self as >::Value, E>`

