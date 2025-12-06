*[syn](../index.md) / [parse](index.md)*

---

# Module `parse`

Parsing interface for parsing a token stream into a syntax tree node.

Parsing in Syn is built on parser functions that take in a [`ParseStream`](#parsestream)
and produce a [`Result<T>`](#result) where `T` is some syntax tree node. Underlying
these parser functions is a lower level mechanism built around the
[`Cursor`](../buffer/index.md) type. `Cursor` is a cheaply copyable cursor over a range of
tokens in a token stream.


# Example

Here is a snippet of parsing code to get a feel for the style of the
library. We define data structures for a subset of Rust syntax including
enums (not shown) and structs, then provide implementations of the [`Parse`](#parse)
trait to parse these syntax tree data structures from a token stream.

Once `Parse` impls have been defined, they can be called conveniently from a
procedural macro through [`parse_macro_input!`](#parse-macro-input) as shown at the bottom of
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

The [`parse_quote!`](#parse-quote) macro also uses this approach.

# The `Parser` trait

Some types can be parsed in several ways depending on context. For example
an [`Attribute`](../attr/index.md) can be either "outer" like `#[...]` or "inner" like
`#![...]` and parsing the wrong one would be a bug. Similarly [`Punctuated`](../punctuated/index.md)
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

## Modules

- [`discouraged`](discouraged/index.md) - Extensions to the parsing API with niche applicability.

## Structs

### `Error`

```rust
struct Error {
    messages: Vec<ErrorMessage>,
}
```

Error returned when a Syn parser cannot parse the input tokens.

# Error reporting in proc macros

The correct way to report errors back to the compiler from a procedural
macro is by emitting an appropriately spanned invocation of
[`compile_error!`](#compile-error) in the generated code. This produces a better diagnostic
message than simply panicking the macro.

When parsing macro input, the [`parse_macro_input!`](#parse-macro-input) macro handles the
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
[`.to_compile_error()`](#to-compile-error) or [`.into_compile_error()`](#into-compile-error) methods can be used to
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

- `fn new<T: Display>(span: Span, message: T) -> Self`

- `fn new_spanned<T: ToTokens, U: Display>(tokens: T, message: U) -> Self`

- `fn span(self: &Self) -> Span`

- `fn to_compile_error(self: &Self) -> TokenStream`

- `fn into_compile_error(self: Self) -> TokenStream`

- `fn combine(self: &mut Self, another: Error)` — [`Error`](../error/index.md)

#### Trait Implementations

##### `impl Clone for Error`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for Error`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for Error`

##### `impl Extend for Error`

- `fn extend<T: IntoIterator<Item = Error>>(self: &mut Self, iter: T)`

##### `impl IntoIterator for Error`

- `type Item = Error`

- `type IntoIter = IntoIter`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl<T> ToString for Error`

- `fn to_string(self: &Self) -> String`

### `End`

```rust
struct End;
```

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

##### `impl Clone for End`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for End`

##### `impl Peek for End`

##### `impl Sealed for End`

##### `impl<T> Token for End`

- `fn peek(cursor: Cursor<'_>) -> bool` — [`Cursor`](../buffer/index.md)

- `fn display() -> &'static str`

### `Lookahead1<'a>`

```rust
struct Lookahead1<'a> {
    scope: proc_macro2::Span,
    cursor: crate::buffer::Cursor<'a>,
    comparisons: std::cell::RefCell<Vec<&'static str>>,
}
```

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

- `fn peek<T: Peek>(self: &Self, token: T) -> bool`

- `fn error(self: Self) -> Error` — [`Error`](../error/index.md)

### `ParseBuffer<'a>`

```rust
struct ParseBuffer<'a> {
    scope: proc_macro2::Span,
    cell: std::cell::Cell<crate::buffer::Cursor<'static>>,
    marker: std::marker::PhantomData<crate::buffer::Cursor<'a>>,
    unexpected: std::cell::Cell<Option<std::rc::Rc<std::cell::Cell<Unexpected>>>>,
}
```

Cursor position within a buffered token stream.

This type is more commonly used through the type alias [`ParseStream`](#parsestream) which
is an alias for `&ParseBuffer`.

`ParseStream` is the input type for all parser functions in Syn. They have
the signature `fn(ParseStream) -> Result<T>`.

## Calling a parser function

There is no public way to construct a `ParseBuffer`. Instead, if you are
looking to invoke a parser function that requires `ParseStream` as input,
you will need to go through one of the public parsing entry points.

- The [`parse_macro_input!`](#parse-macro-input) macro if parsing input of a procedural macro;
- One of [the `syn::parse*` functions][syn-parse]; or
- A method of the [`Parser`](#parser) trait.



#### Implementations

- `fn parse<T: Parse>(self: &Self) -> Result<T>` — [`Result`](../error/index.md)

- `fn call<T>(self: &'a Self, function: fn(ParseStream<'a>) -> Result<T>) -> Result<T>` — [`ParseStream`](#parsestream), [`Result`](../error/index.md)

- `fn peek<T: Peek>(self: &Self, token: T) -> bool`

- `fn peek2<T: Peek>(self: &Self, token: T) -> bool`

- `fn peek3<T: Peek>(self: &Self, token: T) -> bool`

- `fn parse_terminated<T, P>(self: &'a Self, parser: fn(ParseStream<'a>) -> Result<T>, separator: P) -> Result<Punctuated<T, <P as >::Token>>` — [`ParseStream`](#parsestream), [`Result`](../error/index.md), [`Punctuated`](../punctuated/index.md), [`Peek`](../lookahead/index.md)

- `fn is_empty(self: &Self) -> bool`

- `fn lookahead1(self: &Self) -> Lookahead1<'a>` — [`Lookahead1`](../lookahead/index.md)

- `fn fork(self: &Self) -> Self`

- `fn error<T: Display>(self: &Self, message: T) -> Error` — [`Error`](../error/index.md)

- `fn step<F, R>(self: &Self, function: F) -> Result<R>` — [`Result`](../error/index.md)

- `fn span(self: &Self) -> Span`

- `fn cursor(self: &Self) -> Cursor<'a>` — [`Cursor`](../buffer/index.md)

- `fn check_unexpected(self: &Self) -> Result<()>` — [`Result`](../error/index.md)

#### Trait Implementations

##### `impl<'a> AnyDelimiter for crate::parse::ParseBuffer<'a>`

- `fn parse_any_delimiter(self: &Self) -> Result<(Delimiter, DelimSpan, ParseBuffer<'_>)>` — [`Result`](../error/index.md), [`ParseBuffer`](#parsebuffer)

##### `impl<'a> Debug for ParseBuffer<'a>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a> Display for ParseBuffer<'a>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a> Drop for ParseBuffer<'a>`

- `fn drop(self: &mut Self)`

##### `impl<'a> RefUnwindSafe for ParseBuffer<'a>`

##### `impl<'a> Speculative for crate::parse::ParseBuffer<'a>`

- `fn advance_to(self: &Self, fork: &Self)`

##### `impl<T> ToString for ParseBuffer<'a>`

- `fn to_string(self: &Self) -> String`

##### `impl<'a> UnwindSafe for ParseBuffer<'a>`

### `StepCursor<'c, 'a>`

```rust
struct StepCursor<'c, 'a> {
    scope: proc_macro2::Span,
    cursor: crate::buffer::Cursor<'c>,
    marker: std::marker::PhantomData<fn(crate::buffer::Cursor<'c>) -> crate::buffer::Cursor<'a>>,
}
```

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

- `fn error<T: Display>(self: Self, message: T) -> Error` — [`Error`](../error/index.md)

#### Trait Implementations

##### `impl<'c, 'a> Clone for StepCursor<'c, 'a>`

- `fn clone(self: &Self) -> Self`

##### `impl<'c, 'a> Copy for StepCursor<'c, 'a>`

##### `impl<'c, 'a> Deref for StepCursor<'c, 'a>`

- `type Target = Cursor<'c>`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for StepCursor<'c, 'a>`

- `type Target = T`

### `Nothing`

```rust
struct Nothing;
```

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

##### `impl Clone for Nothing`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Nothing`

##### `impl Parse for Nothing`

- `fn parse(_input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](#parsestream), [`Result`](../error/index.md)

##### `impl<T> Sealed for Nothing`

##### `impl<T> Spanned for Nothing`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Nothing`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

## Traits

### `Parse`

```rust
trait Parse: Sized { ... }
```

Parsing interface implemented by all types that can be parsed in a default
way from a token stream.

Refer to the [module documentation] for details about implementing and using
the `Parse` trait.


#### Required Methods

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

### `Parser`

```rust
trait Parser: Sized { ... }
```

Parser that can parse Rust tokens into a particular syntax tree node.

Refer to the [module documentation] for details about parsing in Syn.


#### Required Methods

- `type Output`

- `fn parse2(self: Self, tokens: TokenStream) -> Result<<Self as >::Output>`

  Parse a proc-macro2 token stream into the chosen syntax tree node.

- `fn parse(self: Self, tokens: proc_macro::TokenStream) -> Result<<Self as >::Output>`

  Parse tokens of source code into the chosen syntax tree node.

- `fn parse_str(self: Self, s: &str) -> Result<<Self as >::Output>`

  Parse a string of Rust code into the chosen syntax tree node.

## Type Aliases

### `ParseStream<'a>`

```rust
type ParseStream<'a> = &'a ParseBuffer<'a>;
```

Input to a Syn parser function.

See the methods of this type under the documentation of [`ParseBuffer`](#parsebuffer). For
an overview of parsing in Syn, refer to the [module documentation].


