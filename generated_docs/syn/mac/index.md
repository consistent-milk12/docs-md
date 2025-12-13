*[syn](../index.md) / [mac](index.md)*

---

# Module `mac`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`Macro`](#macro) | struct | A macro invocation: `println!("{}", mac)`. |
| [`MacroDelimiter`](#macrodelimiter) | enum | A grouping token that surrounds a macro body: `m!(...)` or `m!{...}` or `m![...]`. |
| [`parse_delimiter`](#parse-delimiter) | fn |  |

## Modules

- [`parsing`](parsing/index.md)
- [`printing`](printing/index.md)

## Structs

### `Macro`

```rust
struct Macro {
    pub path: crate::path::Path,
    pub bang_token: token::Not,
    pub delimiter: MacroDelimiter,
    pub tokens: proc_macro2::TokenStream,
}
```

*Defined in [`syn-2.0.111/src/mac.rs:14-23`](../../../.source_1765633015/syn-2.0.111/src/mac.rs#L14-L23)*

A macro invocation: `println!("{}", mac)`.

#### Implementations

- <span id="macro-parse-body"></span>`fn parse_body<T: Parse>(&self) -> Result<T>` — [`Result`](../error/index.md#result)

  Parse the tokens within the macro invocation's delimiters into a syntax

  tree.

  

  This is equivalent to `syn::parse2::<T>(mac.tokens)` except that it

  produces a more useful span when `tokens` is empty.

  

  # Example

  

  ```rust

  use syn::{parse_quote, Expr, ExprLit, Ident, Lit, LitStr, Macro, Token};

  use syn::ext::IdentExt;

  use syn::parse::{Error, Parse, ParseStream, Result};

  use syn::punctuated::Punctuated;

  

  // The arguments expected by libcore's format_args macro, and as a

  // result most other formatting and printing macros like println.

  //

  //     println!("{} is {number:.prec$}", "x", prec=5, number=0.01)

  struct FormatArgs {

      format_string: Expr,

      positional_args: Vec<Expr>,

      named_args: Vec<(Ident, Expr)>,

  }

  

  impl Parse for FormatArgs {

      fn parse(input: ParseStream) -> Result<Self> {

          let format_string: Expr;

          let mut positional_args = Vec::new();

          let mut named_args = Vec::new();

  

          format_string = input.parse()?;

          while !input.is_empty() {

              input.parse::<Token![,]>()?;

              if input.is_empty() {

                  break;

              }

              if input.peek(Ident::peek_any) && input.peek2(Token![=]) {

                  while !input.is_empty() {

                      let name: Ident = input.call(Ident::parse_any)?;

                      input.parse::<Token![=]>()?;

                      let value: Expr = input.parse()?;

                      named_args.push((name, value));

                      if input.is_empty() {

                          break;

                      }

                      input.parse::<Token![,]>()?;

                  }

                  break;

              }

              positional_args.push(input.parse()?);

          }

  

          Ok(FormatArgs {

              format_string,

              positional_args,

              named_args,

          })

      }

  }

  

  // Extract the first argument, the format string literal, from an

  // invocation of a formatting or printing macro.

  fn get_format_string(m: &Macro) -> Result<LitStr> {

      let args: FormatArgs = m.parse_body()?;

      match args.format_string {

          Expr::Lit(ExprLit { lit: Lit::Str(lit), .. }) => Ok(lit),

          other => {

              // First argument was not a string literal expression.

              // Maybe something like: println!(concat!(...), ...)

              Err(Error::new_spanned(other, "format string must be a string literal"))

          }

      }

  }

  

  fn main() {

      let invocation = parse_quote! {

          println!("{:?}", Instant::now())

      };

      let lit = get_format_string(&invocation).unwrap();

      assert_eq!(lit.value(), "{:?}");

  }

  ```

- <span id="macro-parse-body-with"></span>`fn parse_body_with<F: Parser>(&self, parser: F) -> Result<<F as >::Output>` — [`Result`](../error/index.md#result), [`Parser`](../parse/index.md#parser)

  Parse the tokens within the macro invocation's delimiters using the

  given parser.

#### Trait Implementations

##### `impl Any for Macro`

- <span id="macro-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Macro`

- <span id="macro-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Macro`

- <span id="macro-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Macro`

- <span id="cratemacro-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Macro`

- <span id="macro-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Macro`

- <span id="cratemacro-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Macro`

##### `impl<T> From for Macro`

- <span id="macro-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Macro`

- <span id="cratemacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Macro`

- <span id="macro-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::mac::Macro`

- <span id="cratemacmacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::Macro`

- <span id="cratemacro-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Macro`

##### `impl Spanned for Macro`

- <span id="macro-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Macro`

- <span id="macro-toowned-type-owned"></span>`type Owned = T`

- <span id="macro-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="macro-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::mac::Macro`

- <span id="cratemacmacro-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Macro`

- <span id="macro-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="macro-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Macro`

- <span id="macro-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="macro-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `MacroDelimiter`

```rust
enum MacroDelimiter {
    Paren(crate::token::Paren),
    Brace(crate::token::Brace),
    Bracket(crate::token::Bracket),
}
```

*Defined in [`syn-2.0.111/src/mac.rs:25-33`](../../../.source_1765633015/syn-2.0.111/src/mac.rs#L25-L33)*

A grouping token that surrounds a macro body: `m!(...)` or `m!{...}` or `m![...]`.

#### Implementations

- <span id="cratemacmacrodelimiter-surround"></span>`fn surround(&self, tokens: &mut TokenStream, inner: TokenStream)`

#### Trait Implementations

##### `impl Any for MacroDelimiter`

- <span id="macrodelimiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MacroDelimiter`

- <span id="macrodelimiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MacroDelimiter`

- <span id="macrodelimiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::MacroDelimiter`

- <span id="cratemacrodelimiter-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for MacroDelimiter`

- <span id="macrodelimiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::MacroDelimiter`

- <span id="cratemacrodelimiter-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::MacroDelimiter`

##### `impl<T> From for MacroDelimiter`

- <span id="macrodelimiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::MacroDelimiter`

- <span id="cratemacrodelimiter-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for MacroDelimiter`

- <span id="macrodelimiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::MacroDelimiter`

- <span id="cratemacrodelimiter-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl ToOwned for MacroDelimiter`

- <span id="macrodelimiter-toowned-type-owned"></span>`type Owned = T`

- <span id="macrodelimiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="macrodelimiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MacroDelimiter`

- <span id="macrodelimiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="macrodelimiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MacroDelimiter`

- <span id="macrodelimiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="macrodelimiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `parse_delimiter`

```rust
fn parse_delimiter(input: crate::parse::ParseStream<'_>) -> crate::error::Result<(MacroDelimiter, proc_macro2::TokenStream)>
```

*Defined in [`syn-2.0.111/src/mac.rs:153-170`](../../../.source_1765633015/syn-2.0.111/src/mac.rs#L153-L170)*

