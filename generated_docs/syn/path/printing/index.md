*[syn](../../index.md) / [path](../index.md) / [printing](index.md)*

---

# Module `printing`

## Contents

- [Enums](#enums)
  - [`PathStyle`](#pathstyle)
- [Functions](#functions)
  - [`print_path`](#print-path)
  - [`print_path_segment`](#print-path-segment)
  - [`print_path_arguments`](#print-path-arguments)
  - [`print_angle_bracketed_generic_arguments`](#print-angle-bracketed-generic-arguments)
  - [`print_parenthesized_generic_arguments`](#print-parenthesized-generic-arguments)
  - [`print_qpath`](#print-qpath)
  - [`conditionally_print_turbofish`](#conditionally-print-turbofish)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PathStyle`](#pathstyle) | enum |  |
| [`print_path`](#print-path) | fn |  |
| [`print_path_segment`](#print-path-segment) | fn |  |
| [`print_path_arguments`](#print-path-arguments) | fn |  |
| [`print_angle_bracketed_generic_arguments`](#print-angle-bracketed-generic-arguments) | fn |  |
| [`print_parenthesized_generic_arguments`](#print-parenthesized-generic-arguments) | fn |  |
| [`print_qpath`](#print-qpath) | fn |  |
| [`conditionally_print_turbofish`](#conditionally-print-turbofish) | fn |  |

## Enums

### `PathStyle`

```rust
enum PathStyle {
    Expr,
    Mod,
    AsWritten,
}
```

*Defined in [`syn-2.0.111/src/path.rs:715-719`](../../../../.source_1765521767/syn-2.0.111/src/path.rs#L715-L719)*

#### Trait Implementations

##### `impl Any for PathStyle`

- <span id="pathstyle-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PathStyle`

- <span id="pathstyle-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PathStyle`

- <span id="pathstyle-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for PathStyle`

- <span id="pathstyle-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PathStyle`

- <span id="pathstyle-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for PathStyle`

##### `impl<T> From for PathStyle`

- <span id="pathstyle-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PathStyle`

- <span id="pathstyle-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for PathStyle`

- <span id="pathstyle-toowned-type-owned"></span>`type Owned = T`

- <span id="pathstyle-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pathstyle-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PathStyle`

- <span id="pathstyle-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pathstyle-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PathStyle`

- <span id="pathstyle-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pathstyle-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `print_path`

```rust
fn print_path(tokens: &mut proc_macro2::TokenStream, path: &crate::path::Path, style: PathStyle)
```

*Defined in [`syn-2.0.111/src/path.rs:736-742`](../../../../.source_1765521767/syn-2.0.111/src/path.rs#L736-L742)*

### `print_path_segment`

```rust
fn print_path_segment(tokens: &mut proc_macro2::TokenStream, segment: &crate::path::PathSegment, style: PathStyle)
```

*Defined in [`syn-2.0.111/src/path.rs:751-754`](../../../../.source_1765521767/syn-2.0.111/src/path.rs#L751-L754)*

### `print_path_arguments`

```rust
fn print_path_arguments(tokens: &mut proc_macro2::TokenStream, arguments: &crate::path::PathArguments, style: PathStyle)
```

*Defined in [`syn-2.0.111/src/path.rs:763-773`](../../../../.source_1765521767/syn-2.0.111/src/path.rs#L763-L773)*

### `print_angle_bracketed_generic_arguments`

```rust
fn print_angle_bracketed_generic_arguments(tokens: &mut proc_macro2::TokenStream, arguments: &crate::path::AngleBracketedGenericArguments, style: PathStyle)
```

*Defined in [`syn-2.0.111/src/path.rs:799-845`](../../../../.source_1765521767/syn-2.0.111/src/path.rs#L799-L845)*

### `print_parenthesized_generic_arguments`

```rust
fn print_parenthesized_generic_arguments(tokens: &mut proc_macro2::TokenStream, arguments: &crate::path::ParenthesizedGenericArguments, style: PathStyle)
```

*Defined in [`syn-2.0.111/src/path.rs:884-898`](../../../../.source_1765521767/syn-2.0.111/src/path.rs#L884-L898)*

### `print_qpath`

```rust
fn print_qpath(tokens: &mut proc_macro2::TokenStream, qself: &Option<crate::path::QSelf>, path: &crate::path::Path, style: PathStyle)
```

*Defined in [`syn-2.0.111/src/path.rs:900-936`](../../../../.source_1765521767/syn-2.0.111/src/path.rs#L900-L936)*

### `conditionally_print_turbofish`

```rust
fn conditionally_print_turbofish(tokens: &mut proc_macro2::TokenStream, colon2_token: &Option<token::PathSep>, style: PathStyle)
```

*Defined in [`syn-2.0.111/src/path.rs:938-948`](../../../../.source_1765521767/syn-2.0.111/src/path.rs#L938-L948)*

