*[serde_derive](../index.md) / [internals](index.md)*

---

# Module `internals`

## Contents

- [Modules](#modules)
  - [`ast`](#ast)
  - [`attr`](#attr)
  - [`name`](#name)
  - [`case`](#case)
  - [`check`](#check)
  - [`ctxt`](#ctxt)
  - [`receiver`](#receiver)
  - [`respan`](#respan)
  - [`symbol`](#symbol)
- [Structs](#structs)
  - [`unnamed`](#unnamed)
- [Enums](#enums)
  - [`Derive`](#derive)
- [Functions](#functions)
  - [`unnamed`](#unnamed)
  - [`ungroup`](#ungroup)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ast`](#ast) | mod | A Serde ast, parsed from the Syn ast and ready to generate Rust code. |
| [`attr`](#attr) | mod |  |
| [`name`](#name) | mod |  |
| [`case`](#case) | mod | Code to convert the Rust-styled field/variant (e.g. `my_field`, `MyType`) to the |
| [`check`](#check) | mod |  |
| [`ctxt`](#ctxt) | mod |  |
| [`receiver`](#receiver) | mod |  |
| [`respan`](#respan) | mod |  |
| [`symbol`](#symbol) | mod |  |
| [`unnamed`](#unnamed) | struct |  |
| [`Derive`](#derive) | enum |  |
| [`unnamed`](#unnamed) | fn |  |
| [`ungroup`](#ungroup) | fn |  |

## Modules

- [`ast`](ast/index.md) - A Serde ast, parsed from the Syn ast and ready to generate Rust code.
- [`attr`](attr/index.md) - 
- [`name`](name/index.md) - 
- [`case`](case/index.md) - Code to convert the Rust-styled field/variant (e.g. `my_field`, `MyType`) to the
- [`check`](check/index.md) - 
- [`ctxt`](ctxt/index.md) - 
- [`receiver`](receiver/index.md) - 
- [`respan`](respan/index.md) - 
- [`symbol`](symbol/index.md) - 

## Structs

### `Ctxt`

```rust
struct Ctxt {
    errors: std::cell::RefCell<Option<Vec<syn::Error>>>,
}
```

A type to collect errors together and format them.

Dropping this object will cause a panic. It must be consumed using `check`.

References can be shared since this type uses run-time exclusive mut checking.

#### Implementations

- <span id="ctxt-new"></span>`fn new() -> Self`

- <span id="ctxt-error-spanned-by"></span>`fn error_spanned_by<A: ToTokens, T: Display>(&self, obj: A, msg: T)`

- <span id="ctxt-syn-error"></span>`fn syn_error(&self, err: syn::Error)`

- <span id="ctxt-check"></span>`fn check(self) -> syn::Result<()>`

#### Trait Implementations

##### `impl Default for Ctxt`

- <span id="ctxt-default"></span>`fn default() -> Ctxt` — [`Ctxt`](#ctxt)

##### `impl Drop for Ctxt`

- <span id="ctxt-drop"></span>`fn drop(&mut self)`

## Enums

### `Derive`

```rust
enum Derive {
    Serialize,
    Deserialize,
}
```

#### Trait Implementations

##### `impl Clone for Derive`

- <span id="derive-clone"></span>`fn clone(&self) -> Derive` — [`Derive`](#derive)

##### `impl Copy for Derive`

## Functions

### `ungroup`

```rust
fn ungroup(ty: &syn::Type) -> &syn::Type
```

