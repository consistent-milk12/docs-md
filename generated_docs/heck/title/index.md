*[heck](../index.md) / [title](index.md)*

---

# Module `title`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AsTitleCase`](#astitlecase) | struct | This wrapper performs a title case conversion in [`fmt::Display`]. |
| [`ToTitleCase`](#totitlecase) | trait | This trait defines a title case conversion. |

## Structs

### `AsTitleCase<T: AsRef<str>>`

```rust
struct AsTitleCase<T: AsRef<str>>(T);
```

*Defined in [`heck-0.5.0/src/title.rs:44`](../../../.source_1765633015/heck-0.5.0/src/title.rs#L44)*

This wrapper performs a title case conversion in [`fmt::Display`](../../miette_derive/index.md).

## Example:

```rust
use heck::AsTitleCase;

let sentence = "We have always lived in slums and holes in the wall.";
assert_eq!(format!("{}", AsTitleCase(sentence)), "We Have Always Lived In Slums And Holes In The Wall");
```

#### Trait Implementations

##### `impl<T> Any for AsTitleCase<T>`

- <span id="astitlecase-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AsTitleCase<T>`

- <span id="astitlecase-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AsTitleCase<T>`

- <span id="astitlecase-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: AsRef<str>> Display for AsTitleCase<T>`

- <span id="astitlecase-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AsTitleCase<T>`

- <span id="astitlecase-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for AsTitleCase<T>`

- <span id="astitlecase-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> ToString for AsTitleCase<T>`

- <span id="astitlecase-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T, U> TryFrom for AsTitleCase<T>`

- <span id="astitlecase-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="astitlecase-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for AsTitleCase<T>`

- <span id="astitlecase-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="astitlecase-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `ToTitleCase`

```rust
trait ToTitleCase: ToOwned { ... }
```

*Defined in [`heck-0.5.0/src/title.rs:23-26`](../../../.source_1765633015/heck-0.5.0/src/title.rs#L23-L26)*

This trait defines a title case conversion.

In Title Case, word boundaries are indicated by spaces, and every word is
capitalized.

## Example:

```rust
use heck::ToTitleCase;

let sentence = "We have always lived in slums and holes in the wall.";
assert_eq!(sentence.to_title_case(), "We Have Always Lived In Slums And Holes In The Wall");
```

#### Required Methods

- `fn to_title_case(&self) -> <Self as >::Owned`

  Convert this type to title case.

#### Implementors

- `str`

