*[serde_derive](../../index.md) / [internals](../index.md) / [case](index.md)*

---

# Module `case`

Code to convert the Rust-styled field/variant (e.g. `my_field`, `MyType`) to the
case of the source (e.g. `my-field`, `MY_FIELD`).

## Structs

### `ParseError<'a>`

```rust
struct ParseError<'a> {
    unknown: &'a str,
}
```

#### Trait Implementations

##### `impl<'a> Display for ParseError<'a>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for ParseError<'a>`

- `fn to_string(self: &Self) -> String`

## Enums

### `RenameRule`

```rust
enum RenameRule {
    None,
    LowerCase,
    UpperCase,
    PascalCase,
    CamelCase,
    SnakeCase,
    ScreamingSnakeCase,
    KebabCase,
    ScreamingKebabCase,
}
```

The different possible ways to change case of fields in a struct, or variants in an enum.

#### Variants

- **`None`**

  Don't apply a default rename rule.

- **`LowerCase`**

  Rename direct children to "lowercase" style.

- **`UpperCase`**

  Rename direct children to "UPPERCASE" style.

- **`PascalCase`**

  Rename direct children to "PascalCase" style, as typically used for
  enum variants.

- **`CamelCase`**

  Rename direct children to "camelCase" style.

- **`SnakeCase`**

  Rename direct children to "snake_case" style, as commonly used for
  fields.

- **`ScreamingSnakeCase`**

  Rename direct children to "SCREAMING_SNAKE_CASE" style, as commonly
  used for constants.

- **`KebabCase`**

  Rename direct children to "kebab-case" style.

- **`ScreamingKebabCase`**

  Rename direct children to "SCREAMING-KEBAB-CASE" style.

#### Implementations

- `fn from_str(rename_all_str: &str) -> Result<Self, ParseError<'_>>` — [`ParseError`](#parseerror)

- `fn apply_to_variant(self: Self, variant: &str) -> String`

- `fn apply_to_field(self: Self, field: &str) -> String`

- `fn or(self: Self, rule_b: Self) -> Self`

#### Trait Implementations

##### `impl Clone for RenameRule`

- `fn clone(self: &Self) -> RenameRule` — [`RenameRule`](#renamerule)

##### `impl Copy for RenameRule`

##### `impl PartialEq for RenameRule`

- `fn eq(self: &Self, other: &RenameRule) -> bool` — [`RenameRule`](#renamerule)

##### `impl StructuralPartialEq for RenameRule`

