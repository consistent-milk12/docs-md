*[heck](../index.md) / [shouty_kebab](index.md)*

---

# Module `shouty_kebab`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AsShoutyKebabCase`](#asshoutykebabcase) | struct | This wrapper performs a kebab case conversion in [`fmt::Display`]. |
| [`ToShoutyKebabCase`](#toshoutykebabcase) | trait | This trait defines a shouty kebab case conversion. |

## Structs

### `AsShoutyKebabCase<T: AsRef<str>>`

```rust
struct AsShoutyKebabCase<T: AsRef<str>>(T);
```

*Defined in [`heck-0.5.0/src/shouty_kebab.rs:41`](../../../.source_1765521767/heck-0.5.0/src/shouty_kebab.rs#L41)*

This wrapper performs a kebab case conversion in [`fmt::Display`](../../miette_derive/index.md).

## Example:

```rust
use heck::AsShoutyKebabCase;

let sentence = "We are going to inherit the earth.";
assert_eq!(format!("{}", AsShoutyKebabCase(sentence)), "WE-ARE-GOING-TO-INHERIT-THE-EARTH");
```

#### Trait Implementations

##### `impl<T> Any for AsShoutyKebabCase<T>`

- <span id="asshoutykebabcase-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AsShoutyKebabCase<T>`

- <span id="asshoutykebabcase-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AsShoutyKebabCase<T>`

- <span id="asshoutykebabcase-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: AsRef<str>> Display for AsShoutyKebabCase<T>`

- <span id="asshoutykebabcase-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AsShoutyKebabCase<T>`

- <span id="asshoutykebabcase-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for AsShoutyKebabCase<T>`

- <span id="asshoutykebabcase-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> ToString for AsShoutyKebabCase<T>`

- <span id="asshoutykebabcase-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T, U> TryFrom for AsShoutyKebabCase<T>`

- <span id="asshoutykebabcase-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="asshoutykebabcase-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for AsShoutyKebabCase<T>`

- <span id="asshoutykebabcase-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="asshoutykebabcase-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `ToShoutyKebabCase`

```rust
trait ToShoutyKebabCase: ToOwned { ... }
```

*Defined in [`heck-0.5.0/src/shouty_kebab.rs:20-23`](../../../.source_1765521767/heck-0.5.0/src/shouty_kebab.rs#L20-L23)*

This trait defines a shouty kebab case conversion.

In SHOUTY-KEBAB-CASE, word boundaries are indicated by hyphens and all
words are in uppercase.

## Example:

```rust
use heck::ToShoutyKebabCase;

let sentence = "We are going to inherit the earth.";
assert_eq!(sentence.to_shouty_kebab_case(), "WE-ARE-GOING-TO-INHERIT-THE-EARTH");
```

#### Required Methods

- `fn to_shouty_kebab_case(&self) -> <Self as >::Owned`

  Convert this type to shouty kebab case.

#### Implementors

- `str`

