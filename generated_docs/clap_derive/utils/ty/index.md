*[clap_derive](../../index.md) / [utils](../index.md) / [ty](index.md)*

---

# Module `ty`

Special types handling

## Contents

- [Enums](#enums)
  - [`Ty`](#ty)
- [Functions](#functions)
  - [`inner_type`](#inner_type)
  - [`sub_type`](#sub_type)
  - [`only_last_segment`](#only_last_segment)
  - [`subty_if`](#subty_if)
  - [`subty_if_name`](#subty_if_name)
  - [`is_simple_ty`](#is_simple_ty)
  - [`is_generic_ty`](#is_generic_ty)
  - [`is_unit_ty`](#is_unit_ty)
  - [`only_one`](#only_one)
  - [`get_vec_ty`](#get_vec_ty)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Ty`](#ty) | enum |  |
| [`inner_type`](#inner_type) | fn |  |
| [`sub_type`](#sub_type) | fn |  |
| [`only_last_segment`](#only_last_segment) | fn |  |
| [`subty_if`](#subty_if) | fn |  |
| [`subty_if_name`](#subty_if_name) | fn |  |
| [`is_simple_ty`](#is_simple_ty) | fn |  |
| [`is_generic_ty`](#is_generic_ty) | fn |  |
| [`is_unit_ty`](#is_unit_ty) | fn |  |
| [`only_one`](#only_one) | fn |  |
| [`get_vec_ty`](#get_vec_ty) | fn |  |

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

*Defined in [`clap_derive-4.5.49/src/utils/ty.rs:11-20`](../../../../.source_1765210505/clap_derive-4.5.49/src/utils/ty.rs#L11-L20)*

#### Implementations

- <span id="ty-from-syn-ty"></span>`fn from_syn_ty(ty: &Type) -> Sp<Self>` — [`Sp`](../spanned/index.md#sp)

- <span id="ty-as-str"></span>`fn as_str(&self) -> &'static str`

#### Trait Implementations

##### `impl Clone for Ty`

- <span id="ty-clone"></span>`fn clone(&self) -> Ty` — [`Ty`](#ty)

##### `impl Copy for Ty`

##### `impl Debug for Ty`

- <span id="ty-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Ty`

##### `impl PartialEq for Ty`

- <span id="ty-eq"></span>`fn eq(&self, other: &Ty) -> bool` — [`Ty`](#ty)

##### `impl StructuralPartialEq for Ty`

## Functions

### `inner_type`

```rust
fn inner_type(field_ty: &syn::Type) -> &syn::Type
```

*Defined in [`clap_derive-4.5.49/src/utils/ty.rs:58-71`](../../../../.source_1765210505/clap_derive-4.5.49/src/utils/ty.rs#L58-L71)*

### `sub_type`

```rust
fn sub_type(ty: &syn::Type) -> Option<&syn::Type>
```

*Defined in [`clap_derive-4.5.49/src/utils/ty.rs:73-75`](../../../../.source_1765210505/clap_derive-4.5.49/src/utils/ty.rs#L73-L75)*

### `only_last_segment`

```rust
fn only_last_segment(ty: &syn::Type) -> Option<&syn::PathSegment>
```

*Defined in [`clap_derive-4.5.49/src/utils/ty.rs:77-93`](../../../../.source_1765210505/clap_derive-4.5.49/src/utils/ty.rs#L77-L93)*

### `subty_if`

```rust
fn subty_if<F>(ty: &syn::Type, f: F) -> Option<&syn::Type>
where
    F: FnOnce(&syn::PathSegment) -> bool
```

*Defined in [`clap_derive-4.5.49/src/utils/ty.rs:95-114`](../../../../.source_1765210505/clap_derive-4.5.49/src/utils/ty.rs#L95-L114)*

### `subty_if_name`

```rust
fn subty_if_name<'a>(ty: &'a syn::Type, name: &str) -> Option<&'a syn::Type>
```

*Defined in [`clap_derive-4.5.49/src/utils/ty.rs:116-118`](../../../../.source_1765210505/clap_derive-4.5.49/src/utils/ty.rs#L116-L118)*

### `is_simple_ty`

```rust
fn is_simple_ty(ty: &syn::Type, name: &str) -> bool
```

*Defined in [`clap_derive-4.5.49/src/utils/ty.rs:120-130`](../../../../.source_1765210505/clap_derive-4.5.49/src/utils/ty.rs#L120-L130)*

### `is_generic_ty`

```rust
fn is_generic_ty(ty: &syn::Type, name: &str) -> bool
```

*Defined in [`clap_derive-4.5.49/src/utils/ty.rs:132-134`](../../../../.source_1765210505/clap_derive-4.5.49/src/utils/ty.rs#L132-L134)*

### `is_unit_ty`

```rust
fn is_unit_ty(ty: &syn::Type) -> bool
```

*Defined in [`clap_derive-4.5.49/src/utils/ty.rs:136-142`](../../../../.source_1765210505/clap_derive-4.5.49/src/utils/ty.rs#L136-L142)*

### `only_one`

```rust
fn only_one<I, T>(iter: I) -> Option<T>
where
    I: Iterator<Item = T>
```

*Defined in [`clap_derive-4.5.49/src/utils/ty.rs:144-149`](../../../../.source_1765210505/clap_derive-4.5.49/src/utils/ty.rs#L144-L149)*

### `get_vec_ty`

```rust
fn get_vec_ty(ty: &syn::Type, vec_ty: Ty, _vecvec_ty: Ty) -> Option<Ty>
```

*Defined in [`clap_derive-4.5.49/src/utils/ty.rs:163-165`](../../../../.source_1765210505/clap_derive-4.5.49/src/utils/ty.rs#L163-L165)*

