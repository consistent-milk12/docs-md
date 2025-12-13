*[semver](../index.md) / [error](index.md)*

---

# Module `error`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`QuotedChar`](#quotedchar) | struct |  |
| [`ErrorKind`](#errorkind) | enum |  |
| [`Position`](#position) | enum |  |

## Structs

### `QuotedChar`

```rust
struct QuotedChar(char);
```

*Defined in [`semver-1.0.27/src/error.rs:113`](../../../.source_1765633015/semver-1.0.27/src/error.rs#L113)*

#### Trait Implementations

##### `impl Any for QuotedChar`

- <span id="quotedchar-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for QuotedChar`

- <span id="quotedchar-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for QuotedChar`

- <span id="quotedchar-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Display for QuotedChar`

- <span id="quotedchar-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for QuotedChar`

- <span id="quotedchar-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for QuotedChar`

- <span id="quotedchar-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for QuotedChar`

- <span id="quotedchar-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for QuotedChar`

- <span id="quotedchar-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="quotedchar-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for QuotedChar`

- <span id="quotedchar-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="quotedchar-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `ErrorKind`

```rust
enum ErrorKind {
    Empty,
    UnexpectedEnd(Position),
    UnexpectedChar(Position, char),
    UnexpectedCharAfter(Position, char),
    ExpectedCommaFound(Position, char),
    LeadingZero(Position),
    Overflow(Position),
    EmptySegment(Position),
    IllegalCharacter(Position),
    WildcardNotTheOnlyComparator(char),
    UnexpectedAfterWildcard,
    ExcessiveComparators,
}
```

*Defined in [`semver-1.0.27/src/error.rs:4-17`](../../../.source_1765633015/semver-1.0.27/src/error.rs#L4-L17)*

#### Trait Implementations

##### `impl Any for ErrorKind`

- <span id="errorkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ErrorKind`

- <span id="errorkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ErrorKind`

- <span id="errorkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ErrorKind`

- <span id="errorkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ErrorKind`

- <span id="errorkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ErrorKind`

- <span id="errorkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="errorkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ErrorKind`

- <span id="errorkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="errorkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Position`

```rust
enum Position {
    Major,
    Minor,
    Patch,
    Pre,
    Build,
}
```

*Defined in [`semver-1.0.27/src/error.rs:20-26`](../../../.source_1765633015/semver-1.0.27/src/error.rs#L20-L26)*

#### Trait Implementations

##### `impl Any for Position`

- <span id="position-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Position`

- <span id="position-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Position`

- <span id="position-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Position`

- <span id="position-clone"></span>`fn clone(&self) -> Position` — [`Position`](#position)

##### `impl CloneToUninit for Position`

- <span id="position-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Position`

##### `impl Display for Position`

- <span id="position-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Position`

##### `impl<T> From for Position`

- <span id="position-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Position`

- <span id="position-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Position`

- <span id="position-partialeq-eq"></span>`fn eq(&self, other: &Position) -> bool` — [`Position`](#position)

##### `impl StructuralPartialEq for Position`

##### `impl ToOwned for Position`

- <span id="position-toowned-type-owned"></span>`type Owned = T`

- <span id="position-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="position-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Position`

- <span id="position-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Position`

- <span id="position-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="position-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Position`

- <span id="position-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="position-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

