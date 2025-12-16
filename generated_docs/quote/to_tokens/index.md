*[quote](../index.md) / [to_tokens](index.md)*

---

# Module `to_tokens`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ToTokens`](#totokens) | trait | Types that can be interpolated inside a `quote!` invocation. |

## Traits

### `ToTokens`

```rust
trait ToTokens { ... }
```

*Defined in [`quote-1.0.42/src/to_tokens.rs:9-72`](../../../.source_1765894658/quote-1.0.42/src/to_tokens.rs#L9-L72)*

Types that can be interpolated inside a `quote!` invocation.

#### Required Methods

- `fn to_tokens(&self, tokens: &mut TokenStream)`

  Write `self` to the given `TokenStream`.
  
  The token append methods provided by the [`TokenStreamExt`](../ext/index.md) extension
  trait may be useful for implementing `ToTokens`.
  
  # Example
  
  Example implementation for a struct representing Rust paths like
  `std::cmp::PartialEq`:
  
  ```rust
  use proc_macro2::{TokenTree, Spacing, Span, Punct, TokenStream};
  use quote::{TokenStreamExt, ToTokens};
  
  pub struct Path {
      pub global: bool,
      pub segments: Vec<PathSegment>,
  }
  
  impl ToTokens for Path {
      fn to_tokens(&self, tokens: &mut TokenStream) {
          for (i, segment) in self.segments.iter().enumerate() {
              if i > 0 || self.global {
                  // Double colon `::`
                  tokens.append(Punct::new(':', Spacing::Joint));
                  tokens.append(Punct::new(':', Spacing::Alone));
              }
              segment.to_tokens(tokens);
          }
      }
  }
  
  pub struct PathSegment;
  
  impl ToTokens for PathSegment {
      fn to_tokens(&self, tokens: &mut TokenStream) {
          unimplemented!()
      }
  }
  ```

#### Provided Methods

- `fn to_token_stream(&self) -> TokenStream`

  Convert `self` directly into a `TokenStream` object.
  
  This method is implicitly implemented using `to_tokens`, and acts as a
  convenience method for consumers of the `ToTokens` trait.

- `fn into_token_stream(self) -> TokenStream`

  Convert `self` directly into a `TokenStream` object.
  
  This method is implicitly implemented using `to_tokens`, and acts as a
  convenience method for consumers of the `ToTokens` trait.

#### Implementors

- `&T`
- `&mut T`
- `Box<T>`
- `Option<T>`
- `String`
- `alloc::borrow::Cow<'a, T>`
- `alloc::rc::Rc<T>`
- `bool`
- `char`
- `f32`
- `f64`
- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `proc_macro2::Group`
- `proc_macro2::Ident`
- `proc_macro2::Literal`
- `proc_macro2::Punct`
- `proc_macro2::TokenStream`
- `proc_macro2::TokenTree`
- `std::ffi::CStr`
- `std::ffi::CString`
- `str`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

