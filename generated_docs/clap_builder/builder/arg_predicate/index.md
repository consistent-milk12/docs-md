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

*Defined in [`clap_builder-4.5.53/src/builder/arg_predicate.rs:8-13`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/arg_predicate.rs#L8-L13)*

Operations to perform on argument values

These do not apply to `ValueSource::DefaultValue`

#### Variants

- **`IsPresent`**

  Is the argument present?

- **`Equals`**

  Does the argument match the specified value?

#### Trait Implementations

##### `impl Clone for ArgPredicate`

- <span id="argpredicate-clone"></span>`fn clone(&self) -> ArgPredicate` — [`ArgPredicate`](#argpredicate)

##### `impl Debug for ArgPredicate`

- <span id="argpredicate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ArgPredicate`

##### `impl PartialEq for ArgPredicate`

- <span id="argpredicate-eq"></span>`fn eq(&self, other: &ArgPredicate) -> bool` — [`ArgPredicate`](#argpredicate)

##### `impl StructuralPartialEq for ArgPredicate`

