*[heck](../index.md) / [kebab](index.md)*

---

# Module `kebab`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AsKebabCase`](#askebabcase) | struct | This wrapper performs a kebab case conversion in [`fmt::Display`]. |
| [`ToKebabCase`](#tokebabcase) | trait | This trait defines a kebab case conversion. |

## Structs

### `AsKebabCase<T: AsRef<str>>`

```rust
struct AsKebabCase<T: AsRef<str>>(T);
```

*Defined in [`heck-0.5.0/src/kebab.rs:40`](../../../.source_1765521767/heck-0.5.0/src/kebab.rs#L40)*

This wrapper performs a kebab case conversion in [`fmt::Display`](../../miette_derive/index.md).

## Example:

```rust
use heck::AsKebabCase;

let sentence = "We are going to inherit the earth.";
assert_eq!(format!("{}", AsKebabCase(sentence)), "we-are-going-to-inherit-the-earth");
```

#### Trait Implementations

##### `impl<T> Any for AsKebabCase<T>`

- <span id="askebabcase-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AsKebabCase<T>`

- <span id="askebabcase-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AsKebabCase<T>`

- <span id="askebabcase-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: AsRef<str>> Display for AsKebabCase<T>`

- <span id="askebabcase-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AsKebabCase<T>`

- <span id="askebabcase-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for AsKebabCase<T>`

- <span id="askebabcase-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> ToString for AsKebabCase<T>`

- <span id="askebabcase-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T, U> TryFrom for AsKebabCase<T>`

- <span id="askebabcase-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="askebabcase-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for AsKebabCase<T>`

- <span id="askebabcase-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="askebabcase-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `ToKebabCase`

```rust
trait ToKebabCase: ToOwned { ... }
```

*Defined in [`heck-0.5.0/src/kebab.rs:19-22`](../../../.source_1765521767/heck-0.5.0/src/kebab.rs#L19-L22)*

This trait defines a kebab case conversion.

In kebab-case, word boundaries are indicated by hyphens.

## Example:

```rust
use heck::ToKebabCase;

let sentence = "We are going to inherit the earth.";
assert_eq!(sentence.to_kebab_case(), "we-are-going-to-inherit-the-earth");
```

#### Required Methods

- `fn to_kebab_case(&self) -> <Self as >::Owned`

  Convert this type to kebab case.

#### Implementors

- `str`

