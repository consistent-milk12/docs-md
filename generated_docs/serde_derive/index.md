# Crate `serde_derive`

This crate provides Serde's two derive macros.

```edition2021
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct S;

fn main() {}
```

Please refer to [https://serde.rs/derive.html] for how to set this up.


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

- `fn ident(self: &Self) -> Ident`

#### Trait Implementations

##### `impl<T> Spanned for private`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for private`

- `fn to_tokens(self: &Self, tokens: &mut proc_macro2::TokenStream)`

