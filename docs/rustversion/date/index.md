*[rustversion](../index.md) / [date](index.md)*

---

# Module `date`

## Structs

### `Date`

```rust
struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}
```

#### Trait Implementations

##### `impl Clone for Date`

- `fn clone(self: &Self) -> Date` — [`Date`](#date)

##### `impl Copy for Date`

##### `impl Debug for Date`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for Date`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Date`

##### `impl Ord for Date`

- `fn cmp(self: &Self, other: &Date) -> $crate::cmp::Ordering` — [`Date`](#date)

##### `impl PartialEq for Date`

- `fn eq(self: &Self, other: &Date) -> bool` — [`Date`](#date)

##### `impl PartialOrd for Date`

- `fn partial_cmp(self: &Self, other: &Date) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Date`](#date)

##### `impl StructuralPartialEq for Date`

##### `impl<T> ToString for Date`

- `fn to_string(self: &Self) -> String`

## Functions

### `parse`

```rust
fn parse(paren: proc_macro::Group, iter: &'_ mut IterImpl) -> std::result::Result<Date, Error>
```

### `try_parse`

```rust
fn try_parse(iter: &'_ mut IterImpl) -> std::result::Result<Date, ()>
```

