*[syn](../index.md) / [lookahead](index.md)*

---

# Module `lookahead`

## Contents

- [Structs](#structs)
  - [`Lookahead1`](#lookahead1)
  - [`CommaSeparated`](#commaseparated)
  - [`End`](#end)
- [Enums](#enums)
  - [`TokenMarker`](#tokenmarker)
- [Traits](#traits)
  - [`Peek`](#peek)
- [Functions](#functions)
  - [`new`](#new)
  - [`peek_impl`](#peek-impl)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Lookahead1`](#lookahead1) | struct | Support for checking the next token in a stream to decide how to parse. |
| [`CommaSeparated`](#commaseparated) | struct |  |
| [`End`](#end) | struct | Pseudo-token used for peeking the end of a parse stream. |
| [`TokenMarker`](#tokenmarker) | enum |  |
| [`Peek`](#peek) | trait | Types that can be parsed by looking at just one token. |
| [`new`](#new) | fn |  |
| [`peek_impl`](#peek-impl) | fn |  |

## Structs

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

- <span id="lookahead1-error"></span>`fn error(self) -> Error` — [`Error`](../error/index.md#error)

### `CommaSeparated<'a>`

```rust
struct CommaSeparated<'a>(&'a [&'a str]);
```

*Defined in [`syn-2.0.111/src/lookahead.rs:150`](../../../.source_1765521767/syn-2.0.111/src/lookahead.rs#L150)*

#### Trait Implementations

##### `impl Display for CommaSeparated<'a>`

- <span id="commaseparated-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for CommaSeparated<'a>`

- <span id="commaseparated-to-string"></span>`fn to_string(&self) -> String`

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

##### `impl Clone for End`

- <span id="end-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for End`

##### `impl Peek for End`

##### `impl Sealed for End`

##### `impl Token for End`

- <span id="end-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool` — [`Cursor`](../buffer/index.md#cursor)

- <span id="end-display"></span>`fn display() -> &'static str`

## Enums

### `TokenMarker`

```rust
enum TokenMarker {
}
```

*Defined in [`syn-2.0.111/src/lookahead.rs:338`](../../../.source_1765521767/syn-2.0.111/src/lookahead.rs#L338)*

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

- [`End`](#end)
- [`PeekFn`](../ext/private/index.md#peekfn)
- `F`

## Functions

### `new`

```rust
fn new(scope: proc_macro2::Span, cursor: crate::buffer::Cursor<'_>) -> Lookahead1<'_>
```

*Defined in [`syn-2.0.111/src/lookahead.rs:69-75`](../../../.source_1765521767/syn-2.0.111/src/lookahead.rs#L69-L75)*

### `peek_impl`

```rust
fn peek_impl(lookahead: &Lookahead1<'_>, peek: fn(crate::buffer::Cursor<'_>) -> bool, display: fn() -> &'static str) -> bool
```

*Defined in [`syn-2.0.111/src/lookahead.rs:77-87`](../../../.source_1765521767/syn-2.0.111/src/lookahead.rs#L77-L87)*

