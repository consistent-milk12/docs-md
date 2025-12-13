# Crate `serde_derive`

This crate provides Serde's two derive macros.

```edition2021
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct S;

fn main() {}
```

Please refer to [https://serde.rs/derive.html] for how to set this up.


## Contents

- [Modules](#modules)
  - [`internals`](#internals)
  - [`bound`](#bound)
  - [`fragment`](#fragment)
  - [`de`](#de)
  - [`deprecated`](#deprecated)
  - [`dummy`](#dummy)
  - [`pretend`](#pretend)
  - [`ser`](#ser)
  - [`this`](#this)
- [Structs](#structs)
  - [`private`](#private)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`internals`](#internals) | mod |  |
| [`bound`](#bound) | mod |  |
| [`fragment`](#fragment) | mod |  |
| [`de`](#de) | mod |  |
| [`deprecated`](#deprecated) | mod |  |
| [`dummy`](#dummy) | mod |  |
| [`pretend`](#pretend) | mod |  |
| [`ser`](#ser) | mod |  |
| [`this`](#this) | mod |  |
| [`private`](#private) | struct |  |

## Modules

- [`internals`](internals/index.md)
- [`bound`](bound/index.md)
- [`fragment`](fragment/index.md)
- [`de`](de/index.md)
- [`deprecated`](deprecated/index.md)
- [`dummy`](dummy/index.md)
- [`pretend`](pretend/index.md)
- [`ser`](ser/index.md)
- [`this`](this/index.md)

## Structs

### `private`

```rust
struct private;
```

*Defined in [`serde_derive-1.0.228/src/lib.rs:96`](../../.source_1765633015/serde_derive-1.0.228/src/lib.rs#L96)*

#### Implementations

- <span id="private-ident"></span>`fn ident(&self) -> Ident`

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

