*[semver](../index.md) / [error](index.md)*

---

# Module `error`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`QuotedChar`](#quotedchar) | struct |  |
| [`ErrorKind`](#errorkind) | enum |  |
| [`Position`](#position) | enum |  |

## Structs

### `QuotedChar`

```rust
struct QuotedChar(char);
```

*Defined in [`semver-1.0.27/src/error.rs:113`](../../../.source_1765521767/semver-1.0.27/src/error.rs#L113)*

#### Trait Implementations

##### `impl Display for QuotedChar`

- <span id="quotedchar-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for QuotedChar`

- <span id="quotedchar-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `ErrorKind`

```rust
enum ErrorKind {
    Empty,
    UnexpectedEnd(Position),
    UnexpectedChar(Position, char),
    UnexpectedCharAfter(Position, char),
    ExpectedCommaFound(Position, char),
    LeadingZero(Position),
    Overflow(Position),
    EmptySegment(Position),
    IllegalCharacter(Position),
    WildcardNotTheOnlyComparator(char),
    UnexpectedAfterWildcard,
    ExcessiveComparators,
}
```

*Defined in [`semver-1.0.27/src/error.rs:4-17`](../../../.source_1765521767/semver-1.0.27/src/error.rs#L4-L17)*

### `Position`

```rust
enum Position {
    Major,
    Minor,
    Patch,
    Pre,
    Build,
}
```

*Defined in [`semver-1.0.27/src/error.rs:20-26`](../../../.source_1765521767/semver-1.0.27/src/error.rs#L20-L26)*

#### Trait Implementations

##### `impl Clone for Position`

- <span id="position-clone"></span>`fn clone(&self) -> Position` — [`Position`](#position)

##### `impl Copy for Position`

##### `impl Display for Position`

- <span id="position-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Position`

##### `impl PartialEq for Position`

- <span id="position-eq"></span>`fn eq(&self, other: &Position) -> bool` — [`Position`](#position)

##### `impl StructuralPartialEq for Position`

##### `impl ToString for Position`

- <span id="position-to-string"></span>`fn to_string(&self) -> String`

