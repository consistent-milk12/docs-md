*[clap_builder](../../index.md) / [util](../index.md) / [id](index.md)*

---

# Module `id`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Id`](#id) | struct | [`Arg`][crate::Arg] or [`ArgGroup`][crate::ArgGroup] identifier |

## Structs

### `Id`

```rust
struct Id(crate::builder::Str);
```

`Arg` or `ArgGroup` identifier

This is used for accessing the value in `ArgMatches` or defining
relationships between `Arg`s and `ArgGroup`s with functions like
`Arg::conflicts_with`.

#### Implementations

- <span id="id-help"></span>`const HELP: &'static str`

- <span id="id-version"></span>`const VERSION: &'static str`

- <span id="id-external"></span>`const EXTERNAL: &'static str`

- <span id="id-from-static-ref"></span>`fn from_static_ref(name: &'static str) -> Self`

- <span id="id-as-str"></span>`fn as_str(&self) -> &str`

- <span id="id-as-internal-str"></span>`fn as_internal_str(&self) -> &Str` — [`Str`](../../builder/index.md)

#### Trait Implementations

##### `impl AsRef for Id`

- <span id="id-as-ref"></span>`fn as_ref(&self) -> &str`

##### `impl Clone for Id`

- <span id="id-clone"></span>`fn clone(&self) -> Id` — [`Id`](../../index.md)

##### `impl Debug for Id`

- <span id="id-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default for Id`

- <span id="id-default"></span>`fn default() -> Id` — [`Id`](../../index.md)

##### `impl Display for Id`

- <span id="id-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for Id`

##### `impl Hash for Id`

- <span id="id-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<I> IntoResettable for Id`

- <span id="id-into-resettable"></span>`fn into_resettable(self) -> Resettable<Str>` — [`Resettable`](../../builder/index.md), [`Str`](../../builder/index.md)

##### `impl Ord for Id`

- <span id="id-cmp"></span>`fn cmp(&self, other: &Id) -> cmp::Ordering` — [`Id`](../../index.md)

##### `impl PartialEq for Id`

- <span id="id-eq"></span>`fn eq(&self, other: &&str) -> bool`

##### `impl PartialOrd for Id`

- <span id="id-partial-cmp"></span>`fn partial_cmp(&self, other: &Id) -> option::Option<cmp::Ordering>` — [`Id`](../../index.md)

##### `impl StructuralPartialEq for Id`

##### `impl<T> ToString for Id`

- <span id="id-to-string"></span>`fn to_string(&self) -> String`

