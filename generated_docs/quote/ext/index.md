*[quote](../index.md) / [ext](index.md)*

---

# Module `ext`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private`](#private) | mod |  |
| [`TokenStreamExt`](#tokenstreamext) | trait | TokenStream extension trait with methods for appending tokens. |

## Modules

- [`private`](private/index.md)

## Traits

### `TokenStreamExt`

```rust
trait TokenStreamExt: private::Sealed { ... }
```

*Defined in [`quote-1.0.42/src/ext.rs:8-57`](../../../.source_1765900590/quote-1.0.42/src/ext.rs#L8-L57)*

TokenStream extension trait with methods for appending tokens.

This trait is sealed and cannot be implemented outside of the `quote` crate.

#### Required Methods

- `fn TokenStreamExt::append<U>(&mut self, token: U)`

  For use by `ToTokens` implementations.
  
  Appends the token specified to this list of tokens.

- `fn TokenStreamExt::append_all<I>(&mut self, iter: I)`

  For use by `ToTokens` implementations.
  
  ```rust
  use quote::{quote, TokenStreamExt, ToTokens};
  use proc_macro2::TokenStream;
  
  struct X;
  
  impl ToTokens for X {
      fn to_tokens(&self, tokens: &mut TokenStream) {
          tokens.append_all(&[true, false]);
      }
  }
  
  let tokens = quote!(#X);
  assert_eq!(tokens.to_string(), "true false");
  ```

- `fn TokenStreamExt::append_separated<I, U>(&mut self, iter: I, op: U)`

  For use by `ToTokens` implementations.
  
  Appends all of the items in the iterator `I`, separated by the tokens
  `U`.

- `fn TokenStreamExt::append_terminated<I, U>(&mut self, iter: I, term: U)`

  For use by `ToTokens` implementations.
  
  Appends all tokens in the iterator `I`, appending `U` after each
  element, including after the last element of the iterator.

#### Implementors

- `proc_macro2::TokenStream`

