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

*Defined in [`serde_json-1.0.145/src/value/index.rs:144`](../../../../.source_1765633015/serde_json-1.0.145/src/value/index.rs#L144)*

Used in panic messages.

#### Trait Implementations

##### `impl Any for Type<'a>`

- <span id="type-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Type<'a>`

- <span id="type-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Type<'a>`

- <span id="type-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Display for Type<'a>`

- <span id="type-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Type<'a>`

- <span id="type-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Type<'a>`

- <span id="type-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for Type<'a>`

- <span id="type-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Type<'a>`

- <span id="type-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="type-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Type<'a>`

- <span id="type-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="type-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Index`

```rust
trait Index: private::Sealed { ... }
```

*Defined in [`serde_json-1.0.145/src/value/index.rs:37-52`](../../../../.source_1765633015/serde_json-1.0.145/src/value/index.rs#L37-L52)*

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

