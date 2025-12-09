*[serde_json](../../index.md) / [value](../index.md) / [index](index.md)*

---

# Module `index`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private`](#private) | mod |  |
| [`Type`](#type) | struct | Used in panic messages. |
| [`Index`](#index) | trait | A type that can be used to index into a `serde_json::Value`. |

## Modules

- [`private`](private/index.md)

## Structs

### `Type<'a>`

```rust
struct Type<'a>(&'a super::Value);
```

*Defined in [`serde_json-1.0.145/src/value/index.rs:144`](../../../../.source_1765210505/serde_json-1.0.145/src/value/index.rs#L144)*

Used in panic messages.

#### Trait Implementations

##### `impl Display for Type<'a>`

- <span id="type-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for Type<'a>`

- <span id="type-to-string"></span>`fn to_string(&self) -> String`

## Traits

### `Index`

```rust
trait Index: private::Sealed { ... }
```

*Defined in [`serde_json-1.0.145/src/value/index.rs:37-52`](../../../../.source_1765210505/serde_json-1.0.145/src/value/index.rs#L37-L52)*

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

#### Implementors

- `&T`
- `alloc::string::String`
- `str`
- `usize`

