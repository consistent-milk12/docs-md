*[clap_builder](../../index.md) / [parser](../index.md) / [error](index.md)*

---

# Module `error`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MatchesError`](#matcheserror) | enum | Violation of [`ArgMatches`][crate::ArgMatches] assumptions |

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

- <span id="matcheserror-unwrap"></span>`fn unwrap<T>(id: &str, r: Result<T, MatchesError>) -> T` — [`MatchesError`](../index.md)

#### Trait Implementations

##### `impl Clone for MatchesError`

- <span id="matcheserror-clone"></span>`fn clone(&self) -> MatchesError` — [`MatchesError`](../index.md)

##### `impl Debug for MatchesError`

- <span id="matcheserror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for MatchesError`

- <span id="matcheserror-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Error for MatchesError`

##### `impl<T> ToString for MatchesError`

- <span id="matcheserror-to-string"></span>`fn to_string(&self) -> String`

