*[syn](../index.md) / [parse](index.md)*

---

# Module `parse`

Parsing interface for parsing a token stream into a syntax tree node.

Parsing in Syn is built on parser functions that take in a [`ParseStream`](#parsestream)
and produce a `Result<T>` where `T` is some syntax tree node. Underlying
these parser functions is a lower level mechanism built around the
[`Cursor`](../buffer/index.md) type. `Cursor` is a cheaply copyable cursor over a range of
tokens in a token stream.


# Example

Here is a snippet of parsing code to get a feel for the style of the
library. We define data structures for a subset of Rust syntax including
enums (not shown) and structs, then provide implementations of the [`Parse`](#parse)
trait to parse these syntax tree data structures from a token stream.

Once `Parse` impls have been defined, they can be called conveniently from a
procedural macro through `parse_macro_input!` as shown at the bottom of
the snippet. If the caller provides syntactically invalid input to the
procedural macro, they will receive a helpful compiler error message
pointing out the exact token that triggered the failure to parse.

```rust
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{braced, parse_macro_input, token, Field, Ident, Result, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

enum Item {
    Struct(ItemStruct),
    Enum(ItemEnum),
}

struct ItemStruct {
    struct_token: Token![struct],
    ident: Ident,
    brace_token: token::Brace,
    fields: Punctuated<Field, Token![,]>,
}

enum ItemEnum {}

impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![struct]) {
            input.parse().map(Item::Struct)
        } else if lookahead.peek(Token![enum]) {
            input.parse().map(Item::Enum)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for ItemStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(ItemStruct {
            struct_token: input.parse()?,
            ident: input.parse()?,
            brace_token: braced!(content in input),
            fields: content.parse_terminated(Field::parse_named, Token![,])?,
        })
    }
}

impl Parse for ItemEnum {
    fn parse(input: ParseStream) -> Result<Self> {
        unimplemented!()
    }
}

const IGNORE: &str = stringify! {
#[proc_macro]
};
pub fn my_macro(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as Item);

    /* ... */
  TokenStream::new()
}
```

# The `syn::parse*` functions

The `syn::parse`, `syn::parse2`, and `syn::parse_str` functions serve
as an entry point for parsing syntax tree nodes that can be parsed in an
obvious default way. These functions can return any syntax tree node that
implements the [`Parse`](#parse) trait, which includes most types in Syn.



```rust
use syn::Type;

fn run_parser() -> syn::Result<()> {
let t: Type = syn::parse_str("std::collections::HashMap<String, Value>")?;
    Ok(())
}

run_parser().unwrap();
```

The `parse_quote!` macro also uses this approach.

# The `Parser` trait

Some types can be parsed in several ways depending on context. For example
an [`Attribute`](../attr/index.md) can be either "outer" like `#[...]` or "inner" like
`#![...]` and parsing the wrong one would be a bug. Similarly [`Punctuated`](../index.md)
may or may not allow trailing punctuation, and parsing it the wrong way
would either reject valid input or accept invalid input.


The `Parse` trait is not implemented in these cases because there is no good
behavior to consider the default.

```compile_fail
extern crate proc_macro;

use syn::punctuated::Punctuated;
use syn::{PathSegment, Result, Token};

fn f(tokens: proc_macro::TokenStream) -> Result<()> {

// Can't parse `Punctuated` without knowing whether trailing punctuation
// should be allowed in this context.
let path: Punctuated<PathSegment, Token![::]> = syn::parse(tokens)?;

    Ok(())
}
```

In these cases the types provide a choice of parser functions rather than a
single `Parse` implementation, and those parser functions can be invoked
through the [`Parser`](#parser) trait.


```rust
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{Attribute, Expr, PathSegment, Result, Token};

fn call_some_parser_methods(input: TokenStream) -> Result<()> {
    // Parse a nonempty sequence of path segments separated by `::` punctuation
    // with no trailing punctuation.
    let tokens = input.clone();
    let parser = Punctuated::<PathSegment, Token![::]>::parse_separated_nonempty;
    let _path = parser.parse(tokens)?;

    // Parse a possibly empty sequence of expressions terminated by commas with
    // an optional trailing punctuation.
    let tokens = input.clone();
    let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
    let _args = parser.parse(tokens)?;

    // Parse zero or more outer attributes but not inner attributes.
    let tokens = input.clone();
    let parser = Attribute::parse_outer;
    let _attrs = parser.parse(tokens)?;

    Ok(())
}
```

## Contents

- [Modules](#modules)
  - [`discouraged`](#discouraged)
- [Structs](#structs)
  - [`Error`](#error)
  - [`End`](#end)
  - [`Lookahead1`](#lookahead1)
  - [`ParseBuffer`](#parsebuffer)
  - [`StepCursor`](#stepcursor)
  - [`Nothing`](#nothing)
- [Enums](#enums)
  - [`Unexpected`](#unexpected)
- [Traits](#traits)
  - [`Peek`](#peek)
  - [`Parse`](#parse)
  - [`Parser`](#parser)
- [Functions](#functions)
  - [`advance_step_cursor`](#advance-step-cursor)
  - [`new_parse_buffer`](#new-parse-buffer)
  - [`cell_clone`](#cell-clone)
  - [`inner_unexpected`](#inner-unexpected)
  - [`get_unexpected`](#get-unexpected)
  - [`span_of_unexpected_ignoring_nones`](#span-of-unexpected-ignoring-nones)
  - [`tokens_to_parse_buffer`](#tokens-to-parse-buffer)
  - [`parse_scoped`](#parse-scoped)
  - [`err_unexpected_token`](#err-unexpected-token)
- [Type Aliases](#type-aliases)
  - [`Result`](#result)
  - [`ParseStream`](#parsestream)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`discouraged`](#discouraged) | mod | Extensions to the parsing API with niche applicability. |
| [`Error`](#error) | struct |  |
| [`End`](#end) | struct |  |
| [`Lookahead1`](#lookahead1) | struct |  |
| [`ParseBuffer`](#parsebuffer) | struct | Cursor position within a buffered token stream. |
| [`StepCursor`](#stepcursor) | struct | Cursor state associated with speculative parsing. |
| [`Nothing`](#nothing) | struct | An empty syntax tree node that consumes no tokens when parsed. |
| [`Unexpected`](#unexpected) | enum |  |
| [`Peek`](#peek) | trait |  |
| [`Parse`](#parse) | trait | Parsing interface implemented by all types that can be parsed in a default way from a token stream. |
| [`Parser`](#parser) | trait | Parser that can parse Rust tokens into a particular syntax tree node. |
| [`advance_step_cursor`](#advance-step-cursor) | fn |  |
| [`new_parse_buffer`](#new-parse-buffer) | fn |  |
| [`cell_clone`](#cell-clone) | fn |  |
| [`inner_unexpected`](#inner-unexpected) | fn |  |
| [`get_unexpected`](#get-unexpected) | fn |  |
| [`span_of_unexpected_ignoring_nones`](#span-of-unexpected-ignoring-nones) | fn |  |
| [`tokens_to_parse_buffer`](#tokens-to-parse-buffer) | fn |  |
| [`parse_scoped`](#parse-scoped) | fn |  |
| [`err_unexpected_token`](#err-unexpected-token) | fn |  |
| [`Result`](#result) | type |  |
| [`ParseStream`](#parsestream) | type | Input to a Syn parser function. |

## Modules

- [`discouraged`](discouraged/index.md) — Extensions to the parsing API with niche applicability.

## Structs

### `Error`

```rust
struct Error {
    messages: Vec<ErrorMessage>,
}
```

*Defined in [`syn-2.0.111/src/error.rs:101-103`](../../../.source_1765521767/syn-2.0.111/src/error.rs#L101-L103)*

Error returned when a Syn parser cannot parse the input tokens.

# Error reporting in proc macros

The correct way to report errors back to the compiler from a procedural
macro is by emitting an appropriately spanned invocation of
`compile_error!` in the generated code. This produces a better diagnostic
message than simply panicking the macro.

When parsing macro input, the `parse_macro_input!` macro handles the
conversion to `compile_error!` automatically.

```rust
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, ItemFn};

const IGNORE: &str = stringify! {
#[proc_macro_attribute]
};
pub fn my_attr(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as MyAttrArgs);
    let input = parse_macro_input!(input as ItemFn);

    /* ... */
    TokenStream::new()
}

struct MyAttrArgs {
    _k: [(); { stringify! {
    ...
    }; 0 }]
}

impl Parse for MyAttrArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        stringify! {
        ...
        };
        unimplemented!()
    }
}
```

For errors that arise later than the initial parsing stage, the
`.to_compile_error()` or `.into_compile_error()` methods can be used to
perform an explicit conversion to `compile_error!`.


```rust
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

const IGNORE: &str = stringify! {
#[proc_macro_derive(MyDerive)]
};
pub fn my_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // fn(DeriveInput) -> syn::Result<proc_macro2::TokenStream>
    expand::my_derive(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

mod expand {
    use proc_macro2::TokenStream;
    use syn::{DeriveInput, Result};

    pub fn my_derive(input: DeriveInput) -> Result<TokenStream> {
        unimplemented!()
    }
}
```

#### Implementations

- <span id="error-new"></span>`fn new<T: Display>(span: Span, message: T) -> Self`

  Usually the `ParseStream::error` method will be used instead, which

  automatically uses the correct span from the current position of the

  parse stream.

  

  Use `Error::new` when the error needs to be triggered on some span other

  than where the parse stream is currently positioned.

  

  # Example

  

  ```rust

  use syn::{Error, Ident, LitStr, Result, Token};

  use syn::parse::ParseStream;

  

  // Parses input that looks like `name = "string"` where the key must be

  // the identifier `name` and the value may be any string literal.

  // Returns the string literal.

  fn parse_name(input: ParseStream) -> Result<LitStr> {

      let name_token: Ident = input.parse()?;

      if name_token != "name" {

          // Trigger an error not on the current position of the stream,

          // but on the position of the unexpected identifier.

          return Err(Error::new(name_token.span(), "expected `name`"));

      }

      input.parse::<Token![=]>()?;

      let s: LitStr = input.parse()?;

      Ok(s)

  }

  ```

- <span id="error-new-spanned"></span>`fn new_spanned<T: ToTokens, U: Display>(tokens: T, message: U) -> Self`

  Creates an error with the specified message spanning the given syntax

  tree node.

  

  Unlike the `Error::new` constructor, this constructor takes an argument

  `tokens` which is a syntax tree node. This allows the resulting `Error`

  to attempt to span all tokens inside of `tokens`. While you would

  typically be able to use the `Spanned` trait with the above `Error::new`

  constructor, implementation limitations today mean that

  `Error::new_spanned` may provide a higher-quality error message on

  stable Rust.

  

  When in doubt it's recommended to stick to `Error::new` (or

  `ParseStream::error`)!

- <span id="error-span"></span>`fn span(&self) -> Span`

  The source location of the error.

  

  Spans are not thread-safe so this function returns `Span::call_site()`

  if called from a different thread than the one on which the `Error` was

  originally created.

- <span id="error-to-compile-error"></span>`fn to_compile_error(&self) -> TokenStream`

  Render the error as an invocation of `compile_error!`.

  

  The `parse_macro_input!` macro provides a convenient way to invoke

  this method correctly in a procedural macro.

  

- <span id="error-into-compile-error"></span>`fn into_compile_error(self) -> TokenStream`

  Render the error as an invocation of `compile_error!`.

  

  # Example

  

  ```rust

  extern crate proc_macro;

  

  use proc_macro::TokenStream;

  use syn::{parse_macro_input, DeriveInput, Error};

  

  const _: &str = stringify! {

  #[proc_macro_derive(MyTrait)]

  };

  pub fn derive_my_trait(input: TokenStream) -> TokenStream {

      let input = parse_macro_input!(input as DeriveInput);

      my_trait::expand(input)

          .unwrap_or_else(Error::into_compile_error)

          .into()

  }

  

  mod my_trait {

      use proc_macro2::TokenStream;

      use syn::{DeriveInput, Result};

  

      pub(crate) fn expand(input: DeriveInput) -> Result<TokenStream> {

          /* ... */

          unimplemented!()

      }

  }

  ```

- <span id="error-combine"></span>`fn combine(&mut self, another: Error)` — [`Error`](../error/index.md#error)

  Add another error message to self such that when `to_compile_error()` is

  called, both errors will be emitted together.

#### Trait Implementations

##### `impl Any for Error`

- <span id="error-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Error`

- <span id="error-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Error`

- <span id="error-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Error`

- <span id="error-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for Error`

##### `impl Extend for Error`

- <span id="error-extend"></span>`fn extend<T: IntoIterator<Item = Error>>(&mut self, iter: T)`

##### `impl<T> From for Error`

- <span id="error-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Error`

- <span id="error-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Error`

- <span id="error-intoiterator-type-item"></span>`type Item = Error`

- <span id="error-intoiterator-type-intoiter"></span>`type IntoIter = IntoIter`

- <span id="error-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl ToOwned for Error`

- <span id="error-toowned-type-owned"></span>`type Owned = T`

- <span id="error-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="error-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Error`

- <span id="error-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Error`

- <span id="error-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="error-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Error`

- <span id="error-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="error-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `End`

```rust
struct End;
```

*Defined in [`syn-2.0.111/src/lookahead.rs:310`](../../../.source_1765521767/syn-2.0.111/src/lookahead.rs#L310)*

Pseudo-token used for peeking the end of a parse stream.

This type is only useful as an argument to one of the following functions:

- `ParseStream::peek`
- `ParseStream::peek2`
- `ParseStream::peek3`
- `Lookahead1::peek`

The peek will return `true` if there are no remaining tokens after that
point in the parse stream.

# Example

Suppose we are parsing attributes containing core::fmt inspired formatting
arguments:

- `#[fmt("simple example")]`
- `#[fmt("interpolation e{}ample", self.x)]`
- `#[fmt("interpolation e{x}ample")]`

and we want to recognize the cases where no interpolation occurs so that
more efficient code can be generated.

The following implementation uses `input.peek(Token![,]) &&
input.peek2(End)` to recognize the case of a trailing comma without
consuming the comma from the parse stream, because if it isn't a trailing
comma, that same comma needs to be parsed as part of `args`.

```rust
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{End, Parse, ParseStream, Result};
use syn::{parse_quote, Attribute, LitStr, Token};

struct FormatArgs {
    template: LitStr,  // "...{}..."
    args: TokenStream, // , self.x
}

impl Parse for FormatArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let template: LitStr = input.parse()?;

        let args = if input.is_empty()
            || input.peek(Token![,]) && input.peek2(End)
        {
            input.parse::<Option<Token![,]>>()?;
            TokenStream::new()
        } else {
            input.parse()?
        };

        Ok(FormatArgs {
            template,
            args,
        })
    }
}

fn main() -> Result<()> {
    let attrs: Vec<Attribute> = parse_quote! {
        #[fmt("simple example")]
        #[fmt("interpolation e{}ample", self.x)]
        #[fmt("interpolation e{x}ample")]
    };

    for attr in &attrs {
        let FormatArgs { template, args } = attr.parse_args()?;
        let requires_fmt_machinery =
            !args.is_empty() || template.value().contains(['{', '}']);
        let out = if requires_fmt_machinery {
            quote! {
                ::core::write!(__formatter, #template #args)
            }
        } else {
            quote! {
                __formatter.write_str(#template)
            }
        };
        println!("{}", out);
    }
    Ok(())
}
```

Implementing this parsing logic without `peek2(End)` is more clumsy because
we'd need a parse stream actually advanced past the comma before being able
to find out whether there is anything after it. It would look something
like:

```rust
use proc_macro2::TokenStream;
use syn::parse::{ParseStream, Result};
use syn::Token;

fn parse(input: ParseStream) -> Result<()> {
use syn::parse::discouraged::Speculative as _;

let ahead = input.fork();
ahead.parse::<Option<Token![,]>>()?;
let args = if ahead.is_empty() {
    input.advance_to(&ahead);
    TokenStream::new()
} else {
    input.parse()?
};
Ok(())
}
```

or:

```rust
use proc_macro2::TokenStream;
use syn::parse::{ParseStream, Result};
use syn::Token;

fn parse(input: ParseStream) -> Result<()> {
use quote::ToTokens as _;

let comma: Option<Token![,]> = input.parse()?;
let mut args = TokenStream::new();
if !input.is_empty() {
    comma.to_tokens(&mut args);
    input.parse::<TokenStream>()?.to_tokens(&mut args);
}
Ok(())
}
```

#### Trait Implementations

##### `impl Any for End`

- <span id="end-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for End`

- <span id="end-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for End`

- <span id="end-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for End`

- <span id="end-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for End`

- <span id="end-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for End`

##### `impl<T> From for End`

- <span id="end-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for End`

- <span id="end-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Peek for End`

##### `impl Sealed for End`

##### `impl ToOwned for End`

- <span id="end-toowned-type-owned"></span>`type Owned = T`

- <span id="end-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="end-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl Token for End`

- <span id="end-token-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool` — [`Cursor`](../buffer/index.md#cursor)

- <span id="end-token-display"></span>`fn display() -> &'static str`

##### `impl<U> TryFrom for End`

- <span id="end-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="end-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for End`

- <span id="end-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="end-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Lookahead1<'a>`

```rust
struct Lookahead1<'a> {
    scope: proc_macro2::Span,
    cursor: crate::buffer::Cursor<'a>,
    comparisons: std::cell::RefCell<Vec<&'static str>>,
}
```

*Defined in [`syn-2.0.111/src/lookahead.rs:63-67`](../../../.source_1765521767/syn-2.0.111/src/lookahead.rs#L63-L67)*

Support for checking the next token in a stream to decide how to parse.

An important advantage over `ParseStream::peek` is that here we
automatically construct an appropriate error message based on the token
alternatives that get peeked. If you are producing your own error message,
go ahead and use `ParseStream::peek` instead.

Use `ParseStream::lookahead1` to construct this object.


Consuming tokens from the source stream after constructing a lookahead
object does not also advance the lookahead object.

# Example

```rust
use syn::{ConstParam, Ident, Lifetime, LifetimeParam, Result, Token, TypeParam};
use syn::parse::{Parse, ParseStream};

// A generic parameter, a single one of the comma-separated elements inside
// angle brackets in:
//
//     fn f<T: Clone, 'a, 'b: 'a, const N: usize>() { ... }
//
// On invalid input, lookahead gives us a reasonable error message.
//
//     error: expected one of: identifier, lifetime, `const`
//       |
//     5 |     fn f<!Sized>() {}
//       |          ^
enum GenericParam {
    Type(TypeParam),
    Lifetime(LifetimeParam),
    Const(ConstParam),
}

impl Parse for GenericParam {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            input.parse().map(GenericParam::Type)
        } else if lookahead.peek(Lifetime) {
            input.parse().map(GenericParam::Lifetime)
        } else if lookahead.peek(Token![const]) {
            input.parse().map(GenericParam::Const)
        } else {
            Err(lookahead.error())
        }
    }
}
```

#### Implementations

- <span id="lookahead1-peek"></span>`fn peek<T: Peek>(&self, token: T) -> bool`

  Looks at the next token in the parse stream to determine whether it

  matches the requested type of token.

  

  # Syntax

  

  Note that this method does not use turbofish syntax. Pass the peek type

  inside of parentheses.

  

  - `input.peek(Token![struct])`

  - `input.peek(Token![==])`

  - `input.peek(Ident)`&emsp;*(does not accept keywords)*

  - `input.peek(Ident::peek_any)`

  - `input.peek(Lifetime)`

  - `input.peek(token::Brace)`

- <span id="lookahead1-error"></span>`fn error(self) -> Error` — [`Error`](../error/index.md#error)

  Triggers an error at the current position of the parse stream.

  

  The error message will identify all of the expected token types that

  have been peeked against this lookahead instance.

#### Trait Implementations

##### `impl Any for Lookahead1<'a>`

- <span id="lookahead1-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Lookahead1<'a>`

- <span id="lookahead1-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Lookahead1<'a>`

- <span id="lookahead1-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Lookahead1<'a>`

- <span id="lookahead1-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Lookahead1<'a>`

- <span id="lookahead1-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Lookahead1<'a>`

- <span id="lookahead1-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lookahead1-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Lookahead1<'a>`

- <span id="lookahead1-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lookahead1-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ParseBuffer<'a>`

```rust
struct ParseBuffer<'a> {
    scope: proc_macro2::Span,
    cell: std::cell::Cell<crate::buffer::Cursor<'static>>,
    marker: std::marker::PhantomData<crate::buffer::Cursor<'a>>,
    unexpected: std::cell::Cell<Option<std::rc::Rc<std::cell::Cell<Unexpected>>>>,
}
```

*Defined in [`syn-2.0.111/src/parse.rs:246-262`](../../../.source_1765521767/syn-2.0.111/src/parse.rs#L246-L262)*

Cursor position within a buffered token stream.

This type is more commonly used through the type alias [`ParseStream`](#parsestream) which
is an alias for `&ParseBuffer`.

`ParseStream` is the input type for all parser functions in Syn. They have
the signature `fn(ParseStream) -> Result<T>`.

## Calling a parser function

There is no public way to construct a `ParseBuffer`. Instead, if you are
looking to invoke a parser function that requires `ParseStream` as input,
you will need to go through one of the public parsing entry points.

- The `parse_macro_input!` macro if parsing input of a procedural macro;
- One of [the `syn::parse*` functions][syn-parse]; or
- A method of the [`Parser`](#parser) trait.



#### Implementations

- <span id="parsebuffer-parse"></span>`fn parse<T: Parse>(&self) -> Result<T>` — [`Result`](../error/index.md#result)

  Parses a syntax tree node of type `T`, advancing the position of our

  parse stream past it.

- <span id="parsebuffer-call"></span>`fn call<T>(self: &'a Self, function: fn(ParseStream<'a>) -> Result<T>) -> Result<T>` — [`ParseStream`](#parsestream), [`Result`](../error/index.md#result)

  Calls the given parser function to parse a syntax tree node of type `T`

  from this stream.

  

  # Example

  

  The parser below invokes `Attribute::parse_outer` to parse a vector of

  zero or more outer attributes.

  

  ```rust

  use syn::{Attribute, Ident, Result, Token};

  use syn::parse::{Parse, ParseStream};

  

  // Parses a unit struct with attributes.

  //

  //     #[path = "s.tmpl"]

  //     struct S;

  struct UnitStruct {

      attrs: Vec<Attribute>,

      struct_token: Token![struct],

      name: Ident,

      semi_token: Token![;],

  }

  

  impl Parse for UnitStruct {

      fn parse(input: ParseStream) -> Result<Self> {

          Ok(UnitStruct {

              attrs: input.call(Attribute::parse_outer)?,

              struct_token: input.parse()?,

              name: input.parse()?,

              semi_token: input.parse()?,

          })

      }

  }

  ```

- <span id="parsebuffer-peek"></span>`fn peek<T: Peek>(&self, token: T) -> bool`

  Looks at the next token in the parse stream to determine whether it

  matches the requested type of token.

  

  Does not advance the position of the parse stream.

  

  # Syntax

  

  Note that this method does not use turbofish syntax. Pass the peek type

  inside of parentheses.

  

  - `input.peek(Token![struct])`

  - `input.peek(Token![==])`

  - `input.peek(syn::Ident)`&emsp;*(does not accept keywords)*

  - `input.peek(syn::Ident::peek_any)`

  - `input.peek(Lifetime)`

  - `input.peek(token::Brace)`

  

  # Example

  

  In this example we finish parsing the list of supertraits when the next

  token in the input is either `where` or an opening curly brace.

  

  ```rust

  use syn::{braced, token, Generics, Ident, Result, Token, TypeParamBound};

  use syn::parse::{Parse, ParseStream};

  use syn::punctuated::Punctuated;

  

  // Parses a trait definition containing no associated items.

  //

  //     trait Marker<'de, T>: A + B<'de> where Box<T>: Clone {}

  struct MarkerTrait {

      trait_token: Token![trait],

      ident: Ident,

      generics: Generics,

      colon_token: Option<Token![:]>,

      supertraits: Punctuated<TypeParamBound, Token![+]>,

      brace_token: token::Brace,

  }

  

  impl Parse for MarkerTrait {

      fn parse(input: ParseStream) -> Result<Self> {

          let trait_token: Token![trait] = input.parse()?;

          let ident: Ident = input.parse()?;

          let mut generics: Generics = input.parse()?;

          let colon_token: Option<Token![:]> = input.parse()?;

  

          let mut supertraits = Punctuated::new();

          if colon_token.is_some() {

              loop {

                  supertraits.push_value(input.parse()?);

                  if input.peek(Token![where]) || input.peek(token::Brace) {

                      break;

                  }

                  supertraits.push_punct(input.parse()?);

              }

          }

  

          generics.where_clause = input.parse()?;

          let content;

          let empty_brace_token = braced!(content in input);

  

          Ok(MarkerTrait {

              trait_token,

              ident,

              generics,

              colon_token,

              supertraits,

              brace_token: empty_brace_token,

          })

      }

  }

  ```

- <span id="parsebuffer-peek2"></span>`fn peek2<T: Peek>(&self, token: T) -> bool`

  Looks at the second-next token in the parse stream.

  

  This is commonly useful as a way to implement contextual keywords.

  

  # Example

  

  This example needs to use `peek2` because the symbol `union` is not a

  keyword in Rust. We can't use just `peek` and decide to parse a union if

  the very next token is `union`, because someone is free to write a `mod

  union` and a macro invocation that looks like `union::some_macro! { ...

  }`. In other words `union` is a contextual keyword.

  

  ```rust

  use syn::{Ident, ItemUnion, Macro, Result, Token};

  use syn::parse::{Parse, ParseStream};

  

  // Parses either a union or a macro invocation.

  enum UnionOrMacro {

      // union MaybeUninit<T> { uninit: (), value: T }

      Union(ItemUnion),

      // lazy_static! { ... }

      Macro(Macro),

  }

  

  impl Parse for UnionOrMacro {

      fn parse(input: ParseStream) -> Result<Self> {

          if input.peek(Token![union]) && input.peek2(Ident) {

              input.parse().map(UnionOrMacro::Union)

          } else {

              input.parse().map(UnionOrMacro::Macro)

          }

      }

  }

  ```

- <span id="parsebuffer-peek3"></span>`fn peek3<T: Peek>(&self, token: T) -> bool`

  Looks at the third-next token in the parse stream.

- <span id="parsebuffer-parse-terminated"></span>`fn parse_terminated<T, P>(self: &'a Self, parser: fn(ParseStream<'a>) -> Result<T>, separator: P) -> Result<Punctuated<T, <P as >::Token>>` — [`ParseStream`](#parsestream), [`Result`](../error/index.md#result), [`Punctuated`](../punctuated/index.md#punctuated), [`Peek`](../lookahead/index.md#peek)

  Parses zero or more occurrences of `T` separated by punctuation of type

  `P`, with optional trailing punctuation.

  

  Parsing continues until the end of this parse stream. The entire content

  of this parse stream must consist of `T` and `P`.

  

  # Example

  

  ```rust

  use quote::quote;

  

  use syn::{parenthesized, token, Ident, Result, Token, Type};

  use syn::parse::{Parse, ParseStream};

  use syn::punctuated::Punctuated;

  

  // Parse a simplified tuple struct syntax like:

  //

  //     struct S(A, B);

  struct TupleStruct {

      struct_token: Token![struct],

      ident: Ident,

      paren_token: token::Paren,

      fields: Punctuated<Type, Token![,]>,

      semi_token: Token![;],

  }

  

  impl Parse for TupleStruct {

      fn parse(input: ParseStream) -> Result<Self> {

          let content;

          Ok(TupleStruct {

              struct_token: input.parse()?,

              ident: input.parse()?,

              paren_token: parenthesized!(content in input),

              fields: content.parse_terminated(Type::parse, Token![,])?,

              semi_token: input.parse()?,

          })

      }

  }

  

  let input = quote! {

      struct S(A, B);

  };

  syn::parse2::<TupleStruct>(input).unwrap();

  ```

  

  # See also

  

  If your separator is anything more complicated than an invocation of the

  `Token!` macro, this method won't be applicable and you can instead

  directly use `Punctuated`'s parser functions: `parse_terminated`,

  `parse_separated_nonempty` etc.

  

  

  ```rust

  use syn::{custom_keyword, Expr, Result, Token};

  use syn::parse::{Parse, ParseStream};

  use syn::punctuated::Punctuated;

  

  mod kw {

      syn::custom_keyword!(fin);

  }

  

  struct Fin(kw::fin, Token![;]);

  

  impl Parse for Fin {

      fn parse(input: ParseStream) -> Result<Self> {

          Ok(Self(input.parse()?, input.parse()?))

      }

  }

  

  struct Thing {

      steps: Punctuated<Expr, Fin>,

  }

  

  impl Parse for Thing {

      fn parse(input: ParseStream) -> Result<Self> {

  if true {

          Ok(Thing {

              steps: Punctuated::parse_terminated(input)?,

          })

  } else {

          // or equivalently, this means the same thing:

        Ok(Thing {

              steps: input.call(Punctuated::parse_terminated)?,

        })

  }

      }

  }

  ```

- <span id="parsebuffer-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns whether there are no more tokens remaining to be parsed from

  this stream.

  

  This method returns true upon reaching the end of the content within a

  set of delimiters, as well as at the end of the tokens provided to the

  outermost parsing entry point.

  

  This is equivalent to

  <code>.<a href="#method.peek">peek</a>(<a href="struct.End.html">syn::parse::End</a>)</code>.

  Use `.peek2(End)` or `.peek3(End)` to look for the end of a parse stream

  further ahead than the current position.

  

  # Example

  

  ```rust

  use syn::{braced, token, Ident, Item, Result, Token};

  use syn::parse::{Parse, ParseStream};

  

  // Parses a Rust `mod m { ... }` containing zero or more items.

  struct Mod {

      mod_token: Token![mod],

      name: Ident,

      brace_token: token::Brace,

      items: Vec<Item>,

  }

  

  impl Parse for Mod {

      fn parse(input: ParseStream) -> Result<Self> {

          let content;

          Ok(Mod {

              mod_token: input.parse()?,

              name: input.parse()?,

              brace_token: braced!(content in input),

              items: {

                  let mut items = Vec::new();

                  while !content.is_empty() {

                      items.push(content.parse()?);

                  }

                  items

              },

          })

      }

  }

  ```

- <span id="parsebuffer-lookahead1"></span>`fn lookahead1(&self) -> Lookahead1<'a>` — [`Lookahead1`](../lookahead/index.md#lookahead1)

  Constructs a helper for peeking at the next token in this stream and

  building an error message if it is not one of a set of expected tokens.

  

  # Example

  

  ```rust

  use syn::{ConstParam, Ident, Lifetime, LifetimeParam, Result, Token, TypeParam};

  use syn::parse::{Parse, ParseStream};

  

  // A generic parameter, a single one of the comma-separated elements inside

  // angle brackets in:

  //

  //     fn f<T: Clone, 'a, 'b: 'a, const N: usize>() { ... }

  //

  // On invalid input, lookahead gives us a reasonable error message.

  //

  //     error: expected one of: identifier, lifetime, `const`

  //       |

  //     5 |     fn f<!Sized>() {}

  //       |          ^

  enum GenericParam {

      Type(TypeParam),

      Lifetime(LifetimeParam),

      Const(ConstParam),

  }

  

  impl Parse for GenericParam {

      fn parse(input: ParseStream) -> Result<Self> {

          let lookahead = input.lookahead1();

          if lookahead.peek(Ident) {

              input.parse().map(GenericParam::Type)

          } else if lookahead.peek(Lifetime) {

              input.parse().map(GenericParam::Lifetime)

          } else if lookahead.peek(Token![const]) {

              input.parse().map(GenericParam::Const)

          } else {

              Err(lookahead.error())

          }

      }

  }

  ```

- <span id="parsebuffer-fork"></span>`fn fork(&self) -> Self`

  Forks a parse stream so that parsing tokens out of either the original

  or the fork does not advance the position of the other.

  

  # Performance

  

  Forking a parse stream is a cheap fixed amount of work and does not

  involve copying token buffers. Where you might hit performance problems

  is if your macro ends up parsing a large amount of content more than

  once.

  

  ```rust

  use syn::{Expr, Result};

  use syn::parse::ParseStream;

  

  fn bad(input: ParseStream) -> Result<Expr> {

  // Do not do this.

  if input.fork().parse::<Expr>().is_ok() {

      return input.parse::<Expr>();

  }

  unimplemented!()

  }

  ```

  

  As a rule, avoid parsing an unbounded amount of tokens out of a forked

  parse stream. Only use a fork when the amount of work performed against

  the fork is small and bounded.

  

  When complex speculative parsing against the forked stream is

  unavoidable, use `parse::discouraged::Speculative` to advance the

  original stream once the fork's parse is determined to have been

  successful.

  

  For a lower level way to perform speculative parsing at the token level,

  consider using `ParseStream::step` instead.

  

  

  # Example

  

  The parse implementation shown here parses possibly restricted `pub`

  visibilities.

  

  - `pub`

  - `pub(crate)`

  - `pub(self)`

  - `pub(super)`

  - `pub(in some::path)`

  

  To handle the case of visibilities inside of tuple structs, the parser

  needs to distinguish parentheses that specify visibility restrictions

  from parentheses that form part of a tuple type.

  

  ```rust

  struct A;

  struct B;

  struct C;

  

  struct S(pub(crate) A, pub (B, C));

  ```

  

  In this example input the first tuple struct element of `S` has

  `pub(crate)` visibility while the second tuple struct element has `pub`

  visibility; the parentheses around `(B, C)` are part of the type rather

  than part of a visibility restriction.

  

  The parser uses a forked parse stream to check the first token inside of

  parentheses after the `pub` keyword. This is a small bounded amount of

  work performed against the forked parse stream.

  

  ```rust

  use syn::{parenthesized, token, Ident, Path, Result, Token};

  use syn::ext::IdentExt;

  use syn::parse::{Parse, ParseStream};

  

  struct PubVisibility {

      pub_token: Token![pub],

      restricted: Option<Restricted>,

  }

  

  struct Restricted {

      paren_token: token::Paren,

      in_token: Option<Token![in]>,

      path: Path,

  }

  

  impl Parse for PubVisibility {

      fn parse(input: ParseStream) -> Result<Self> {

          let pub_token: Token![pub] = input.parse()?;

  

          if input.peek(token::Paren) {

              let ahead = input.fork();

              let mut content;

              parenthesized!(content in ahead);

  

              if content.peek(Token![crate])

                  || content.peek(Token![self])

                  || content.peek(Token![super])

              {

                  return Ok(PubVisibility {

                      pub_token,

                      restricted: Some(Restricted {

                          paren_token: parenthesized!(content in input),

                          in_token: None,

                          path: Path::from(content.call(Ident::parse_any)?),

                      }),

                  });

              } else if content.peek(Token![in]) {

                  return Ok(PubVisibility {

                      pub_token,

                      restricted: Some(Restricted {

                          paren_token: parenthesized!(content in input),

                          in_token: Some(content.parse()?),

                          path: content.call(Path::parse_mod_style)?,

                      }),

                  });

              }

          }

  

          Ok(PubVisibility {

              pub_token,

              restricted: None,

          })

      }

  }

  ```

- <span id="parsebuffer-error"></span>`fn error<T: Display>(&self, message: T) -> Error` — [`Error`](../error/index.md#error)

  Triggers an error at the current position of the parse stream.

  

  # Example

  

  ```rust

  use syn::{Expr, Result, Token};

  use syn::parse::{Parse, ParseStream};

  

  // Some kind of loop: `while` or `for` or `loop`.

  struct Loop {

      expr: Expr,

  }

  

  impl Parse for Loop {

      fn parse(input: ParseStream) -> Result<Self> {

          if input.peek(Token![while])

              || input.peek(Token![for])

              || input.peek(Token![loop])

          {

              Ok(Loop {

                  expr: input.parse()?,

              })

          } else {

              Err(input.error("expected some kind of loop"))

          }

      }

  }

  ```

- <span id="parsebuffer-step"></span>`fn step<F, R>(&self, function: F) -> Result<R>` — [`Result`](../error/index.md#result)

  Speculatively parses tokens from this parse stream, advancing the

  position of this stream only if parsing succeeds.

  

  This is a powerful low-level API used for defining the `Parse` impls of

  the basic built-in token types. It is not something that will be used

  widely outside of the Syn codebase.

  

  # Example

  

  ```rust

  use proc_macro2::TokenTree;

  use syn::Result;

  use syn::parse::ParseStream;

  

  // This function advances the stream past the next occurrence of `@`. If

  // no `@` is present in the stream, the stream position is unchanged and

  // an error is returned.

  fn skip_past_next_at(input: ParseStream) -> Result<()> {

      input.step(|cursor| {

          let mut rest = *cursor;

          while let Some((tt, next)) = rest.token_tree() {

              match &tt {

                  TokenTree::Punct(punct) if punct.as_char() == '@' => {

                      return Ok(((), next));

                  }

                  _ => rest = next,

              }

          }

          Err(cursor.error("no `@` was found after this point"))

      })

  }

  

  fn remainder_after_skipping_past_next_at(

      input: ParseStream,

  ) -> Result<proc_macro2::TokenStream> {

      skip_past_next_at(input)?;

      input.parse()

  }

  

  use syn::parse::Parser;

  let remainder = remainder_after_skipping_past_next_at

      .parse_str("a @ b c")

      .unwrap();

  assert_eq!(remainder.to_string(), "b c");

  ```

- <span id="parsebuffer-span"></span>`fn span(&self) -> Span`

  Returns the `Span` of the next token in the parse stream, or

  `Span::call_site()` if this parse stream has completely exhausted its

  input `TokenStream`.

- <span id="parsebuffer-cursor"></span>`fn cursor(&self) -> Cursor<'a>` — [`Cursor`](../buffer/index.md#cursor)

  Provides low-level access to the token representation underlying this

  parse stream.

  

  Cursors are immutable so no operations you perform against the cursor

  will affect the state of this parse stream.

  

  # Example

  

  ```rust

  use proc_macro2::TokenStream;

  use syn::buffer::Cursor;

  use syn::parse::{ParseStream, Result};

  

  // Run a parser that returns T, but get its output as TokenStream instead of T.

  // This works without T needing to implement ToTokens.

  fn recognize_token_stream<T>(

      recognizer: fn(ParseStream) -> Result<T>,

  ) -> impl Fn(ParseStream) -> Result<TokenStream> {

      move |input| {

          let begin = input.cursor();

          recognizer(input)?;

          let end = input.cursor();

          Ok(tokens_between(begin, end))

      }

  }

  

  // Collect tokens between two cursors as a TokenStream.

  fn tokens_between(begin: Cursor, end: Cursor) -> TokenStream {

      assert!(begin <= end);

  

      let mut cursor = begin;

      let mut tokens = TokenStream::new();

      while cursor < end {

          let (token, next) = cursor.token_tree().unwrap();

          tokens.extend(std::iter::once(token));

          cursor = next;

      }

      tokens

  }

  

  fn main() {

      use quote::quote;

      use syn::parse::{Parse, Parser};

      use syn::Token;

  

      // Parse syn::Type as a TokenStream, surrounded by angle brackets.

      fn example(input: ParseStream) -> Result<TokenStream> {

          let _langle: Token![<] = input.parse()?;

          let ty = recognize_token_stream(syn::Type::parse)(input)?;

          let _rangle: Token![>] = input.parse()?;

          Ok(ty)

      }

  

      let tokens = quote! { <fn() -> u8> };

      println!("{}", example.parse2(tokens).unwrap());

  }

  ```

- <span id="parsebuffer-check-unexpected"></span>`fn check_unexpected(&self) -> Result<()>` — [`Result`](../error/index.md#result)

#### Trait Implementations

##### `impl Any for ParseBuffer<'a>`

- <span id="parsebuffer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl AnyDelimiter for crate::parse::ParseBuffer<'a>`

- <span id="crateparseparsebuffer-anydelimiter-parse-any-delimiter"></span>`fn parse_any_delimiter(&self) -> Result<(Delimiter, DelimSpan, ParseBuffer<'_>)>` — [`Result`](../error/index.md#result), [`ParseBuffer`](#parsebuffer)

##### `impl<T> Borrow for ParseBuffer<'a>`

- <span id="parsebuffer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ParseBuffer<'a>`

- <span id="parsebuffer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ParseBuffer<'a>`

- <span id="parsebuffer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ParseBuffer<'a>`

- <span id="parsebuffer-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for ParseBuffer<'a>`

- <span id="parsebuffer-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for ParseBuffer<'a>`

- <span id="parsebuffer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ParseBuffer<'a>`

- <span id="parsebuffer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl RefUnwindSafe for ParseBuffer<'a>`

##### `impl Speculative for crate::parse::ParseBuffer<'a>`

- <span id="crateparseparsebuffer-speculative-advance-to"></span>`fn advance_to(&self, fork: &Self)`

##### `impl ToString for ParseBuffer<'a>`

- <span id="parsebuffer-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for ParseBuffer<'a>`

- <span id="parsebuffer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parsebuffer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ParseBuffer<'a>`

- <span id="parsebuffer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parsebuffer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UnwindSafe for ParseBuffer<'a>`

### `StepCursor<'c, 'a>`

```rust
struct StepCursor<'c, 'a> {
    scope: proc_macro2::Span,
    cursor: crate::buffer::Cursor<'c>,
    marker: std::marker::PhantomData<fn(crate::buffer::Cursor<'c>) -> crate::buffer::Cursor<'a>>,
}
```

*Defined in [`syn-2.0.111/src/parse.rs:335-348`](../../../.source_1765521767/syn-2.0.111/src/parse.rs#L335-L348)*

Cursor state associated with speculative parsing.

This type is the input of the closure provided to `ParseStream::step`.

# Example

```rust
use proc_macro2::TokenTree;
use syn::Result;
use syn::parse::ParseStream;

// This function advances the stream past the next occurrence of `@`. If
// no `@` is present in the stream, the stream position is unchanged and
// an error is returned.
fn skip_past_next_at(input: ParseStream) -> Result<()> {
    input.step(|cursor| {
        let mut rest = *cursor;
        while let Some((tt, next)) = rest.token_tree() {
            match &tt {
                TokenTree::Punct(punct) if punct.as_char() == '@' => {
                    return Ok(((), next));
                }
                _ => rest = next,
            }
        }
        Err(cursor.error("no `@` was found after this point"))
    })
}

fn remainder_after_skipping_past_next_at(
    input: ParseStream,
) -> Result<proc_macro2::TokenStream> {
    skip_past_next_at(input)?;
    input.parse()
}

use syn::parse::Parser;
let remainder = remainder_after_skipping_past_next_at
    .parse_str("a @ b c")
    .unwrap();
assert_eq!(remainder.to_string(), "b c");
```

#### Implementations

- <span id="stepcursor-error"></span>`fn error<T: Display>(self, message: T) -> Error` — [`Error`](../error/index.md#error)

  Triggers an error at the current position of the parse stream.

  

  The `ParseStream::step` invocation will return this same error without

  advancing the stream state.

#### Trait Implementations

##### `impl Any for StepCursor<'c, 'a>`

- <span id="stepcursor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StepCursor<'c, 'a>`

- <span id="stepcursor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StepCursor<'c, 'a>`

- <span id="stepcursor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StepCursor<'c, 'a>`

- <span id="stepcursor-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for StepCursor<'c, 'a>`

- <span id="stepcursor-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for StepCursor<'c, 'a>`

##### `impl Deref for StepCursor<'c, 'a>`

- <span id="stepcursor-deref-type-target"></span>`type Target = Cursor<'c>`

- <span id="stepcursor-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T> From for StepCursor<'c, 'a>`

- <span id="stepcursor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StepCursor<'c, 'a>`

- <span id="stepcursor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Receiver for StepCursor<'c, 'a>`

- <span id="stepcursor-receiver-type-target"></span>`type Target = T`

##### `impl ToOwned for StepCursor<'c, 'a>`

- <span id="stepcursor-toowned-type-owned"></span>`type Owned = T`

- <span id="stepcursor-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="stepcursor-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StepCursor<'c, 'a>`

- <span id="stepcursor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stepcursor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StepCursor<'c, 'a>`

- <span id="stepcursor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stepcursor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Nothing`

```rust
struct Nothing;
```

*Defined in [`syn-2.0.111/src/parse.rs:1367`](../../../.source_1765521767/syn-2.0.111/src/parse.rs#L1367)*

An empty syntax tree node that consumes no tokens when parsed.

This is useful for attribute macros that want to ensure they are not
provided any attribute args.

```rust
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::parse::Nothing;

const IGNORE: &str = stringify! {
#[proc_macro_attribute]
};
pub fn my_attr(args: TokenStream, input: TokenStream) -> TokenStream {
    parse_macro_input!(args as Nothing);

    /* ... */
  TokenStream::new()
}
```

```text
error: unexpected token
 --> src/main.rs:3:19
  |
3 | #[my_attr(asdf)]
  |           ^^^^
```

#### Trait Implementations

##### `impl Any for Nothing`

- <span id="nothing-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Nothing`

- <span id="nothing-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Nothing`

- <span id="nothing-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Nothing`

- <span id="nothing-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Nothing`

- <span id="nothing-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Nothing`

##### `impl Debug for Nothing`

- <span id="nothing-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Nothing`

##### `impl<T> From for Nothing`

- <span id="nothing-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Nothing`

- <span id="nothing-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Nothing`

- <span id="nothing-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Nothing`

- <span id="nothing-parse"></span>`fn parse(_input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Nothing`

- <span id="nothing-partialeq-eq"></span>`fn eq(&self, _other: &Self) -> bool`

##### `impl Sealed for Nothing`

##### `impl Spanned for Nothing`

- <span id="nothing-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Nothing`

- <span id="nothing-toowned-type-owned"></span>`type Owned = T`

- <span id="nothing-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="nothing-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Nothing`

- <span id="nothing-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Nothing`

- <span id="nothing-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="nothing-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Nothing`

- <span id="nothing-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="nothing-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Unexpected`

```rust
enum Unexpected {
    None,
    Some(proc_macro2::Span, proc_macro2::Delimiter),
    Chain(std::rc::Rc<std::cell::Cell<Unexpected>>),
}
```

*Defined in [`syn-2.0.111/src/parse.rs:399-403`](../../../.source_1765521767/syn-2.0.111/src/parse.rs#L399-L403)*

#### Trait Implementations

##### `impl Any for Unexpected`

- <span id="unexpected-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Unexpected`

- <span id="unexpected-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Unexpected`

- <span id="unexpected-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Unexpected`

- <span id="unexpected-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Unexpected`

- <span id="unexpected-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Default for Unexpected`

- <span id="unexpected-default"></span>`fn default() -> Self`

##### `impl<T> From for Unexpected`

- <span id="unexpected-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Unexpected`

- <span id="unexpected-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Unexpected`

- <span id="unexpected-toowned-type-owned"></span>`type Owned = T`

- <span id="unexpected-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unexpected-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Unexpected`

- <span id="unexpected-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unexpected-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Unexpected`

- <span id="unexpected-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unexpected-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Peek`

```rust
trait Peek: Sealed { ... }
```

*Defined in [`syn-2.0.111/src/lookahead.rs:174-178`](../../../.source_1765521767/syn-2.0.111/src/lookahead.rs#L174-L178)*

Types that can be parsed by looking at just one token.

Use `ParseStream::peek` to peek one of these types in a parse stream
without consuming it from the stream.

This trait is sealed and cannot be implemented for types outside of Syn.


#### Implementors

- [`End`](../lookahead/index.md#end)
- [`PeekFn`](../ext/private/index.md#peekfn)
- `F`

### `Parse`

```rust
trait Parse: Sized { ... }
```

*Defined in [`syn-2.0.111/src/parse.rs:214-216`](../../../.source_1765521767/syn-2.0.111/src/parse.rs#L214-L216)*

Parsing interface implemented by all types that can be parsed in a default
way from a token stream.

Refer to the [module documentation] for details about implementing and using
the `Parse` trait.


#### Required Methods

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

#### Implementors

- [`Abi`](../ty/index.md#abi)
- [`Abstract`](../token/index.md#abstract)
- [`AndAnd`](../token/index.md#andand)
- [`AndEq`](../token/index.md#andeq)
- [`And`](../token/index.md#and)
- [`AngleBracketedGenericArguments`](../path/index.md#anglebracketedgenericarguments)
- [`Arm`](../expr/index.md#arm)
- [`As`](../token/index.md#as)
- [`Async`](../token/index.md#async)
- [`At`](../token/index.md#at)
- [`Auto`](../token/index.md#auto)
- [`Await`](../token/index.md#await)
- [`BareFnArg`](../ty/index.md#barefnarg)
- [`Become`](../token/index.md#become)
- [`BinOp`](../op/index.md#binop)
- [`Block`](../stmt/index.md#block)
- [`BoundLifetimes`](../generics/index.md#boundlifetimes)
- [`Box`](../token/index.md#box)
- [`Break`](../token/index.md#break)
- [`CapturedParam`](../generics/index.md#capturedparam)
- [`CaretEq`](../token/index.md#careteq)
- [`Caret`](../token/index.md#caret)
- [`Colon`](../token/index.md#colon)
- [`Comma`](../token/index.md#comma)
- [`ConstParam`](../generics/index.md#constparam)
- [`Const`](../token/index.md#const)
- [`Continue`](../token/index.md#continue)
- [`Crate`](../token/index.md#crate)
- [`Default`](../token/index.md#default)
- [`DeriveInput`](../derive/index.md#deriveinput)
- [`Do`](../token/index.md#do)
- [`Dollar`](../token/index.md#dollar)
- [`DotDotDot`](../token/index.md#dotdotdot)
- [`DotDotEq`](../token/index.md#dotdoteq)
- [`DotDot`](../token/index.md#dotdot)
- [`Dot`](../token/index.md#dot)
- [`Dyn`](../token/index.md#dyn)
- [`Else`](../token/index.md#else)
- [`Enum`](../token/index.md#enum)
- [`EqEq`](../token/index.md#eqeq)
- [`Eq`](../token/index.md#eq)
- [`ExprArray`](../expr/index.md#exprarray)
- [`ExprAssign`](../expr/index.md#exprassign)
- [`ExprAsync`](../expr/index.md#exprasync)
- [`ExprAwait`](../expr/index.md#exprawait)
- [`ExprBinary`](../expr/index.md#exprbinary)
- [`ExprBlock`](../expr/index.md#exprblock)
- [`ExprBreak`](../expr/index.md#exprbreak)
- [`ExprCall`](../expr/index.md#exprcall)
- [`ExprCast`](../expr/index.md#exprcast)
- [`ExprClosure`](../expr/index.md#exprclosure)
- [`ExprConst`](../expr/index.md#exprconst)
- [`ExprContinue`](../expr/index.md#exprcontinue)
- [`ExprField`](../expr/index.md#exprfield)
- [`ExprForLoop`](../expr/index.md#exprforloop)
- [`ExprIf`](../expr/index.md#exprif)
- [`ExprIndex`](../expr/index.md#exprindex)
- [`ExprInfer`](../expr/index.md#exprinfer)
- [`ExprLet`](../expr/index.md#exprlet)
- [`ExprLit`](../expr/index.md#exprlit)
- [`ExprLoop`](../expr/index.md#exprloop)
- [`ExprMacro`](../expr/index.md#exprmacro)
- [`ExprMatch`](../expr/index.md#exprmatch)
- [`ExprMethodCall`](../expr/index.md#exprmethodcall)
- [`ExprParen`](../expr/index.md#exprparen)
- [`ExprPath`](../expr/index.md#exprpath)
- [`ExprRange`](../expr/index.md#exprrange)
- [`ExprRawAddr`](../expr/index.md#exprrawaddr)
- [`ExprReference`](../expr/index.md#exprreference)
- [`ExprRepeat`](../expr/index.md#exprrepeat)
- [`ExprReturn`](../expr/index.md#exprreturn)
- [`ExprStruct`](../expr/index.md#exprstruct)
- [`ExprTryBlock`](../expr/index.md#exprtryblock)
- [`ExprTry`](../expr/index.md#exprtry)
- [`ExprTuple`](../expr/index.md#exprtuple)
- [`ExprUnary`](../expr/index.md#exprunary)
- [`ExprUnsafe`](../expr/index.md#exprunsafe)
- [`ExprWhile`](../expr/index.md#exprwhile)
- [`ExprYield`](../expr/index.md#expryield)
- [`Expr`](../expr/index.md#expr)
- [`Extern`](../token/index.md#extern)
- [`FatArrow`](../token/index.md#fatarrow)
- [`FieldValue`](../expr/index.md#fieldvalue)
- [`FieldsNamed`](../data/index.md#fieldsnamed)
- [`FieldsUnnamed`](../data/index.md#fieldsunnamed)
- [`File`](../file/index.md#file)
- [`Final`](../token/index.md#final)
- [`FnArg`](../item/index.md#fnarg)
- [`Fn`](../token/index.md#fn)
- [`For`](../token/index.md#for)
- [`ForeignItemFn`](../item/index.md#foreignitemfn)
- [`ForeignItemMacro`](../item/index.md#foreignitemmacro)
- [`ForeignItemStatic`](../item/index.md#foreignitemstatic)
- [`ForeignItemType`](../item/index.md#foreignitemtype)
- [`ForeignItem`](../item/index.md#foreignitem)
- [`Ge`](../token/index.md#ge)
- [`GenericArgument`](../path/index.md#genericargument)
- [`GenericParam`](../generics/index.md#genericparam)
- [`Generics`](../generics/index.md#generics)
- [`Gt`](../token/index.md#gt)
- [`Ident`](../ident/index.md#ident)
- [`If`](../token/index.md#if)
- [`ImplItemConst`](../item/index.md#implitemconst)
- [`ImplItemFn`](../item/index.md#implitemfn)
- [`ImplItemMacro`](../item/index.md#implitemmacro)
- [`ImplItemType`](../item/index.md#implitemtype)
- [`ImplItem`](../item/index.md#implitem)
- [`Impl`](../token/index.md#impl)
- [`In`](../token/index.md#in)
- [`Index`](../expr/index.md#index)
- [`ItemConst`](../item/index.md#itemconst)
- [`ItemEnum`](../item/index.md#itemenum)
- [`ItemExternCrate`](../item/index.md#itemexterncrate)
- [`ItemFn`](../item/index.md#itemfn)
- [`ItemForeignMod`](../item/index.md#itemforeignmod)
- [`ItemImpl`](../item/index.md#itemimpl)
- [`ItemMacro`](../item/index.md#itemmacro)
- [`ItemMod`](../item/index.md#itemmod)
- [`ItemStatic`](../item/index.md#itemstatic)
- [`ItemStruct`](../item/index.md#itemstruct)
- [`ItemTraitAlias`](../item/index.md#itemtraitalias)
- [`ItemTrait`](../item/index.md#itemtrait)
- [`ItemType`](../item/index.md#itemtype)
- [`ItemUnion`](../item/index.md#itemunion)
- [`ItemUse`](../item/index.md#itemuse)
- [`Item`](../item/index.md#item)
- [`LArrow`](../token/index.md#larrow)
- [`Label`](../expr/index.md#label)
- [`Le`](../token/index.md#le)
- [`Let`](../token/index.md#let)
- [`LifetimeParam`](../generics/index.md#lifetimeparam)
- [`Lifetime`](../lifetime/index.md#lifetime)
- [`LitBool`](../lit/index.md#litbool)
- [`LitByteStr`](../lit/index.md#litbytestr)
- [`LitByte`](../lit/index.md#litbyte)
- [`LitCStr`](../lit/index.md#litcstr)
- [`LitChar`](../lit/index.md#litchar)
- [`LitFloat`](../lit/index.md#litfloat)
- [`LitInt`](../lit/index.md#litint)
- [`LitStr`](../lit/index.md#litstr)
- [`Lit`](../lit/index.md#lit)
- [`Loop`](../token/index.md#loop)
- [`Lt`](../token/index.md#lt)
- [`Macro`](../mac/index.md#macro)
- [`Macro`](../token/index.md#macro)
- [`Match`](../token/index.md#match)
- [`Member`](../expr/index.md#member)
- [`MetaList`](../attr/index.md#metalist)
- [`MetaNameValue`](../attr/index.md#metanamevalue)
- [`Meta`](../attr/index.md#meta)
- [`MinusEq`](../token/index.md#minuseq)
- [`Minus`](../token/index.md#minus)
- [`Mod`](../token/index.md#mod)
- [`Move`](../token/index.md#move)
- [`Mut`](../token/index.md#mut)
- [`Ne`](../token/index.md#ne)
- [`Not`](../token/index.md#not)
- [`Nothing`](#nothing)
- [`OrEq`](../token/index.md#oreq)
- [`OrOr`](../token/index.md#oror)
- [`Or`](../token/index.md#or)
- [`Override`](../token/index.md#override)
- [`ParenthesizedGenericArguments`](../path/index.md#parenthesizedgenericarguments)
- [`PatType`](../pat/index.md#pattype)
- [`PathSegment`](../path/index.md#pathsegment)
- [`PathSep`](../token/index.md#pathsep)
- [`Path`](../path/index.md#path)
- [`PercentEq`](../token/index.md#percenteq)
- [`Percent`](../token/index.md#percent)
- [`PlusEq`](../token/index.md#pluseq)
- [`Plus`](../token/index.md#plus)
- [`PointerMutability`](../expr/index.md#pointermutability)
- [`Pound`](../token/index.md#pound)
- [`PreciseCapture`](../generics/index.md#precisecapture)
- [`Priv`](../token/index.md#priv)
- [`Pub`](../token/index.md#pub)
- [`Question`](../token/index.md#question)
- [`RArrow`](../token/index.md#rarrow)
- [`RangeLimits`](../expr/index.md#rangelimits)
- [`Raw`](../token/index.md#raw)
- [`Receiver`](../item/index.md#receiver)
- [`Ref`](../token/index.md#ref)
- [`ReturnType`](../ty/index.md#returntype)
- [`Return`](../token/index.md#return)
- [`SelfType`](../token/index.md#selftype)
- [`SelfValue`](../token/index.md#selfvalue)
- [`Semi`](../token/index.md#semi)
- [`ShlEq`](../token/index.md#shleq)
- [`Shl`](../token/index.md#shl)
- [`ShrEq`](../token/index.md#shreq)
- [`Shr`](../token/index.md#shr)
- [`Signature`](../item/index.md#signature)
- [`SlashEq`](../token/index.md#slasheq)
- [`Slash`](../token/index.md#slash)
- [`StarEq`](../token/index.md#stareq)
- [`Star`](../token/index.md#star)
- [`StaticMutability`](../item/index.md#staticmutability)
- [`Static`](../token/index.md#static)
- [`Stmt`](../stmt/index.md#stmt)
- [`Struct`](../token/index.md#struct)
- [`Super`](../token/index.md#super)
- [`Tilde`](../token/index.md#tilde)
- [`TraitBoundModifier`](../generics/index.md#traitboundmodifier)
- [`TraitBound`](../generics/index.md#traitbound)
- [`TraitItemConst`](../item/index.md#traititemconst)
- [`TraitItemFn`](../item/index.md#traititemfn)
- [`TraitItemMacro`](../item/index.md#traititemmacro)
- [`TraitItemType`](../item/index.md#traititemtype)
- [`TraitItem`](../item/index.md#traititem)
- [`Trait`](../token/index.md#trait)
- [`Try`](../token/index.md#try)
- [`TypeArray`](../ty/index.md#typearray)
- [`TypeBareFn`](../ty/index.md#typebarefn)
- [`TypeGroup`](../ty/index.md#typegroup)
- [`TypeImplTrait`](../ty/index.md#typeimpltrait)
- [`TypeInfer`](../ty/index.md#typeinfer)
- [`TypeMacro`](../ty/index.md#typemacro)
- [`TypeNever`](../ty/index.md#typenever)
- [`TypeParamBound`](../generics/index.md#typeparambound)
- [`TypeParam`](../generics/index.md#typeparam)
- [`TypeParen`](../ty/index.md#typeparen)
- [`TypePath`](../ty/index.md#typepath)
- [`TypePtr`](../ty/index.md#typeptr)
- [`TypeReference`](../ty/index.md#typereference)
- [`TypeSlice`](../ty/index.md#typeslice)
- [`TypeTraitObject`](../ty/index.md#typetraitobject)
- [`TypeTuple`](../ty/index.md#typetuple)
- [`Type`](../token/index.md#type)
- [`Type`](../ty/index.md#type)
- [`Typeof`](../token/index.md#typeof)
- [`UnOp`](../op/index.md#unop)
- [`Underscore`](../token/index.md#underscore)
- [`Union`](../token/index.md#union)
- [`Unsafe`](../token/index.md#unsafe)
- [`Unsized`](../token/index.md#unsized)
- [`UseTree`](../item/index.md#usetree)
- [`Use`](../token/index.md#use)
- [`Variant`](../data/index.md#variant)
- [`Virtual`](../token/index.md#virtual)
- [`Visibility`](../restriction/index.md#visibility)
- [`WhereClause`](../generics/index.md#whereclause)
- [`WherePredicate`](../generics/index.md#wherepredicate)
- [`Where`](../token/index.md#where)
- [`While`](../token/index.md#while)
- [`Yield`](../token/index.md#yield)
- `Box<T>`
- `Option<T>`
- `Option<crate::expr::Label>`
- `Option<crate::generics::BoundLifetimes>`
- `Option<crate::generics::WhereClause>`
- `Option<crate::ty::Abi>`
- `proc_macro2::Group`
- `proc_macro2::Literal`
- `proc_macro2::Punct`
- `proc_macro2::TokenStream`
- `proc_macro2::TokenTree`

### `Parser`

```rust
trait Parser: Sized { ... }
```

*Defined in [`syn-2.0.111/src/parse.rs:1239-1277`](../../../.source_1765521767/syn-2.0.111/src/parse.rs#L1239-L1277)*

Parser that can parse Rust tokens into a particular syntax tree node.

Refer to the [module documentation] for details about parsing in Syn.


#### Associated Types

- `type Output`

#### Required Methods

- `fn parse2(self, tokens: TokenStream) -> Result<<Self as >::Output>`

  Parse a proc-macro2 token stream into the chosen syntax tree node.

#### Provided Methods

- `fn parse(self, tokens: proc_macro::TokenStream) -> Result<<Self as >::Output>`

  Parse tokens of source code into the chosen syntax tree node.

- `fn parse_str(self, s: &str) -> Result<<Self as >::Output>`

  Parse a string of Rust code into the chosen syntax tree node.

#### Implementors

- `F`

## Functions

### `advance_step_cursor`

```rust
fn advance_step_cursor<'c, 'a>(proof: StepCursor<'c, 'a>, to: crate::buffer::Cursor<'c>) -> crate::buffer::Cursor<'a>
```

*Defined in [`syn-2.0.111/src/parse.rs:376-383`](../../../.source_1765521767/syn-2.0.111/src/parse.rs#L376-L383)*

### `new_parse_buffer`

```rust
fn new_parse_buffer(scope: proc_macro2::Span, cursor: crate::buffer::Cursor<'_>, unexpected: std::rc::Rc<std::cell::Cell<Unexpected>>) -> ParseBuffer<'_>
```

*Defined in [`syn-2.0.111/src/parse.rs:385-397`](../../../.source_1765521767/syn-2.0.111/src/parse.rs#L385-L397)*

### `cell_clone`

```rust
fn cell_clone<T: Default + Clone>(cell: &std::cell::Cell<T>) -> T
```

*Defined in [`syn-2.0.111/src/parse.rs:423-428`](../../../.source_1765521767/syn-2.0.111/src/parse.rs#L423-L428)*

### `inner_unexpected`

```rust
fn inner_unexpected(buffer: &ParseBuffer<'_>) -> (std::rc::Rc<std::cell::Cell<Unexpected>>, Option<(proc_macro2::Span, proc_macro2::Delimiter)>)
```

*Defined in [`syn-2.0.111/src/parse.rs:430-439`](../../../.source_1765521767/syn-2.0.111/src/parse.rs#L430-L439)*

### `get_unexpected`

```rust
fn get_unexpected(buffer: &ParseBuffer<'_>) -> std::rc::Rc<std::cell::Cell<Unexpected>>
```

*Defined in [`syn-2.0.111/src/parse.rs:441-443`](../../../.source_1765521767/syn-2.0.111/src/parse.rs#L441-L443)*

### `span_of_unexpected_ignoring_nones`

```rust
fn span_of_unexpected_ignoring_nones(cursor: crate::buffer::Cursor<'_>) -> Option<(proc_macro2::Span, proc_macro2::Delimiter)>
```

*Defined in [`syn-2.0.111/src/parse.rs:445-460`](../../../.source_1765521767/syn-2.0.111/src/parse.rs#L445-L460)*

### `tokens_to_parse_buffer`

```rust
fn tokens_to_parse_buffer(tokens: &crate::buffer::TokenBuffer) -> ParseBuffer<'_>
```

*Defined in [`syn-2.0.111/src/parse.rs:1279-1284`](../../../.source_1765521767/syn-2.0.111/src/parse.rs#L1279-L1284)*

### `parse_scoped`

```rust
fn parse_scoped<F: Parser>(f: F, scope: proc_macro2::Span, tokens: proc_macro2::TokenStream) -> Result<<F as >::Output>
```

*Defined in [`syn-2.0.111/src/parse.rs:1323-1325`](../../../.source_1765521767/syn-2.0.111/src/parse.rs#L1323-L1325)*

### `err_unexpected_token`

```rust
fn err_unexpected_token(span: proc_macro2::Span, delimiter: proc_macro2::Delimiter) -> Error
```

*Defined in [`syn-2.0.111/src/parse.rs:1327-1335`](../../../.source_1765521767/syn-2.0.111/src/parse.rs#L1327-L1335)*

## Type Aliases

### `Result<T>`

```rust
type Result<T> = std::result::Result<T, Error>;
```

*Defined in [`syn-2.0.111/src/error.rs:15`](../../../.source_1765521767/syn-2.0.111/src/error.rs#L15)*

The result of a Syn parser.

### `ParseStream<'a>`

```rust
type ParseStream<'a> = &'a ParseBuffer<'a>;
```

*Defined in [`syn-2.0.111/src/parse.rs:224`](../../../.source_1765521767/syn-2.0.111/src/parse.rs#L224)*

Input to a Syn parser function.

See the methods of this type under the documentation of [`ParseBuffer`](#parsebuffer). For
an overview of parsing in Syn, refer to the [module documentation].


