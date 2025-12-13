*[clap_derive](../../index.md) / [utils](../index.md) / [ty](index.md)*

---

# Module `ty`

Special types handling

## Contents

- [Enums](#enums)
  - [`Ty`](#ty)
- [Functions](#functions)
  - [`inner_type`](#inner-type)
  - [`sub_type`](#sub-type)
  - [`only_last_segment`](#only-last-segment)
  - [`subty_if`](#subty-if)
  - [`subty_if_name`](#subty-if-name)
  - [`is_simple_ty`](#is-simple-ty)
  - [`is_generic_ty`](#is-generic-ty)
  - [`is_unit_ty`](#is-unit-ty)
  - [`only_one`](#only-one)
  - [`get_vec_ty`](#get-vec-ty)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Ty`](#ty) | enum |  |
| [`inner_type`](#inner-type) | fn |  |
| [`sub_type`](#sub-type) | fn |  |
| [`only_last_segment`](#only-last-segment) | fn |  |
| [`subty_if`](#subty-if) | fn |  |
| [`subty_if_name`](#subty-if-name) | fn |  |
| [`is_simple_ty`](#is-simple-ty) | fn |  |
| [`is_generic_ty`](#is-generic-ty) | fn |  |
| [`is_unit_ty`](#is-unit-ty) | fn |  |
| [`only_one`](#only-one) | fn |  |
| [`get_vec_ty`](#get-vec-ty) | fn |  |

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

*Defined in [`clap_derive-4.5.49/src/utils/ty.rs:11-20`](../../../../.source_1765633015/clap_derive-4.5.49/src/utils/ty.rs#L11-L20)*

#### Implementations

- <span id="ty-from-syn-ty"></span>`fn from_syn_ty(ty: &Type) -> Sp<Self>` — [`Sp`](../spanned/index.md#sp)

- <span id="ty-as-str"></span>`fn as_str(&self) -> &'static str`

#### Trait Implementations

##### `impl Any for Ty`

- <span id="ty-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Ty`

- <span id="ty-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Ty`

- <span id="ty-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Ty`

- <span id="ty-clone"></span>`fn clone(&self) -> Ty` — [`Ty`](#ty)

##### `impl CloneToUninit for Ty`

- <span id="ty-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Ty`

##### `impl Debug for Ty`

- <span id="ty-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Ty`

##### `impl<T> From for Ty`

- <span id="ty-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Ty`

- <span id="ty-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Ty`

- <span id="ty-partialeq-eq"></span>`fn eq(&self, other: &Ty) -> bool` — [`Ty`](#ty)

##### `impl StructuralPartialEq for Ty`

##### `impl ToOwned for Ty`

- <span id="ty-toowned-type-owned"></span>`type Owned = T`

- <span id="ty-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="ty-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Ty`

- <span id="ty-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ty-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Ty`

- <span id="ty-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ty-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `inner_type`

```rust
fn inner_type(field_ty: &syn::Type) -> &syn::Type
```

*Defined in [`clap_derive-4.5.49/src/utils/ty.rs:58-71`](../../../../.source_1765633015/clap_derive-4.5.49/src/utils/ty.rs#L58-L71)*

### `sub_type`

```rust
fn sub_type(ty: &syn::Type) -> Option<&syn::Type>
```

*Defined in [`clap_derive-4.5.49/src/utils/ty.rs:73-75`](../../../../.source_1765633015/clap_derive-4.5.49/src/utils/ty.rs#L73-L75)*

### `only_last_segment`

```rust
fn only_last_segment(ty: &syn::Type) -> Option<&syn::PathSegment>
```

*Defined in [`clap_derive-4.5.49/src/utils/ty.rs:77-93`](../../../../.source_1765633015/clap_derive-4.5.49/src/utils/ty.rs#L77-L93)*

### `subty_if`

```rust
fn subty_if<F>(ty: &syn::Type, f: F) -> Option<&syn::Type>
where
    F: FnOnce(&syn::PathSegment) -> bool
```

*Defined in [`clap_derive-4.5.49/src/utils/ty.rs:95-114`](../../../../.source_1765633015/clap_derive-4.5.49/src/utils/ty.rs#L95-L114)*

### `subty_if_name`

```rust
fn subty_if_name<'a>(ty: &'a syn::Type, name: &str) -> Option<&'a syn::Type>
```

*Defined in [`clap_derive-4.5.49/src/utils/ty.rs:116-118`](../../../../.source_1765633015/clap_derive-4.5.49/src/utils/ty.rs#L116-L118)*

### `is_simple_ty`

```rust
fn is_simple_ty(ty: &syn::Type, name: &str) -> bool
```

*Defined in [`clap_derive-4.5.49/src/utils/ty.rs:120-130`](../../../../.source_1765633015/clap_derive-4.5.49/src/utils/ty.rs#L120-L130)*

### `is_generic_ty`

```rust
fn is_generic_ty(ty: &syn::Type, name: &str) -> bool
```

*Defined in [`clap_derive-4.5.49/src/utils/ty.rs:132-134`](../../../../.source_1765633015/clap_derive-4.5.49/src/utils/ty.rs#L132-L134)*

### `is_unit_ty`

```rust
fn is_unit_ty(ty: &syn::Type) -> bool
```

*Defined in [`clap_derive-4.5.49/src/utils/ty.rs:136-142`](../../../../.source_1765633015/clap_derive-4.5.49/src/utils/ty.rs#L136-L142)*

### `only_one`

```rust
fn only_one<I, T>(iter: I) -> Option<T>
where
    I: Iterator<Item = T>
```

*Defined in [`clap_derive-4.5.49/src/utils/ty.rs:144-149`](../../../../.source_1765633015/clap_derive-4.5.49/src/utils/ty.rs#L144-L149)*

### `get_vec_ty`

```rust
fn get_vec_ty(ty: &syn::Type, vec_ty: Ty, _vecvec_ty: Ty) -> Option<Ty>
```

*Defined in [`clap_derive-4.5.49/src/utils/ty.rs:163-165`](../../../../.source_1765633015/clap_derive-4.5.49/src/utils/ty.rs#L163-L165)*

