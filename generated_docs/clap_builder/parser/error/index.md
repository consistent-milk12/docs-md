*[clap_builder](../../index.md) / [parser](../index.md) / [error](index.md)*

---

# Module `error`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MatchesError`](#matcheserror) | enum | Violation of [`ArgMatches`][crate::ArgMatches] assumptions |

## Enums

### `MatchesError`

```rust
enum MatchesError {
    Downcast {
        actual: self::any_value::AnyValueId,
        expected: self::any_value::AnyValueId,
    },
    UnknownArgument {
    },
}
```

*Defined in [`clap_builder-4.5.53/src/parser/error.rs:7-21`](../../../../.source_1765633015/clap_builder-4.5.53/src/parser/error.rs#L7-L21)*

Violation of `ArgMatches` assumptions

#### Variants

- **`Downcast`**

  Failed to downcast `AnyValue` to the specified type

- **`UnknownArgument`**

  Argument not defined in `Command`

#### Implementations

- <span id="matcheserror-unwrap"></span>`fn unwrap<T>(id: &str, r: Result<T, MatchesError>) -> T` — [`MatchesError`](#matcheserror)

#### Trait Implementations

##### `impl Any for MatchesError`

- <span id="matcheserror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MatchesError`

- <span id="matcheserror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MatchesError`

- <span id="matcheserror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MatchesError`

- <span id="matcheserror-clone"></span>`fn clone(&self) -> MatchesError` — [`MatchesError`](#matcheserror)

##### `impl CloneToUninit for MatchesError`

- <span id="matcheserror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for MatchesError`

- <span id="matcheserror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for MatchesError`

- <span id="matcheserror-display-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Error for MatchesError`

##### `impl<T> From for MatchesError`

- <span id="matcheserror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MatchesError`

- <span id="matcheserror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for MatchesError`

- <span id="matcheserror-toowned-type-owned"></span>`type Owned = T`

- <span id="matcheserror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="matcheserror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for MatchesError`

- <span id="matcheserror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for MatchesError`

- <span id="matcheserror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="matcheserror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MatchesError`

- <span id="matcheserror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="matcheserror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

