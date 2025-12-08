*[clap_builder](../../index.md) / [builder](../index.md) / [arg_predicate](index.md)*

---

# Module `arg_predicate`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ArgPredicate`](#argpredicate) | enum | Operations to perform on argument values |

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

- <span id="argpredicate-clone"></span>`fn clone(&self) -> ArgPredicate` — [`ArgPredicate`](../index.md)

##### `impl Debug for ArgPredicate`

- <span id="argpredicate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ArgPredicate`

##### `impl PartialEq for ArgPredicate`

- <span id="argpredicate-eq"></span>`fn eq(&self, other: &ArgPredicate) -> bool` — [`ArgPredicate`](../index.md)

##### `impl StructuralPartialEq for ArgPredicate`

