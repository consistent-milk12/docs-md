*[unit_prefix](../index.md) / [parse](index.md)*

---

# Module `parse`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NumberPrefixParseError`](#numberprefixparseerror) | struct | The error returned when a `NumberPrefix` is failed to be parsed. |

## Structs

### `NumberPrefixParseError`

```rust
struct NumberPrefixParseError(());
```

*Defined in [`unit-prefix-0.5.2/src/parse.rs:50`](../../../.source_1765521767/unit-prefix-0.5.2/src/parse.rs#L50)*

The error returned when a `NumberPrefix` is failed to be parsed.

#### Trait Implementations

##### `impl Any for NumberPrefixParseError`

- <span id="numberprefixparseerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NumberPrefixParseError`

- <span id="numberprefixparseerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NumberPrefixParseError`

- <span id="numberprefixparseerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for NumberPrefixParseError`

- <span id="numberprefixparseerror-clone"></span>`fn clone(&self) -> NumberPrefixParseError` — [`NumberPrefixParseError`](#numberprefixparseerror)

##### `impl CloneToUninit for NumberPrefixParseError`

- <span id="numberprefixparseerror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for NumberPrefixParseError`

##### `impl Debug for NumberPrefixParseError`

- <span id="numberprefixparseerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for NumberPrefixParseError`

- <span id="numberprefixparseerror-display-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for NumberPrefixParseError`

##### `impl Error for NumberPrefixParseError`

##### `impl<T> From for NumberPrefixParseError`

- <span id="numberprefixparseerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for NumberPrefixParseError`

- <span id="numberprefixparseerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for NumberPrefixParseError`

- <span id="numberprefixparseerror-partialeq-eq"></span>`fn eq(&self, other: &NumberPrefixParseError) -> bool` — [`NumberPrefixParseError`](#numberprefixparseerror)

##### `impl StructuralPartialEq for NumberPrefixParseError`

##### `impl ToOwned for NumberPrefixParseError`

- <span id="numberprefixparseerror-toowned-type-owned"></span>`type Owned = T`

- <span id="numberprefixparseerror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="numberprefixparseerror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for NumberPrefixParseError`

- <span id="numberprefixparseerror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for NumberPrefixParseError`

- <span id="numberprefixparseerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="numberprefixparseerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NumberPrefixParseError`

- <span id="numberprefixparseerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="numberprefixparseerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

