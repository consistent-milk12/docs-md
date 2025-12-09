*[unit_prefix](../index.md) / [parse](index.md)*

---

# Module `parse`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NumberPrefixParseError`](#numberprefixparseerror) | struct | The error returned when a `NumberPrefix` is failed to be parsed. |

## Structs

### `NumberPrefixParseError`

```rust
struct NumberPrefixParseError(());
```

*Defined in [`unit-prefix-0.5.2/src/parse.rs:50`](../../../.source_1765210505/unit-prefix-0.5.2/src/parse.rs#L50)*

The error returned when a `NumberPrefix` is failed to be parsed.

#### Trait Implementations

##### `impl Clone for NumberPrefixParseError`

- <span id="numberprefixparseerror-clone"></span>`fn clone(&self) -> NumberPrefixParseError` — [`NumberPrefixParseError`](#numberprefixparseerror)

##### `impl Copy for NumberPrefixParseError`

##### `impl Debug for NumberPrefixParseError`

- <span id="numberprefixparseerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for NumberPrefixParseError`

- <span id="numberprefixparseerror-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for NumberPrefixParseError`

##### `impl Error for NumberPrefixParseError`

##### `impl PartialEq for NumberPrefixParseError`

- <span id="numberprefixparseerror-eq"></span>`fn eq(&self, other: &NumberPrefixParseError) -> bool` — [`NumberPrefixParseError`](#numberprefixparseerror)

##### `impl StructuralPartialEq for NumberPrefixParseError`

##### `impl ToString for NumberPrefixParseError`

- <span id="numberprefixparseerror-to-string"></span>`fn to_string(&self) -> String`

