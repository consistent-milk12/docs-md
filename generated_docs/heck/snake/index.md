*[heck](../index.md) / [snake](index.md)*

---

# Module `snake`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AsSnakeCase`](#assnakecase) | struct | This wrapper performs a snake case conversion in [`fmt::Display`]. |
| [`ToSnakeCase`](#tosnakecase) | trait | This trait defines a snake case conversion. |
| [`ToSnekCase`](#tosnekcase) | trait | Oh heck, `SnekCase` is an alias for [`ToSnakeCase`]. |

## Structs

### `AsSnakeCase<T: AsRef<str>>`

```rust
struct AsSnakeCase<T: AsRef<str>>(T);
```

*Defined in [`heck-0.5.0/src/snake.rs:55`](../../../.source_1765521767/heck-0.5.0/src/snake.rs#L55)*

This wrapper performs a snake case conversion in [`fmt::Display`](../../miette_derive/index.md).

## Example:

```rust
use heck::AsSnakeCase;

let sentence = "We carry a new world here, in our hearts.";
assert_eq!(format!("{}", AsSnakeCase(sentence)), "we_carry_a_new_world_here_in_our_hearts");
```

#### Trait Implementations

##### `impl<T> Any for AsSnakeCase<T>`

- <span id="assnakecase-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AsSnakeCase<T>`

- <span id="assnakecase-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AsSnakeCase<T>`

- <span id="assnakecase-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: AsRef<str>> Display for AsSnakeCase<T>`

- <span id="assnakecase-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AsSnakeCase<T>`

- <span id="assnakecase-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for AsSnakeCase<T>`

- <span id="assnakecase-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> ToString for AsSnakeCase<T>`

- <span id="assnakecase-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T, U> TryFrom for AsSnakeCase<T>`

- <span id="assnakecase-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="assnakecase-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for AsSnakeCase<T>`

- <span id="assnakecase-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="assnakecase-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `ToSnakeCase`

```rust
trait ToSnakeCase: ToOwned { ... }
```

*Defined in [`heck-0.5.0/src/snake.rs:21-24`](../../../.source_1765521767/heck-0.5.0/src/snake.rs#L21-L24)*

This trait defines a snake case conversion.

In snake_case, word boundaries are indicated by underscores.

## Example:

```rust
use heck::ToSnakeCase;

let sentence = "We carry a new world here, in our hearts.";
assert_eq!(sentence.to_snake_case(), "we_carry_a_new_world_here_in_our_hearts");
```

#### Required Methods

- `fn to_snake_case(&self) -> <Self as >::Owned`

  Convert this type to snake case.

#### Implementors

- `str`

### `ToSnekCase`

```rust
trait ToSnekCase: ToOwned { ... }
```

*Defined in [`heck-0.5.0/src/snake.rs:28-31`](../../../.source_1765521767/heck-0.5.0/src/snake.rs#L28-L31)*

Oh heck, `SnekCase` is an alias for [`ToSnakeCase`](#tosnakecase). See ToSnakeCase for
more documentation.

#### Required Methods

- `fn to_snek_case(&self) -> <Self as >::Owned`

  Convert this type to snek case.

#### Implementors

- `T`

