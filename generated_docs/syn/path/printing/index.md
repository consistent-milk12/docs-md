*[syn](../../index.md) / [path](../index.md) / [printing](index.md)*

---

# Module `printing`

## Contents

- [Enums](#enums)
  - [`PathStyle`](#pathstyle)
- [Functions](#functions)
  - [`print_path`](#print_path)
  - [`print_path_segment`](#print_path_segment)
  - [`print_path_arguments`](#print_path_arguments)
  - [`print_angle_bracketed_generic_arguments`](#print_angle_bracketed_generic_arguments)
  - [`print_parenthesized_generic_arguments`](#print_parenthesized_generic_arguments)
  - [`print_qpath`](#print_qpath)
  - [`conditionally_print_turbofish`](#conditionally_print_turbofish)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PathStyle`](#pathstyle) | enum |  |
| [`print_path`](#print_path) | fn |  |
| [`print_path_segment`](#print_path_segment) | fn |  |
| [`print_path_arguments`](#print_path_arguments) | fn |  |
| [`print_angle_bracketed_generic_arguments`](#print_angle_bracketed_generic_arguments) | fn |  |
| [`print_parenthesized_generic_arguments`](#print_parenthesized_generic_arguments) | fn |  |
| [`print_qpath`](#print_qpath) | fn |  |
| [`conditionally_print_turbofish`](#conditionally_print_turbofish) | fn |  |

## Enums

### `PathStyle`

```rust
enum PathStyle {
    Expr,
    Mod,
    AsWritten,
}
```

#### Trait Implementations

##### `impl Clone for PathStyle`

- <span id="pathstyle-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for PathStyle`

## Functions

### `print_path`

```rust
fn print_path(tokens: &mut proc_macro2::TokenStream, path: &crate::path::Path, style: PathStyle)
```

### `print_path_segment`

```rust
fn print_path_segment(tokens: &mut proc_macro2::TokenStream, segment: &crate::path::PathSegment, style: PathStyle)
```

### `print_path_arguments`

```rust
fn print_path_arguments(tokens: &mut proc_macro2::TokenStream, arguments: &crate::path::PathArguments, style: PathStyle)
```

### `print_angle_bracketed_generic_arguments`

```rust
fn print_angle_bracketed_generic_arguments(tokens: &mut proc_macro2::TokenStream, arguments: &crate::path::AngleBracketedGenericArguments, style: PathStyle)
```

### `print_parenthesized_generic_arguments`

```rust
fn print_parenthesized_generic_arguments(tokens: &mut proc_macro2::TokenStream, arguments: &crate::path::ParenthesizedGenericArguments, style: PathStyle)
```

### `print_qpath`

```rust
fn print_qpath(tokens: &mut proc_macro2::TokenStream, qself: &Option<crate::path::QSelf>, path: &crate::path::Path, style: PathStyle)
```

### `conditionally_print_turbofish`

```rust
fn conditionally_print_turbofish(tokens: &mut proc_macro2::TokenStream, colon2_token: &Option<token::PathSep>, style: PathStyle)
```

