# Crate `thiserror_impl`

## Contents

- [Modules](#modules)
  - [`ast`](#ast)
  - [`attr`](#attr)
  - [`expand`](#expand)
  - [`fallback`](#fallback)
  - [`fmt`](#fmt)
  - [`generics`](#generics)
  - [`prop`](#prop)
  - [`scan_expr`](#scan-expr)
  - [`unraw`](#unraw)
  - [`valid`](#valid)
- [Structs](#structs)
  - [`private`](#private)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ast`](#ast) | mod |  |
| [`attr`](#attr) | mod |  |
| [`expand`](#expand) | mod |  |
| [`fallback`](#fallback) | mod |  |
| [`fmt`](#fmt) | mod |  |
| [`generics`](#generics) | mod |  |
| [`prop`](#prop) | mod |  |
| [`scan_expr`](#scan-expr) | mod |  |
| [`unraw`](#unraw) | mod |  |
| [`valid`](#valid) | mod |  |
| [`private`](#private) | struct |  |

## Modules

- [`ast`](ast/index.md)
- [`attr`](attr/index.md)
- [`expand`](expand/index.md)
- [`fallback`](fallback/index.md)
- [`fmt`](fmt/index.md)
- [`generics`](generics/index.md)
- [`prop`](prop/index.md)
- [`scan_expr`](scan_expr/index.md)
- [`unraw`](unraw/index.md)
- [`valid`](valid/index.md)

## Structs

### `private`

```rust
struct private;
```

*Defined in [`thiserror-impl-2.0.17/src/lib.rs:45`](../../.source_1765521767/thiserror-impl-2.0.17/src/lib.rs#L45)*

#### Trait Implementations

##### `impl Any for private`

- <span id="private-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for private`

- <span id="private-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for private`

- <span id="private-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for private`

- <span id="private-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for private`

- <span id="private-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Spanned for private`

- <span id="private-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for private`

- <span id="private-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream)`

##### `impl<U> TryFrom for private`

- <span id="private-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="private-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for private`

- <span id="private-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="private-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

