*[rustversion](../index.md) / [constfn](index.md)*

---

# Module `constfn`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Qualifiers`](#qualifiers) | enum |  |
| [`insert_const`](#insert_const) | fn |  |

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

*Defined in [`rustversion-1.0.22/src/constfn.rs:6-12`](../../../.source_1765210505/rustversion-1.0.22/src/constfn.rs#L6-L12)*

#### Implementations

- <span id="qualifiers-from-ident"></span>`fn from_ident(ident: &Ident) -> Self`

#### Trait Implementations

##### `impl PartialEq for Qualifiers`

- <span id="qualifiers-eq"></span>`fn eq(&self, other: &Qualifiers) -> bool` — [`Qualifiers`](#qualifiers)

##### `impl PartialOrd for Qualifiers`

- <span id="qualifiers-partial-cmp"></span>`fn partial_cmp(&self, other: &Qualifiers) -> option::Option<cmp::Ordering>` — [`Qualifiers`](#qualifiers)

##### `impl StructuralPartialEq for Qualifiers`

## Functions

### `insert_const`

```rust
fn insert_const(input: proc_macro::TokenStream, const_span: proc_macro::Span) -> std::result::Result<proc_macro::TokenStream, Error>
```

*Defined in [`rustversion-1.0.22/src/constfn.rs:25-58`](../../../.source_1765210505/rustversion-1.0.22/src/constfn.rs#L25-L58)*

