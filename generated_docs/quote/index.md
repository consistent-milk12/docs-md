# Crate `quote`

[![github]](https://github.com/dtolnay/quote)&ensp;[![crates-io]](https://crates.io/crates/quote)&ensp;[![docs-rs]](https://docs.rs/quote)



<br>

This crate provides the [`quote!`](#quote) macro for turning Rust syntax tree data
structures into tokens of source code.

Procedural macros in Rust receive a stream of tokens as input, execute
arbitrary Rust code to determine how to manipulate those tokens, and produce
a stream of tokens to hand back to the compiler to compile into the caller's
crate. Quasi-quoting is a solution to one piece of that &mdash; producing
tokens to return to the compiler.

The idea of quasi-quoting is that we write *code* that we treat as *data*.
Within the `quote!` macro, we can write what looks like code to our text
editor or IDE. We get all the benefits of the editor's brace matching,
syntax highlighting, indentation, and maybe autocompletion. But rather than
compiling that as code into the current crate, we can treat it as data, pass
it around, mutate it, and eventually hand it back to the compiler as tokens
to compile into the macro caller's crate.

This crate is motivated by the procedural macro use case, but is a
general-purpose Rust quasi-quoting library and is not specific to procedural
macros.

```toml
[dependencies]
quote = "1.0"
```

<br>

# Example

The following quasi-quoted block of code is something you might find in [a]
procedural macro having to do with data structure serialization. The `#var`
syntax performs interpolation of runtime variables into the quoted tokens.
Check out the documentation of the [`quote!`](#quote) macro for more detail about
the syntax. See also the [`quote_spanned!`](#quote-spanned) macro which is important for
implementing hygienic procedural macros.

```rust
use quote::quote;

let generics = "";
let where_clause = "";
let field_ty = "";
let item_ty = "";
let path = "";
let value = "";

let tokens = quote! {
    struct SerializeWith #generics #where_clause {
        value: &'a #field_ty,
        phantom: core::marker::PhantomData<#item_ty>,
    }

    impl #generics serde::Serialize for SerializeWith #generics #where_clause {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            #path(self.value, serializer)
        }
    }

    SerializeWith {
        value: #value,
        phantom: core::marker::PhantomData::<#item_ty>,
    }
};
```

<br>

# Non-macro code generators

When using `quote` in a build.rs or main.rs and writing the output out to a
file, consider having the code generator pass the tokens through
[prettyplease] before writing. This way if an error occurs in the generated
code it is convenient for a human to read and debug.


## Contents

- [Modules](#modules)
  - [`ext`](#ext)
  - [`format`](#format)
  - [`ident_fragment`](#ident_fragment)
  - [`to_tokens`](#to_tokens)
- [Traits](#traits)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
- [Macros](#macros)
  - [`__quote!`](#__quote)
  - [`__quote_spanned!`](#__quote_spanned)
  - [`format_ident!`](#format_ident)
  - [`quote!`](#quote)
  - [`quote_spanned!`](#quote_spanned)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ext`](#ext) | mod |  |
| [`format`](#format) | mod |  |
| [`ident_fragment`](#ident_fragment) | mod |  |
| [`to_tokens`](#to_tokens) | mod |  |
| [`unnamed`](#unnamed) | trait |  |
| [`unnamed`](#unnamed) | trait |  |
| [`unnamed`](#unnamed) | trait |  |
| [`__quote!`](#__quote) | macro |  |
| [`__quote_spanned!`](#__quote_spanned) | macro |  |
| [`format_ident!`](#format_ident) | macro | Formatting macro for constructing `Ident`s. |
| [`quote!`](#quote) | macro | The whole point. |
| [`quote_spanned!`](#quote_spanned) | macro | Same as `quote!`, but applies a given span to all tokens originating within |

## Modules

- [`ext`](ext/index.md) - 
- [`format`](format/index.md) - 
- [`ident_fragment`](ident_fragment/index.md) - 
- [`to_tokens`](to_tokens/index.md) - 

## Traits

## Macros

### `__quote!`

### `__quote_spanned!`

### `format_ident!`

Formatting macro for constructing `Ident`s.

<br>

# Syntax

Syntax is copied from the [`format!`](../clap_builder/error/format/index.md) macro, supporting both positional and
named arguments.

Only a limited set of formatting traits are supported. The current mapping
of format types to traits is:

* `{}` ⇒ [`IdentFragment`](#identfragment)
* `{:o}` ⇒ [`Octal`](std::fmt::Octal)
* `{:x}` ⇒ [`LowerHex`](std::fmt::LowerHex)
* `{:X}` ⇒ [`UpperHex`](std::fmt::UpperHex)
* `{:b}` ⇒ [`Binary`](std::fmt::Binary)

See [`std::fmt`](../anstream/fmt/index.md) for more information.

<br>

# IdentFragment

Unlike `format!`, this macro uses the [`IdentFragment`](#identfragment) formatting trait by
default. This trait is like `Display`, with a few differences:

* `IdentFragment` is only implemented for a limited set of types, such as
  unsigned integers and strings.
* `Ident` arguments will have their `r#` prefixes stripped, if present.


<br>

# Hygiene

The [`Span`](#span) of the first `Ident` argument is used as the span of the final
identifier, falling back to `Span::call_site` when no identifiers are
provided.

```rust
use quote::format_ident;
let ident = format_ident!("Ident");
// If `ident` is an Ident, the span of `my_ident` will be inherited from it.
let my_ident = format_ident!("My{}{}", ident, "IsCool");
assert_eq!(my_ident, "MyIdentIsCool");
```

Alternatively, the span can be overridden by passing the `span` named
argument.

```rust
use quote::format_ident;
const IGNORE_TOKENS: &'static str = stringify! {
let my_span = /* ... */;
};
let my_span = proc_macro2::Span::call_site();
format_ident!("MyIdent", span = my_span);
```


<p><br></p>

# Panics

This method will panic if the resulting formatted string is not a valid
identifier.

<br>

# Examples

Composing raw and non-raw identifiers:
```rust
use quote::format_ident;
let my_ident = format_ident!("My{}", "Ident");
assert_eq!(my_ident, "MyIdent");

let raw = format_ident!("r#Raw");
assert_eq!(raw, "r#Raw");

let my_ident_raw = format_ident!("{}Is{}", my_ident, raw);
assert_eq!(my_ident_raw, "MyIdentIsRaw");
```

Integer formatting options:
```rust
use quote::format_ident;
let num: u32 = 10;

let decimal = format_ident!("Id_{}", num);
assert_eq!(decimal, "Id_10");

let octal = format_ident!("Id_{:o}", num);
assert_eq!(octal, "Id_12");

let binary = format_ident!("Id_{:b}", num);
assert_eq!(binary, "Id_1010");

let lower_hex = format_ident!("Id_{:x}", num);
assert_eq!(lower_hex, "Id_a");

let upper_hex = format_ident!("Id_{:X}", num);
assert_eq!(upper_hex, "Id_A");
```

### `quote!`

The whole point.

Performs variable interpolation against the input and produces it as
[`proc_macro2::TokenStream`](../proc_macro2/imp/index.md).

Note: for returning tokens to the compiler in a procedural macro, use
`.into()` on the result to convert to [`proc_macro::TokenStream`](../proc_macro2/imp/index.md).

<br>

# Interpolation

Variable interpolation is done with `#var` (similar to `$var` in
`macro_rules!` macros). This grabs the `var` variable that is currently in
scope and inserts it in that location in the output tokens. Any type
implementing the [`ToTokens`](#totokens) trait can be interpolated. This includes most
Rust primitive types as well as most of the syntax tree types from the [Syn]
crate.

Repetition is done using `#(...)*` or `#(...),*` again similar to
`macro_rules!`. This iterates through the elements of any variable
interpolated within the repetition and inserts a copy of the repetition body
for each one. The variables in an interpolation may be a `Vec`, slice,
`BTreeSet`, or any `Iterator`.

- `#(#var)*` — no separators
- `#(#var),*` — the character before the asterisk is used as a separator
- `#( struct #var; )*` — the repetition can contain other tokens
- `#( #k => println!("{}", #v), )*` — even multiple interpolations

<br>

# Hygiene

Any interpolated tokens preserve the `Span` information provided by their
`ToTokens` implementation. Tokens that originate within the `quote!`
invocation are spanned with `Span::call_site()`.

A different span can be provided through the [`quote_spanned!`](#quote-spanned) macro.

<br>

# Return type

The macro evaluates to an expression of type `proc_macro2::TokenStream`.
Meanwhile Rust procedural macros are expected to return the type
`proc_macro::TokenStream`.

The difference between the two types is that `proc_macro` types are entirely
specific to procedural macros and cannot ever exist in code outside of a
procedural macro, while `proc_macro2` types may exist anywhere including
tests and non-macro code like main.rs and build.rs. This is why even the
procedural macro ecosystem is largely built around `proc_macro2`, because
that ensures the libraries are unit testable and accessible in non-macro
contexts.

There is a [`From`](../thiserror_impl/attr/index.md)-conversion in both directions so returning the output of
`quote!` from a procedural macro usually looks like `tokens.into()` or
`proc_macro::TokenStream::from(tokens)`.

<br>

# Examples

### Procedural macro

The structure of a basic procedural macro is as follows. Refer to the [Syn]
crate for further useful guidance on using `quote!` as part of a procedural
macro.

```rust
#[cfg(any())]
extern crate proc_macro;
extern crate proc_macro2;

#[cfg(any())]
use proc_macro::TokenStream;
use proc_macro2::TokenStream;
use quote::quote;

const IGNORE_TOKENS: &'static str = stringify! {
#[proc_macro_derive(HeapSize)]
};
pub fn derive_heap_size(input: TokenStream) -> TokenStream {
    // Parse the input and figure out what implementation to generate...
    const IGNORE_TOKENS: &'static str = stringify! {
    let name = /* ... */;
    let expr = /* ... */;
    };

    let name = 0;
    let expr = 0;

    let expanded = quote! {
        // The generated impl.
        impl heapsize::HeapSize for #name {
            fn heap_size_of_children(&self) -> usize {
                #expr
            }
        }
    };

    // Hand the output tokens back to the compiler.
    TokenStream::from(expanded)
}
```

<p><br></p>

### Combining quoted fragments

Usually you don't end up constructing an entire final `TokenStream` in one
piece. Different parts may come from different helper functions. The tokens
produced by `quote!` themselves implement `ToTokens` and so can be
interpolated into later `quote!` invocations to build up a final result.

```rust
use quote::quote;

let type_definition = quote! {...};
let methods = quote! {...};

let tokens = quote! {
    #type_definition
    #methods
};
```

<p><br></p>

### Constructing identifiers

Suppose we have an identifier `ident` which came from somewhere in a macro
input and we need to modify it in some way for the macro output. Let's
consider prepending the identifier with an underscore.

Simply interpolating the identifier next to an underscore will not have the
behavior of concatenating them. The underscore and the identifier will
continue to be two separate tokens as if you had written `_ x`.

```rust
use proc_macro2::{self as syn, Span};
use quote::quote;

let ident = syn::Ident::new("i", Span::call_site());

// incorrect
quote! {
    let mut _#ident = 0;
}
;
```

The solution is to build a new identifier token with the correct value. As
this is such a common case, the [`format_ident!`](#format-ident) macro provides a
convenient utility for doing so correctly.

```rust
use proc_macro2::{Ident, Span};
use quote::{format_ident, quote};

let ident = Ident::new("i", Span::call_site());

let varname = format_ident!("_{}", ident);
quote! {
    let mut #varname = 0;
}
;
```

Alternatively, the APIs provided by Syn and proc-macro2 can be used to
directly build the identifier. This is roughly equivalent to the above, but
will not handle `ident` being a raw identifier.

```rust
use proc_macro2::{self as syn, Span};
use quote::quote;

let ident = syn::Ident::new("i", Span::call_site());

let concatenated = format!("_{}", ident);
let varname = syn::Ident::new(&concatenated, ident.span());
quote! {
    let mut #varname = 0;
}
;
```

<p><br></p>

### Making method calls

Let's say our macro requires some type specified in the macro input to have
a constructor called `new`. We have the type in a variable called
`field_type` of type `syn::Type` and want to invoke the constructor.

```rust
use quote::quote;

let field_type = quote!(...);

// incorrect
quote! {
    let value = #field_type::new();
}
;
```

This works only sometimes. If `field_type` is `String`, the expanded code
contains `String::new()` which is fine. But if `field_type` is something
like `Vec<i32>` then the expanded code is `Vec<i32>::new()` which is invalid
syntax. Ordinarily in handwritten Rust we would write `Vec::<i32>::new()`
but for macros often the following is more convenient.

```rust
use quote::quote;

let field_type = quote!(...);

quote! {
    let value = <#field_type>::new();
}
;
```

This expands to `<Vec<i32>>::new()` which behaves correctly.

A similar pattern is appropriate for trait methods.

```rust
use quote::quote;

let field_type = quote!(...);

quote! {
    let value = <#field_type as core::default::Default>::default();
}
;
```

<p><br></p>

### Interpolating text inside of doc comments

Neither doc comments nor string literals get interpolation behavior in
quote:

```compile_fail
quote! {
    /// try to interpolate: #ident
    ///
    /// ...
}
```

```compile_fail
quote! {
    #[doc = "try to interpolate: #ident"]
}
```

Instead the best way to build doc comments that involve variables is by
formatting the doc string literal outside of quote.

```rust
use proc_macro2::{Ident, Span};
use quote::quote;

const IGNORE: &str = stringify! {
let msg = format!(...);
};

let ident = Ident::new("var", Span::call_site());
let msg = format!("try to interpolate: {}", ident);
quote! {
    #[doc = #msg]
    ///
    /// ...
}
;
```

<p><br></p>

### Indexing into a tuple struct

When interpolating indices of a tuple or tuple struct, we need them not to
appears suffixed as integer literals by interpolating them as `syn::Index`
instead.

```compile_fail
let i = 0usize..self.fields.len();

// expands to 0 + self.0usize.heap_size() + self.1usize.heap_size() + ...
// which is not valid syntax
quote! {
    0 #( + self.#i.heap_size() )*
}
```

```rust
use proc_macro2::{Ident, TokenStream};
use quote::quote;

mod syn {
    use proc_macro2::{Literal, TokenStream};
    use quote::{ToTokens, TokenStreamExt};

    pub struct Index(usize);

    impl From<usize> for Index {
        fn from(i: usize) -> Self {
            Index(i)
        }
    }

    impl ToTokens for Index {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(Literal::usize_unsuffixed(self.0));
        }
    }
}

struct Struct {
    fields: Vec<Ident>,
}

impl Struct {
    fn example(&self) -> TokenStream {
let i = (0..self.fields.len()).map(syn::Index::from);

// expands to 0 + self.0.heap_size() + self.1.heap_size() + ...
quote! {
    0 #( + self.#i.heap_size() )*
}
    }
}
```

### `quote_spanned!`

Same as `quote!`, but applies a given span to all tokens originating within
the macro invocation.

<br>

# Syntax

A span expression of type [`Span`](#span), followed by `=>`, followed by the tokens
to quote. The span expression should be brief &mdash; use a variable for
anything more than a few characters. There should be no space before the
`=>` token.

```rust
use proc_macro2::Span;
use quote::quote_spanned;

const IGNORE_TOKENS: &'static str = stringify! {
let span = /* ... */;
};
let span = Span::call_site();
let init = 0;

// On one line, use parentheses.
let tokens = quote_spanned!(span=> Box::into_raw(Box::new(#init)));

// On multiple lines, place the span at the top and use braces.
let tokens = quote_spanned! {span=>
    Box::into_raw(Box::new(#init))
};
```

The lack of space before the `=>` should look jarring to Rust programmers
and this is intentional. The formatting is designed to be visibly
off-balance and draw the eye a particular way, due to the span expression
being evaluated in the context of the procedural macro and the remaining
tokens being evaluated in the generated code.

<br>

# Hygiene

Any interpolated tokens preserve the `Span` information provided by their
`ToTokens` implementation. Tokens that originate within the `quote_spanned!`
invocation are spanned with the given span argument.

<br>

# Example

The following procedural macro code uses `quote_spanned!` to assert that a
particular Rust type implements the [`Sync`](../miniz_oxide/index.md) trait so that references can be
safely shared between threads.

```rust
use quote::{quote_spanned, TokenStreamExt, ToTokens};
use proc_macro2::{Span, TokenStream};

struct Type;

impl Type {
    fn span(&self) -> Span {
        Span::call_site()
    }
}

impl ToTokens for Type {
    fn to_tokens(&self, _tokens: &mut TokenStream) {}
}

let ty = Type;
let call_site = Span::call_site();

let ty_span = ty.span();
let assert_sync = quote_spanned! {ty_span=>
    struct _AssertSync where #ty: Sync;
};
```

If the assertion fails, the user will see an error like the following. The
input span of their type is highlighted in the error.

```text
error[E0277]: the trait bound `*const (): std::marker::Sync` is not satisfied
  --> src/main.rs:10:21
   |
10 |     static ref PTR: *const () = &();
   |                     ^^^^^^^^^ `*const ()` cannot be shared between threads safely
```

In this example it is important for the where-clause to be spanned with the
line/column information of the user's input type so that error messages are
placed appropriately by the compiler.

