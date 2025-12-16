*[syn](../../index.md) / [parse](../index.md) / [discouraged](index.md)*

---

# Module `discouraged`

Extensions to the parsing API with niche applicability.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Speculative`](#speculative) | trait | Extensions to the `ParseStream` API to support speculative parsing. |
| [`AnyDelimiter`](#anydelimiter) | trait | Extensions to the `ParseStream` API to support manipulating invisible delimiters the same as if they were visible. |

## Traits

### `Speculative`

```rust
trait Speculative { ... }
```

*Defined in [`syn-2.0.111/src/discouraged.rs:13-165`](../../../../.source_1765900590/syn-2.0.111/src/discouraged.rs#L13-L165)*

Extensions to the `ParseStream` API to support speculative parsing.

#### Required Methods

- `fn Speculative::advance_to(&self, fork: &Self)`

  Advance this parse stream to the position of a forked parse stream.
  
  This is the opposite operation to `ParseStream::fork`. You can fork a
  parse stream, perform some speculative parsing, then join the original
  stream to the fork to "commit" the parsing from the fork to the main
  stream.
  
  If you can avoid doing this, you should, as it limits the ability to
  generate useful errors. That said, it is often the only way to parse
  syntax of the form `A* B*` for arbitrary syntax `A` and `B`. The problem
  is that when the fork fails to parse an `A`, it's impossible to tell
  whether that was because of a syntax error and the user meant to provide
  an `A`, or that the `A`s are finished and it's time to start parsing
  `B`s. Use with care.
  
  Also note that if `A` is a subset of `B`, `A* B*` can be parsed by
  parsing `B*` and removing the leading members of `A` from the
  repetition, bypassing the need to involve the downsides associated with
  speculative parsing.
  
  ##### Example
  
  There has been chatter about the possibility of making the colons in the
  turbofish syntax like `path::to::<T>` no longer required by accepting
  `path::to<T>` in expression position. Specifically, according to [RFC
  2544], [`PathSegment`](../../path/index.md) parsing should always try to consume a following
  `<` token as the start of generic arguments, and reset to the `<` if
  that fails (e.g. the token is acting as a less-than operator).
  
  This is the exact kind of parsing behavior which requires the "fork,
  try, commit" behavior that `ParseStream::fork` discourages. With
  `advance_to`, we can avoid having to parse the speculatively parsed
  content a second time.
  
  This change in behavior can be implemented in syn by replacing just the
  `Parse` implementation for `PathSegment`:
  
  ```rust
  use syn::ext::IdentExt;
  use syn::parse::discouraged::Speculative;
  use syn::parse::{Parse, ParseStream};
  use syn::{Ident, PathArguments, Result, Token};
  
  pub struct PathSegment {
      pub ident: Ident,
      pub arguments: PathArguments,
  }
  
  impl<T> From<T> for PathSegment
  where
      T: Into<Ident>,
  {
      fn from(ident: T) -> Self {
          PathSegment {
              ident: ident.into(),
              arguments: PathArguments::None,
          }
      }
  }
  
  impl Parse for PathSegment {
      fn parse(input: ParseStream) -> Result<Self> {
          if input.peek(Token![super])
              || input.peek(Token![self])
              || input.peek(Token![Self])
              || input.peek(Token![crate])
          {
              let ident = input.call(Ident::parse_any)?;
              return Ok(PathSegment::from(ident));
          }
  
          let ident = input.parse()?;
          if input.peek(Token![::]) && input.peek3(Token![<]) {
              return Ok(PathSegment {
                  ident,
                  arguments: PathArguments::AngleBracketed(input.parse()?),
              });
          }
          if input.peek(Token![<]) && !input.peek(Token![<=]) {
              let fork = input.fork();
              if let Ok(arguments) = fork.parse() {
                  input.advance_to(&fork);
                  return Ok(PathSegment {
                      ident,
                      arguments: PathArguments::AngleBracketed(arguments),
                  });
              }
          }
          Ok(PathSegment::from(ident))
      }
  }
  
  syn::parse_str::<PathSegment>("a<b,c>").unwrap();
  ```
  
  ##### Drawbacks
  
  The main drawback of this style of speculative parsing is in error
  presentation. Even if the lookahead is the "correct" parse, the error
  that is shown is that of the "fallback" parse. To use the same example
  as the turbofish above, take the following unfinished "turbofish":
  
  ```text
  let _ = f<&'a fn(), for<'a> serde::>();
  ```
  
  If this is parsed as generic arguments, we can provide the error message
  
  ```text
  error: expected identifier
   --> src.rs:L:C
    |
  L | let _ = f<&'a fn(), for<'a> serde::>();
    |                                    ^
  ```
  
  but if parsed using the above speculative parsing, it falls back to
  assuming that the `<` is a less-than when it fails to parse the generic
  arguments, and tries to interpret the `&'a` as the start of a labelled
  loop, resulting in the much less helpful error
  
  ```text
  error: expected `:`
   --> src.rs:L:C
    |
  L | let _ = f<&'a fn(), for<'a> serde::>();
    |               ^^
  ```
  
  This can be mitigated with various heuristics (two examples: show both
  forks' parse errors, or show the one that consumed more tokens), but
  when you can control the grammar, sticking to something that can be
  parsed LL(3) and without the LL(*) speculative parsing this makes
  possible, displaying reasonable errors becomes much more simple.
  
  
  ##### Performance
  
  This method performs a cheap fixed amount of work that does not depend
  on how far apart the two streams are positioned.
  
  ##### Panics
  
  The forked stream in the argument of `advance_to` must have been
  obtained by forking `self`. Attempting to advance to any other stream
  will cause a panic.

#### Implementors

- [`ParseBuffer`](../index.md#parsebuffer)

### `AnyDelimiter`

```rust
trait AnyDelimiter { ... }
```

*Defined in [`syn-2.0.111/src/discouraged.rs:205-209`](../../../../.source_1765900590/syn-2.0.111/src/discouraged.rs#L205-L209)*

Extensions to the `ParseStream` API to support manipulating invisible
delimiters the same as if they were visible.

#### Required Methods

- `fn AnyDelimiter::parse_any_delimiter(&self) -> Result<(Delimiter, DelimSpan, ParseBuffer<'_>)>`

  Returns the delimiter, the span of the delimiter token, and the nested
  contents for further parsing.

#### Implementors

- [`ParseBuffer`](../index.md#parsebuffer)

