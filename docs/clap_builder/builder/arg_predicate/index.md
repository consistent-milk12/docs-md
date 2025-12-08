*[clap_builder](../../index.md) / [builder](../index.md) / [arg_predicate](index.md)*

---

# Module `arg_predicate`

## Enums

### `ArgPredicate`

```rust
enum ArgPredicate {
    IsPresent,
    Equals(crate::builder::OsStr),
}
```

Operations to perform on argument values

These do not apply to `ValueSource::DefaultValue`

#### Variants

- **`IsPresent`**

  Is the argument present?

- **`Equals`**

  Does the argument match the specified value?

#### Trait Implementations

##### `impl Clone for ArgPredicate`

- `fn clone(self: &Self) -> ArgPredicate` — [`ArgPredicate`](../index.md)

##### `impl Debug for ArgPredicate`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for ArgPredicate`

##### `impl PartialEq for ArgPredicate`

- `fn eq(self: &Self, other: &ArgPredicate) -> bool` — [`ArgPredicate`](../index.md)

##### `impl StructuralPartialEq for ArgPredicate`

