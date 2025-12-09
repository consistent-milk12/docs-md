*[clap_builder](../../../index.md) / [parser](../../index.md) / [matches](../index.md) / [value_source](index.md)*

---

# Module `value_source`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ValueSource`](#valuesource) | enum | Origin of the argument's value |

## Enums

### `ValueSource`

```rust
enum ValueSource {
    DefaultValue,
    EnvVariable,
    CommandLine,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/matches/value_source.rs:4-11`](../../../../../.source_1765210505/clap_builder-4.5.53/src/parser/matches/value_source.rs#L4-L11)*

Origin of the argument's value

#### Variants

- **`DefaultValue`**

  Value came `Arg::default_value`

- **`EnvVariable`**

  Value came `Arg::env`

- **`CommandLine`**

  Value was passed in on the command-line

#### Implementations

- <span id="valuesource-is-explicit"></span>`fn is_explicit(self) -> bool`

#### Trait Implementations

##### `impl Clone for ValueSource`

- <span id="valuesource-clone"></span>`fn clone(&self) -> ValueSource` — [`ValueSource`](#valuesource)

##### `impl Copy for ValueSource`

##### `impl Debug for ValueSource`

- <span id="valuesource-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ValueSource`

##### `impl Ord for ValueSource`

- <span id="valuesource-cmp"></span>`fn cmp(&self, other: &ValueSource) -> cmp::Ordering` — [`ValueSource`](#valuesource)

##### `impl PartialEq for ValueSource`

- <span id="valuesource-eq"></span>`fn eq(&self, other: &ValueSource) -> bool` — [`ValueSource`](#valuesource)

##### `impl PartialOrd for ValueSource`

- <span id="valuesource-partial-cmp"></span>`fn partial_cmp(&self, other: &ValueSource) -> option::Option<cmp::Ordering>` — [`ValueSource`](#valuesource)

##### `impl StructuralPartialEq for ValueSource`

