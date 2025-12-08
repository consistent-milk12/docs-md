*[unit_prefix](../index.md) / [parse](index.md)*

---

# Module `parse`

## Structs

### `NumberPrefixParseError`

```rust
struct NumberPrefixParseError(());
```

The error returned when a `NumberPrefix` is failed to be parsed.

#### Trait Implementations

##### `impl Clone for NumberPrefixParseError`

- `fn clone(self: &Self) -> NumberPrefixParseError` — [`NumberPrefixParseError`](#numberprefixparseerror)

##### `impl Copy for NumberPrefixParseError`

##### `impl Debug for NumberPrefixParseError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for NumberPrefixParseError`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for NumberPrefixParseError`

##### `impl Error for NumberPrefixParseError`

##### `impl PartialEq for NumberPrefixParseError`

- `fn eq(self: &Self, other: &NumberPrefixParseError) -> bool` — [`NumberPrefixParseError`](#numberprefixparseerror)

##### `impl StructuralPartialEq for NumberPrefixParseError`

##### `impl<T> ToString for NumberPrefixParseError`

- `fn to_string(self: &Self) -> String`

