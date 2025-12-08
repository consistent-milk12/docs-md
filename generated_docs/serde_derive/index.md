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

- [`internals`](internals/index.md) - 
- [`bound`](bound/index.md) - 
- [`fragment`](fragment/index.md) - 
- [`de`](de/index.md) - 
- [`deprecated`](deprecated/index.md) - 
- [`dummy`](dummy/index.md) - 
- [`pretend`](pretend/index.md) - 
- [`ser`](ser/index.md) - 
- [`this`](this/index.md) - 

## Structs

### `private`

```rust
struct private;
```

#### Implementations

- <span id="private-ident"></span>`fn ident(&self) -> Ident`

#### Trait Implementations

##### `impl<T> Spanned for private`

- <span id="private-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for private`

- <span id="private-to-tokens"></span>`fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream)`

