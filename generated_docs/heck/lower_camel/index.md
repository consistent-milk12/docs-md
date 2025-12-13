*[heck](../index.md) / [lower_camel](index.md)*

---

# Module `lower_camel`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AsLowerCamelCase`](#aslowercamelcase) | struct | This wrapper performs a lower camel case conversion in [`fmt::Display`]. |
| [`ToLowerCamelCase`](#tolowercamelcase) | trait | This trait defines a lower camel case conversion. |

## Structs

### `AsLowerCamelCase<T: AsRef<str>>`

```rust
struct AsLowerCamelCase<T: AsRef<str>>(T);
```

*Defined in [`heck-0.5.0/src/lower_camel.rs:44`](../../../.source_1765521767/heck-0.5.0/src/lower_camel.rs#L44)*

This wrapper performs a lower camel case conversion in [`fmt::Display`](../../miette_derive/index.md).

## Example:

```rust
use heck::AsLowerCamelCase;

let sentence = "It is we who built these palaces and cities.";
assert_eq!(format!("{}", AsLowerCamelCase(sentence)), "itIsWeWhoBuiltThesePalacesAndCities");
```

#### Trait Implementations

##### `impl<T> Any for AsLowerCamelCase<T>`

- <span id="aslowercamelcase-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AsLowerCamelCase<T>`

- <span id="aslowercamelcase-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AsLowerCamelCase<T>`

- <span id="aslowercamelcase-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: AsRef<str>> Display for AsLowerCamelCase<T>`

- <span id="aslowercamelcase-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AsLowerCamelCase<T>`

- <span id="aslowercamelcase-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for AsLowerCamelCase<T>`

- <span id="aslowercamelcase-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> ToString for AsLowerCamelCase<T>`

- <span id="aslowercamelcase-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T, U> TryFrom for AsLowerCamelCase<T>`

- <span id="aslowercamelcase-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="aslowercamelcase-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for AsLowerCamelCase<T>`

- <span id="aslowercamelcase-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="aslowercamelcase-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `ToLowerCamelCase`

```rust
trait ToLowerCamelCase: ToOwned { ... }
```

*Defined in [`heck-0.5.0/src/lower_camel.rs:23-26`](../../../.source_1765521767/heck-0.5.0/src/lower_camel.rs#L23-L26)*

This trait defines a lower camel case conversion.

In lowerCamelCase, word boundaries are indicated by capital letters,
excepting the first word.

## Example:

```rust
use heck::ToLowerCamelCase;

let sentence = "It is we who built these palaces and cities.";
assert_eq!(sentence.to_lower_camel_case(), "itIsWeWhoBuiltThesePalacesAndCities");
```

#### Required Methods

- `fn to_lower_camel_case(&self) -> <Self as >::Owned`

  Convert this type to lower camel case.

#### Implementors

- `str`

