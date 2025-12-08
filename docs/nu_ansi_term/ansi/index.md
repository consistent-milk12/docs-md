*[nu_ansi_term](../index.md) / [ansi](index.md)*

---

# Module `ansi`

## Structs

### `Prefix`

```rust
struct Prefix(crate::style::Style);
```

Like `AnsiString`, but only displays the style prefix.

This type implements the `Display` trait, meaning it can be written to a
`std::fmt` formatting without doing any extra allocation, and written to a
string with the `.to_string()` method. For examples, see
`Style::prefix`.

#### Trait Implementations

##### `impl Clone for Prefix`

- `fn clone(self: &Self) -> Prefix` — [`Prefix`](#prefix)

##### `impl Copy for Prefix`

##### `impl Debug for Prefix`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for Prefix`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for Prefix`

- `fn to_string(self: &Self) -> String`

### `Infix`

```rust
struct Infix(crate::style::Style, crate::style::Style);
```

Like `AnsiString`, but only displays the difference between two
styles.

This type implements the `Display` trait, meaning it can be written to a
`std::fmt` formatting without doing any extra allocation, and written to a
string with the `.to_string()` method. For examples, see
`Style::infix`.

#### Trait Implementations

##### `impl Clone for Infix`

- `fn clone(self: &Self) -> Infix` — [`Infix`](#infix)

##### `impl Copy for Infix`

##### `impl Debug for Infix`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for Infix`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for Infix`

- `fn to_string(self: &Self) -> String`

### `Suffix`

```rust
struct Suffix(crate::style::Style);
```

Like `AnsiString`, but only displays the style suffix.

This type implements the `Display` trait, meaning it can be written to a
`std::fmt` formatting without doing any extra allocation, and written to a
string with the `.to_string()` method. For examples, see
`Style::suffix`.

#### Trait Implementations

##### `impl Clone for Suffix`

- `fn clone(self: &Self) -> Suffix` — [`Suffix`](#suffix)

##### `impl Copy for Suffix`

##### `impl Debug for Suffix`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for Suffix`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for Suffix`

- `fn to_string(self: &Self) -> String`

