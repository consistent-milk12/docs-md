*[heck](../index.md) / [upper_camel](index.md)*

---

# Module `upper_camel`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AsUpperCamelCase`](#asuppercamelcase) | struct | This wrapper performs a upper camel case conversion in [`fmt::Display`]. |
| [`ToUpperCamelCase`](#touppercamelcase) | trait | This trait defines an upper camel case conversion. |
| [`ToPascalCase`](#topascalcase) | trait | `ToPascalCase` is an alias for [`ToUpperCamelCase`]. |

## Structs

### `AsUpperCamelCase<T: AsRef<str>>`

```rust
struct AsUpperCamelCase<T: AsRef<str>>(T);
```

*Defined in [`heck-0.5.0/src/upper_camel.rs:57`](../../../.source_1765633015/heck-0.5.0/src/upper_camel.rs#L57)*

This wrapper performs a upper camel case conversion in [`fmt::Display`](../../miette_derive/index.md).

## Example:

```rust
use heck::AsUpperCamelCase;

let sentence = "We are not in the least afraid of ruins.";
assert_eq!(format!("{}", AsUpperCamelCase(sentence)), "WeAreNotInTheLeastAfraidOfRuins");
```

#### Trait Implementations

##### `impl<T> Any for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: AsRef<str>> Display for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> ToString for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T, U> TryFrom for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="asuppercamelcase-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="asuppercamelcase-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `ToUpperCamelCase`

```rust
trait ToUpperCamelCase: ToOwned { ... }
```

*Defined in [`heck-0.5.0/src/upper_camel.rs:23-26`](../../../.source_1765633015/heck-0.5.0/src/upper_camel.rs#L23-L26)*

This trait defines an upper camel case conversion.

In UpperCamelCase, word boundaries are indicated by capital letters,
including the first word.

## Example:

```rust
use heck::ToUpperCamelCase;

let sentence = "We are not in the least afraid of ruins.";
assert_eq!(sentence.to_upper_camel_case(), "WeAreNotInTheLeastAfraidOfRuins");
```

#### Required Methods

- `fn to_upper_camel_case(&self) -> <Self as >::Owned`

  Convert this type to upper camel case.

#### Implementors

- `str`

### `ToPascalCase`

```rust
trait ToPascalCase: ToOwned { ... }
```

*Defined in [`heck-0.5.0/src/upper_camel.rs:36-39`](../../../.source_1765633015/heck-0.5.0/src/upper_camel.rs#L36-L39)*

`ToPascalCase` is an alias for [`ToUpperCamelCase`](#touppercamelcase). See ToUpperCamelCase for more
documentation.

#### Required Methods

- `fn to_pascal_case(&self) -> <Self as >::Owned`

  Convert this type to upper camel case.

#### Implementors

- `T`

