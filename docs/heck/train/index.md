*[heck](../index.md) / [train](index.md)*

---

# Module `train`

## Structs

### `AsTrainCase<T: AsRef<str>>`

```rust
struct AsTrainCase<T: AsRef<str>>(T);
```

This wrapper performs a train case conversion in [`fmt::Display`](../../miette_derive/index.md).

## Example:

```rust
use heck::AsTrainCase;

let sentence = "We are going to inherit the earth.";
assert_eq!(format!("{}", AsTrainCase(sentence)), "We-Are-Going-To-Inherit-The-Earth");
```

#### Trait Implementations

##### `impl<T: AsRef<str>> Display for AsTrainCase<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for AsTrainCase<T>`

- `fn to_string(self: &Self) -> String`

## Traits

### `ToTrainCase`

```rust
trait ToTrainCase: ToOwned { ... }
```

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

- `fn to_train_case(self: &Self) -> <Self as >::Owned`

  Convert this type to Train-Case.

