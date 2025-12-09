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

*Defined in [`syn-2.0.111/src/path.rs:715-719`](../../../../.source_1765210505/syn-2.0.111/src/path.rs#L715-L719)*

#### Trait Implementations

##### `impl Clone for PathStyle`

- <span id="pathstyle-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for PathStyle`

## Functions

### `print_path`

```rust
fn print_path(tokens: &mut proc_macro2::TokenStream, path: &crate::path::Path, style: PathStyle)
```

*Defined in [`syn-2.0.111/src/path.rs:736-742`](../../../../.source_1765210505/syn-2.0.111/src/path.rs#L736-L742)*

### `print_path_segment`

```rust
fn print_path_segment(tokens: &mut proc_macro2::TokenStream, segment: &crate::path::PathSegment, style: PathStyle)
```

*Defined in [`syn-2.0.111/src/path.rs:751-754`](../../../../.source_1765210505/syn-2.0.111/src/path.rs#L751-L754)*

### `print_path_arguments`

```rust
fn print_path_arguments(tokens: &mut proc_macro2::TokenStream, arguments: &crate::path::PathArguments, style: PathStyle)
```

*Defined in [`syn-2.0.111/src/path.rs:763-773`](../../../../.source_1765210505/syn-2.0.111/src/path.rs#L763-L773)*

### `print_angle_bracketed_generic_arguments`

```rust
fn print_angle_bracketed_generic_arguments(tokens: &mut proc_macro2::TokenStream, arguments: &crate::path::AngleBracketedGenericArguments, style: PathStyle)
```

*Defined in [`syn-2.0.111/src/path.rs:799-845`](../../../../.source_1765210505/syn-2.0.111/src/path.rs#L799-L845)*

### `print_parenthesized_generic_arguments`

```rust
fn print_parenthesized_generic_arguments(tokens: &mut proc_macro2::TokenStream, arguments: &crate::path::ParenthesizedGenericArguments, style: PathStyle)
```

*Defined in [`syn-2.0.111/src/path.rs:884-898`](../../../../.source_1765210505/syn-2.0.111/src/path.rs#L884-L898)*

### `print_qpath`

```rust
fn print_qpath(tokens: &mut proc_macro2::TokenStream, qself: &Option<crate::path::QSelf>, path: &crate::path::Path, style: PathStyle)
```

*Defined in [`syn-2.0.111/src/path.rs:900-936`](../../../../.source_1765210505/syn-2.0.111/src/path.rs#L900-L936)*

### `conditionally_print_turbofish`

```rust
fn conditionally_print_turbofish(tokens: &mut proc_macro2::TokenStream, colon2_token: &Option<token::PathSep>, style: PathStyle)
```

*Defined in [`syn-2.0.111/src/path.rs:938-948`](../../../../.source_1765210505/syn-2.0.111/src/path.rs#L938-L948)*

