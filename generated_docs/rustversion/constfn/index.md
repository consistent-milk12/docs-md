*[rustversion](../index.md) / [constfn](index.md)*

---

# Module `constfn`

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

#### Implementations

- `fn from_ident(ident: &Ident) -> Self`

#### Trait Implementations

##### `impl PartialEq for Qualifiers`

- `fn eq(self: &Self, other: &Qualifiers) -> bool` — [`Qualifiers`](#qualifiers)

##### `impl PartialOrd for Qualifiers`

- `fn partial_cmp(self: &Self, other: &Qualifiers) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Qualifiers`](#qualifiers)

##### `impl StructuralPartialEq for Qualifiers`

## Functions

### `insert_const`

```rust
fn insert_const(input: proc_macro::TokenStream, const_span: proc_macro::Span) -> std::result::Result<proc_macro::TokenStream, Error>
```

