*[clap_builder](../../index.md) / [parser](../index.md) / [error](index.md)*

---

# Module `error`

## Enums

### `MatchesError`

```rust
enum MatchesError {
    Downcast {
        actual: self::any_value::AnyValueId,
        expected: self::any_value::AnyValueId,
    },
    UnknownArgument {
    },
}
```

Violation of `ArgMatches` assumptions

#### Variants

- **`Downcast`**

  Failed to downcast `AnyValue` to the specified type

- **`UnknownArgument`**

  Argument not defined in `Command`

#### Implementations

- `fn unwrap<T>(id: &str, r: Result<T, MatchesError>) -> T` — [`MatchesError`](../index.md)

#### Trait Implementations

##### `impl Clone for MatchesError`

- `fn clone(self: &Self) -> MatchesError` — [`MatchesError`](../index.md)

##### `impl Debug for MatchesError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for MatchesError`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Error for MatchesError`

##### `impl<T> ToString for MatchesError`

- `fn to_string(self: &Self) -> String`

