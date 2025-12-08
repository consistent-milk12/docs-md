*[clap_builder](../../../index.md) / [parser](../../index.md) / [matches](../index.md) / [value_source](index.md)*

---

# Module `value_source`

## Enums

### `ValueSource`

```rust
enum ValueSource {
    DefaultValue,
    EnvVariable,
    CommandLine,
}
```

Origin of the argument's value

#### Variants

- **`DefaultValue`**

  Value came `Arg::default_value`

- **`EnvVariable`**

  Value came `Arg::env`

- **`CommandLine`**

  Value was passed in on the command-line

#### Implementations

- `fn is_explicit(self: Self) -> bool`

#### Trait Implementations

##### `impl Clone for ValueSource`

- `fn clone(self: &Self) -> ValueSource` — [`ValueSource`](../../index.md)

##### `impl Copy for ValueSource`

##### `impl Debug for ValueSource`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for ValueSource`

##### `impl Ord for ValueSource`

- `fn cmp(self: &Self, other: &ValueSource) -> $crate::cmp::Ordering` — [`ValueSource`](../../index.md)

##### `impl PartialEq for ValueSource`

- `fn eq(self: &Self, other: &ValueSource) -> bool` — [`ValueSource`](../../index.md)

##### `impl PartialOrd for ValueSource`

- `fn partial_cmp(self: &Self, other: &ValueSource) -> $crate::option::Option<$crate::cmp::Ordering>` — [`ValueSource`](../../index.md)

##### `impl StructuralPartialEq for ValueSource`

