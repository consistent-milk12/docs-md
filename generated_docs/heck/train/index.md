*[heck](../index.md) / [train](index.md)*

---

# Module `train`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AsTrainCase`](#astraincase) | struct | This wrapper performs a train case conversion in [`fmt::Display`]. |
| [`ToTrainCase`](#totraincase) | trait | This trait defines a train case conversion. |

## Structs

### `AsTrainCase<T: AsRef<str>>`

```rust
struct AsTrainCase<T: AsRef<str>>(T);
```

*Defined in [`heck-0.5.0/src/train.rs:41`](../../../.source_1765633015/heck-0.5.0/src/train.rs#L41)*

This wrapper performs a train case conversion in [`fmt::Display`](../../miette_derive/index.md).

## Example:

```rust
use heck::AsTrainCase;

let sentence = "We are going to inherit the earth.";
assert_eq!(format!("{}", AsTrainCase(sentence)), "We-Are-Going-To-Inherit-The-Earth");
```

#### Trait Implementations

##### `impl<T> Any for AsTrainCase<T>`

- <span id="astraincase-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AsTrainCase<T>`

- <span id="astraincase-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AsTrainCase<T>`

- <span id="astraincase-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: AsRef<str>> Display for AsTrainCase<T>`

- <span id="astraincase-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AsTrainCase<T>`

- <span id="astraincase-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for AsTrainCase<T>`

- <span id="astraincase-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> ToString for AsTrainCase<T>`

- <span id="astraincase-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T, U> TryFrom for AsTrainCase<T>`

- <span id="astraincase-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="astraincase-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for AsTrainCase<T>`

- <span id="astraincase-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="astraincase-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `ToTrainCase`

```rust
trait ToTrainCase: ToOwned { ... }
```

*Defined in [`heck-0.5.0/src/train.rs:20-23`](../../../.source_1765633015/heck-0.5.0/src/train.rs#L20-L23)*

This trait defines a train case conversion.

In Train-Case, word boundaries are indicated by hyphens and words start
with Capital Letters.

## Example:

```rust
use heck::ToTrainCase;

let sentence = "We are going to inherit the earth.";
assert_eq!(sentence.to_train_case(), "We-Are-Going-To-Inherit-The-Earth");
```

#### Required Methods

- `fn to_train_case(&self) -> <Self as >::Owned`

  Convert this type to Train-Case.

#### Implementors

- `str`

