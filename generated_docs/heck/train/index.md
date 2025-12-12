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

*Defined in [`heck-0.5.0/src/train.rs:41`](../../../.source_1765521767/heck-0.5.0/src/train.rs#L41)*

This wrapper performs a train case conversion in [`fmt::Display`](../../miette_derive/fmt/index.md).

## Example:

```rust
use heck::AsTrainCase;

let sentence = "We are going to inherit the earth.";
assert_eq!(format!("{}", AsTrainCase(sentence)), "We-Are-Going-To-Inherit-The-Earth");
```

#### Trait Implementations

##### `impl<T: AsRef<str>> Display for AsTrainCase<T>`

- <span id="astraincase-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for AsTrainCase<T>`

- <span id="astraincase-to-string"></span>`fn to_string(&self) -> String`

## Traits

### `ToTrainCase`

```rust
trait ToTrainCase: ToOwned { ... }
```

*Defined in [`heck-0.5.0/src/train.rs:20-23`](../../../.source_1765521767/heck-0.5.0/src/train.rs#L20-L23)*

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

