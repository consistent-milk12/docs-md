*[libc](../index.md) / [macros](index.md)*

---

# Module `macros`

## Contents

- [Macros](#macros)
  - [`cfg_if!`](#cfg_if)
  - [`prelude!`](#prelude)
  - [`s!`](#s)
  - [`s_paren!`](#s_paren)
  - [`s_no_extra_traits!`](#s_no_extra_traits)
  - [`extern_ty!`](#extern_ty)
  - [`e!`](#e)
  - [`c_enum!`](#c_enum)
  - [`f!`](#f)
  - [`safe_f!`](#safe_f)
  - [`__item!`](#__item)
  - [`deprecated_mach!`](#deprecated_mach)
  - [`offset_of!`](#offset_of)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`cfg_if!`](#cfg_if) | macro | A macro for defining #[cfg] if-else statements. |
| [`prelude!`](#prelude) | macro | Create an internal crate prelude with `core` reexports and common types. |
| [`s!`](#s) | macro | Implement `Clone` and `Copy` for a struct, as well as `Debug`, `Eq`, `Hash`, and |
| [`s_paren!`](#s_paren) | macro | Implement `Clone` and `Copy` for a tuple struct, as well as `Debug`, `Eq`, `Hash` |
| [`s_no_extra_traits!`](#s_no_extra_traits) | macro | Implement `Clone`, `Copy`, and `Debug` since those can be derived, but exclude `PartialEq` |
| [`extern_ty!`](#extern_ty) | macro | Create an uninhabited type that can't be constructed. |
| [`e!`](#e) | macro | Implement `Clone` and `Copy` for an enum, as well as `Debug`, `Eq`, `Hash`, and |
| [`c_enum!`](#c_enum) | macro | Represent a C enum as Rust constants and a type. |
| [`f!`](#f) | macro | Define a `unsafe` function. |
| [`safe_f!`](#safe_f) | macro | Define a safe function. |
| [`__item!`](#__item) | macro |  |
| [`deprecated_mach!`](#deprecated_mach) | macro |  |
| [`offset_of!`](#offset_of) | macro | Polyfill for std's `offset_of`. |

## Macros

### `cfg_if!`

A macro for defining #[`cfg`](../../rustversion/expand/index.md) if-else statements.

This is similar to the `if/elif` C preprocessor macro by allowing definition
of a cascade of `#[cfg]` cases, emitting the implementation which matches
first.

This allows you to conveniently provide a long list #[`cfg`](../../rustversion/expand/index.md)'d blocks of code
without having to rewrite each clause multiple times.

### `prelude!`

Create an internal crate prelude with `core` reexports and common types.

### `s!`

Implement `Clone` and `Copy` for a struct, as well as `Debug`, `Eq`, `Hash`, and
`PartialEq` if the `extra_traits` feature is enabled.

Use [`s_no_extra_traits`](#s-no-extra-traits) for structs where the `extra_traits` feature does not
make sense, and for unions.

### `s_paren!`

Implement `Clone` and `Copy` for a tuple struct, as well as `Debug`, `Eq`, `Hash`,
and `PartialEq` if the `extra_traits` feature is enabled.

This is the same as [`s`](#s) but works for tuple structs.

### `s_no_extra_traits!`

Implement `Clone`, `Copy`, and `Debug` since those can be derived, but exclude `PartialEq`,
`Eq`, and `Hash`.

Most items will prefer to use [`s`](#s).

### `extern_ty!`

Create an uninhabited type that can't be constructed. It implements `Debug`, `Clone`,
and `Copy`, but these aren't meaningful for extern types so they should eventually
be removed.

Really what we want here is something that also can't be named without indirection (in
ADTs or function signatures), but this doesn't exist.

### `e!`

Implement `Clone` and `Copy` for an enum, as well as `Debug`, `Eq`, `Hash`, and
`PartialEq` if the `extra_traits` feature is enabled.

### `c_enum!`

Represent a C enum as Rust constants and a type.

C enums can't soundly be mapped to Rust enums since C enums are allowed to have duplicates or
unlisted values, but this is UB in Rust. This enum doesn't implement any traits, its main
purpose is to calculate the correct enum values.

Use the magic name `#anon` if the C enum doesn't create a type.

See <https://github.com/rust-lang/libc/issues/4419> for more.

### `f!`

Define a `unsafe` function.

### `safe_f!`

Define a safe function.

### `__item!`

### `deprecated_mach!`

### `offset_of!`

Polyfill for std's `offset_of`.

