*[serde_json](../../index.md) / [value](../index.md) / [index](index.md)*

---

# Module `index`

## Modules

- [`private`](private/index.md) - 

## Structs

### `Type<'a>`

```rust
struct Type<'a>(&'a super::Value);
```

Used in panic messages.

#### Trait Implementations

##### `impl<'a> Display for Type<'a>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for Type<'a>`

- `fn to_string(self: &Self) -> String`

## Traits

### `Index`

```rust
trait Index: private::Sealed { ... }
```

A type that can be used to index into a `serde_json::Value`.

The [`get`](#get) and `get_mut` methods of `Value` accept any type that
implements `Index`, as does the [square-bracket indexing operator]. This
trait is implemented for strings which are used as the index into a JSON
map, and for `usize` which is used as the index into a JSON array.



This trait is sealed and cannot be implemented for types outside of
`serde_json`.

# Examples

```rust
use serde_json::json;

let data = json!({ "inner": [1, 2, 3] });

// Data is a JSON map so it can be indexed with a string.
let inner = &data["inner"];

// Inner is a JSON array so it can be indexed with an integer.
let first = &inner[0];

assert_eq!(first, 1);
```

