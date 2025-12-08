*[clap_derive](../../index.md) / [utils](../index.md) / [ty](index.md)*

---

# Module `ty`

Special types handling

## Enums

### `Ty`

```rust
enum Ty {
    Unit,
    Vec,
    VecVec,
    Option,
    OptionOption,
    OptionVec,
    OptionVecVec,
    Other,
}
```

#### Implementations

- `fn from_syn_ty(ty: &Type) -> Sp<Self>` — [`Sp`](../spanned/index.md)

- `fn as_str(self: &Self) -> &'static str`

#### Trait Implementations

##### `impl Clone for Ty`

- `fn clone(self: &Self) -> Ty` — [`Ty`](#ty)

##### `impl Copy for Ty`

##### `impl Debug for Ty`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Ty`

##### `impl PartialEq for Ty`

- `fn eq(self: &Self, other: &Ty) -> bool` — [`Ty`](#ty)

##### `impl StructuralPartialEq for Ty`

## Functions

### `inner_type`

```rust
fn inner_type(field_ty: &syn::Type) -> &syn::Type
```

### `sub_type`

```rust
fn sub_type(ty: &syn::Type) -> Option<&syn::Type>
```

### `only_last_segment`

```rust
fn only_last_segment(ty: &syn::Type) -> Option<&syn::PathSegment>
```

### `subty_if`

```rust
fn subty_if<F>(ty: &syn::Type, f: F) -> Option<&syn::Type>
where
    F: FnOnce(&syn::PathSegment) -> bool
```

### `subty_if_name`

```rust
fn subty_if_name<'a>(ty: &'a syn::Type, name: &str) -> Option<&'a syn::Type>
```

### `is_simple_ty`

```rust
fn is_simple_ty(ty: &syn::Type, name: &str) -> bool
```

### `is_generic_ty`

```rust
fn is_generic_ty(ty: &syn::Type, name: &str) -> bool
```

### `is_unit_ty`

```rust
fn is_unit_ty(ty: &syn::Type) -> bool
```

### `only_one`

```rust
fn only_one<I, T>(iter: I) -> Option<T>
where
    I: Iterator<Item = T>
```

### `get_vec_ty`

```rust
fn get_vec_ty(ty: &syn::Type, vec_ty: Ty, _vecvec_ty: Ty) -> Option<Ty>
```

