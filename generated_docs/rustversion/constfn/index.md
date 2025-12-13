*[rustversion](../index.md) / [constfn](index.md)*

---

# Module `constfn`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Qualifiers`](#qualifiers) | enum |  |
| [`insert_const`](#insert-const) | fn |  |

## Enums

### `Qualifiers`

```rust
enum Qualifiers {
    None,
    Async,
    Unsafe,
    Extern,
    Abi,
}
```

*Defined in [`rustversion-1.0.22/src/constfn.rs:6-12`](../../../.source_1765521767/rustversion-1.0.22/src/constfn.rs#L6-L12)*

#### Implementations

- <span id="qualifiers-from-ident"></span>`fn from_ident(ident: &Ident) -> Self`

#### Trait Implementations

##### `impl Any for Qualifiers`

- <span id="qualifiers-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Qualifiers`

- <span id="qualifiers-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Qualifiers`

- <span id="qualifiers-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Qualifiers`

- <span id="qualifiers-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Qualifiers`

- <span id="qualifiers-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Qualifiers`

- <span id="qualifiers-partialeq-eq"></span>`fn eq(&self, other: &Qualifiers) -> bool` — [`Qualifiers`](#qualifiers)

##### `impl PartialOrd for Qualifiers`

- <span id="qualifiers-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Qualifiers) -> option::Option<cmp::Ordering>` — [`Qualifiers`](#qualifiers)

##### `impl StructuralPartialEq for Qualifiers`

##### `impl<U> TryFrom for Qualifiers`

- <span id="qualifiers-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="qualifiers-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Qualifiers`

- <span id="qualifiers-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="qualifiers-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `insert_const`

```rust
fn insert_const(input: proc_macro::TokenStream, const_span: proc_macro::Span) -> std::result::Result<proc_macro::TokenStream, Error>
```

*Defined in [`rustversion-1.0.22/src/constfn.rs:25-58`](../../../.source_1765521767/rustversion-1.0.22/src/constfn.rs#L25-L58)*

