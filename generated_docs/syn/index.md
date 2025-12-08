# Crate `syn`

[![github]](https://github.com/dtolnay/syn)&ensp;[![crates-io]](https://crates.io/crates/syn)&ensp;[![docs-rs]](crate)



<br>

Syn is a parsing library for parsing a stream of Rust tokens into a syntax
tree of Rust source code.

Currently this library is geared toward use in Rust procedural macros, but
contains some APIs that may be useful more generally.

- **Data structures** — Syn provides a complete syntax tree that can
  represent any valid Rust source code. The syntax tree is rooted at
  `syn::File` which represents a full source file, but there are other
  entry points that may be useful to procedural macros including
  `syn::Item`, `syn::Expr` and `syn::Type`.

- **Derives** — Of particular interest to derive macros is
  `syn::DeriveInput` which is any of the three legal input items to a
  derive macro. An example below shows using this type in a library that can
  derive implementations of a user-defined trait.

- **Parsing** — Parsing in Syn is built around [parser functions] with the
  signature `fn(ParseStream) -> Result<T>`. Every syntax tree node defined
  by Syn is individually parsable and may be used as a building block for
  custom syntaxes, or you may dream up your own brand new syntax without
  involving any of our syntax tree types.

- **Location information** — Every token parsed by Syn is associated with a
  `Span` that tracks line and column information back to the source of that
  token. These spans allow a procedural macro to display detailed error
  messages pointing to all the right places in the user's code. There is an
  example of this below.

- **Feature flags** — Functionality is aggressively feature gated so your
  procedural macros enable only what they need, and do not pay in compile
  time for all the rest.






<br>

# Example of a derive macro

The canonical derive macro using Syn looks like this. We write an ordinary
Rust function tagged with a `proc_macro_derive` attribute and the name of
the trait we are deriving. Any time that derive appears in the user's code,
the Rust compiler passes their data structure as tokens into our macro. We
get to execute arbitrary Rust code to figure out what to do with those
tokens, then hand some tokens back to the compiler to compile into the
user's crate.

```toml
[dependencies]
syn = "2.0"
quote = "1.0"

[lib]
proc-macro = true
```

```rust
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

const IGNORE_TOKENS: &str = stringify! {
#[proc_macro_derive(MyMacro)]
};
pub fn my_macro(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        // ...
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
```

The `heapsize` example directory shows a complete working implementation
of a derive macro. The example derives a `HeapSize` trait which computes an
estimate of the amount of heap memory owned by a value.

```rust
pub trait HeapSize {
    /// Total number of bytes of heap memory owned by `self`.
    fn heap_size_of_children(&self) -> usize;
}
```

The derive macro allows users to write `#[derive(HeapSize)]` on data
structures in their program.

```rust
const IGNORE_TOKENS: &str = stringify! {
#[derive(HeapSize)]
};
struct Demo<'a, T: ?Sized> {
    a: Box<T>,
    b: u8,
    c: &'a str,
    d: String,
}
```

<p><br></p>

# Spans and error reporting

The token-based procedural macro API provides great control over where the
compiler's error messages are displayed in user code. Consider the error the
user sees if one of their field types does not implement `HeapSize`.

```rust
const IGNORE_TOKENS: &str = stringify! {
#[derive(HeapSize)]
};
struct Broken {
    ok: String,
    bad: std::thread::Thread,
}
```

By tracking span information all the way through the expansion of a
procedural macro as shown in the `heapsize` example, token-based macros in
Syn are able to trigger errors that directly pinpoint the source of the
problem.

```text
error[E0277]: the trait bound `std::thread::Thread: HeapSize` is not satisfied
 --> src/main.rs:7:5
  |
7 |     bad: std::thread::Thread,
  |     ^^^^^^^^^^^^^^^^^^^^^^^^ the trait `HeapSize` is not implemented for `Thread`
```

<br>

# Parsing a custom syntax

The `lazy-static` example directory shows the implementation of a
`functionlike!(...)` procedural macro in which the input tokens are parsed
using Syn's parsing API.

The example reimplements the popular `lazy_static` crate from crates.io as a
procedural macro.

```rust
macro_rules! lazy_static {
    ($($tt:tt)*) => {}
}

lazy_static! {
    static ref USERNAME: Regex = Regex::new("^[a-z0-9_-]{3,16}$").unwrap();
}
```

The implementation shows how to trigger custom warnings and error messages
on the macro input.

```text
warning: come on, pick a more creative name
  --> src/main.rs:10:16
   |
10 |     static ref FOO: String = "lazy_static".to_owned();
   |                ^^^
```

<br>

# Testing

When testing macros, we often care not just that the macro can be used
successfully but also that when the macro is provided with invalid input it
produces maximally helpful error messages. Consider using the `trybuild`
crate to write tests for errors that are emitted by your macro or errors
detected by the Rust compiler in the expanded code following misuse of the
macro. Such tests help avoid regressions from later refactors that
mistakenly make an error no longer trigger or be less helpful than it used
to be.

<br>

# Debugging

When developing a procedural macro it can be helpful to look at what the
generated code looks like. Use `cargo rustc -- -Zunstable-options
--pretty=expanded` or the `cargo expand` subcommand.

To show the expanded code for some crate that uses your procedural macro,
run `cargo expand` from that crate. To show the expanded code for one of
your own test cases, run `cargo expand --test the_test_case` where the last
argument is the name of the test file without the `.rs` extension.

This write-up by Brandon W Maister discusses debugging in more detail:
[Debugging Rust's new Custom Derive system][debugging].

<br>

# Optional features

Syn puts a lot of functionality behind optional features in order to
optimize compile time for the most common use cases. The following features
are available.

- **`derive`** *(enabled by default)* — Data structures for representing the
  possible input to a derive macro, including structs and enums and types.
- **`full`** — Data structures for representing the syntax tree of all valid
  Rust source code, including items and expressions.
- **`parsing`** *(enabled by default)* — Ability to parse input tokens into
  a syntax tree node of a chosen type.
- **`printing`** *(enabled by default)* — Ability to print a syntax tree
  node as tokens of Rust source code.
- **`visit`** — Trait for traversing a syntax tree.
- **`visit-mut`** — Trait for traversing and mutating in place a syntax
  tree.
- **`fold`** — Trait for transforming an owned syntax tree.
- **`clone-impls`** *(enabled by default)* — Clone impls for all syntax tree
  types.
- **`extra-traits`** — Debug, Eq, PartialEq, Hash impls for all syntax tree
  types.
- **`proc-macro`** *(enabled by default)* — Runtime dependency on the
  dynamic library libproc_macro from rustc toolchain.

## Contents

- [Modules](#modules)
  - [`macros`](#macros)
  - [`group`](#group)
  - [`token`](#token)
  - [`attr`](#attr)
  - [`bigint`](#bigint)
  - [`buffer`](#buffer)
  - [`classify`](#classify)
  - [`custom_keyword`](#custom_keyword)
  - [`custom_punctuation`](#custom_punctuation)
  - [`data`](#data)
  - [`derive`](#derive)
  - [`drops`](#drops)
  - [`error`](#error)
  - [`expr`](#expr)
  - [`ext`](#ext)
  - [`file`](#file)
  - [`fixup`](#fixup)
  - [`generics`](#generics)
  - [`ident`](#ident)
  - [`item`](#item)
  - [`lifetime`](#lifetime)
  - [`lit`](#lit)
  - [`lookahead`](#lookahead)
  - [`mac`](#mac)
  - [`meta`](#meta)
  - [`op`](#op)
  - [`parse`](#parse)
  - [`parse_macro_input`](#parse_macro_input)
  - [`parse_quote`](#parse_quote)
  - [`pat`](#pat)
  - [`path`](#path)
  - [`precedence`](#precedence)
  - [`print`](#print)
  - [`punctuated`](#punctuated)
  - [`restriction`](#restriction)
  - [`sealed`](#sealed)
  - [`span`](#span)
  - [`spanned`](#spanned)
  - [`stmt`](#stmt)
  - [`thread`](#thread)
  - [`tt`](#tt)
  - [`ty`](#ty)
  - [`verbatim`](#verbatim)
  - [`whitespace`](#whitespace)
  - [`gen`](#gen)
  - [`unnamed`](#unnamed)
- [Structs](#structs)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
- [Enums](#enums)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
- [Functions](#functions)
  - [`parse`](#parse)
  - [`parse2`](#parse2)
  - [`parse_str`](#parse_str)
  - [`parse_file`](#parse_file)
- [Type Aliases](#type-aliases)
  - [`unnamed`](#unnamed)
- [Macros](#macros)
  - [`parenthesized!`](#parenthesized)
  - [`braced!`](#braced)
  - [`bracketed!`](#bracketed)
  - [`Token!`](#token)
  - [`custom_keyword!`](#custom_keyword)
  - [`custom_punctuation!`](#custom_punctuation)
  - [`parse_macro_input!`](#parse_macro_input)
  - [`parse_quote!`](#parse_quote)
  - [`parse_quote_spanned!`](#parse_quote_spanned)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`macros`](#macros) | mod |  |
| [`group`](#group) | mod |  |
| [`token`](#token) | mod | Tokens representing Rust punctuation, keywords, and delimiters. |
| [`attr`](#attr) | mod |  |
| [`bigint`](#bigint) | mod |  |
| [`buffer`](#buffer) | mod | A stably addressed token buffer supporting efficient traversal based on a |
| [`classify`](#classify) | mod |  |
| [`custom_keyword`](#custom_keyword) | mod |  |
| [`custom_punctuation`](#custom_punctuation) | mod |  |
| [`data`](#data) | mod |  |
| [`derive`](#derive) | mod |  |
| [`drops`](#drops) | mod |  |
| [`error`](#error) | mod |  |
| [`expr`](#expr) | mod |  |
| [`ext`](#ext) | mod | Extension traits to provide parsing methods on foreign types. |
| [`file`](#file) | mod |  |
| [`fixup`](#fixup) | mod |  |
| [`generics`](#generics) | mod |  |
| [`ident`](#ident) | mod |  |
| [`item`](#item) | mod |  |
| [`lifetime`](#lifetime) | mod |  |
| [`lit`](#lit) | mod |  |
| [`lookahead`](#lookahead) | mod |  |
| [`mac`](#mac) | mod |  |
| [`meta`](#meta) | mod | Facility for interpreting structured content inside of an `Attribute`. |
| [`op`](#op) | mod |  |
| [`parse`](#parse) | mod | Parsing interface for parsing a token stream into a syntax tree node. |
| [`parse_macro_input`](#parse_macro_input) | mod |  |
| [`parse_quote`](#parse_quote) | mod |  |
| [`pat`](#pat) | mod |  |
| [`path`](#path) | mod |  |
| [`precedence`](#precedence) | mod |  |
| [`print`](#print) | mod |  |
| [`punctuated`](#punctuated) | mod | A punctuated sequence of syntax tree nodes separated by punctuation. |
| [`restriction`](#restriction) | mod |  |
| [`sealed`](#sealed) | mod |  |
| [`span`](#span) | mod |  |
| [`spanned`](#spanned) | mod | A trait that can provide the `Span` of the complete contents of a syntax |
| [`stmt`](#stmt) | mod |  |
| [`thread`](#thread) | mod |  |
| [`tt`](#tt) | mod |  |
| [`ty`](#ty) | mod |  |
| [`verbatim`](#verbatim) | mod |  |
| [`whitespace`](#whitespace) | mod |  |
| [`gen`](#gen) | mod |  |
| [`unnamed`](#unnamed) | mod |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`parse`](#parse) | fn | Parse tokens of source code into the chosen syntax tree node. |
| [`parse2`](#parse2) | fn | Parse a proc-macro2 token stream into the chosen syntax tree node. |
| [`parse_str`](#parse_str) | fn | Parse a string of Rust code into the chosen syntax tree node. |
| [`parse_file`](#parse_file) | fn | Parse the content of a file of Rust code. |
| [`unnamed`](#unnamed) | type |  |
| [`parenthesized!`](#parenthesized) | macro | Parse a set of parentheses and expose their content to subsequent parsers. |
| [`braced!`](#braced) | macro | Parse a set of curly braces and expose their content to subsequent parsers. |
| [`bracketed!`](#bracketed) | macro | Parse a set of square brackets and expose their content to subsequent |
| [`Token!`](#token) | macro | A type-macro that expands to the name of the Rust type representation of a |
| [`custom_keyword!`](#custom_keyword) | macro | Define a type that supports parsing and printing a given identifier as if it |
| [`custom_punctuation!`](#custom_punctuation) | macro | Define a type that supports parsing and printing a multi-character symbol |
| [`parse_macro_input!`](#parse_macro_input) | macro | Parse the input TokenStream of a macro, triggering a compile error if the |
| [`parse_quote!`](#parse_quote) | macro | Quasi-quotation macro that accepts input like the [`quote!`] macro but uses |
| [`parse_quote_spanned!`](#parse_quote_spanned) | macro | This macro is [`parse_quote!`] + [`quote_spanned!`][quote::quote_spanned]. |

## Modules

- [`macros`](macros/index.md) - 
- [`group`](group/index.md) - 
- [`token`](token/index.md) - Tokens representing Rust punctuation, keywords, and delimiters.
- [`attr`](attr/index.md) - 
- [`bigint`](bigint/index.md) - 
- [`buffer`](buffer/index.md) - A stably addressed token buffer supporting efficient traversal based on a
- [`classify`](classify/index.md) - 
- [`custom_keyword`](custom_keyword/index.md) - 
- [`custom_punctuation`](custom_punctuation/index.md) - 
- [`data`](data/index.md) - 
- [`derive`](derive/index.md) - 
- [`drops`](drops/index.md) - 
- [`error`](error/index.md) - 
- [`expr`](expr/index.md) - 
- [`ext`](ext/index.md) - Extension traits to provide parsing methods on foreign types.
- [`file`](file/index.md) - 
- [`fixup`](fixup/index.md) - 
- [`generics`](generics/index.md) - 
- [`ident`](ident/index.md) - 
- [`item`](item/index.md) - 
- [`lifetime`](lifetime/index.md) - 
- [`lit`](lit/index.md) - 
- [`lookahead`](lookahead/index.md) - 
- [`mac`](mac/index.md) - 
- [`meta`](meta/index.md) - Facility for interpreting structured content inside of an `Attribute`.
- [`op`](op/index.md) - 
- [`parse`](parse/index.md) - Parsing interface for parsing a token stream into a syntax tree node.
- [`parse_macro_input`](parse_macro_input/index.md) - 
- [`parse_quote`](parse_quote/index.md) - 
- [`pat`](pat/index.md) - 
- [`path`](path/index.md) - 
- [`precedence`](precedence/index.md) - 
- [`print`](print/index.md) - 
- [`punctuated`](punctuated/index.md) - A punctuated sequence of syntax tree nodes separated by punctuation.
- [`restriction`](restriction/index.md) - 
- [`sealed`](sealed/index.md) - 
- [`span`](span/index.md) - 
- [`spanned`](spanned/index.md) - A trait that can provide the `Span` of the complete contents of a syntax
- [`stmt`](stmt/index.md) - 
- [`thread`](thread/index.md) - 
- [`tt`](tt/index.md) - 
- [`ty`](ty/index.md) - 
- [`verbatim`](verbatim/index.md) - 
- [`whitespace`](whitespace/index.md) - 
- [`gen`](gen/index.md) - 
- [`visit_mut`](visit_mut/index.md) - 

## Structs

### `Attribute`

```rust
struct Attribute {
    pub pound_token: token::Pound,
    pub style: AttrStyle,
    pub bracket_token: token::Bracket,
    pub meta: Meta,
}
```

An attribute, like `#[repr(transparent)]`.

<br>

# Syntax

Rust has six types of attributes.

- Outer attributes like `#[repr(transparent)]`. These appear outside or
  in front of the item they describe.

- Inner attributes like `#![feature(proc_macro)]`. These appear inside
  of the item they describe, usually a module.

- Outer one-line doc comments like `/// Example`.

- Inner one-line doc comments like `//! Please file an issue`.

- Outer documentation blocks `/** Example */`.

- Inner documentation blocks `/*! Please file an issue */`.

The `style` field of type `AttrStyle` distinguishes whether an attribute
is outer or inner.

Every attribute has a `path` that indicates the intended interpretation
of the rest of the attribute's contents. The path and the optional
additional contents are represented together in the `meta` field of the
attribute in three possible varieties:

- Meta::Path &mdash; attributes whose information content conveys just a
  path, for example the `#[test]` attribute.

- Meta::List &mdash; attributes that carry arbitrary tokens after the
  path, surrounded by a delimiter (parenthesis, bracket, or brace). For
  example `#[derive(Copy)]` or `#[precondition(x < 5)]`.

- Meta::NameValue &mdash; attributes with an `=` sign after the path,
  followed by a Rust expression. For example `#[path =
  "sys/windows.rs"]`.

All doc comments are represented in the NameValue style with a path of
"doc", as this is how they are processed by the compiler and by
`macro_rules!` macros.

```text
#[derive(Copy, Clone)]
  ~~~~~~Path
  ^^^^^^^^^^^^^^^^^^^Meta::List

#[path = "sys/windows.rs"]
  ~~~~Path
  ^^^^^^^^^^^^^^^^^^^^^^^Meta::NameValue

#[test]
  ^^^^Meta::Path
```

<br>

# Parsing from tokens to Attribute

This type does not implement the [`Parse`](parse/index.md) trait and thus cannot be
parsed directly by `ParseStream::parse`. Instead use
`ParseStream::call` with one of the two parser functions
`Attribute::parse_outer` or `Attribute::parse_inner` depending on
which you intend to parse.



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

<p><br></p>

# Parsing from Attribute to structured arguments

The grammar of attributes in Rust is very flexible, which makes the
syntax tree not that useful on its own. In particular, arguments of the
`Meta::List` variety of attribute are held in an arbitrary `tokens:
TokenStream`. Macros are expected to check the `path` of the attribute,
decide whether they recognize it, and then parse the remaining tokens
according to whatever grammar they wish to require for that kind of
attribute. Use `parse_args()` to parse those tokens into the expected
data structure.

<p><br></p>

# Doc comments

The compiler transforms doc comments, such as `/// comment` and `/*!
comment */`, into attributes before macros are expanded. Each comment is
expanded into an attribute of the form `#[doc = r"comment"]`.

As an example, the following `mod` items are expanded identically:

```rust
use syn::{ItemMod, parse_quote};
let doc: ItemMod = parse_quote! {
    /// Single line doc comments
    /// We write so many!
    /**
     * Multi-line comments...
     * May span many lines
     */
    mod example {
        //! Of course, they can be inner too
        /*! And fit in a single line */
    }
};
let attr: ItemMod = parse_quote! {
    #[doc = r" Single line doc comments"]
    #[doc = r" We write so many!"]
    #[doc = r"
     * Multi-line comments...
     * May span many lines
     "]
    mod example {
        #![doc = r" Of course, they can be inner too"]
        #![doc = r" And fit in a single line "]
    }
};
assert_eq!(doc, attr);
```

#### Implementations

- <span id="attribute-path"></span>`fn path(&self) -> &Path` — [`Path`](#path)

- <span id="attribute-parse-args"></span>`fn parse_args<T: Parse>(&self) -> Result<T>` — [`Result`](#result)

- <span id="attribute-parse-args-with"></span>`fn parse_args_with<F: Parser>(&self, parser: F) -> Result<<F as >::Output>` — [`Result`](#result), [`Parser`](parse/index.md)

- <span id="attribute-parse-nested-meta"></span>`fn parse_nested_meta(&self, logic: impl FnMut(ParseNestedMeta<'_>) -> Result<()>) -> Result<()>` — [`ParseNestedMeta`](meta/index.md), [`Result`](#result)

- <span id="attribute-parse-outer"></span>`fn parse_outer(input: ParseStream<'_>) -> Result<Vec<Self>>` — [`ParseStream`](parse/index.md), [`Result`](#result)

- <span id="attribute-parse-inner"></span>`fn parse_inner(input: ParseStream<'_>) -> Result<Vec<Self>>` — [`ParseStream`](parse/index.md), [`Result`](#result)

#### Trait Implementations

##### `impl Clone for crate::Attribute`

- <span id="crateattribute-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Attribute`

- <span id="crateattribute-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Attribute`

##### `impl Hash for crate::Attribute`

- <span id="crateattribute-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::Attribute`

- <span id="crateattribute-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Attribute`

##### `impl<T> Spanned for Attribute`

- <span id="attribute-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::attr::Attribute`

- <span id="crateattrattribute-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `MetaList`

```rust
struct MetaList {
    pub path: crate::path::Path,
    pub delimiter: crate::mac::MacroDelimiter,
    pub tokens: proc_macro2::TokenStream,
}
```

A structured list within an attribute, like `derive(Copy, Clone)`.

#### Implementations

- <span id="cratemetalist-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::MetaList`

- <span id="cratemetalist-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::MetaList`

- <span id="cratemetalist-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::MetaList`

##### `impl Hash for crate::MetaList`

- <span id="cratemetalist-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::attr::MetaList`

- <span id="crateattrmetalist-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::MetaList`

- <span id="cratemetalist-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for MetaList`

##### `impl<T> Spanned for MetaList`

- <span id="metalist-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::attr::MetaList`

- <span id="crateattrmetalist-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `MetaNameValue`

```rust
struct MetaNameValue {
    pub path: crate::path::Path,
    pub eq_token: token::Eq,
    pub value: crate::expr::Expr,
}
```

A name-value pair within an attribute, like `feature = "nightly"`.

#### Implementations

- <span id="cratemetanamevalue-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::MetaNameValue`

- <span id="cratemetanamevalue-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::MetaNameValue`

- <span id="cratemetanamevalue-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::MetaNameValue`

##### `impl Hash for crate::MetaNameValue`

- <span id="cratemetanamevalue-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::attr::MetaNameValue`

- <span id="crateattrmetanamevalue-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::MetaNameValue`

- <span id="cratemetanamevalue-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for MetaNameValue`

##### `impl<T> Spanned for MetaNameValue`

- <span id="metanamevalue-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::attr::MetaNameValue`

- <span id="crateattrmetanamevalue-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Field`

```rust
struct Field {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub mutability: crate::restriction::FieldMutability,
    pub ident: Option<crate::ident::Ident>,
    pub colon_token: Option<token::Colon>,
    pub ty: crate::ty::Type,
}
```

A field of a struct or enum variant.

#### Fields

- **`ident`**: `Option<crate::ident::Ident>`

  Name of the field, if any.
  
  Fields of tuple structs have no names.

#### Implementations

- <span id="cratedatafield-parse-named"></span>`fn parse_named(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

- <span id="cratedatafield-parse-unnamed"></span>`fn parse_unnamed(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

#### Trait Implementations

##### `impl Clone for crate::Field`

- <span id="cratefield-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Field`

- <span id="cratefield-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Field`

##### `impl Hash for crate::Field`

- <span id="cratefield-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::Field`

- <span id="cratefield-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Field`

##### `impl<T> Spanned for Field`

- <span id="field-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::data::Field`

- <span id="cratedatafield-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `FieldsNamed`

```rust
struct FieldsNamed {
    pub brace_token: token::Brace,
    pub named: crate::punctuated::Punctuated<Field, token::Comma>,
}
```

Named fields of a struct or struct variant such as `Point { x: f64,
y: f64 }`.

#### Implementations

- <span id="cratefieldsnamed-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::FieldsNamed`

- <span id="cratefieldsnamed-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::FieldsNamed`

- <span id="cratefieldsnamed-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldsNamed`

##### `impl Hash for crate::FieldsNamed`

- <span id="cratefieldsnamed-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::data::FieldsNamed`

- <span id="cratedatafieldsnamed-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::FieldsNamed`

- <span id="cratefieldsnamed-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for FieldsNamed`

##### `impl<T> Spanned for FieldsNamed`

- <span id="fieldsnamed-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::data::FieldsNamed`

- <span id="cratedatafieldsnamed-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `FieldsUnnamed`

```rust
struct FieldsUnnamed {
    pub paren_token: token::Paren,
    pub unnamed: crate::punctuated::Punctuated<Field, token::Comma>,
}
```

Unnamed fields of a tuple struct or tuple variant such as `Some(T)`.

#### Implementations

- <span id="cratefieldsunnamed-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::FieldsUnnamed`

- <span id="cratefieldsunnamed-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::FieldsUnnamed`

- <span id="cratefieldsunnamed-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldsUnnamed`

##### `impl Hash for crate::FieldsUnnamed`

- <span id="cratefieldsunnamed-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::data::FieldsUnnamed`

- <span id="cratedatafieldsunnamed-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::FieldsUnnamed`

- <span id="cratefieldsunnamed-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for FieldsUnnamed`

##### `impl<T> Spanned for FieldsUnnamed`

- <span id="fieldsunnamed-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::data::FieldsUnnamed`

- <span id="cratedatafieldsunnamed-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Variant`

```rust
struct Variant {
    pub attrs: Vec<crate::attr::Attribute>,
    pub ident: crate::ident::Ident,
    pub fields: Fields,
    pub discriminant: Option<(token::Eq, crate::expr::Expr)>,
}
```

An enum variant.

#### Fields

- **`ident`**: `crate::ident::Ident`

  Name of the variant.

- **`fields`**: `Fields`

  Content stored in the variant.

- **`discriminant`**: `Option<(token::Eq, crate::expr::Expr)>`

  Explicit discriminant: `Variant = 1`

#### Trait Implementations

##### `impl Clone for crate::Variant`

- <span id="cratevariant-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Variant`

- <span id="cratevariant-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Variant`

##### `impl Hash for crate::Variant`

- <span id="cratevariant-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::data::Variant`

- <span id="cratedatavariant-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Variant`

- <span id="cratevariant-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Variant`

##### `impl<T> Spanned for Variant`

- <span id="variant-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::data::Variant`

- <span id="cratedatavariant-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `DataEnum`

```rust
struct DataEnum {
    pub enum_token: token::Enum,
    pub brace_token: token::Brace,
    pub variants: crate::punctuated::Punctuated<crate::data::Variant, token::Comma>,
}
```

An enum input to a `proc_macro_derive` macro.

#### Implementations

- <span id="cratedataenum-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::DataEnum`

- <span id="cratedataenum-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::DataEnum`

- <span id="cratedataenum-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::DataEnum`

##### `impl Hash for crate::DataEnum`

- <span id="cratedataenum-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::DataEnum`

- <span id="cratedataenum-eq"></span>`fn eq(&self, other: &Self) -> bool`

### `DataStruct`

```rust
struct DataStruct {
    pub struct_token: token::Struct,
    pub fields: crate::data::Fields,
    pub semi_token: Option<token::Semi>,
}
```

A struct input to a `proc_macro_derive` macro.

#### Implementations

- <span id="cratedatastruct-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::DataStruct`

- <span id="cratedatastruct-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::DataStruct`

- <span id="cratedatastruct-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::DataStruct`

##### `impl Hash for crate::DataStruct`

- <span id="cratedatastruct-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::DataStruct`

- <span id="cratedatastruct-eq"></span>`fn eq(&self, other: &Self) -> bool`

### `DataUnion`

```rust
struct DataUnion {
    pub union_token: token::Union,
    pub fields: crate::data::FieldsNamed,
}
```

An untagged union input to a `proc_macro_derive` macro.

#### Implementations

- <span id="cratedataunion-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::DataUnion`

- <span id="cratedataunion-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::DataUnion`

- <span id="cratedataunion-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::DataUnion`

##### `impl Hash for crate::DataUnion`

- <span id="cratedataunion-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::DataUnion`

- <span id="cratedataunion-eq"></span>`fn eq(&self, other: &Self) -> bool`

### `DeriveInput`

```rust
struct DeriveInput {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub data: Data,
}
```

Data structure sent to a `proc_macro_derive` macro.

#### Trait Implementations

##### `impl Clone for crate::DeriveInput`

- <span id="cratederiveinput-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::DeriveInput`

- <span id="cratederiveinput-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::DeriveInput`

##### `impl Hash for crate::DeriveInput`

- <span id="cratederiveinput-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::derive::DeriveInput`

- <span id="cratederivederiveinput-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::DeriveInput`

- <span id="cratederiveinput-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for DeriveInput`

##### `impl<T> Spanned for DeriveInput`

- <span id="deriveinput-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::derive::DeriveInput`

- <span id="cratederivederiveinput-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

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

- <span id="error-new-spanned"></span>`fn new_spanned<T: ToTokens, U: Display>(tokens: T, message: U) -> Self`

- <span id="error-span"></span>`fn span(&self) -> Span`

- <span id="error-to-compile-error"></span>`fn to_compile_error(&self) -> TokenStream`

- <span id="error-into-compile-error"></span>`fn into_compile_error(self) -> TokenStream`

- <span id="error-combine"></span>`fn combine(&mut self, another: Error)` — [`Error`](#error)

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for Error`

- <span id="error-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for Error`

##### `impl Extend for Error`

- <span id="error-extend"></span>`fn extend<T: IntoIterator<Item = Error>>(&mut self, iter: T)`

##### `impl IntoIterator for Error`

- <span id="error-item"></span>`type Item = Error`

- <span id="error-intoiter"></span>`type IntoIter = IntoIter`

- <span id="error-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<T> ToString for Error`

- <span id="error-to-string"></span>`fn to_string(&self) -> String`

### `Arm`

```rust
struct Arm {
    pub attrs: Vec<crate::attr::Attribute>,
    pub pat: crate::pat::Pat,
    pub guard: Option<(token::If, Box<Expr>)>,
    pub fat_arrow_token: token::FatArrow,
    pub body: Box<Expr>,
    pub comma: Option<token::Comma>,
}
```

One arm of a `match` expression: `0..=10 => { return true; }`.

As in:

```rust
fn f() -> bool {
    let n = 0;
match n {
    0..=10 => {
        return true;
    }
    // ...
    _ => {}
}
  false
}
```

#### Implementations

- <span id="crateexprarm-parse-multiple"></span>`fn parse_multiple(input: ParseStream<'_>) -> Result<Vec<Self>>` — [`ParseStream`](parse/index.md), [`Result`](#result)

#### Trait Implementations

##### `impl Clone for crate::Arm`

- <span id="cratearm-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Arm`

- <span id="cratearm-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Arm`

##### `impl Hash for crate::Arm`

- <span id="cratearm-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::Arm`

- <span id="crateexprarm-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Arm>` — [`ParseStream`](parse/index.md), [`Result`](#result), [`Arm`](#arm)

##### `impl PartialEq for crate::Arm`

- <span id="cratearm-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Arm`

##### `impl<T> Spanned for Arm`

- <span id="arm-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::Arm`

- <span id="crateexprarm-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Label`

```rust
struct Label {
    pub name: crate::lifetime::Lifetime,
    pub colon_token: token::Colon,
}
```

A lifetime labeling a `for`, `while`, or `loop`.

#### Trait Implementations

##### `impl Clone for crate::Label`

- <span id="cratelabel-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Label`

- <span id="cratelabel-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Label`

##### `impl Hash for crate::Label`

- <span id="cratelabel-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::Label`

- <span id="crateexprlabel-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Label`

- <span id="cratelabel-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Label`

##### `impl<T> Spanned for Label`

- <span id="label-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::Label`

- <span id="crateexprlabel-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprBinary`

```rust
struct ExprBinary {
    pub attrs: Vec<crate::attr::Attribute>,
    pub left: Box<Expr>,
    pub op: crate::op::BinOp,
    pub right: Box<Expr>,
}
```

A binary operation: `a + b`, `a += b`.

#### Implementations

- <span id="crateexprbinary-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprBinary`

- <span id="crateexprbinary-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprBinary`

- <span id="crateexprbinary-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprBinary`

##### `impl Hash for crate::ExprBinary`

- <span id="crateexprbinary-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprBinary`

- <span id="crateexprexprbinary-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprBinary`

- <span id="crateexprbinary-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprBinary`

##### `impl<T> Spanned for ExprBinary`

- <span id="exprbinary-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprBinary`

- <span id="crateexprexprbinary-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprCall`

```rust
struct ExprCall {
    pub attrs: Vec<crate::attr::Attribute>,
    pub func: Box<Expr>,
    pub paren_token: token::Paren,
    pub args: crate::punctuated::Punctuated<Expr, token::Comma>,
}
```

A function call expression: `invoke(a, b)`.

#### Implementations

- <span id="crateexprcall-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprCall`

- <span id="crateexprcall-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprCall`

- <span id="crateexprcall-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprCall`

##### `impl Hash for crate::ExprCall`

- <span id="crateexprcall-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprCall`

- <span id="crateexprexprcall-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprCall`

- <span id="crateexprcall-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprCall`

##### `impl<T> Spanned for ExprCall`

- <span id="exprcall-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprCall`

- <span id="crateexprexprcall-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprCast`

```rust
struct ExprCast {
    pub attrs: Vec<crate::attr::Attribute>,
    pub expr: Box<Expr>,
    pub as_token: token::As,
    pub ty: Box<crate::ty::Type>,
}
```

A cast expression: `foo as f64`.

#### Implementations

- <span id="crateexprcast-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprCast`

- <span id="crateexprcast-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprCast`

- <span id="crateexprcast-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprCast`

##### `impl Hash for crate::ExprCast`

- <span id="crateexprcast-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprCast`

- <span id="crateexprexprcast-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprCast`

- <span id="crateexprcast-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprCast`

##### `impl<T> Spanned for ExprCast`

- <span id="exprcast-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprCast`

- <span id="crateexprexprcast-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprField`

```rust
struct ExprField {
    pub attrs: Vec<crate::attr::Attribute>,
    pub base: Box<Expr>,
    pub dot_token: token::Dot,
    pub member: Member,
}
```

Access of a named struct field (`obj.k`) or unnamed tuple struct
field (`obj.0`).

#### Implementations

- <span id="crateexprfield-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprField`

- <span id="crateexprfield-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprField`

- <span id="crateexprfield-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprField`

##### `impl Hash for crate::ExprField`

- <span id="crateexprfield-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprField`

- <span id="crateexprexprfield-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprField`

- <span id="crateexprfield-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprField`

##### `impl<T> Spanned for ExprField`

- <span id="exprfield-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprField`

- <span id="crateexprexprfield-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprIndex`

```rust
struct ExprIndex {
    pub attrs: Vec<crate::attr::Attribute>,
    pub expr: Box<Expr>,
    pub bracket_token: token::Bracket,
    pub index: Box<Expr>,
}
```

A square bracketed indexing expression: `vector[2]`.

#### Implementations

- <span id="crateexprindex-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprIndex`

- <span id="crateexprindex-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprIndex`

- <span id="crateexprindex-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprIndex`

##### `impl Hash for crate::ExprIndex`

- <span id="crateexprindex-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprIndex`

- <span id="crateexprexprindex-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprIndex`

- <span id="crateexprindex-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprIndex`

##### `impl<T> Spanned for ExprIndex`

- <span id="exprindex-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprIndex`

- <span id="crateexprexprindex-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprLit`

```rust
struct ExprLit {
    pub attrs: Vec<crate::attr::Attribute>,
    pub lit: crate::lit::Lit,
}
```

A literal in place of an expression: `1`, `"foo"`.

#### Implementations

- <span id="crateexprlit-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprLit`

- <span id="crateexprlit-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprLit`

- <span id="crateexprlit-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprLit`

##### `impl Hash for crate::ExprLit`

- <span id="crateexprlit-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprLit`

- <span id="crateexprexprlit-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprLit`

- <span id="crateexprlit-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprLit`

##### `impl<T> Spanned for ExprLit`

- <span id="exprlit-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprLit`

- <span id="crateexprexprlit-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprMacro`

```rust
struct ExprMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
}
```

A macro invocation expression: `format!("{}", q)`.

#### Implementations

- <span id="crateexprmacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprMacro`

- <span id="crateexprmacro-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprMacro`

- <span id="crateexprmacro-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprMacro`

##### `impl Hash for crate::ExprMacro`

- <span id="crateexprmacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprMacro`

- <span id="crateexprexprmacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprMacro`

- <span id="crateexprmacro-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprMacro`

##### `impl<T> Spanned for ExprMacro`

- <span id="exprmacro-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprMacro`

- <span id="crateexprexprmacro-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprMethodCall`

```rust
struct ExprMethodCall {
    pub attrs: Vec<crate::attr::Attribute>,
    pub receiver: Box<Expr>,
    pub dot_token: token::Dot,
    pub method: crate::ident::Ident,
    pub turbofish: Option<crate::path::AngleBracketedGenericArguments>,
    pub paren_token: token::Paren,
    pub args: crate::punctuated::Punctuated<Expr, token::Comma>,
}
```

A method call expression: `x.foo::<T>(a, b)`.

#### Implementations

- <span id="crateexprmethodcall-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprMethodCall`

- <span id="crateexprmethodcall-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprMethodCall`

- <span id="crateexprmethodcall-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprMethodCall`

##### `impl Hash for crate::ExprMethodCall`

- <span id="crateexprmethodcall-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprMethodCall`

- <span id="crateexprexprmethodcall-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprMethodCall`

- <span id="crateexprmethodcall-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprMethodCall`

##### `impl<T> Spanned for ExprMethodCall`

- <span id="exprmethodcall-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprMethodCall`

- <span id="crateexprexprmethodcall-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprParen`

```rust
struct ExprParen {
    pub attrs: Vec<crate::attr::Attribute>,
    pub paren_token: token::Paren,
    pub expr: Box<Expr>,
}
```

A parenthesized expression: `(a + b)`.

#### Implementations

- <span id="crateexprparen-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprParen`

- <span id="crateexprparen-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprParen`

- <span id="crateexprparen-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprParen`

##### `impl Hash for crate::ExprParen`

- <span id="crateexprparen-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprParen`

- <span id="crateexprexprparen-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprParen`

- <span id="crateexprparen-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprParen`

##### `impl<T> Spanned for ExprParen`

- <span id="exprparen-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprParen`

- <span id="crateexprexprparen-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprPath`

```rust
struct ExprPath {
    pub attrs: Vec<crate::attr::Attribute>,
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
}
```

A path like `std::mem::replace` possibly containing generic
parameters and a qualified self-type.

A plain identifier like `x` is a path of length 1.

#### Implementations

- <span id="crateexprpath-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprPath`

- <span id="crateexprpath-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprPath`

- <span id="crateexprpath-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprPath`

##### `impl Hash for crate::ExprPath`

- <span id="crateexprpath-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprPath`

- <span id="crateexprexprpath-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprPath`

- <span id="crateexprpath-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprPath`

##### `impl<T> Spanned for ExprPath`

- <span id="exprpath-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprPath`

- <span id="crateexprexprpath-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprReference`

```rust
struct ExprReference {
    pub attrs: Vec<crate::attr::Attribute>,
    pub and_token: token::And,
    pub mutability: Option<token::Mut>,
    pub expr: Box<Expr>,
}
```

A referencing operation: `&a` or `&mut a`.

#### Implementations

- <span id="crateexprreference-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprReference`

- <span id="crateexprreference-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprReference`

- <span id="crateexprreference-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprReference`

##### `impl Hash for crate::ExprReference`

- <span id="crateexprreference-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprReference`

- <span id="crateexprexprreference-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprReference`

- <span id="crateexprreference-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprReference`

##### `impl<T> Spanned for ExprReference`

- <span id="exprreference-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprReference`

- <span id="crateexprexprreference-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprStruct`

```rust
struct ExprStruct {
    pub attrs: Vec<crate::attr::Attribute>,
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
    pub brace_token: token::Brace,
    pub fields: crate::punctuated::Punctuated<FieldValue, token::Comma>,
    pub dot2_token: Option<token::DotDot>,
    pub rest: Option<Box<Expr>>,
}
```

A struct literal expression: `Point { x: 1, y: 1 }`.

The `rest` provides the value of the remaining fields as in `S { a:
1, b: 1, ..rest }`.

#### Implementations

- <span id="crateexprstruct-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprStruct`

- <span id="crateexprstruct-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprStruct`

- <span id="crateexprstruct-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprStruct`

##### `impl Hash for crate::ExprStruct`

- <span id="crateexprstruct-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprStruct`

- <span id="crateexprexprstruct-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprStruct`

- <span id="crateexprstruct-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprStruct`

##### `impl<T> Spanned for ExprStruct`

- <span id="exprstruct-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprStruct`

- <span id="crateexprexprstruct-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprUnary`

```rust
struct ExprUnary {
    pub attrs: Vec<crate::attr::Attribute>,
    pub op: crate::op::UnOp,
    pub expr: Box<Expr>,
}
```

A unary operation: `!x`, `*x`.

#### Implementations

- <span id="crateexprunary-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprUnary`

- <span id="crateexprunary-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprUnary`

- <span id="crateexprunary-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprUnary`

##### `impl Hash for crate::ExprUnary`

- <span id="crateexprunary-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprUnary`

- <span id="crateexprexprunary-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprUnary`

- <span id="crateexprunary-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprUnary`

##### `impl<T> Spanned for ExprUnary`

- <span id="exprunary-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprUnary`

- <span id="crateexprexprunary-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `FieldValue`

```rust
struct FieldValue {
    pub attrs: Vec<crate::attr::Attribute>,
    pub member: Member,
    pub colon_token: Option<token::Colon>,
    pub expr: Expr,
}
```

A field-value pair in a struct literal.

#### Fields

- **`colon_token`**: `Option<token::Colon>`

  The colon in `Struct { x: x }`. If written in shorthand like
  `Struct { x }`, there is no colon.

#### Trait Implementations

##### `impl Clone for crate::FieldValue`

- <span id="cratefieldvalue-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::FieldValue`

- <span id="cratefieldvalue-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldValue`

##### `impl Hash for crate::FieldValue`

- <span id="cratefieldvalue-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::FieldValue`

- <span id="crateexprfieldvalue-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::FieldValue`

- <span id="cratefieldvalue-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for FieldValue`

##### `impl<T> Spanned for FieldValue`

- <span id="fieldvalue-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::FieldValue`

- <span id="crateexprfieldvalue-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Index`

```rust
struct Index {
    pub index: u32,
    pub span: proc_macro2::Span,
}
```

The index of an unnamed tuple struct field.

#### Trait Implementations

##### `impl Clone for crate::Index`

- <span id="crateindex-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Index`

- <span id="crateindex-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Index`

##### `impl Hash for Index`

- <span id="index-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl IdentFragment for Index`

- <span id="index-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="index-span"></span>`fn span(&self) -> Option<Span>`

##### `impl Parse for crate::expr::Index`

- <span id="crateexprindex-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for Index`

- <span id="index-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Index`

##### `impl<T> Spanned for Index`

- <span id="index-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::Index`

- <span id="crateexprindex-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprArray`

```rust
struct ExprArray {
    pub attrs: Vec<crate::attr::Attribute>,
    pub bracket_token: token::Bracket,
    pub elems: crate::punctuated::Punctuated<Expr, token::Comma>,
}
```

A slice literal expression: `[a, b, c, d]`.

#### Implementations

- <span id="crateexprarray-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprArray`

- <span id="crateexprarray-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprArray`

- <span id="crateexprarray-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprArray`

##### `impl Hash for crate::ExprArray`

- <span id="crateexprarray-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprArray`

- <span id="crateexprexprarray-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprArray`

- <span id="crateexprarray-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprArray`

##### `impl<T> Spanned for ExprArray`

- <span id="exprarray-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprArray`

- <span id="crateexprexprarray-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprAssign`

```rust
struct ExprAssign {
    pub attrs: Vec<crate::attr::Attribute>,
    pub left: Box<Expr>,
    pub eq_token: token::Eq,
    pub right: Box<Expr>,
}
```

An assignment expression: `a = compute()`.

#### Implementations

- <span id="crateexprassign-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprAssign`

- <span id="crateexprassign-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprAssign`

- <span id="crateexprassign-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprAssign`

##### `impl Hash for crate::ExprAssign`

- <span id="crateexprassign-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprAssign`

- <span id="crateexprexprassign-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprAssign`

- <span id="crateexprassign-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprAssign`

##### `impl<T> Spanned for ExprAssign`

- <span id="exprassign-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprAssign`

- <span id="crateexprexprassign-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprAsync`

```rust
struct ExprAsync {
    pub attrs: Vec<crate::attr::Attribute>,
    pub async_token: token::Async,
    pub capture: Option<token::Move>,
    pub block: crate::stmt::Block,
}
```

An async block: `async { ... }`.

#### Implementations

- <span id="crateexprasync-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprAsync`

- <span id="crateexprasync-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprAsync`

- <span id="crateexprasync-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprAsync`

##### `impl Hash for crate::ExprAsync`

- <span id="crateexprasync-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprAsync`

- <span id="crateexprexprasync-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprAsync`

- <span id="crateexprasync-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprAsync`

##### `impl<T> Spanned for ExprAsync`

- <span id="exprasync-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprAsync`

- <span id="crateexprexprasync-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprAwait`

```rust
struct ExprAwait {
    pub attrs: Vec<crate::attr::Attribute>,
    pub base: Box<Expr>,
    pub dot_token: token::Dot,
    pub await_token: token::Await,
}
```

An await expression: `fut.await`.

#### Implementations

- <span id="crateexprawait-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprAwait`

- <span id="crateexprawait-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprAwait`

- <span id="crateexprawait-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprAwait`

##### `impl Hash for crate::ExprAwait`

- <span id="crateexprawait-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprAwait`

- <span id="crateexprexprawait-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprAwait`

- <span id="crateexprawait-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprAwait`

##### `impl<T> Spanned for ExprAwait`

- <span id="exprawait-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprAwait`

- <span id="crateexprexprawait-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprBlock`

```rust
struct ExprBlock {
    pub attrs: Vec<crate::attr::Attribute>,
    pub label: Option<Label>,
    pub block: crate::stmt::Block,
}
```

A blocked scope: `{ ... }`.

#### Implementations

- <span id="crateexprblock-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprBlock`

- <span id="crateexprblock-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprBlock`

- <span id="crateexprblock-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprBlock`

##### `impl Hash for crate::ExprBlock`

- <span id="crateexprblock-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprBlock`

- <span id="crateexprexprblock-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprBlock`

- <span id="crateexprblock-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprBlock`

##### `impl<T> Spanned for ExprBlock`

- <span id="exprblock-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprBlock`

- <span id="crateexprexprblock-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprBreak`

```rust
struct ExprBreak {
    pub attrs: Vec<crate::attr::Attribute>,
    pub break_token: token::Break,
    pub label: Option<crate::lifetime::Lifetime>,
    pub expr: Option<Box<Expr>>,
}
```

A `break`, with an optional label to break and an optional
expression.

#### Implementations

- <span id="crateexprbreak-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprBreak`

- <span id="crateexprbreak-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprBreak`

- <span id="crateexprbreak-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprBreak`

##### `impl Hash for crate::ExprBreak`

- <span id="crateexprbreak-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprBreak`

- <span id="crateexprexprbreak-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprBreak`

- <span id="crateexprbreak-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprBreak`

##### `impl<T> Spanned for ExprBreak`

- <span id="exprbreak-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprBreak`

- <span id="crateexprexprbreak-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprClosure`

```rust
struct ExprClosure {
    pub attrs: Vec<crate::attr::Attribute>,
    pub lifetimes: Option<crate::generics::BoundLifetimes>,
    pub constness: Option<token::Const>,
    pub movability: Option<token::Static>,
    pub asyncness: Option<token::Async>,
    pub capture: Option<token::Move>,
    pub or1_token: token::Or,
    pub inputs: crate::punctuated::Punctuated<crate::pat::Pat, token::Comma>,
    pub or2_token: token::Or,
    pub output: crate::ty::ReturnType,
    pub body: Box<Expr>,
}
```

A closure expression: `|a, b| a + b`.

#### Implementations

- <span id="crateexprclosure-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprClosure`

- <span id="crateexprclosure-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprClosure`

- <span id="crateexprclosure-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprClosure`

##### `impl Hash for crate::ExprClosure`

- <span id="crateexprclosure-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprClosure`

- <span id="crateexprexprclosure-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprClosure`

- <span id="crateexprclosure-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprClosure`

##### `impl<T> Spanned for ExprClosure`

- <span id="exprclosure-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprClosure`

- <span id="crateexprexprclosure-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprConst`

```rust
struct ExprConst {
    pub attrs: Vec<crate::attr::Attribute>,
    pub const_token: token::Const,
    pub block: crate::stmt::Block,
}
```

A const block: `const { ... }`.

#### Implementations

- <span id="crateexprconst-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprConst`

- <span id="crateexprconst-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprConst`

- <span id="crateexprconst-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprConst`

##### `impl Hash for crate::ExprConst`

- <span id="crateexprconst-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprConst`

- <span id="crateexprexprconst-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprConst`

- <span id="crateexprconst-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprConst`

##### `impl<T> Spanned for ExprConst`

- <span id="exprconst-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprConst`

- <span id="crateexprexprconst-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprContinue`

```rust
struct ExprContinue {
    pub attrs: Vec<crate::attr::Attribute>,
    pub continue_token: token::Continue,
    pub label: Option<crate::lifetime::Lifetime>,
}
```

A `continue`, with an optional label.

#### Implementations

- <span id="crateexprcontinue-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprContinue`

- <span id="crateexprcontinue-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprContinue`

- <span id="crateexprcontinue-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprContinue`

##### `impl Hash for crate::ExprContinue`

- <span id="crateexprcontinue-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprContinue`

- <span id="crateexprexprcontinue-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprContinue`

- <span id="crateexprcontinue-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprContinue`

##### `impl<T> Spanned for ExprContinue`

- <span id="exprcontinue-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprContinue`

- <span id="crateexprexprcontinue-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprForLoop`

```rust
struct ExprForLoop {
    pub attrs: Vec<crate::attr::Attribute>,
    pub label: Option<Label>,
    pub for_token: token::For,
    pub pat: Box<crate::pat::Pat>,
    pub in_token: token::In,
    pub expr: Box<Expr>,
    pub body: crate::stmt::Block,
}
```

A for loop: `for pat in expr { ... }`.

#### Implementations

- <span id="crateexprforloop-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprForLoop`

- <span id="crateexprforloop-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprForLoop`

- <span id="crateexprforloop-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprForLoop`

##### `impl Hash for crate::ExprForLoop`

- <span id="crateexprforloop-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprForLoop`

- <span id="crateexprexprforloop-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprForLoop`

- <span id="crateexprforloop-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprForLoop`

##### `impl<T> Spanned for ExprForLoop`

- <span id="exprforloop-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprForLoop`

- <span id="crateexprexprforloop-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprGroup`

```rust
struct ExprGroup {
    pub attrs: Vec<crate::attr::Attribute>,
    pub group_token: token::Group,
    pub expr: Box<Expr>,
}
```

An expression contained within invisible delimiters.

This variant is important for faithfully representing the precedence
of expressions and is related to `None`-delimited spans in a
`TokenStream`.

#### Implementations

- <span id="crateexprgroup-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprGroup`

- <span id="crateexprgroup-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprGroup`

- <span id="crateexprgroup-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprGroup`

##### `impl Hash for crate::ExprGroup`

- <span id="crateexprgroup-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::ExprGroup`

- <span id="crateexprgroup-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprGroup`

##### `impl<T> Spanned for ExprGroup`

- <span id="exprgroup-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprGroup`

- <span id="crateexprexprgroup-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprIf`

```rust
struct ExprIf {
    pub attrs: Vec<crate::attr::Attribute>,
    pub if_token: token::If,
    pub cond: Box<Expr>,
    pub then_branch: crate::stmt::Block,
    pub else_branch: Option<(token::Else, Box<Expr>)>,
}
```

An `if` expression with an optional `else` block: `if expr { ... }
else { ... }`.

The `else` branch expression may only be an `If` or `Block`
expression, not any of the other types of expression.

#### Implementations

- <span id="crateexprif-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprIf`

- <span id="crateexprif-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprIf`

- <span id="crateexprif-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprIf`

##### `impl Hash for crate::ExprIf`

- <span id="crateexprif-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprIf`

- <span id="crateexprexprif-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprIf`

- <span id="crateexprif-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprIf`

##### `impl<T> Spanned for ExprIf`

- <span id="exprif-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprIf`

- <span id="crateexprexprif-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprInfer`

```rust
struct ExprInfer {
    pub attrs: Vec<crate::attr::Attribute>,
    pub underscore_token: token::Underscore,
}
```

The inferred value of a const generic argument, denoted `_`.

#### Implementations

- <span id="crateexprinfer-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprInfer`

- <span id="crateexprinfer-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprInfer`

- <span id="crateexprinfer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprInfer`

##### `impl Hash for crate::ExprInfer`

- <span id="crateexprinfer-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprInfer`

- <span id="crateexprexprinfer-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprInfer`

- <span id="crateexprinfer-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprInfer`

##### `impl<T> Spanned for ExprInfer`

- <span id="exprinfer-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprInfer`

- <span id="crateexprexprinfer-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprLet`

```rust
struct ExprLet {
    pub attrs: Vec<crate::attr::Attribute>,
    pub let_token: token::Let,
    pub pat: Box<crate::pat::Pat>,
    pub eq_token: token::Eq,
    pub expr: Box<Expr>,
}
```

A `let` guard: `let Some(x) = opt`.

#### Implementations

- <span id="crateexprlet-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprLet`

- <span id="crateexprlet-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprLet`

- <span id="crateexprlet-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprLet`

##### `impl Hash for crate::ExprLet`

- <span id="crateexprlet-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprLet`

- <span id="crateexprexprlet-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprLet`

- <span id="crateexprlet-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprLet`

##### `impl<T> Spanned for ExprLet`

- <span id="exprlet-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprLet`

- <span id="crateexprexprlet-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprLoop`

```rust
struct ExprLoop {
    pub attrs: Vec<crate::attr::Attribute>,
    pub label: Option<Label>,
    pub loop_token: token::Loop,
    pub body: crate::stmt::Block,
}
```

Conditionless loop: `loop { ... }`.

#### Implementations

- <span id="crateexprloop-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprLoop`

- <span id="crateexprloop-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprLoop`

- <span id="crateexprloop-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprLoop`

##### `impl Hash for crate::ExprLoop`

- <span id="crateexprloop-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprLoop`

- <span id="crateexprexprloop-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprLoop`

- <span id="crateexprloop-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprLoop`

##### `impl<T> Spanned for ExprLoop`

- <span id="exprloop-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprLoop`

- <span id="crateexprexprloop-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprMatch`

```rust
struct ExprMatch {
    pub attrs: Vec<crate::attr::Attribute>,
    pub match_token: token::Match,
    pub expr: Box<Expr>,
    pub brace_token: token::Brace,
    pub arms: Vec<Arm>,
}
```

A `match` expression: `match n { Some(n) => {}, None => {} }`.

#### Implementations

- <span id="crateexprmatch-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprMatch`

- <span id="crateexprmatch-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprMatch`

- <span id="crateexprmatch-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprMatch`

##### `impl Hash for crate::ExprMatch`

- <span id="crateexprmatch-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprMatch`

- <span id="crateexprexprmatch-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprMatch`

- <span id="crateexprmatch-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprMatch`

##### `impl<T> Spanned for ExprMatch`

- <span id="exprmatch-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprMatch`

- <span id="crateexprexprmatch-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprRange`

```rust
struct ExprRange {
    pub attrs: Vec<crate::attr::Attribute>,
    pub start: Option<Box<Expr>>,
    pub limits: RangeLimits,
    pub end: Option<Box<Expr>>,
}
```

A range expression: `1..2`, `1..`, `..2`, `1..=2`, `..=2`.

#### Implementations

- <span id="crateexprrange-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprRange`

- <span id="crateexprrange-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprRange`

- <span id="crateexprrange-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprRange`

##### `impl Hash for crate::ExprRange`

- <span id="crateexprrange-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprRange`

- <span id="crateexprexprrange-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprRange`

- <span id="crateexprrange-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprRange`

##### `impl<T> Spanned for ExprRange`

- <span id="exprrange-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprRange`

- <span id="crateexprexprrange-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprRawAddr`

```rust
struct ExprRawAddr {
    pub attrs: Vec<crate::attr::Attribute>,
    pub and_token: token::And,
    pub raw: token::Raw,
    pub mutability: PointerMutability,
    pub expr: Box<Expr>,
}
```

Address-of operation: `&raw const place` or `&raw mut place`.

#### Implementations

- <span id="crateexprrawaddr-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprRawAddr`

- <span id="crateexprrawaddr-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprRawAddr`

- <span id="crateexprrawaddr-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprRawAddr`

##### `impl Hash for crate::ExprRawAddr`

- <span id="crateexprrawaddr-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprRawAddr`

- <span id="crateexprexprrawaddr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprRawAddr`

- <span id="crateexprrawaddr-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprRawAddr`

##### `impl<T> Spanned for ExprRawAddr`

- <span id="exprrawaddr-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprRawAddr`

- <span id="crateexprexprrawaddr-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprRepeat`

```rust
struct ExprRepeat {
    pub attrs: Vec<crate::attr::Attribute>,
    pub bracket_token: token::Bracket,
    pub expr: Box<Expr>,
    pub semi_token: token::Semi,
    pub len: Box<Expr>,
}
```

An array literal constructed from one repeated element: `[0u8; N]`.

#### Implementations

- <span id="crateexprrepeat-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprRepeat`

- <span id="crateexprrepeat-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprRepeat`

- <span id="crateexprrepeat-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprRepeat`

##### `impl Hash for crate::ExprRepeat`

- <span id="crateexprrepeat-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprRepeat`

- <span id="crateexprexprrepeat-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprRepeat`

- <span id="crateexprrepeat-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprRepeat`

##### `impl<T> Spanned for ExprRepeat`

- <span id="exprrepeat-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprRepeat`

- <span id="crateexprexprrepeat-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprReturn`

```rust
struct ExprReturn {
    pub attrs: Vec<crate::attr::Attribute>,
    pub return_token: token::Return,
    pub expr: Option<Box<Expr>>,
}
```

A `return`, with an optional value to be returned.

#### Implementations

- <span id="crateexprreturn-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprReturn`

- <span id="crateexprreturn-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprReturn`

- <span id="crateexprreturn-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprReturn`

##### `impl Hash for crate::ExprReturn`

- <span id="crateexprreturn-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprReturn`

- <span id="crateexprexprreturn-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprReturn`

- <span id="crateexprreturn-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprReturn`

##### `impl<T> Spanned for ExprReturn`

- <span id="exprreturn-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprReturn`

- <span id="crateexprexprreturn-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprTry`

```rust
struct ExprTry {
    pub attrs: Vec<crate::attr::Attribute>,
    pub expr: Box<Expr>,
    pub question_token: token::Question,
}
```

A try-expression: `expr?`.

#### Implementations

- <span id="crateexprtry-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprTry`

- <span id="crateexprtry-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprTry`

- <span id="crateexprtry-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprTry`

##### `impl Hash for crate::ExprTry`

- <span id="crateexprtry-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprTry`

- <span id="crateexprexprtry-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprTry`

- <span id="crateexprtry-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprTry`

##### `impl<T> Spanned for ExprTry`

- <span id="exprtry-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprTry`

- <span id="crateexprexprtry-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprTryBlock`

```rust
struct ExprTryBlock {
    pub attrs: Vec<crate::attr::Attribute>,
    pub try_token: token::Try,
    pub block: crate::stmt::Block,
}
```

A try block: `try { ... }`.

#### Implementations

- <span id="crateexprtryblock-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprTryBlock`

- <span id="crateexprtryblock-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprTryBlock`

- <span id="crateexprtryblock-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprTryBlock`

##### `impl Hash for crate::ExprTryBlock`

- <span id="crateexprtryblock-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprTryBlock`

- <span id="crateexprexprtryblock-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprTryBlock`

- <span id="crateexprtryblock-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprTryBlock`

##### `impl<T> Spanned for ExprTryBlock`

- <span id="exprtryblock-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprTryBlock`

- <span id="crateexprexprtryblock-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprTuple`

```rust
struct ExprTuple {
    pub attrs: Vec<crate::attr::Attribute>,
    pub paren_token: token::Paren,
    pub elems: crate::punctuated::Punctuated<Expr, token::Comma>,
}
```

A tuple expression: `(a, b, c, d)`.

#### Implementations

- <span id="crateexprtuple-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprTuple`

- <span id="crateexprtuple-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprTuple`

- <span id="crateexprtuple-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprTuple`

##### `impl Hash for crate::ExprTuple`

- <span id="crateexprtuple-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprTuple`

- <span id="crateexprexprtuple-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprTuple`

- <span id="crateexprtuple-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprTuple`

##### `impl<T> Spanned for ExprTuple`

- <span id="exprtuple-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprTuple`

- <span id="crateexprexprtuple-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprUnsafe`

```rust
struct ExprUnsafe {
    pub attrs: Vec<crate::attr::Attribute>,
    pub unsafe_token: token::Unsafe,
    pub block: crate::stmt::Block,
}
```

An unsafe block: `unsafe { ... }`.

#### Implementations

- <span id="crateexprunsafe-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprUnsafe`

- <span id="crateexprunsafe-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprUnsafe`

- <span id="crateexprunsafe-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprUnsafe`

##### `impl Hash for crate::ExprUnsafe`

- <span id="crateexprunsafe-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprUnsafe`

- <span id="crateexprexprunsafe-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprUnsafe`

- <span id="crateexprunsafe-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprUnsafe`

##### `impl<T> Spanned for ExprUnsafe`

- <span id="exprunsafe-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprUnsafe`

- <span id="crateexprexprunsafe-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprWhile`

```rust
struct ExprWhile {
    pub attrs: Vec<crate::attr::Attribute>,
    pub label: Option<Label>,
    pub while_token: token::While,
    pub cond: Box<Expr>,
    pub body: crate::stmt::Block,
}
```

A while loop: `while expr { ... }`.

#### Implementations

- <span id="crateexprwhile-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprWhile`

- <span id="crateexprwhile-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprWhile`

- <span id="crateexprwhile-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprWhile`

##### `impl Hash for crate::ExprWhile`

- <span id="crateexprwhile-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprWhile`

- <span id="crateexprexprwhile-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprWhile`

- <span id="crateexprwhile-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprWhile`

##### `impl<T> Spanned for ExprWhile`

- <span id="exprwhile-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprWhile`

- <span id="crateexprexprwhile-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprYield`

```rust
struct ExprYield {
    pub attrs: Vec<crate::attr::Attribute>,
    pub yield_token: token::Yield,
    pub expr: Option<Box<Expr>>,
}
```

A yield expression: `yield expr`.

#### Implementations

- <span id="crateexpryield-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprYield`

- <span id="crateexpryield-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprYield`

- <span id="crateexpryield-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprYield`

##### `impl Hash for crate::ExprYield`

- <span id="crateexpryield-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprYield`

- <span id="crateexprexpryield-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprYield`

- <span id="crateexpryield-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprYield`

##### `impl<T> Spanned for ExprYield`

- <span id="expryield-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprYield`

- <span id="crateexprexpryield-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `File`

```rust
struct File {
    pub shebang: Option<String>,
    pub attrs: Vec<crate::attr::Attribute>,
    pub items: Vec<crate::item::Item>,
}
```

A complete file of Rust source code.

Typically `File` objects are created with [`parse_file`](#parse-file).

# Example

Parse a Rust source file into a `syn::File` and print out a debug
representation of the syntax tree.

```rust
use std::env;
use std::fs;
use std::process;

fn main() {
}

fn fake_main() {
    let mut args = env::args();
    let _ = args.next(); // executable name

    let filename = match (args.next(), args.next()) {
        (Some(filename), None) => filename,
        _ => {
            eprintln!("Usage: dump-syntax path/to/filename.rs");
            process::exit(1);
        }
    };

    let src = fs::read_to_string(&filename).expect("unable to read file");
    let syntax = syn::parse_file(&src).expect("unable to parse file");

    // Debug impl is available if Syn is built with "extra-traits" feature.
    println!("{:#?}", syntax);
}
```

Running with its own source code as input, this program prints output
that begins with:

```text
File {
    shebang: None,
    attrs: [],
    items: [
        Use(
            ItemUse {
                attrs: [],
                vis: Inherited,
                use_token: Use,
                leading_colon: None,
                tree: Path(
                    UsePath {
                        ident: Ident(
                            std,
                        ),
                        colon2_token: Colon2,
                        tree: Name(
                            UseName {
                                ident: Ident(
                                    env,
                                ),
                            },
                        ),
                    },
                ),
                semi_token: Semi,
            },
        ),
...
```

#### Trait Implementations

##### `impl Clone for crate::File`

- <span id="cratefile-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::File`

- <span id="cratefile-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::File`

##### `impl Hash for crate::File`

- <span id="cratefile-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::file::File`

- <span id="cratefilefile-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::File`

- <span id="cratefile-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for File`

##### `impl<T> Spanned for File`

- <span id="file-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::file::File`

- <span id="cratefilefile-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `BoundLifetimes`

```rust
struct BoundLifetimes {
    pub for_token: token::For,
    pub lt_token: token::Lt,
    pub lifetimes: crate::punctuated::Punctuated<GenericParam, token::Comma>,
    pub gt_token: token::Gt,
}
```

A set of bound lifetimes: `for<'a, 'b, 'c>`.

#### Trait Implementations

##### `impl Clone for crate::BoundLifetimes`

- <span id="crateboundlifetimes-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::BoundLifetimes`

- <span id="crateboundlifetimes-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BoundLifetimes`

- <span id="boundlifetimes-default"></span>`fn default() -> Self`

##### `impl Eq for crate::BoundLifetimes`

##### `impl Hash for crate::BoundLifetimes`

- <span id="crateboundlifetimes-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::generics::BoundLifetimes`

- <span id="crategenericsboundlifetimes-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::BoundLifetimes`

- <span id="crateboundlifetimes-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for BoundLifetimes`

##### `impl<T> Spanned for BoundLifetimes`

- <span id="boundlifetimes-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::generics::BoundLifetimes`

- <span id="crategenericsboundlifetimes-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ConstParam`

```rust
struct ConstParam {
    pub attrs: Vec<crate::attr::Attribute>,
    pub const_token: token::Const,
    pub ident: crate::ident::Ident,
    pub colon_token: token::Colon,
    pub ty: crate::ty::Type,
    pub eq_token: Option<token::Eq>,
    pub default: Option<crate::expr::Expr>,
}
```

A const generic parameter: `const LENGTH: usize`.

#### Trait Implementations

##### `impl Clone for crate::ConstParam`

- <span id="crateconstparam-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ConstParam`

- <span id="crateconstparam-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ConstParam`

##### `impl Hash for crate::ConstParam`

- <span id="crateconstparam-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::generics::ConstParam`

- <span id="crategenericsconstparam-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ConstParam`

- <span id="crateconstparam-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ConstParam`

##### `impl<T> Spanned for ConstParam`

- <span id="constparam-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::generics::ConstParam`

- <span id="crategenericsconstparam-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Generics`

```rust
struct Generics {
    pub lt_token: Option<token::Lt>,
    pub params: crate::punctuated::Punctuated<GenericParam, token::Comma>,
    pub gt_token: Option<token::Gt>,
    pub where_clause: Option<WhereClause>,
}
```

Lifetimes and type parameters attached to a declaration of a function,
enum, trait, etc.

This struct represents two distinct optional syntactic elements,
[generic parameters] and [where clause]. In some locations of the
grammar, there may be other tokens in between these two things.



#### Implementations

- <span id="generics-lifetimes"></span>`fn lifetimes(&self) -> Lifetimes<'_>` — [`Lifetimes`](generics/index.md)

- <span id="generics-lifetimes-mut"></span>`fn lifetimes_mut(&mut self) -> LifetimesMut<'_>` — [`LifetimesMut`](generics/index.md)

- <span id="generics-type-params"></span>`fn type_params(&self) -> TypeParams<'_>` — [`TypeParams`](generics/index.md)

- <span id="generics-type-params-mut"></span>`fn type_params_mut(&mut self) -> TypeParamsMut<'_>` — [`TypeParamsMut`](generics/index.md)

- <span id="generics-const-params"></span>`fn const_params(&self) -> ConstParams<'_>` — [`ConstParams`](generics/index.md)

- <span id="generics-const-params-mut"></span>`fn const_params_mut(&mut self) -> ConstParamsMut<'_>` — [`ConstParamsMut`](generics/index.md)

- <span id="generics-make-where-clause"></span>`fn make_where_clause(&mut self) -> &mut WhereClause` — [`WhereClause`](#whereclause)

- <span id="generics-split-for-impl"></span>`fn split_for_impl(&self) -> (ImplGenerics<'_>, TypeGenerics<'_>, Option<&WhereClause>)` — [`ImplGenerics`](#implgenerics), [`TypeGenerics`](#typegenerics), [`WhereClause`](#whereclause)

#### Trait Implementations

##### `impl Clone for crate::Generics`

- <span id="crategenerics-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Generics`

- <span id="crategenerics-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Generics`

- <span id="generics-default"></span>`fn default() -> Self`

##### `impl Eq for crate::Generics`

##### `impl Hash for crate::Generics`

- <span id="crategenerics-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::generics::Generics`

- <span id="crategenericsgenerics-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Generics`

- <span id="crategenerics-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Generics`

##### `impl<T> Spanned for Generics`

- <span id="generics-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::generics::Generics`

- <span id="crategenericsgenerics-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `LifetimeParam`

```rust
struct LifetimeParam {
    pub attrs: Vec<crate::attr::Attribute>,
    pub lifetime: crate::lifetime::Lifetime,
    pub colon_token: Option<token::Colon>,
    pub bounds: crate::punctuated::Punctuated<crate::lifetime::Lifetime, token::Plus>,
}
```

A lifetime definition: `'a: 'b + 'c + 'd`.

#### Implementations

- <span id="lifetimeparam-new"></span>`fn new(lifetime: Lifetime) -> Self` — [`Lifetime`](#lifetime)

#### Trait Implementations

##### `impl Clone for crate::LifetimeParam`

- <span id="cratelifetimeparam-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::LifetimeParam`

- <span id="cratelifetimeparam-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LifetimeParam`

##### `impl Hash for crate::LifetimeParam`

- <span id="cratelifetimeparam-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::generics::LifetimeParam`

- <span id="crategenericslifetimeparam-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::LifetimeParam`

- <span id="cratelifetimeparam-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for LifetimeParam`

##### `impl<T> Spanned for LifetimeParam`

- <span id="lifetimeparam-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::generics::LifetimeParam`

- <span id="crategenericslifetimeparam-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PredicateLifetime`

```rust
struct PredicateLifetime {
    pub lifetime: crate::lifetime::Lifetime,
    pub colon_token: token::Colon,
    pub bounds: crate::punctuated::Punctuated<crate::lifetime::Lifetime, token::Plus>,
}
```

A lifetime predicate in a `where` clause: `'a: 'b + 'c`.

#### Trait Implementations

##### `impl Clone for crate::PredicateLifetime`

- <span id="cratepredicatelifetime-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PredicateLifetime`

- <span id="cratepredicatelifetime-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PredicateLifetime`

##### `impl Hash for crate::PredicateLifetime`

- <span id="cratepredicatelifetime-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::PredicateLifetime`

- <span id="cratepredicatelifetime-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for PredicateLifetime`

##### `impl<T> Spanned for PredicateLifetime`

- <span id="predicatelifetime-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::generics::PredicateLifetime`

- <span id="crategenericspredicatelifetime-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PredicateType`

```rust
struct PredicateType {
    pub lifetimes: Option<BoundLifetimes>,
    pub bounded_ty: crate::ty::Type,
    pub colon_token: token::Colon,
    pub bounds: crate::punctuated::Punctuated<TypeParamBound, token::Plus>,
}
```

A type predicate in a `where` clause: `for<'c> Foo<'c>: Trait<'c>`.

#### Fields

- **`lifetimes`**: `Option<BoundLifetimes>`

  Any lifetimes from a `for` binding

- **`bounded_ty`**: `crate::ty::Type`

  The type being bounded

- **`bounds`**: `crate::punctuated::Punctuated<TypeParamBound, token::Plus>`

  Trait and lifetime bounds (`Clone+Send+'static`)

#### Trait Implementations

##### `impl Clone for crate::PredicateType`

- <span id="cratepredicatetype-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PredicateType`

- <span id="cratepredicatetype-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PredicateType`

##### `impl Hash for crate::PredicateType`

- <span id="cratepredicatetype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::PredicateType`

- <span id="cratepredicatetype-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for PredicateType`

##### `impl<T> Spanned for PredicateType`

- <span id="predicatetype-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::generics::PredicateType`

- <span id="crategenericspredicatetype-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TraitBound`

```rust
struct TraitBound {
    pub paren_token: Option<token::Paren>,
    pub modifier: TraitBoundModifier,
    pub lifetimes: Option<BoundLifetimes>,
    pub path: crate::path::Path,
}
```

A trait used as a bound on a type parameter.

#### Fields

- **`lifetimes`**: `Option<BoundLifetimes>`

  The `for<'a>` in `for<'a> Foo<&'a T>`

- **`path`**: `crate::path::Path`

  The `Foo<&'a T>` in `for<'a> Foo<&'a T>`

#### Implementations

- <span id="crategenericstraitbound-do-parse"></span>`fn do_parse(input: ParseStream<'_>, allow_const: bool) -> Result<Option<Self>>` — [`ParseStream`](parse/index.md), [`Result`](#result)

#### Trait Implementations

##### `impl Clone for crate::TraitBound`

- <span id="cratetraitbound-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TraitBound`

- <span id="cratetraitbound-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitBound`

##### `impl Hash for crate::TraitBound`

- <span id="cratetraitbound-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::generics::TraitBound`

- <span id="crategenericstraitbound-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TraitBound`

- <span id="cratetraitbound-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TraitBound`

##### `impl<T> Spanned for TraitBound`

- <span id="traitbound-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::generics::TraitBound`

- <span id="crategenericstraitbound-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypeParam`

```rust
struct TypeParam {
    pub attrs: Vec<crate::attr::Attribute>,
    pub ident: crate::ident::Ident,
    pub colon_token: Option<token::Colon>,
    pub bounds: crate::punctuated::Punctuated<TypeParamBound, token::Plus>,
    pub eq_token: Option<token::Eq>,
    pub default: Option<crate::ty::Type>,
}
```

A generic type parameter: `T: Into<String>`.

#### Trait Implementations

##### `impl Clone for crate::TypeParam`

- <span id="cratetypeparam-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypeParam`

- <span id="cratetypeparam-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeParam`

##### `impl Hash for crate::TypeParam`

- <span id="cratetypeparam-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::generics::TypeParam`

- <span id="crategenericstypeparam-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypeParam`

- <span id="cratetypeparam-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeParam`

##### `impl<T> Spanned for TypeParam`

- <span id="typeparam-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::generics::TypeParam`

- <span id="crategenericstypeparam-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `WhereClause`

```rust
struct WhereClause {
    pub where_token: token::Where,
    pub predicates: crate::punctuated::Punctuated<WherePredicate, token::Comma>,
}
```

A `where` clause in a definition: `where T: Deserialize<'de>, D:
'static`.

#### Trait Implementations

##### `impl Clone for crate::WhereClause`

- <span id="cratewhereclause-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::WhereClause`

- <span id="cratewhereclause-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::WhereClause`

##### `impl Hash for crate::WhereClause`

- <span id="cratewhereclause-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::generics::WhereClause`

- <span id="crategenericswhereclause-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::WhereClause`

- <span id="cratewhereclause-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for WhereClause`

##### `impl<T> Spanned for WhereClause`

- <span id="whereclause-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::generics::WhereClause`

- <span id="crategenericswhereclause-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PreciseCapture`

```rust
struct PreciseCapture {
    pub use_token: token::Use,
    pub lt_token: token::Lt,
    pub params: crate::punctuated::Punctuated<CapturedParam, token::Comma>,
    pub gt_token: token::Gt,
}
```

Precise capturing bound: the 'use&lt;&hellip;&gt;' in `impl Trait +
use<'a, T>`.

#### Trait Implementations

##### `impl Clone for crate::PreciseCapture`

- <span id="crateprecisecapture-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PreciseCapture`

- <span id="crateprecisecapture-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PreciseCapture`

##### `impl Hash for crate::PreciseCapture`

- <span id="crateprecisecapture-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::generics::PreciseCapture`

- <span id="crategenericsprecisecapture-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::PreciseCapture`

- <span id="crateprecisecapture-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for PreciseCapture`

##### `impl<T> Spanned for PreciseCapture`

- <span id="precisecapture-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::generics::PreciseCapture`

- <span id="crategenericsprecisecapture-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ImplGenerics<'a>`

```rust
struct ImplGenerics<'a>(&'a Generics);
```

Returned by `Generics::split_for_impl`.

#### Trait Implementations

##### `impl<'a> Clone for ImplGenerics<'a>`

- <span id="implgenerics-clone"></span>`fn clone(&self) -> Self`

##### `impl<'a> Debug for ImplGenerics<'a>`

- <span id="implgenerics-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a> Eq for ImplGenerics<'a>`

##### `impl<'a> Hash for ImplGenerics<'a>`

- <span id="implgenerics-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<'a> PartialEq for ImplGenerics<'a>`

- <span id="implgenerics-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ImplGenerics<'a>`

##### `impl<T> Spanned for ImplGenerics<'a>`

- <span id="implgenerics-span"></span>`fn span(&self) -> Span`

##### `impl<'a> ToTokens for crate::generics::ImplGenerics<'a>`

- <span id="crategenericsimplgenerics-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Turbofish<'a>`

```rust
struct Turbofish<'a>(&'a Generics);
```

Returned by `TypeGenerics::as_turbofish`.

#### Trait Implementations

##### `impl<'a> Clone for Turbofish<'a>`

- <span id="turbofish-clone"></span>`fn clone(&self) -> Self`

##### `impl<'a> Debug for Turbofish<'a>`

- <span id="turbofish-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a> Eq for Turbofish<'a>`

##### `impl<'a> Hash for Turbofish<'a>`

- <span id="turbofish-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<'a> PartialEq for Turbofish<'a>`

- <span id="turbofish-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Turbofish<'a>`

##### `impl<T> Spanned for Turbofish<'a>`

- <span id="turbofish-span"></span>`fn span(&self) -> Span`

##### `impl<'a> ToTokens for crate::generics::Turbofish<'a>`

- <span id="crategenericsturbofish-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypeGenerics<'a>`

```rust
struct TypeGenerics<'a>(&'a Generics);
```

Returned by `Generics::split_for_impl`.

#### Implementations

- <span id="typegenerics-as-turbofish"></span>`fn as_turbofish(&self) -> Turbofish<'a>` — [`Turbofish`](#turbofish)

#### Trait Implementations

##### `impl<'a> Clone for TypeGenerics<'a>`

- <span id="typegenerics-clone"></span>`fn clone(&self) -> Self`

##### `impl<'a> Debug for TypeGenerics<'a>`

- <span id="typegenerics-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a> Eq for TypeGenerics<'a>`

##### `impl<'a> Hash for TypeGenerics<'a>`

- <span id="typegenerics-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<'a> PartialEq for TypeGenerics<'a>`

- <span id="typegenerics-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeGenerics<'a>`

##### `impl<T> Spanned for TypeGenerics<'a>`

- <span id="typegenerics-span"></span>`fn span(&self) -> Span`

##### `impl<'a> ToTokens for crate::generics::TypeGenerics<'a>`

- <span id="crategenericstypegenerics-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ForeignItemFn`

```rust
struct ForeignItemFn {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub sig: Signature,
    pub semi_token: token::Semi,
}
```

A foreign function in an `extern` block.

#### Implementations

- <span id="crateforeignitemfn-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ForeignItemFn`

- <span id="crateforeignitemfn-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ForeignItemFn`

- <span id="crateforeignitemfn-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItemFn`

##### `impl Hash for crate::ForeignItemFn`

- <span id="crateforeignitemfn-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ForeignItemFn`

- <span id="crateitemforeignitemfn-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ForeignItemFn`

- <span id="crateforeignitemfn-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ForeignItemFn`

##### `impl<T> Spanned for ForeignItemFn`

- <span id="foreignitemfn-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ForeignItemFn`

- <span id="crateitemforeignitemfn-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ForeignItemMacro`

```rust
struct ForeignItemMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<token::Semi>,
}
```

A macro invocation within an extern block.

#### Implementations

- <span id="crateforeignitemmacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ForeignItemMacro`

- <span id="crateforeignitemmacro-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ForeignItemMacro`

- <span id="crateforeignitemmacro-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItemMacro`

##### `impl Hash for crate::ForeignItemMacro`

- <span id="crateforeignitemmacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ForeignItemMacro`

- <span id="crateitemforeignitemmacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ForeignItemMacro`

- <span id="crateforeignitemmacro-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ForeignItemMacro`

##### `impl<T> Spanned for ForeignItemMacro`

- <span id="foreignitemmacro-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ForeignItemMacro`

- <span id="crateitemforeignitemmacro-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ForeignItemStatic`

```rust
struct ForeignItemStatic {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub static_token: token::Static,
    pub mutability: StaticMutability,
    pub ident: crate::ident::Ident,
    pub colon_token: token::Colon,
    pub ty: Box<crate::ty::Type>,
    pub semi_token: token::Semi,
}
```

A foreign static item in an `extern` block: `static ext: u8`.

#### Implementations

- <span id="crateforeignitemstatic-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ForeignItemStatic`

- <span id="crateforeignitemstatic-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ForeignItemStatic`

- <span id="crateforeignitemstatic-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItemStatic`

##### `impl Hash for crate::ForeignItemStatic`

- <span id="crateforeignitemstatic-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ForeignItemStatic`

- <span id="crateitemforeignitemstatic-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ForeignItemStatic`

- <span id="crateforeignitemstatic-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ForeignItemStatic`

##### `impl<T> Spanned for ForeignItemStatic`

- <span id="foreignitemstatic-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ForeignItemStatic`

- <span id="crateitemforeignitemstatic-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ForeignItemType`

```rust
struct ForeignItemType {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub type_token: token::Type,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub semi_token: token::Semi,
}
```

A foreign type in an `extern` block: `type void`.

#### Implementations

- <span id="crateforeignitemtype-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ForeignItemType`

- <span id="crateforeignitemtype-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ForeignItemType`

- <span id="crateforeignitemtype-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItemType`

##### `impl Hash for crate::ForeignItemType`

- <span id="crateforeignitemtype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ForeignItemType`

- <span id="crateitemforeignitemtype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ForeignItemType`

- <span id="crateforeignitemtype-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ForeignItemType`

##### `impl<T> Spanned for ForeignItemType`

- <span id="foreignitemtype-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ForeignItemType`

- <span id="crateitemforeignitemtype-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ImplItemConst`

```rust
struct ImplItemConst {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub defaultness: Option<token::Default>,
    pub const_token: token::Const,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub colon_token: token::Colon,
    pub ty: crate::ty::Type,
    pub eq_token: token::Eq,
    pub expr: crate::expr::Expr,
    pub semi_token: token::Semi,
}
```

An associated constant within an impl block.

#### Implementations

- <span id="crateimplitemconst-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ImplItemConst`

- <span id="crateimplitemconst-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ImplItemConst`

- <span id="crateimplitemconst-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItemConst`

##### `impl Hash for crate::ImplItemConst`

- <span id="crateimplitemconst-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ImplItemConst`

- <span id="crateitemimplitemconst-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ImplItemConst`

- <span id="crateimplitemconst-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ImplItemConst`

##### `impl<T> Spanned for ImplItemConst`

- <span id="implitemconst-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ImplItemConst`

- <span id="crateitemimplitemconst-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ImplItemFn`

```rust
struct ImplItemFn {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub defaultness: Option<token::Default>,
    pub sig: Signature,
    pub block: crate::stmt::Block,
}
```

An associated function within an impl block.

#### Implementations

- <span id="crateimplitemfn-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ImplItemFn`

- <span id="crateimplitemfn-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ImplItemFn`

- <span id="crateimplitemfn-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItemFn`

##### `impl Hash for crate::ImplItemFn`

- <span id="crateimplitemfn-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ImplItemFn`

- <span id="crateitemimplitemfn-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ImplItemFn`

- <span id="crateimplitemfn-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ImplItemFn`

##### `impl<T> Spanned for ImplItemFn`

- <span id="implitemfn-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ImplItemFn`

- <span id="crateitemimplitemfn-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ImplItemMacro`

```rust
struct ImplItemMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<token::Semi>,
}
```

A macro invocation within an impl block.

#### Implementations

- <span id="crateimplitemmacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ImplItemMacro`

- <span id="crateimplitemmacro-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ImplItemMacro`

- <span id="crateimplitemmacro-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItemMacro`

##### `impl Hash for crate::ImplItemMacro`

- <span id="crateimplitemmacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ImplItemMacro`

- <span id="crateitemimplitemmacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ImplItemMacro`

- <span id="crateimplitemmacro-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ImplItemMacro`

##### `impl<T> Spanned for ImplItemMacro`

- <span id="implitemmacro-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ImplItemMacro`

- <span id="crateitemimplitemmacro-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ImplItemType`

```rust
struct ImplItemType {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub defaultness: Option<token::Default>,
    pub type_token: token::Type,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub eq_token: token::Eq,
    pub ty: crate::ty::Type,
    pub semi_token: token::Semi,
}
```

An associated type within an impl block.

#### Implementations

- <span id="crateimplitemtype-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ImplItemType`

- <span id="crateimplitemtype-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ImplItemType`

- <span id="crateimplitemtype-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItemType`

##### `impl Hash for crate::ImplItemType`

- <span id="crateimplitemtype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ImplItemType`

- <span id="crateitemimplitemtype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ImplItemType`

- <span id="crateimplitemtype-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ImplItemType`

##### `impl<T> Spanned for ImplItemType`

- <span id="implitemtype-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ImplItemType`

- <span id="crateitemimplitemtype-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemConst`

```rust
struct ItemConst {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub const_token: token::Const,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub colon_token: token::Colon,
    pub ty: Box<crate::ty::Type>,
    pub eq_token: token::Eq,
    pub expr: Box<crate::expr::Expr>,
    pub semi_token: token::Semi,
}
```

A constant item: `const MAX: u16 = 65535`.

#### Implementations

- <span id="crateitemconst-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemConst`

- <span id="crateitemconst-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemConst`

- <span id="crateitemconst-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemConst`

##### `impl Hash for crate::ItemConst`

- <span id="crateitemconst-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemConst`

- <span id="crateitemitemconst-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemConst`

- <span id="crateitemconst-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemConst`

##### `impl<T> Spanned for ItemConst`

- <span id="itemconst-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemConst`

- <span id="crateitemitemconst-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemEnum`

```rust
struct ItemEnum {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub enum_token: token::Enum,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub brace_token: token::Brace,
    pub variants: crate::punctuated::Punctuated<crate::data::Variant, token::Comma>,
}
```

An enum definition: `enum Foo<A, B> { A(A), B(B) }`.

#### Implementations

- <span id="crateitemenum-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemEnum`

- <span id="crateitemenum-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemEnum`

- <span id="crateitemenum-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemEnum`

##### `impl Hash for crate::ItemEnum`

- <span id="crateitemenum-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemEnum`

- <span id="crateitemitemenum-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemEnum`

- <span id="crateitemenum-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemEnum`

##### `impl<T> Spanned for ItemEnum`

- <span id="itemenum-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemEnum`

- <span id="crateitemitemenum-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemExternCrate`

```rust
struct ItemExternCrate {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub extern_token: token::Extern,
    pub crate_token: token::Crate,
    pub ident: crate::ident::Ident,
    pub rename: Option<(token::As, crate::ident::Ident)>,
    pub semi_token: token::Semi,
}
```

An `extern crate` item: `extern crate serde`.

#### Implementations

- <span id="crateitemexterncrate-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemExternCrate`

- <span id="crateitemexterncrate-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemExternCrate`

- <span id="crateitemexterncrate-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemExternCrate`

##### `impl Hash for crate::ItemExternCrate`

- <span id="crateitemexterncrate-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemExternCrate`

- <span id="crateitemitemexterncrate-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemExternCrate`

- <span id="crateitemexterncrate-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemExternCrate`

##### `impl<T> Spanned for ItemExternCrate`

- <span id="itemexterncrate-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemExternCrate`

- <span id="crateitemitemexterncrate-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemFn`

```rust
struct ItemFn {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub sig: Signature,
    pub block: Box<crate::stmt::Block>,
}
```

A free-standing function: `fn process(n: usize) -> Result<()> { ... }`.

#### Implementations

- <span id="crateitemfn-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemFn`

- <span id="crateitemfn-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemFn`

- <span id="crateitemfn-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemFn`

##### `impl Hash for crate::ItemFn`

- <span id="crateitemfn-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemFn`

- <span id="crateitemitemfn-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemFn`

- <span id="crateitemfn-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemFn`

##### `impl<T> Spanned for ItemFn`

- <span id="itemfn-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemFn`

- <span id="crateitemitemfn-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemForeignMod`

```rust
struct ItemForeignMod {
    pub attrs: Vec<crate::attr::Attribute>,
    pub unsafety: Option<token::Unsafe>,
    pub abi: crate::ty::Abi,
    pub brace_token: token::Brace,
    pub items: Vec<ForeignItem>,
}
```

A block of foreign items: `extern "C" { ... }`.

#### Implementations

- <span id="crateitemforeignmod-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemForeignMod`

- <span id="crateitemforeignmod-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemForeignMod`

- <span id="crateitemforeignmod-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemForeignMod`

##### `impl Hash for crate::ItemForeignMod`

- <span id="crateitemforeignmod-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemForeignMod`

- <span id="crateitemitemforeignmod-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemForeignMod`

- <span id="crateitemforeignmod-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemForeignMod`

##### `impl<T> Spanned for ItemForeignMod`

- <span id="itemforeignmod-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemForeignMod`

- <span id="crateitemitemforeignmod-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemImpl`

```rust
struct ItemImpl {
    pub attrs: Vec<crate::attr::Attribute>,
    pub defaultness: Option<token::Default>,
    pub unsafety: Option<token::Unsafe>,
    pub impl_token: token::Impl,
    pub generics: crate::generics::Generics,
    pub trait_: Option<(Option<token::Not>, crate::path::Path, token::For)>,
    pub self_ty: Box<crate::ty::Type>,
    pub brace_token: token::Brace,
    pub items: Vec<ImplItem>,
}
```

An impl block providing trait or associated items: `impl<A> Trait
for Data<A> { ... }`.

#### Fields

- **`trait_`**: `Option<(Option<token::Not>, crate::path::Path, token::For)>`

  Trait this impl implements.

- **`self_ty`**: `Box<crate::ty::Type>`

  The Self type of the impl.

#### Implementations

- <span id="crateitemimpl-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemImpl`

- <span id="crateitemimpl-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemImpl`

- <span id="crateitemimpl-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemImpl`

##### `impl Hash for crate::ItemImpl`

- <span id="crateitemimpl-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemImpl`

- <span id="crateitemitemimpl-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemImpl`

- <span id="crateitemimpl-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemImpl`

##### `impl<T> Spanned for ItemImpl`

- <span id="itemimpl-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemImpl`

- <span id="crateitemitemimpl-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemMacro`

```rust
struct ItemMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub ident: Option<crate::ident::Ident>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<token::Semi>,
}
```

A macro invocation, which includes `macro_rules!` definitions.

#### Fields

- **`ident`**: `Option<crate::ident::Ident>`

  The `example` in `macro_rules! example { ... }`.

#### Implementations

- <span id="crateitemmacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemMacro`

- <span id="crateitemmacro-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemMacro`

- <span id="crateitemmacro-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemMacro`

##### `impl Hash for crate::ItemMacro`

- <span id="crateitemmacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemMacro`

- <span id="crateitemitemmacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemMacro`

- <span id="crateitemmacro-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemMacro`

##### `impl<T> Spanned for ItemMacro`

- <span id="itemmacro-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemMacro`

- <span id="crateitemitemmacro-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemMod`

```rust
struct ItemMod {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub unsafety: Option<token::Unsafe>,
    pub mod_token: token::Mod,
    pub ident: crate::ident::Ident,
    pub content: Option<(token::Brace, Vec<Item>)>,
    pub semi: Option<token::Semi>,
}
```

A module or module declaration: `mod m` or `mod m { ... }`.

#### Implementations

- <span id="crateitemmod-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemMod`

- <span id="crateitemmod-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemMod`

- <span id="crateitemmod-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemMod`

##### `impl Hash for crate::ItemMod`

- <span id="crateitemmod-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemMod`

- <span id="crateitemitemmod-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemMod`

- <span id="crateitemmod-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemMod`

##### `impl<T> Spanned for ItemMod`

- <span id="itemmod-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemMod`

- <span id="crateitemitemmod-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemStatic`

```rust
struct ItemStatic {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub static_token: token::Static,
    pub mutability: StaticMutability,
    pub ident: crate::ident::Ident,
    pub colon_token: token::Colon,
    pub ty: Box<crate::ty::Type>,
    pub eq_token: token::Eq,
    pub expr: Box<crate::expr::Expr>,
    pub semi_token: token::Semi,
}
```

A static item: `static BIKE: Shed = Shed(42)`.

#### Implementations

- <span id="crateitemstatic-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemStatic`

- <span id="crateitemstatic-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemStatic`

- <span id="crateitemstatic-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemStatic`

##### `impl Hash for crate::ItemStatic`

- <span id="crateitemstatic-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemStatic`

- <span id="crateitemitemstatic-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemStatic`

- <span id="crateitemstatic-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemStatic`

##### `impl<T> Spanned for ItemStatic`

- <span id="itemstatic-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemStatic`

- <span id="crateitemitemstatic-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemStruct`

```rust
struct ItemStruct {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub struct_token: token::Struct,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub fields: crate::data::Fields,
    pub semi_token: Option<token::Semi>,
}
```

A struct definition: `struct Foo<A> { x: A }`.

#### Implementations

- <span id="crateitemstruct-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemStruct`

- <span id="crateitemstruct-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemStruct`

- <span id="crateitemstruct-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemStruct`

##### `impl Hash for crate::ItemStruct`

- <span id="crateitemstruct-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemStruct`

- <span id="crateitemitemstruct-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemStruct`

- <span id="crateitemstruct-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemStruct`

##### `impl<T> Spanned for ItemStruct`

- <span id="itemstruct-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemStruct`

- <span id="crateitemitemstruct-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemTrait`

```rust
struct ItemTrait {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub unsafety: Option<token::Unsafe>,
    pub auto_token: Option<token::Auto>,
    pub restriction: Option<ImplRestriction>,
    pub trait_token: token::Trait,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub colon_token: Option<token::Colon>,
    pub supertraits: crate::punctuated::Punctuated<crate::generics::TypeParamBound, token::Plus>,
    pub brace_token: token::Brace,
    pub items: Vec<TraitItem>,
}
```

A trait definition: `pub trait Iterator { ... }`.

#### Implementations

- <span id="crateitemtrait-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemTrait`

- <span id="crateitemtrait-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemTrait`

- <span id="crateitemtrait-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemTrait`

##### `impl Hash for crate::ItemTrait`

- <span id="crateitemtrait-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemTrait`

- <span id="crateitemitemtrait-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemTrait`

- <span id="crateitemtrait-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemTrait`

##### `impl<T> Spanned for ItemTrait`

- <span id="itemtrait-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemTrait`

- <span id="crateitemitemtrait-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemTraitAlias`

```rust
struct ItemTraitAlias {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub trait_token: token::Trait,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub eq_token: token::Eq,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, token::Plus>,
    pub semi_token: token::Semi,
}
```

A trait alias: `pub trait SharableIterator = Iterator + Sync`.

#### Implementations

- <span id="crateitemtraitalias-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemTraitAlias`

- <span id="crateitemtraitalias-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemTraitAlias`

- <span id="crateitemtraitalias-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemTraitAlias`

##### `impl Hash for crate::ItemTraitAlias`

- <span id="crateitemtraitalias-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemTraitAlias`

- <span id="crateitemitemtraitalias-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemTraitAlias`

- <span id="crateitemtraitalias-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemTraitAlias`

##### `impl<T> Spanned for ItemTraitAlias`

- <span id="itemtraitalias-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemTraitAlias`

- <span id="crateitemitemtraitalias-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemType`

```rust
struct ItemType {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub type_token: token::Type,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub eq_token: token::Eq,
    pub ty: Box<crate::ty::Type>,
    pub semi_token: token::Semi,
}
```

A type alias: `type Result<T> = std::result::Result<T, MyError>`.

#### Implementations

- <span id="crateitemtype-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemType`

- <span id="crateitemtype-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemType`

- <span id="crateitemtype-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemType`

##### `impl Hash for crate::ItemType`

- <span id="crateitemtype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemType`

- <span id="crateitemitemtype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemType`

- <span id="crateitemtype-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemType`

##### `impl<T> Spanned for ItemType`

- <span id="itemtype-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemType`

- <span id="crateitemitemtype-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemUnion`

```rust
struct ItemUnion {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub union_token: token::Union,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub fields: crate::data::FieldsNamed,
}
```

A union definition: `union Foo<A, B> { x: A, y: B }`.

#### Implementations

- <span id="crateitemunion-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemUnion`

- <span id="crateitemunion-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemUnion`

- <span id="crateitemunion-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemUnion`

##### `impl Hash for crate::ItemUnion`

- <span id="crateitemunion-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemUnion`

- <span id="crateitemitemunion-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemUnion`

- <span id="crateitemunion-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemUnion`

##### `impl<T> Spanned for ItemUnion`

- <span id="itemunion-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemUnion`

- <span id="crateitemitemunion-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemUse`

```rust
struct ItemUse {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub use_token: token::Use,
    pub leading_colon: Option<token::PathSep>,
    pub tree: UseTree,
    pub semi_token: token::Semi,
}
```

A use declaration: `use std::collections::HashMap`.

#### Implementations

- <span id="crateitemuse-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemUse`

- <span id="crateitemuse-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemUse`

- <span id="crateitemuse-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemUse`

##### `impl Hash for crate::ItemUse`

- <span id="crateitemuse-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemUse`

- <span id="crateitemitemuse-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemUse`

- <span id="crateitemuse-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemUse`

##### `impl<T> Spanned for ItemUse`

- <span id="itemuse-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemUse`

- <span id="crateitemitemuse-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Receiver`

```rust
struct Receiver {
    pub attrs: Vec<crate::attr::Attribute>,
    pub reference: Option<(token::And, Option<crate::lifetime::Lifetime>)>,
    pub mutability: Option<token::Mut>,
    pub self_token: token::SelfValue,
    pub colon_token: Option<token::Colon>,
    pub ty: Box<crate::ty::Type>,
}
```

The `self` argument of an associated method.

If `colon_token` is present, the receiver is written with an explicit
type such as `self: Box<Self>`. If `colon_token` is absent, the receiver
is written in shorthand such as `self` or `&self` or `&mut self`. In the
shorthand case, the type in `ty` is reconstructed as one of `Self`,
`&Self`, or `&mut Self`.

#### Implementations

- <span id="receiver-lifetime"></span>`fn lifetime(&self) -> Option<&Lifetime>` — [`Lifetime`](#lifetime)

#### Trait Implementations

##### `impl Clone for crate::Receiver`

- <span id="cratereceiver-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Receiver`

- <span id="cratereceiver-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Receiver`

##### `impl Hash for crate::Receiver`

- <span id="cratereceiver-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::Receiver`

- <span id="crateitemreceiver-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Receiver`

- <span id="cratereceiver-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Receiver`

##### `impl<T> Spanned for Receiver`

- <span id="receiver-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::Receiver`

- <span id="crateitemreceiver-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Signature`

```rust
struct Signature {
    pub constness: Option<token::Const>,
    pub asyncness: Option<token::Async>,
    pub unsafety: Option<token::Unsafe>,
    pub abi: Option<crate::ty::Abi>,
    pub fn_token: token::Fn,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub paren_token: token::Paren,
    pub inputs: crate::punctuated::Punctuated<FnArg, token::Comma>,
    pub variadic: Option<Variadic>,
    pub output: crate::ty::ReturnType,
}
```

A function signature in a trait or implementation: `unsafe fn
initialize(&self)`.

#### Implementations

- <span id="signature-receiver"></span>`fn receiver(&self) -> Option<&Receiver>` — [`Receiver`](#receiver)

#### Trait Implementations

##### `impl Clone for crate::Signature`

- <span id="cratesignature-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Signature`

- <span id="cratesignature-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Signature`

##### `impl Hash for crate::Signature`

- <span id="cratesignature-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::Signature`

- <span id="crateitemsignature-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Signature`

- <span id="cratesignature-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Signature`

##### `impl<T> Spanned for Signature`

- <span id="signature-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::Signature`

- <span id="crateitemsignature-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TraitItemConst`

```rust
struct TraitItemConst {
    pub attrs: Vec<crate::attr::Attribute>,
    pub const_token: token::Const,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub colon_token: token::Colon,
    pub ty: crate::ty::Type,
    pub default: Option<(token::Eq, crate::expr::Expr)>,
    pub semi_token: token::Semi,
}
```

An associated constant within the definition of a trait.

#### Implementations

- <span id="cratetraititemconst-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TraitItemConst`

- <span id="cratetraititemconst-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TraitItemConst`

- <span id="cratetraititemconst-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItemConst`

##### `impl Hash for crate::TraitItemConst`

- <span id="cratetraititemconst-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::TraitItemConst`

- <span id="crateitemtraititemconst-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TraitItemConst`

- <span id="cratetraititemconst-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TraitItemConst`

##### `impl<T> Spanned for TraitItemConst`

- <span id="traititemconst-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::TraitItemConst`

- <span id="crateitemtraititemconst-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TraitItemFn`

```rust
struct TraitItemFn {
    pub attrs: Vec<crate::attr::Attribute>,
    pub sig: Signature,
    pub default: Option<crate::stmt::Block>,
    pub semi_token: Option<token::Semi>,
}
```

An associated function within the definition of a trait.

#### Implementations

- <span id="cratetraititemfn-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TraitItemFn`

- <span id="cratetraititemfn-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TraitItemFn`

- <span id="cratetraititemfn-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItemFn`

##### `impl Hash for crate::TraitItemFn`

- <span id="cratetraititemfn-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::TraitItemFn`

- <span id="crateitemtraititemfn-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TraitItemFn`

- <span id="cratetraititemfn-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TraitItemFn`

##### `impl<T> Spanned for TraitItemFn`

- <span id="traititemfn-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::TraitItemFn`

- <span id="crateitemtraititemfn-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TraitItemMacro`

```rust
struct TraitItemMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<token::Semi>,
}
```

A macro invocation within the definition of a trait.

#### Implementations

- <span id="cratetraititemmacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TraitItemMacro`

- <span id="cratetraititemmacro-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TraitItemMacro`

- <span id="cratetraititemmacro-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItemMacro`

##### `impl Hash for crate::TraitItemMacro`

- <span id="cratetraititemmacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::TraitItemMacro`

- <span id="crateitemtraititemmacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TraitItemMacro`

- <span id="cratetraititemmacro-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TraitItemMacro`

##### `impl<T> Spanned for TraitItemMacro`

- <span id="traititemmacro-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::TraitItemMacro`

- <span id="crateitemtraititemmacro-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TraitItemType`

```rust
struct TraitItemType {
    pub attrs: Vec<crate::attr::Attribute>,
    pub type_token: token::Type,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub colon_token: Option<token::Colon>,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, token::Plus>,
    pub default: Option<(token::Eq, crate::ty::Type)>,
    pub semi_token: token::Semi,
}
```

An associated type within the definition of a trait.

#### Implementations

- <span id="cratetraititemtype-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TraitItemType`

- <span id="cratetraititemtype-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TraitItemType`

- <span id="cratetraititemtype-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItemType`

##### `impl Hash for crate::TraitItemType`

- <span id="cratetraititemtype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::TraitItemType`

- <span id="crateitemtraititemtype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TraitItemType`

- <span id="cratetraititemtype-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TraitItemType`

##### `impl<T> Spanned for TraitItemType`

- <span id="traititemtype-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::TraitItemType`

- <span id="crateitemtraititemtype-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `UseGlob`

```rust
struct UseGlob {
    pub star_token: token::Star,
}
```

A glob import in a `use` item: `*`.

#### Trait Implementations

##### `impl Clone for crate::UseGlob`

- <span id="crateuseglob-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::UseGlob`

- <span id="crateuseglob-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseGlob`

##### `impl Hash for crate::UseGlob`

- <span id="crateuseglob-hash"></span>`fn hash<H>(&self, _state: &mut H)`

##### `impl PartialEq for crate::UseGlob`

- <span id="crateuseglob-eq"></span>`fn eq(&self, _other: &Self) -> bool`

##### `impl<T> Sealed for UseGlob`

##### `impl<T> Spanned for UseGlob`

- <span id="useglob-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::UseGlob`

- <span id="crateitemuseglob-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `UseGroup`

```rust
struct UseGroup {
    pub brace_token: token::Brace,
    pub items: crate::punctuated::Punctuated<UseTree, token::Comma>,
}
```

A braced group of imports in a `use` item: `{A, B, C}`.

#### Trait Implementations

##### `impl Clone for crate::UseGroup`

- <span id="crateusegroup-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::UseGroup`

- <span id="crateusegroup-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseGroup`

##### `impl Hash for crate::UseGroup`

- <span id="crateusegroup-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::UseGroup`

- <span id="crateusegroup-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for UseGroup`

##### `impl<T> Spanned for UseGroup`

- <span id="usegroup-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::UseGroup`

- <span id="crateitemusegroup-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `UseName`

```rust
struct UseName {
    pub ident: crate::ident::Ident,
}
```

An identifier imported by a `use` item: `HashMap`.

#### Trait Implementations

##### `impl Clone for crate::UseName`

- <span id="crateusename-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::UseName`

- <span id="crateusename-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseName`

##### `impl Hash for crate::UseName`

- <span id="crateusename-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::UseName`

- <span id="crateusename-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for UseName`

##### `impl<T> Spanned for UseName`

- <span id="usename-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::UseName`

- <span id="crateitemusename-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `UsePath`

```rust
struct UsePath {
    pub ident: crate::ident::Ident,
    pub colon2_token: token::PathSep,
    pub tree: Box<UseTree>,
}
```

A path prefix of imports in a `use` item: `std::...`.

#### Trait Implementations

##### `impl Clone for crate::UsePath`

- <span id="crateusepath-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::UsePath`

- <span id="crateusepath-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UsePath`

##### `impl Hash for crate::UsePath`

- <span id="crateusepath-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::UsePath`

- <span id="crateusepath-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for UsePath`

##### `impl<T> Spanned for UsePath`

- <span id="usepath-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::UsePath`

- <span id="crateitemusepath-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `UseRename`

```rust
struct UseRename {
    pub ident: crate::ident::Ident,
    pub as_token: token::As,
    pub rename: crate::ident::Ident,
}
```

An renamed identifier imported by a `use` item: `HashMap as Map`.

#### Trait Implementations

##### `impl Clone for crate::UseRename`

- <span id="crateuserename-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::UseRename`

- <span id="crateuserename-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseRename`

##### `impl Hash for crate::UseRename`

- <span id="crateuserename-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::UseRename`

- <span id="crateuserename-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for UseRename`

##### `impl<T> Spanned for UseRename`

- <span id="userename-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::UseRename`

- <span id="crateitemuserename-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Variadic`

```rust
struct Variadic {
    pub attrs: Vec<crate::attr::Attribute>,
    pub pat: Option<(Box<crate::pat::Pat>, token::Colon)>,
    pub dots: token::DotDotDot,
    pub comma: Option<token::Comma>,
}
```

The variadic argument of a foreign function.

```rust
struct c_char;
struct c_int;

extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    //                               ^^^
}
```

#### Trait Implementations

##### `impl Clone for crate::Variadic`

- <span id="cratevariadic-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Variadic`

- <span id="cratevariadic-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Variadic`

##### `impl Hash for crate::Variadic`

- <span id="cratevariadic-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::Variadic`

- <span id="cratevariadic-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Variadic`

##### `impl<T> Spanned for Variadic`

- <span id="variadic-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::Variadic`

- <span id="crateitemvariadic-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Lifetime`

```rust
struct Lifetime {
    pub apostrophe: proc_macro2::Span,
    pub ident: proc_macro2::Ident,
}
```

A Rust lifetime: `'a`.

Lifetime names must conform to the following rules:

- Must start with an apostrophe.
- Must not consist of just an apostrophe: `'`.
- Character after the apostrophe must be `_` or a Unicode code point with
  the XID_Start property.
- All following characters must be Unicode code points with the XID_Continue
  property.

#### Implementations

- <span id="lifetime-new"></span>`fn new(symbol: &str, span: Span) -> Self`

- <span id="lifetime-span"></span>`fn span(&self) -> Span`

- <span id="lifetime-set-span"></span>`fn set_span(&mut self, span: Span)`

#### Trait Implementations

##### `impl Clone for Lifetime`

- <span id="lifetime-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Lifetime`

- <span id="cratelifetime-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Lifetime`

- <span id="lifetime-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Lifetime`

##### `impl Hash for Lifetime`

- <span id="lifetime-hash"></span>`fn hash<H: Hasher>(&self, h: &mut H)`

##### `impl Ord for Lifetime`

- <span id="lifetime-cmp"></span>`fn cmp(&self, other: &Lifetime) -> Ordering` — [`Lifetime`](#lifetime)

##### `impl Parse for crate::lifetime::Lifetime`

- <span id="cratelifetimelifetime-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for Lifetime`

- <span id="lifetime-eq"></span>`fn eq(&self, other: &Lifetime) -> bool` — [`Lifetime`](#lifetime)

##### `impl PartialOrd for Lifetime`

- <span id="lifetime-partial-cmp"></span>`fn partial_cmp(&self, other: &Lifetime) -> Option<Ordering>` — [`Lifetime`](#lifetime)

##### `impl Sealed for crate::lifetime::Lifetime`

##### `impl<T> Spanned for Lifetime`

- <span id="lifetime-span"></span>`fn span(&self) -> Span`

##### `impl<T> ToString for Lifetime`

- <span id="lifetime-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for crate::lifetime::Lifetime`

- <span id="cratelifetimelifetime-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lifetime::Lifetime`

### `LitBool`

```rust
struct LitBool {
    pub value: bool,
    pub span: proc_macro2::Span,
}
```

A boolean literal: `true` or `false`.

#### Implementations

- <span id="litbool-new"></span>`fn new(value: bool, span: Span) -> Self`

- <span id="litbool-value"></span>`fn value(&self) -> bool`

- <span id="litbool-span"></span>`fn span(&self) -> Span`

- <span id="litbool-set-span"></span>`fn set_span(&mut self, span: Span)`

- <span id="litbool-token"></span>`fn token(&self) -> Ident` — [`Ident`](#ident)

#### Trait Implementations

##### `impl Clone for crate::LitBool`

- <span id="cratelitbool-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::lit::LitBool`

- <span id="cratelitlitbool-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitBool`

##### `impl Hash for crate::LitBool`

- <span id="cratelitbool-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::lit::LitBool`

- <span id="cratelitlitbool-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::LitBool`

- <span id="cratelitbool-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for crate::lit::LitBool`

##### `impl<T> Spanned for LitBool`

- <span id="litbool-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::lit::LitBool`

- <span id="cratelitlitbool-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitBool`

### `LitByte`

```rust
struct LitByte {
    repr: Box<LitRepr>,
}
```

A byte literal: `b'f'`.

#### Implementations

- <span id="cratelitlitbyte-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for LitByte`

- <span id="litbyte-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::lit::LitByte`

- <span id="cratelitlitbyte-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitByte`

##### `impl Hash for LitByte`

- <span id="litbyte-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::lit::LitByte`

- <span id="cratelitlitbyte-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for LitByte`

- <span id="litbyte-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for crate::lit::LitByte`

##### `impl<T> Spanned for LitByte`

- <span id="litbyte-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::lit::LitByte`

- <span id="cratelitlitbyte-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitByte`

### `LitByteStr`

```rust
struct LitByteStr {
    repr: Box<LitRepr>,
}
```

A byte string literal: `b"foo"`.

#### Implementations

- <span id="litbytestr-new"></span>`fn new(value: &[u8], span: Span) -> Self`

- <span id="litbytestr-value"></span>`fn value(&self) -> Vec<u8>`

- <span id="litbytestr-span"></span>`fn span(&self) -> Span`

- <span id="litbytestr-set-span"></span>`fn set_span(&mut self, span: Span)`

- <span id="litbytestr-suffix"></span>`fn suffix(&self) -> &str`

- <span id="litbytestr-token"></span>`fn token(&self) -> Literal`

#### Trait Implementations

##### `impl Clone for LitByteStr`

- <span id="litbytestr-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::lit::LitByteStr`

- <span id="cratelitlitbytestr-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitByteStr`

##### `impl Hash for LitByteStr`

- <span id="litbytestr-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::lit::LitByteStr`

- <span id="cratelitlitbytestr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for LitByteStr`

- <span id="litbytestr-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for crate::lit::LitByteStr`

##### `impl<T> Spanned for LitByteStr`

- <span id="litbytestr-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::lit::LitByteStr`

- <span id="cratelitlitbytestr-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitByteStr`

### `LitCStr`

```rust
struct LitCStr {
    repr: Box<LitRepr>,
}
```

A nul-terminated C-string literal: `c"foo"`.

#### Implementations

- <span id="litcstr-new"></span>`fn new(value: &CStr, span: Span) -> Self`

- <span id="litcstr-value"></span>`fn value(&self) -> CString`

- <span id="litcstr-span"></span>`fn span(&self) -> Span`

- <span id="litcstr-set-span"></span>`fn set_span(&mut self, span: Span)`

- <span id="litcstr-suffix"></span>`fn suffix(&self) -> &str`

- <span id="litcstr-token"></span>`fn token(&self) -> Literal`

#### Trait Implementations

##### `impl Clone for LitCStr`

- <span id="litcstr-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::lit::LitCStr`

- <span id="cratelitlitcstr-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitCStr`

##### `impl Hash for LitCStr`

- <span id="litcstr-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::lit::LitCStr`

- <span id="cratelitlitcstr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for LitCStr`

- <span id="litcstr-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for LitCStr`

##### `impl<T> Spanned for LitCStr`

- <span id="litcstr-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::lit::LitCStr`

- <span id="cratelitlitcstr-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitCStr`

### `LitChar`

```rust
struct LitChar {
    repr: Box<LitRepr>,
}
```

A character literal: `'a'`.

#### Implementations

- <span id="litchar-new"></span>`fn new(value: char, span: Span) -> Self`

- <span id="litchar-value"></span>`fn value(&self) -> char`

- <span id="litchar-span"></span>`fn span(&self) -> Span`

- <span id="litchar-set-span"></span>`fn set_span(&mut self, span: Span)`

- <span id="litchar-suffix"></span>`fn suffix(&self) -> &str`

- <span id="litchar-token"></span>`fn token(&self) -> Literal`

#### Trait Implementations

##### `impl Clone for LitChar`

- <span id="litchar-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::lit::LitChar`

- <span id="cratelitlitchar-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitChar`

##### `impl Hash for LitChar`

- <span id="litchar-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::lit::LitChar`

- <span id="cratelitlitchar-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for LitChar`

- <span id="litchar-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for LitChar`

##### `impl<T> Spanned for LitChar`

- <span id="litchar-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::lit::LitChar`

- <span id="cratelitlitchar-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitChar`

### `LitFloat`

```rust
struct LitFloat {
    repr: Box<LitFloatRepr>,
}
```

A floating point literal: `1f64` or `1.0e10f64`.

Must be finite. May not be infinite or NaN.

#### Implementations

- <span id="cratelitlitfloat-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for LitFloat`

- <span id="litfloat-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::lit::LitFloat`

- <span id="cratelitlitfloat-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for LitFloat`

- <span id="litfloat-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitFloat`

##### `impl Hash for LitFloat`

- <span id="litfloat-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::lit::LitFloat`

- <span id="cratelitlitfloat-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for LitFloat`

- <span id="litfloat-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for crate::lit::LitFloat`

##### `impl<T> Spanned for LitFloat`

- <span id="litfloat-span"></span>`fn span(&self) -> Span`

##### `impl<T> ToString for LitFloat`

- <span id="litfloat-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for crate::lit::LitFloat`

- <span id="cratelitlitfloat-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitFloat`

### `LitInt`

```rust
struct LitInt {
    repr: Box<LitIntRepr>,
}
```

An integer literal: `1` or `1u16`.

#### Implementations

- <span id="litint-new"></span>`fn new(repr: &str, span: Span) -> Self`

- <span id="litint-base10-digits"></span>`fn base10_digits(&self) -> &str`

- <span id="litint-base10-parse"></span>`fn base10_parse<N>(&self) -> Result<N>` — [`Result`](#result)

- <span id="litint-suffix"></span>`fn suffix(&self) -> &str`

- <span id="litint-span"></span>`fn span(&self) -> Span`

- <span id="litint-set-span"></span>`fn set_span(&mut self, span: Span)`

- <span id="litint-token"></span>`fn token(&self) -> Literal`

#### Trait Implementations

##### `impl Clone for LitInt`

- <span id="litint-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::lit::LitInt`

- <span id="cratelitlitint-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for LitInt`

- <span id="litint-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitInt`

##### `impl Hash for LitInt`

- <span id="litint-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::lit::LitInt`

- <span id="cratelitlitint-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for LitInt`

- <span id="litint-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for LitInt`

##### `impl<T> Spanned for LitInt`

- <span id="litint-span"></span>`fn span(&self) -> Span`

##### `impl<T> ToString for LitInt`

- <span id="litint-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for crate::lit::LitInt`

- <span id="cratelitlitint-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitInt`

### `LitStr`

```rust
struct LitStr {
    repr: Box<LitRepr>,
}
```

A UTF-8 string literal: `"foo"`.

#### Implementations

- <span id="litstr-new"></span>`fn new(value: &str, span: Span) -> Self`

- <span id="litstr-value"></span>`fn value(&self) -> String`

- <span id="litstr-parse"></span>`fn parse<T: Parse>(&self) -> Result<T>` — [`Result`](#result)

- <span id="litstr-parse-with"></span>`fn parse_with<F: Parser>(&self, parser: F) -> Result<<F as >::Output>` — [`Result`](#result), [`Parser`](parse/index.md)

- <span id="litstr-span"></span>`fn span(&self) -> Span`

- <span id="litstr-set-span"></span>`fn set_span(&mut self, span: Span)`

- <span id="litstr-suffix"></span>`fn suffix(&self) -> &str`

- <span id="litstr-token"></span>`fn token(&self) -> Literal`

#### Trait Implementations

##### `impl Clone for LitStr`

- <span id="litstr-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::lit::LitStr`

- <span id="cratelitlitstr-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitStr`

##### `impl Hash for LitStr`

- <span id="litstr-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::lit::LitStr`

- <span id="cratelitlitstr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for LitStr`

- <span id="litstr-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for LitStr`

##### `impl<T> Spanned for LitStr`

- <span id="litstr-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::lit::LitStr`

- <span id="cratelitlitstr-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitStr`

### `Macro`

```rust
struct Macro {
    pub path: crate::path::Path,
    pub bang_token: token::Not,
    pub delimiter: MacroDelimiter,
    pub tokens: proc_macro2::TokenStream,
}
```

A macro invocation: `println!("{}", mac)`.

#### Implementations

- <span id="macro-parse-body"></span>`fn parse_body<T: Parse>(&self) -> Result<T>` — [`Result`](#result)

- <span id="macro-parse-body-with"></span>`fn parse_body_with<F: Parser>(&self, parser: F) -> Result<<F as >::Output>` — [`Result`](#result), [`Parser`](parse/index.md)

#### Trait Implementations

##### `impl Clone for crate::Macro`

- <span id="cratemacro-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Macro`

- <span id="cratemacro-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Macro`

##### `impl Hash for crate::Macro`

- <span id="cratemacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::mac::Macro`

- <span id="cratemacmacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Macro`

- <span id="cratemacro-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Macro`

##### `impl<T> Spanned for Macro`

- <span id="macro-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::mac::Macro`

- <span id="cratemacmacro-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `FieldPat`

```rust
struct FieldPat {
    pub attrs: Vec<crate::attr::Attribute>,
    pub member: crate::expr::Member,
    pub colon_token: Option<token::Colon>,
    pub pat: Box<Pat>,
}
```

A single field in a struct pattern.

Patterns like the fields of Foo `{ x, ref y, ref mut z }` are treated
the same as `x: x, y: ref y, z: ref mut z` but there is no colon token.

#### Trait Implementations

##### `impl Clone for crate::FieldPat`

- <span id="cratefieldpat-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::FieldPat`

- <span id="cratefieldpat-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldPat`

##### `impl Hash for crate::FieldPat`

- <span id="cratefieldpat-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::FieldPat`

- <span id="cratefieldpat-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for FieldPat`

##### `impl<T> Spanned for FieldPat`

- <span id="fieldpat-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::pat::FieldPat`

- <span id="cratepatfieldpat-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatConst`

```rust
struct PatConst {
    pub attrs: Vec<crate::attr::Attribute>,
    pub const_token: token::Const,
    pub block: crate::stmt::Block,
}
```

A const block: `const { ... }`.

#### Implementations

- <span id="crateexprconst-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprConst`

- <span id="crateexprconst-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprConst`

- <span id="crateexprconst-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprConst`

##### `impl Hash for crate::ExprConst`

- <span id="crateexprconst-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprConst`

- <span id="crateexprexprconst-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprConst`

- <span id="crateexprconst-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprConst`

##### `impl<T> Spanned for ExprConst`

- <span id="exprconst-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprConst`

- <span id="crateexprexprconst-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatIdent`

```rust
struct PatIdent {
    pub attrs: Vec<crate::attr::Attribute>,
    pub by_ref: Option<token::Ref>,
    pub mutability: Option<token::Mut>,
    pub ident: crate::ident::Ident,
    pub subpat: Option<(token::At, Box<Pat>)>,
}
```

A pattern that binds a new variable: `ref mut binding @ SUBPATTERN`.

It may also be a unit struct or struct variant (e.g. `None`), or a
constant; these cannot be distinguished syntactically.

#### Implementations

- <span id="cratepatident-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatIdent`

- <span id="cratepatident-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PatIdent`

- <span id="cratepatident-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatIdent`

##### `impl Hash for crate::PatIdent`

- <span id="cratepatident-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::PatIdent`

- <span id="cratepatident-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for PatIdent`

##### `impl<T> Spanned for PatIdent`

- <span id="patident-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::pat::PatIdent`

- <span id="cratepatpatident-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatLit`

```rust
struct PatLit {
    pub attrs: Vec<crate::attr::Attribute>,
    pub lit: crate::lit::Lit,
}
```

A literal in place of an expression: `1`, `"foo"`.

#### Implementations

- <span id="crateexprlit-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprLit`

- <span id="crateexprlit-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprLit`

- <span id="crateexprlit-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprLit`

##### `impl Hash for crate::ExprLit`

- <span id="crateexprlit-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprLit`

- <span id="crateexprexprlit-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprLit`

- <span id="crateexprlit-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprLit`

##### `impl<T> Spanned for ExprLit`

- <span id="exprlit-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprLit`

- <span id="crateexprexprlit-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatMacro`

```rust
struct PatMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
}
```

A macro invocation expression: `format!("{}", q)`.

#### Implementations

- <span id="crateexprmacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprMacro`

- <span id="crateexprmacro-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprMacro`

- <span id="crateexprmacro-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprMacro`

##### `impl Hash for crate::ExprMacro`

- <span id="crateexprmacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprMacro`

- <span id="crateexprexprmacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprMacro`

- <span id="crateexprmacro-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprMacro`

##### `impl<T> Spanned for ExprMacro`

- <span id="exprmacro-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprMacro`

- <span id="crateexprexprmacro-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatOr`

```rust
struct PatOr {
    pub attrs: Vec<crate::attr::Attribute>,
    pub leading_vert: Option<token::Or>,
    pub cases: crate::punctuated::Punctuated<Pat, token::Or>,
}
```

A pattern that matches any one of a set of cases.

#### Implementations

- <span id="cratepator-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatOr`

- <span id="cratepator-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PatOr`

- <span id="cratepator-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatOr`

##### `impl Hash for crate::PatOr`

- <span id="cratepator-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::PatOr`

- <span id="cratepator-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for PatOr`

##### `impl<T> Spanned for PatOr`

- <span id="pator-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::pat::PatOr`

- <span id="cratepatpator-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatParen`

```rust
struct PatParen {
    pub attrs: Vec<crate::attr::Attribute>,
    pub paren_token: token::Paren,
    pub pat: Box<Pat>,
}
```

A parenthesized pattern: `(A | B)`.

#### Implementations

- <span id="cratepatparen-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatParen`

- <span id="cratepatparen-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PatParen`

- <span id="cratepatparen-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatParen`

##### `impl Hash for crate::PatParen`

- <span id="cratepatparen-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::PatParen`

- <span id="cratepatparen-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for PatParen`

##### `impl<T> Spanned for PatParen`

- <span id="patparen-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::pat::PatParen`

- <span id="cratepatpatparen-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatPath`

```rust
struct PatPath {
    pub attrs: Vec<crate::attr::Attribute>,
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
}
```

A path like `std::mem::replace` possibly containing generic
parameters and a qualified self-type.

A plain identifier like `x` is a path of length 1.

#### Implementations

- <span id="crateexprpath-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprPath`

- <span id="crateexprpath-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprPath`

- <span id="crateexprpath-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprPath`

##### `impl Hash for crate::ExprPath`

- <span id="crateexprpath-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprPath`

- <span id="crateexprexprpath-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprPath`

- <span id="crateexprpath-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprPath`

##### `impl<T> Spanned for ExprPath`

- <span id="exprpath-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprPath`

- <span id="crateexprexprpath-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatRange`

```rust
struct PatRange {
    pub attrs: Vec<crate::attr::Attribute>,
    pub start: Option<Box<Expr>>,
    pub limits: RangeLimits,
    pub end: Option<Box<Expr>>,
}
```

A range expression: `1..2`, `1..`, `..2`, `1..=2`, `..=2`.

#### Implementations

- <span id="crateexprrange-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprRange`

- <span id="crateexprrange-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprRange`

- <span id="crateexprrange-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprRange`

##### `impl Hash for crate::ExprRange`

- <span id="crateexprrange-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprRange`

- <span id="crateexprexprrange-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprRange`

- <span id="crateexprrange-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprRange`

##### `impl<T> Spanned for ExprRange`

- <span id="exprrange-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprRange`

- <span id="crateexprexprrange-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatReference`

```rust
struct PatReference {
    pub attrs: Vec<crate::attr::Attribute>,
    pub and_token: token::And,
    pub mutability: Option<token::Mut>,
    pub pat: Box<Pat>,
}
```

A reference pattern: `&mut var`.

#### Implementations

- <span id="cratepatreference-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatReference`

- <span id="cratepatreference-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PatReference`

- <span id="cratepatreference-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatReference`

##### `impl Hash for crate::PatReference`

- <span id="cratepatreference-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::PatReference`

- <span id="cratepatreference-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for PatReference`

##### `impl<T> Spanned for PatReference`

- <span id="patreference-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::pat::PatReference`

- <span id="cratepatpatreference-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatRest`

```rust
struct PatRest {
    pub attrs: Vec<crate::attr::Attribute>,
    pub dot2_token: token::DotDot,
}
```

The dots in a tuple or slice pattern: `[0, 1, ..]`.

#### Implementations

- <span id="cratepatrest-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatRest`

- <span id="cratepatrest-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PatRest`

- <span id="cratepatrest-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatRest`

##### `impl Hash for crate::PatRest`

- <span id="cratepatrest-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::PatRest`

- <span id="cratepatrest-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for PatRest`

##### `impl<T> Spanned for PatRest`

- <span id="patrest-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::pat::PatRest`

- <span id="cratepatpatrest-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatSlice`

```rust
struct PatSlice {
    pub attrs: Vec<crate::attr::Attribute>,
    pub bracket_token: token::Bracket,
    pub elems: crate::punctuated::Punctuated<Pat, token::Comma>,
}
```

A dynamically sized slice pattern: `[a, b, ref i @ .., y, z]`.

#### Implementations

- <span id="cratepatslice-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatSlice`

- <span id="cratepatslice-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PatSlice`

- <span id="cratepatslice-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatSlice`

##### `impl Hash for crate::PatSlice`

- <span id="cratepatslice-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::PatSlice`

- <span id="cratepatslice-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for PatSlice`

##### `impl<T> Spanned for PatSlice`

- <span id="patslice-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::pat::PatSlice`

- <span id="cratepatpatslice-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatStruct`

```rust
struct PatStruct {
    pub attrs: Vec<crate::attr::Attribute>,
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
    pub brace_token: token::Brace,
    pub fields: crate::punctuated::Punctuated<FieldPat, token::Comma>,
    pub rest: Option<PatRest>,
}
```

A struct or struct variant pattern: `Variant { x, y, .. }`.

#### Implementations

- <span id="cratepatstruct-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatStruct`

- <span id="cratepatstruct-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PatStruct`

- <span id="cratepatstruct-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatStruct`

##### `impl Hash for crate::PatStruct`

- <span id="cratepatstruct-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::PatStruct`

- <span id="cratepatstruct-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for PatStruct`

##### `impl<T> Spanned for PatStruct`

- <span id="patstruct-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::pat::PatStruct`

- <span id="cratepatpatstruct-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatTuple`

```rust
struct PatTuple {
    pub attrs: Vec<crate::attr::Attribute>,
    pub paren_token: token::Paren,
    pub elems: crate::punctuated::Punctuated<Pat, token::Comma>,
}
```

A tuple pattern: `(a, b)`.

#### Implementations

- <span id="cratepattuple-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatTuple`

- <span id="cratepattuple-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PatTuple`

- <span id="cratepattuple-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatTuple`

##### `impl Hash for crate::PatTuple`

- <span id="cratepattuple-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::PatTuple`

- <span id="cratepattuple-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for PatTuple`

##### `impl<T> Spanned for PatTuple`

- <span id="pattuple-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::pat::PatTuple`

- <span id="cratepatpattuple-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatTupleStruct`

```rust
struct PatTupleStruct {
    pub attrs: Vec<crate::attr::Attribute>,
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
    pub paren_token: token::Paren,
    pub elems: crate::punctuated::Punctuated<Pat, token::Comma>,
}
```

A tuple struct or tuple variant pattern: `Variant(x, y, .., z)`.

#### Implementations

- <span id="cratepattuplestruct-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatTupleStruct`

- <span id="cratepattuplestruct-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PatTupleStruct`

- <span id="cratepattuplestruct-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatTupleStruct`

##### `impl Hash for crate::PatTupleStruct`

- <span id="cratepattuplestruct-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::PatTupleStruct`

- <span id="cratepattuplestruct-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for PatTupleStruct`

##### `impl<T> Spanned for PatTupleStruct`

- <span id="pattuplestruct-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::pat::PatTupleStruct`

- <span id="cratepatpattuplestruct-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatType`

```rust
struct PatType {
    pub attrs: Vec<crate::attr::Attribute>,
    pub pat: Box<Pat>,
    pub colon_token: token::Colon,
    pub ty: Box<crate::ty::Type>,
}
```

A type ascription pattern: `foo: f64`.

#### Implementations

- <span id="cratepattype-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatType`

- <span id="cratepattype-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PatType`

- <span id="cratepattype-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatType`

##### `impl Hash for crate::PatType`

- <span id="cratepattype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::pat::PatType`

- <span id="cratepatpattype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::PatType`

- <span id="cratepattype-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for PatType`

##### `impl<T> Spanned for PatType`

- <span id="pattype-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::pat::PatType`

- <span id="cratepatpattype-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PatWild`

```rust
struct PatWild {
    pub attrs: Vec<crate::attr::Attribute>,
    pub underscore_token: token::Underscore,
}
```

A pattern that matches any value: `_`.

#### Implementations

- <span id="cratepatwild-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatWild`

- <span id="cratepatwild-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PatWild`

- <span id="cratepatwild-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatWild`

##### `impl Hash for crate::PatWild`

- <span id="cratepatwild-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::PatWild`

- <span id="cratepatwild-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for PatWild`

##### `impl<T> Spanned for PatWild`

- <span id="patwild-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::pat::PatWild`

- <span id="cratepatpatwild-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `AngleBracketedGenericArguments`

```rust
struct AngleBracketedGenericArguments {
    pub colon2_token: Option<token::PathSep>,
    pub lt_token: token::Lt,
    pub args: crate::punctuated::Punctuated<GenericArgument, token::Comma>,
    pub gt_token: token::Gt,
}
```

Angle bracketed arguments of a path segment: the `<K, V>` in `HashMap<K,
V>`.

#### Implementations

- <span id="crateanglebracketedgenericarguments-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::AngleBracketedGenericArguments`

- <span id="crateanglebracketedgenericarguments-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::AngleBracketedGenericArguments`

- <span id="crateanglebracketedgenericarguments-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::AngleBracketedGenericArguments`

##### `impl Hash for crate::AngleBracketedGenericArguments`

- <span id="crateanglebracketedgenericarguments-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::path::AngleBracketedGenericArguments`

- <span id="cratepathanglebracketedgenericarguments-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::AngleBracketedGenericArguments`

- <span id="crateanglebracketedgenericarguments-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for AngleBracketedGenericArguments`

##### `impl<T> Spanned for AngleBracketedGenericArguments`

- <span id="anglebracketedgenericarguments-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::path::AngleBracketedGenericArguments`

- <span id="cratepathanglebracketedgenericarguments-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `AssocConst`

```rust
struct AssocConst {
    pub ident: crate::ident::Ident,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub eq_token: token::Eq,
    pub value: crate::expr::Expr,
}
```

An equality constraint on an associated constant: the `PANIC = false` in
`Trait<PANIC = false>`.

#### Trait Implementations

##### `impl Clone for crate::AssocConst`

- <span id="crateassocconst-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::AssocConst`

- <span id="crateassocconst-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::AssocConst`

##### `impl Hash for crate::AssocConst`

- <span id="crateassocconst-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::AssocConst`

- <span id="crateassocconst-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for AssocConst`

##### `impl<T> Spanned for AssocConst`

- <span id="assocconst-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::path::AssocConst`

- <span id="cratepathassocconst-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `AssocType`

```rust
struct AssocType {
    pub ident: crate::ident::Ident,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub eq_token: token::Eq,
    pub ty: crate::ty::Type,
}
```

A binding (equality constraint) on an associated type: the `Item = u8`
in `Iterator<Item = u8>`.

#### Trait Implementations

##### `impl Clone for crate::AssocType`

- <span id="crateassoctype-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::AssocType`

- <span id="crateassoctype-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::AssocType`

##### `impl Hash for crate::AssocType`

- <span id="crateassoctype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::AssocType`

- <span id="crateassoctype-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for AssocType`

##### `impl<T> Spanned for AssocType`

- <span id="assoctype-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::path::AssocType`

- <span id="cratepathassoctype-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Constraint`

```rust
struct Constraint {
    pub ident: crate::ident::Ident,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub colon_token: token::Colon,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, token::Plus>,
}
```

An associated type bound: `Iterator<Item: Display>`.

#### Trait Implementations

##### `impl Clone for crate::Constraint`

- <span id="crateconstraint-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Constraint`

- <span id="crateconstraint-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Constraint`

##### `impl Hash for crate::Constraint`

- <span id="crateconstraint-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::Constraint`

- <span id="crateconstraint-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Constraint`

##### `impl<T> Spanned for Constraint`

- <span id="constraint-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::path::Constraint`

- <span id="cratepathconstraint-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ParenthesizedGenericArguments`

```rust
struct ParenthesizedGenericArguments {
    pub paren_token: token::Paren,
    pub inputs: crate::punctuated::Punctuated<crate::ty::Type, token::Comma>,
    pub output: crate::ty::ReturnType,
}
```

Arguments of a function path segment: the `(A, B) -> C` in `Fn(A,B) ->
C`.

#### Fields

- **`inputs`**: `crate::punctuated::Punctuated<crate::ty::Type, token::Comma>`

  `(A, B)`

- **`output`**: `crate::ty::ReturnType`

  `C`

#### Implementations

- <span id="crateparenthesizedgenericarguments-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ParenthesizedGenericArguments`

- <span id="crateparenthesizedgenericarguments-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ParenthesizedGenericArguments`

- <span id="crateparenthesizedgenericarguments-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ParenthesizedGenericArguments`

##### `impl Hash for crate::ParenthesizedGenericArguments`

- <span id="crateparenthesizedgenericarguments-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::path::ParenthesizedGenericArguments`

- <span id="cratepathparenthesizedgenericarguments-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ParenthesizedGenericArguments`

- <span id="crateparenthesizedgenericarguments-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ParenthesizedGenericArguments`

##### `impl<T> Spanned for ParenthesizedGenericArguments`

- <span id="parenthesizedgenericarguments-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::path::ParenthesizedGenericArguments`

- <span id="cratepathparenthesizedgenericarguments-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Path`

```rust
struct Path {
    pub leading_colon: Option<token::PathSep>,
    pub segments: crate::punctuated::Punctuated<PathSegment, token::PathSep>,
}
```

A path at which a named item is exported (e.g. `std::collections::HashMap`).

#### Implementations

- <span id="cratepath-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::Path`

- <span id="cratepath-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Path`

- <span id="cratepath-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Path`

##### `impl Hash for crate::Path`

- <span id="cratepath-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::path::Path`

- <span id="cratepathpath-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Path`

- <span id="cratepath-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialEq for syn::Path`

##### `impl<T> Sealed for Path`

##### `impl<T> Spanned for Path`

- <span id="path-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::path::Path`

- <span id="cratepathpath-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PathSegment`

```rust
struct PathSegment {
    pub ident: crate::ident::Ident,
    pub arguments: PathArguments,
}
```

A segment of a path together with any path arguments on that segment.

#### Implementations

- <span id="cratepathpathsegment-parse-helper"></span>`fn parse_helper(input: ParseStream<'_>, expr_style: bool) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

#### Trait Implementations

##### `impl Clone for crate::PathSegment`

- <span id="cratepathsegment-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PathSegment`

- <span id="cratepathsegment-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PathSegment`

##### `impl Hash for crate::PathSegment`

- <span id="cratepathsegment-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::path::PathSegment`

- <span id="cratepathpathsegment-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::PathSegment`

- <span id="cratepathsegment-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for PathSegment`

##### `impl<T> Spanned for PathSegment`

- <span id="pathsegment-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::path::PathSegment`

- <span id="cratepathpathsegment-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `QSelf`

```rust
struct QSelf {
    pub lt_token: token::Lt,
    pub ty: Box<crate::ty::Type>,
    pub position: usize,
    pub as_token: Option<token::As>,
    pub gt_token: token::Gt,
}
```

The explicit Self type in a qualified path: the `T` in `<T as
Display>::fmt`.

The actual path, including the trait and the associated item, is stored
separately. The `position` field represents the index of the associated
item qualified with this Self type.

```text
<Vec<T> as a::b::Trait>::AssociatedItem
 ^~~~~~    ~~~~~~~~~~~~~~^
 ty        position = 3

<Vec<T>>::AssociatedItem
 ^~~~~~   ^
 ty       position = 0
```

#### Trait Implementations

##### `impl Clone for crate::QSelf`

- <span id="crateqself-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::QSelf`

- <span id="crateqself-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::QSelf`

##### `impl Hash for crate::QSelf`

- <span id="crateqself-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::QSelf`

- <span id="crateqself-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for crate::QSelf`

##### `impl Spanned for crate::path::QSelf`

- <span id="cratepathqself-span"></span>`fn span(&self) -> Span`

### `VisRestricted`

```rust
struct VisRestricted {
    pub pub_token: token::Pub,
    pub paren_token: token::Paren,
    pub in_token: Option<token::In>,
    pub path: Box<crate::path::Path>,
}
```

A visibility level restricted to some path: `pub(self)` or
`pub(super)` or `pub(crate)` or `pub(in some::module)`.

#### Implementations

- <span id="cratevisrestricted-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::VisRestricted`

- <span id="cratevisrestricted-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::VisRestricted`

- <span id="cratevisrestricted-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::VisRestricted`

##### `impl Hash for crate::VisRestricted`

- <span id="cratevisrestricted-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::VisRestricted`

- <span id="cratevisrestricted-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for VisRestricted`

##### `impl<T> Spanned for VisRestricted`

- <span id="visrestricted-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::restriction::VisRestricted`

- <span id="craterestrictionvisrestricted-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Block`

```rust
struct Block {
    pub brace_token: token::Brace,
    pub stmts: Vec<Stmt>,
}
```

A braced block containing Rust statements.

#### Fields

- **`stmts`**: `Vec<Stmt>`

  Statements in a block

#### Implementations

- <span id="cratestmtblock-parse-within"></span>`fn parse_within(input: ParseStream<'_>) -> Result<Vec<Stmt>>` — [`ParseStream`](parse/index.md), [`Result`](#result), [`Stmt`](#stmt)

#### Trait Implementations

##### `impl Clone for crate::Block`

- <span id="crateblock-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Block`

- <span id="crateblock-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Block`

##### `impl Hash for crate::Block`

- <span id="crateblock-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::stmt::Block`

- <span id="cratestmtblock-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Block`

- <span id="crateblock-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Block`

##### `impl<T> Spanned for Block`

- <span id="block-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::stmt::Block`

- <span id="cratestmtblock-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Local`

```rust
struct Local {
    pub attrs: Vec<crate::attr::Attribute>,
    pub let_token: token::Let,
    pub pat: crate::pat::Pat,
    pub init: Option<LocalInit>,
    pub semi_token: token::Semi,
}
```

A local `let` binding: `let x: u64 = s.parse()?;`.

#### Implementations

- <span id="cratelocal-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::Local`

- <span id="cratelocal-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Local`

- <span id="cratelocal-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Local`

##### `impl Hash for crate::Local`

- <span id="cratelocal-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::Local`

- <span id="cratelocal-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Local`

##### `impl<T> Spanned for Local`

- <span id="local-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::stmt::Local`

- <span id="cratestmtlocal-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `LocalInit`

```rust
struct LocalInit {
    pub eq_token: token::Eq,
    pub expr: Box<crate::expr::Expr>,
    pub diverge: Option<(token::Else, Box<crate::expr::Expr>)>,
}
```

The expression assigned in a local `let` binding, including optional
diverging `else` block.

`LocalInit` represents `= s.parse()?` in `let x: u64 = s.parse()?` and
`= r else { return }` in `let Ok(x) = r else { return }`.

#### Trait Implementations

##### `impl Clone for crate::LocalInit`

- <span id="cratelocalinit-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::LocalInit`

- <span id="cratelocalinit-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LocalInit`

##### `impl Hash for crate::LocalInit`

- <span id="cratelocalinit-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::LocalInit`

- <span id="cratelocalinit-eq"></span>`fn eq(&self, other: &Self) -> bool`

### `StmtMacro`

```rust
struct StmtMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<token::Semi>,
}
```

A macro invocation in statement position.

Syntactically it's ambiguous which other kind of statement this macro
would expand to. It can be any of local variable (`let`), item, or
expression.

#### Implementations

- <span id="cratestmtmacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::StmtMacro`

- <span id="cratestmtmacro-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::StmtMacro`

- <span id="cratestmtmacro-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::StmtMacro`

##### `impl Hash for crate::StmtMacro`

- <span id="cratestmtmacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::StmtMacro`

- <span id="cratestmtmacro-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for StmtMacro`

##### `impl<T> Spanned for StmtMacro`

- <span id="stmtmacro-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::stmt::StmtMacro`

- <span id="cratestmtstmtmacro-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Abi`

```rust
struct Abi {
    pub extern_token: token::Extern,
    pub name: Option<crate::lit::LitStr>,
}
```

The binary interface of a function: `extern "C"`.

#### Trait Implementations

##### `impl Clone for crate::Abi`

- <span id="crateabi-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Abi`

- <span id="crateabi-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Abi`

##### `impl Hash for crate::Abi`

- <span id="crateabi-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::Abi`

- <span id="cratetyabi-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Abi`

- <span id="crateabi-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Abi`

##### `impl<T> Spanned for Abi`

- <span id="abi-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::Abi`

- <span id="cratetyabi-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `BareFnArg`

```rust
struct BareFnArg {
    pub attrs: Vec<crate::attr::Attribute>,
    pub name: Option<(crate::ident::Ident, token::Colon)>,
    pub ty: Type,
}
```

An argument in a function type: the `usize` in `fn(usize) -> bool`.

#### Trait Implementations

##### `impl Clone for crate::BareFnArg`

- <span id="cratebarefnarg-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::BareFnArg`

- <span id="cratebarefnarg-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::BareFnArg`

##### `impl Hash for crate::BareFnArg`

- <span id="cratebarefnarg-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::BareFnArg`

- <span id="cratetybarefnarg-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::BareFnArg`

- <span id="cratebarefnarg-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for BareFnArg`

##### `impl<T> Spanned for BareFnArg`

- <span id="barefnarg-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::BareFnArg`

- <span id="cratetybarefnarg-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `BareVariadic`

```rust
struct BareVariadic {
    pub attrs: Vec<crate::attr::Attribute>,
    pub name: Option<(crate::ident::Ident, token::Colon)>,
    pub dots: token::DotDotDot,
    pub comma: Option<token::Comma>,
}
```

The variadic argument of a function pointer like `fn(usize, ...)`.

#### Trait Implementations

##### `impl Clone for crate::BareVariadic`

- <span id="cratebarevariadic-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::BareVariadic`

- <span id="cratebarevariadic-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::BareVariadic`

##### `impl Hash for crate::BareVariadic`

- <span id="cratebarevariadic-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::BareVariadic`

- <span id="cratebarevariadic-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for BareVariadic`

##### `impl<T> Spanned for BareVariadic`

- <span id="barevariadic-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::BareVariadic`

- <span id="cratetybarevariadic-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypeArray`

```rust
struct TypeArray {
    pub bracket_token: token::Bracket,
    pub elem: Box<Type>,
    pub semi_token: token::Semi,
    pub len: crate::expr::Expr,
}
```

A fixed size array type: `[T; n]`.

#### Implementations

- <span id="cratetypearray-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeArray`

- <span id="cratetypearray-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypeArray`

- <span id="cratetypearray-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeArray`

##### `impl Hash for crate::TypeArray`

- <span id="cratetypearray-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::TypeArray`

- <span id="cratetytypearray-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypeArray`

- <span id="cratetypearray-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeArray`

##### `impl<T> Spanned for TypeArray`

- <span id="typearray-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::TypeArray`

- <span id="cratetytypearray-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypeBareFn`

```rust
struct TypeBareFn {
    pub lifetimes: Option<crate::generics::BoundLifetimes>,
    pub unsafety: Option<token::Unsafe>,
    pub abi: Option<Abi>,
    pub fn_token: token::Fn,
    pub paren_token: token::Paren,
    pub inputs: crate::punctuated::Punctuated<BareFnArg, token::Comma>,
    pub variadic: Option<BareVariadic>,
    pub output: ReturnType,
}
```

A bare function type: `fn(usize) -> bool`.

#### Implementations

- <span id="cratetypebarefn-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeBareFn`

- <span id="cratetypebarefn-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypeBareFn`

- <span id="cratetypebarefn-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeBareFn`

##### `impl Hash for crate::TypeBareFn`

- <span id="cratetypebarefn-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::TypeBareFn`

- <span id="cratetytypebarefn-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypeBareFn`

- <span id="cratetypebarefn-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeBareFn`

##### `impl<T> Spanned for TypeBareFn`

- <span id="typebarefn-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::TypeBareFn`

- <span id="cratetytypebarefn-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypeGroup`

```rust
struct TypeGroup {
    pub group_token: token::Group,
    pub elem: Box<Type>,
}
```

A type contained within invisible delimiters.

#### Implementations

- <span id="cratetypegroup-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeGroup`

- <span id="cratetypegroup-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypeGroup`

- <span id="cratetypegroup-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeGroup`

##### `impl Hash for crate::TypeGroup`

- <span id="cratetypegroup-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::TypeGroup`

- <span id="cratetytypegroup-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypeGroup`

- <span id="cratetypegroup-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeGroup`

##### `impl<T> Spanned for TypeGroup`

- <span id="typegroup-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::TypeGroup`

- <span id="cratetytypegroup-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypeImplTrait`

```rust
struct TypeImplTrait {
    pub impl_token: token::Impl,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, token::Plus>,
}
```

An `impl Bound1 + Bound2 + Bound3` type where `Bound` is a trait or
a lifetime.

#### Implementations

- <span id="cratetytypeimpltrait-without-plus"></span>`fn without_plus(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

- <span id="cratetytypeimpltrait-parse"></span>`fn parse(input: ParseStream<'_>, allow_plus: bool) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

#### Trait Implementations

##### `impl Clone for crate::TypeImplTrait`

- <span id="cratetypeimpltrait-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypeImplTrait`

- <span id="cratetypeimpltrait-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeImplTrait`

##### `impl Hash for crate::TypeImplTrait`

- <span id="cratetypeimpltrait-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::TypeImplTrait`

- <span id="cratetytypeimpltrait-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypeImplTrait`

- <span id="cratetypeimpltrait-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeImplTrait`

##### `impl<T> Spanned for TypeImplTrait`

- <span id="typeimpltrait-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::TypeImplTrait`

- <span id="cratetytypeimpltrait-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypeInfer`

```rust
struct TypeInfer {
    pub underscore_token: token::Underscore,
}
```

Indication that a type should be inferred by the compiler: `_`.

#### Implementations

- <span id="cratetypeinfer-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeInfer`

- <span id="cratetypeinfer-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypeInfer`

- <span id="cratetypeinfer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeInfer`

##### `impl Hash for crate::TypeInfer`

- <span id="cratetypeinfer-hash"></span>`fn hash<H>(&self, _state: &mut H)`

##### `impl Parse for crate::ty::TypeInfer`

- <span id="cratetytypeinfer-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypeInfer`

- <span id="cratetypeinfer-eq"></span>`fn eq(&self, _other: &Self) -> bool`

##### `impl<T> Sealed for TypeInfer`

##### `impl<T> Spanned for TypeInfer`

- <span id="typeinfer-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::TypeInfer`

- <span id="cratetytypeinfer-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypeMacro`

```rust
struct TypeMacro {
    pub mac: crate::mac::Macro,
}
```

A macro in the type position.

#### Implementations

- <span id="cratetypemacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeMacro`

- <span id="cratetypemacro-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypeMacro`

- <span id="cratetypemacro-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeMacro`

##### `impl Hash for crate::TypeMacro`

- <span id="cratetypemacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::TypeMacro`

- <span id="cratetytypemacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypeMacro`

- <span id="cratetypemacro-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeMacro`

##### `impl<T> Spanned for TypeMacro`

- <span id="typemacro-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::TypeMacro`

- <span id="cratetytypemacro-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypeNever`

```rust
struct TypeNever {
    pub bang_token: token::Not,
}
```

The never type: `!`.

#### Implementations

- <span id="cratetypenever-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeNever`

- <span id="cratetypenever-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypeNever`

- <span id="cratetypenever-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeNever`

##### `impl Hash for crate::TypeNever`

- <span id="cratetypenever-hash"></span>`fn hash<H>(&self, _state: &mut H)`

##### `impl Parse for crate::ty::TypeNever`

- <span id="cratetytypenever-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypeNever`

- <span id="cratetypenever-eq"></span>`fn eq(&self, _other: &Self) -> bool`

##### `impl<T> Sealed for TypeNever`

##### `impl<T> Spanned for TypeNever`

- <span id="typenever-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::TypeNever`

- <span id="cratetytypenever-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypeParen`

```rust
struct TypeParen {
    pub paren_token: token::Paren,
    pub elem: Box<Type>,
}
```

A parenthesized type equivalent to the inner type.

#### Implementations

- <span id="cratetypeparen-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeParen`

- <span id="cratetypeparen-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypeParen`

- <span id="cratetypeparen-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeParen`

##### `impl Hash for crate::TypeParen`

- <span id="cratetypeparen-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::TypeParen`

- <span id="cratetytypeparen-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypeParen`

- <span id="cratetypeparen-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeParen`

##### `impl<T> Spanned for TypeParen`

- <span id="typeparen-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::TypeParen`

- <span id="cratetytypeparen-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypePath`

```rust
struct TypePath {
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
}
```

A path like `std::slice::Iter`, optionally qualified with a
self-type as in `<Vec<T> as SomeTrait>::Associated`.

#### Implementations

- <span id="cratetypepath-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypePath`

- <span id="cratetypepath-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypePath`

- <span id="cratetypepath-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypePath`

##### `impl Hash for crate::TypePath`

- <span id="cratetypepath-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::TypePath`

- <span id="cratetytypepath-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypePath`

- <span id="cratetypepath-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TypePath`

##### `impl<T> Spanned for TypePath`

- <span id="typepath-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::TypePath`

- <span id="cratetytypepath-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypePtr`

```rust
struct TypePtr {
    pub star_token: token::Star,
    pub const_token: Option<token::Const>,
    pub mutability: Option<token::Mut>,
    pub elem: Box<Type>,
}
```

A raw pointer type: `*const T` or `*mut T`.

#### Implementations

- <span id="cratetypeptr-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypePtr`

- <span id="cratetypeptr-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypePtr`

- <span id="cratetypeptr-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypePtr`

##### `impl Hash for crate::TypePtr`

- <span id="cratetypeptr-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::TypePtr`

- <span id="cratetytypeptr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypePtr`

- <span id="cratetypeptr-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TypePtr`

##### `impl<T> Spanned for TypePtr`

- <span id="typeptr-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::TypePtr`

- <span id="cratetytypeptr-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypeReference`

```rust
struct TypeReference {
    pub and_token: token::And,
    pub lifetime: Option<crate::lifetime::Lifetime>,
    pub mutability: Option<token::Mut>,
    pub elem: Box<Type>,
}
```

A reference type: `&'a T` or `&'a mut T`.

#### Implementations

- <span id="cratetypereference-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeReference`

- <span id="cratetypereference-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypeReference`

- <span id="cratetypereference-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeReference`

##### `impl Hash for crate::TypeReference`

- <span id="cratetypereference-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::TypeReference`

- <span id="cratetytypereference-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypeReference`

- <span id="cratetypereference-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeReference`

##### `impl<T> Spanned for TypeReference`

- <span id="typereference-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::TypeReference`

- <span id="cratetytypereference-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypeSlice`

```rust
struct TypeSlice {
    pub bracket_token: token::Bracket,
    pub elem: Box<Type>,
}
```

A dynamically sized slice type: `[T]`.

#### Implementations

- <span id="cratetypeslice-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeSlice`

- <span id="cratetypeslice-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypeSlice`

- <span id="cratetypeslice-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeSlice`

##### `impl Hash for crate::TypeSlice`

- <span id="cratetypeslice-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::TypeSlice`

- <span id="cratetytypeslice-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypeSlice`

- <span id="cratetypeslice-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeSlice`

##### `impl<T> Spanned for TypeSlice`

- <span id="typeslice-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::TypeSlice`

- <span id="cratetytypeslice-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypeTraitObject`

```rust
struct TypeTraitObject {
    pub dyn_token: Option<token::Dyn>,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, token::Plus>,
}
```

A trait object type `dyn Bound1 + Bound2 + Bound3` where `Bound` is a
trait or a lifetime.

#### Implementations

- <span id="cratetypetraitobject-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeTraitObject`

- <span id="cratetypetraitobject-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypeTraitObject`

- <span id="cratetypetraitobject-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeTraitObject`

##### `impl Hash for crate::TypeTraitObject`

- <span id="cratetypetraitobject-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::TypeTraitObject`

- <span id="cratetytypetraitobject-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypeTraitObject`

- <span id="cratetypetraitobject-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeTraitObject`

##### `impl<T> Spanned for TypeTraitObject`

- <span id="typetraitobject-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::TypeTraitObject`

- <span id="cratetytypetraitobject-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypeTuple`

```rust
struct TypeTuple {
    pub paren_token: token::Paren,
    pub elems: crate::punctuated::Punctuated<Type, token::Comma>,
}
```

A tuple type: `(A, B, C, String)`.

#### Implementations

- <span id="cratetypetuple-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeTuple`

- <span id="cratetypetuple-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypeTuple`

- <span id="cratetypetuple-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeTuple`

##### `impl Hash for crate::TypeTuple`

- <span id="cratetypetuple-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::TypeTuple`

- <span id="cratetytypetuple-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypeTuple`

- <span id="cratetypetuple-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeTuple`

##### `impl<T> Spanned for TypeTuple`

- <span id="typetuple-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::TypeTuple`

- <span id="cratetytypetuple-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

## Enums

### `AttrStyle`

```rust
enum AttrStyle {
    Outer,
    Inner(token::Not),
}
```

Distinguishes between attributes that decorate an item and attributes
that are contained within an item.

# Outer attributes

- `#[repr(transparent)]`
- `/// # Example`
- `/** Please file an issue */`

# Inner attributes

- `#![feature(proc_macro)]`
- `//! # Example`
- `/*! Please file an issue */`

#### Trait Implementations

##### `impl Clone for crate::AttrStyle`

- <span id="crateattrstyle-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for crate::AttrStyle`

##### `impl Debug for crate::AttrStyle`

- <span id="crateattrstyle-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::AttrStyle`

##### `impl Hash for crate::AttrStyle`

- <span id="crateattrstyle-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::AttrStyle`

- <span id="crateattrstyle-eq"></span>`fn eq(&self, other: &Self) -> bool`

### `Meta`

```rust
enum Meta {
    Path(crate::path::Path),
    List(MetaList),
    NameValue(MetaNameValue),
}
```

Content of a compile-time structured attribute.

## Path

A meta path is like the `test` in `#[test]`.

## List

A meta list is like the `derive(Copy)` in `#[derive(Copy)]`.

## NameValue

A name-value meta is like the `path = "..."` in `#[path =
"sys/windows.rs"]`.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`List`**

  A structured list within an attribute, like `derive(Copy, Clone)`.

- **`NameValue`**

  A name-value pair within an attribute, like `feature = "nightly"`.

#### Implementations

- <span id="meta-path"></span>`fn path(&self) -> &Path` — [`Path`](#path)

- <span id="meta-require-path-only"></span>`fn require_path_only(&self) -> Result<&Path>` — [`Result`](#result), [`Path`](#path)

- <span id="meta-require-list"></span>`fn require_list(&self) -> Result<&MetaList>` — [`Result`](#result), [`MetaList`](#metalist)

- <span id="meta-require-name-value"></span>`fn require_name_value(&self) -> Result<&MetaNameValue>` — [`Result`](#result), [`MetaNameValue`](#metanamevalue)

#### Trait Implementations

##### `impl Clone for crate::Meta`

- <span id="cratemeta-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Meta`

- <span id="cratemeta-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Meta`

##### `impl Hash for crate::Meta`

- <span id="cratemeta-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::attr::Meta`

- <span id="crateattrmeta-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Meta`

- <span id="cratemeta-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Meta`

##### `impl<T> Spanned for Meta`

- <span id="meta-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::attr::Meta`

- <span id="crateattrmeta-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Fields`

```rust
enum Fields {
    Named(FieldsNamed),
    Unnamed(FieldsUnnamed),
    Unit,
}
```

Data stored within an enum variant or struct.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Named`**

  Named fields of a struct or struct variant such as `Point { x: f64,
  y: f64 }`.

- **`Unnamed`**

  Unnamed fields of a tuple struct or tuple variant such as `Some(T)`.

- **`Unit`**

  Unit struct or unit variant such as `None`.

#### Implementations

- <span id="fields-iter"></span>`fn iter(&self) -> punctuated::Iter<'_, Field>` — [`Iter`](punctuated/index.md), [`Field`](#field)

- <span id="fields-iter-mut"></span>`fn iter_mut(&mut self) -> punctuated::IterMut<'_, Field>` — [`IterMut`](punctuated/index.md), [`Field`](#field)

- <span id="fields-len"></span>`fn len(&self) -> usize`

- <span id="fields-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="fields-members"></span>`fn members(&self) -> Members<'_>` — [`Members`](data/index.md)

#### Trait Implementations

##### `impl Clone for crate::Fields`

- <span id="cratefields-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Fields`

- <span id="cratefields-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Fields`

##### `impl Hash for crate::Fields`

- <span id="cratefields-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl IntoIterator for Fields`

- <span id="fields-item"></span>`type Item = Field`

- <span id="fields-intoiter"></span>`type IntoIter = IntoIter<Field>`

- <span id="fields-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl PartialEq for crate::Fields`

- <span id="cratefields-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Fields`

##### `impl<T> Spanned for Fields`

- <span id="fields-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Fields`

- <span id="fields-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

### `Data`

```rust
enum Data {
    Struct(DataStruct),
    Enum(DataEnum),
    Union(DataUnion),
}
```

The storage of a struct, enum or union data structure.

# Syntax tree enum

This type is a [syntax tree enum].


#### Trait Implementations

##### `impl Clone for crate::Data`

- <span id="cratedata-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Data`

- <span id="cratedata-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Data`

##### `impl Hash for crate::Data`

- <span id="cratedata-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::Data`

- <span id="cratedata-eq"></span>`fn eq(&self, other: &Self) -> bool`

### `PointerMutability`

```rust
enum PointerMutability {
    Const(token::Const),
    Mut(token::Mut),
}
```

Mutability of a raw pointer (`*const T`, `*mut T`), in which non-mutable
isn't the implicit default.

#### Trait Implementations

##### `impl Clone for crate::PointerMutability`

- <span id="cratepointermutability-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PointerMutability`

- <span id="cratepointermutability-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PointerMutability`

##### `impl Hash for crate::PointerMutability`

- <span id="cratepointermutability-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::PointerMutability`

- <span id="crateexprpointermutability-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::PointerMutability`

- <span id="cratepointermutability-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for PointerMutability`

##### `impl<T> Spanned for PointerMutability`

- <span id="pointermutability-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::PointerMutability`

- <span id="crateexprpointermutability-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `RangeLimits`

```rust
enum RangeLimits {
    HalfOpen(token::DotDot),
    Closed(token::DotDotEq),
}
```

Limit types of a range, inclusive or exclusive.

#### Variants

- **`HalfOpen`**

  Inclusive at the beginning, exclusive at the end.

- **`Closed`**

  Inclusive at the beginning and end.

#### Implementations

- <span id="crateexprrangelimits-parse-obsolete"></span>`fn parse_obsolete(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

#### Trait Implementations

##### `impl Clone for crate::RangeLimits`

- <span id="craterangelimits-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for crate::RangeLimits`

##### `impl Debug for crate::RangeLimits`

- <span id="craterangelimits-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::RangeLimits`

##### `impl Hash for crate::RangeLimits`

- <span id="craterangelimits-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::RangeLimits`

- <span id="crateexprrangelimits-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::RangeLimits`

- <span id="craterangelimits-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for RangeLimits`

##### `impl<T> Spanned for RangeLimits`

- <span id="rangelimits-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::RangeLimits`

- <span id="crateexprrangelimits-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Expr`

```rust
enum Expr {
    Array(ExprArray),
    Assign(ExprAssign),
    Async(ExprAsync),
    Await(ExprAwait),
    Binary(ExprBinary),
    Block(ExprBlock),
    Break(ExprBreak),
    Call(ExprCall),
    Cast(ExprCast),
    Closure(ExprClosure),
    Const(ExprConst),
    Continue(ExprContinue),
    Field(ExprField),
    ForLoop(ExprForLoop),
    Group(ExprGroup),
    If(ExprIf),
    Index(ExprIndex),
    Infer(ExprInfer),
    Let(ExprLet),
    Lit(ExprLit),
    Loop(ExprLoop),
    Macro(ExprMacro),
    Match(ExprMatch),
    MethodCall(ExprMethodCall),
    Paren(ExprParen),
    Path(ExprPath),
    Range(ExprRange),
    RawAddr(ExprRawAddr),
    Reference(ExprReference),
    Repeat(ExprRepeat),
    Return(ExprReturn),
    Struct(ExprStruct),
    Try(ExprTry),
    TryBlock(ExprTryBlock),
    Tuple(ExprTuple),
    Unary(ExprUnary),
    Unsafe(ExprUnsafe),
    Verbatim(proc_macro2::TokenStream),
    While(ExprWhile),
    Yield(ExprYield),
}
```

A Rust expression.

*This type is available only if Syn is built with the `"derive"` or `"full"`
feature, but most of the variants are not available unless "full" is enabled.*

# Syntax tree enums

This type is a syntax tree enum. In Syn this and other syntax tree enums
are designed to be traversed using the following rebinding idiom.

```rust
use syn::Expr;

fn example(expr: Expr) {
const IGNORE: &str = stringify! {
let expr: Expr = /* ... */;
};
match expr {
    Expr::MethodCall(expr) => {
        /* ... */
    }
    Expr::Cast(expr) => {
        /* ... */
    }
    Expr::If(expr) => {
        /* ... */
    }

    /* ... */
    _ => {}
}
}
```

We begin with a variable `expr` of type `Expr` that has no fields
(because it is an enum), and by matching on it and rebinding a variable
with the same name `expr` we effectively imbue our variable with all of
the data fields provided by the variant that it turned out to be. So for
example above if we ended up in the `MethodCall` case then we get to use
`expr.receiver`, `expr.args` etc; if we ended up in the `If` case we get
to use `expr.cond`, `expr.then_branch`, `expr.else_branch`.

This approach avoids repeating the variant names twice on every line.

```rust
use syn::{Expr, ExprMethodCall};

fn example(expr: Expr) {
// Repetitive; recommend not doing this.
match expr {
    Expr::MethodCall(ExprMethodCall { method, args, .. }) => {
}
_ => {}
}
}
```

In general, the name to which a syntax tree enum variant is bound should
be a suitable name for the complete syntax tree enum type.

```rust
use syn::{Expr, ExprField};

fn example(discriminant: ExprField) {
// Binding is called `base` which is the name I would use if I were
// assigning `*discriminant.base` without an `if let`.
if let Expr::Tuple(base) = *discriminant.base {
}
}
```

A sign that you may not be choosing the right variable names is if you
see names getting repeated in your code, like accessing
`receiver.receiver` or `pat.pat` or `cond.cond`.

#### Variants

- **`Array`**

  A slice literal expression: `[a, b, c, d]`.

- **`Assign`**

  An assignment expression: `a = compute()`.

- **`Async`**

  An async block: `async { ... }`.

- **`Await`**

  An await expression: `fut.await`.

- **`Binary`**

  A binary operation: `a + b`, `a += b`.

- **`Block`**

  A blocked scope: `{ ... }`.

- **`Break`**

  A `break`, with an optional label to break and an optional
  expression.

- **`Call`**

  A function call expression: `invoke(a, b)`.

- **`Cast`**

  A cast expression: `foo as f64`.

- **`Closure`**

  A closure expression: `|a, b| a + b`.

- **`Const`**

  A const block: `const { ... }`.

- **`Continue`**

  A `continue`, with an optional label.

- **`Field`**

  Access of a named struct field (`obj.k`) or unnamed tuple struct
  field (`obj.0`).

- **`ForLoop`**

  A for loop: `for pat in expr { ... }`.

- **`Group`**

  An expression contained within invisible delimiters.
  
  This variant is important for faithfully representing the precedence
  of expressions and is related to `None`-delimited spans in a
  `TokenStream`.

- **`If`**

  An `if` expression with an optional `else` block: `if expr { ... }
  else { ... }`.
  
  The `else` branch expression may only be an `If` or `Block`
  expression, not any of the other types of expression.

- **`Index`**

  A square bracketed indexing expression: `vector[2]`.

- **`Infer`**

  The inferred value of a const generic argument, denoted `_`.

- **`Let`**

  A `let` guard: `let Some(x) = opt`.

- **`Lit`**

  A literal in place of an expression: `1`, `"foo"`.

- **`Loop`**

  Conditionless loop: `loop { ... }`.

- **`Macro`**

  A macro invocation expression: `format!("{}", q)`.

- **`Match`**

  A `match` expression: `match n { Some(n) => {}, None => {} }`.

- **`MethodCall`**

  A method call expression: `x.foo::<T>(a, b)`.

- **`Paren`**

  A parenthesized expression: `(a + b)`.

- **`Path`**

  A path like `std::mem::replace` possibly containing generic
  parameters and a qualified self-type.
  
  A plain identifier like `x` is a path of length 1.

- **`Range`**

  A range expression: `1..2`, `1..`, `..2`, `1..=2`, `..=2`.

- **`RawAddr`**

  Address-of operation: `&raw const place` or `&raw mut place`.

- **`Reference`**

  A referencing operation: `&a` or `&mut a`.

- **`Repeat`**

  An array literal constructed from one repeated element: `[0u8; N]`.

- **`Return`**

  A `return`, with an optional value to be returned.

- **`Struct`**

  A struct literal expression: `Point { x: 1, y: 1 }`.
  
  The `rest` provides the value of the remaining fields as in `S { a:
  1, b: 1, ..rest }`.

- **`Try`**

  A try-expression: `expr?`.

- **`TryBlock`**

  A try block: `try { ... }`.

- **`Tuple`**

  A tuple expression: `(a, b, c, d)`.

- **`Unary`**

  A unary operation: `!x`, `*x`.

- **`Unsafe`**

  An unsafe block: `unsafe { ... }`.

- **`Verbatim`**

  Tokens in expression position not interpreted by Syn.

- **`While`**

  A while loop: `while expr { ... }`.

- **`Yield`**

  A yield expression: `yield expr`.

#### Implementations

- <span id="expr-placeholder"></span>`const PLACEHOLDER: Self`

- <span id="expr-parse-without-eager-brace"></span>`fn parse_without_eager_brace(input: ParseStream<'_>) -> Result<Expr>` — [`ParseStream`](parse/index.md), [`Result`](#result), [`Expr`](#expr)

- <span id="expr-parse-with-earlier-boundary-rule"></span>`fn parse_with_earlier_boundary_rule(input: ParseStream<'_>) -> Result<Expr>` — [`ParseStream`](parse/index.md), [`Result`](#result), [`Expr`](#expr)

- <span id="expr-peek"></span>`fn peek(input: ParseStream<'_>) -> bool` — [`ParseStream`](parse/index.md)

- <span id="expr-replace-attrs"></span>`fn replace_attrs(&mut self, new: Vec<Attribute>) -> Vec<Attribute>` — [`Attribute`](#attribute)

#### Trait Implementations

##### `impl Clone for crate::Expr`

- <span id="crateexpr-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Expr`

- <span id="crateexpr-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Expr`

##### `impl Hash for crate::Expr`

- <span id="crateexpr-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::Expr`

- <span id="crateexprexpr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Expr`

- <span id="crateexpr-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Expr`

##### `impl<T> Spanned for Expr`

- <span id="expr-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Expr`

- <span id="expr-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

### `Member`

```rust
enum Member {
    Named(crate::ident::Ident),
    Unnamed(Index),
}
```

A struct or tuple struct field accessed in a struct literal or field
expression.

#### Variants

- **`Named`**

  A named field like `self.x`.

- **`Unnamed`**

  An unnamed field like `self.0`.

#### Implementations

- <span id="member-is-named"></span>`fn is_named(&self) -> bool`

#### Trait Implementations

##### `impl Clone for crate::Member`

- <span id="cratemember-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Member`

- <span id="cratemember-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Member`

##### `impl Hash for Member`

- <span id="member-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl IdentFragment for Member`

- <span id="member-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="member-span"></span>`fn span(&self) -> Option<Span>`

##### `impl Parse for crate::expr::Member`

- <span id="crateexprmember-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for Member`

- <span id="member-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Member`

##### `impl<T> Spanned for Member`

- <span id="member-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::Member`

- <span id="crateexprmember-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `GenericParam`

```rust
enum GenericParam {
    Lifetime(LifetimeParam),
    Type(TypeParam),
    Const(ConstParam),
}
```

A generic type parameter, lifetime, or const generic: `T: Into<String>`,
`'a: 'b`, `const LEN: usize`.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Lifetime`**

  A lifetime parameter: `'a: 'b + 'c + 'd`.

- **`Type`**

  A generic type parameter: `T: Into<String>`.

- **`Const`**

  A const generic parameter: `const LENGTH: usize`.

#### Trait Implementations

##### `impl Clone for crate::GenericParam`

- <span id="crategenericparam-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::GenericParam`

- <span id="crategenericparam-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::GenericParam`

##### `impl Hash for crate::GenericParam`

- <span id="crategenericparam-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::generics::GenericParam`

- <span id="crategenericsgenericparam-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::GenericParam`

- <span id="crategenericparam-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for GenericParam`

##### `impl<T> Spanned for GenericParam`

- <span id="genericparam-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for GenericParam`

- <span id="genericparam-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

### `TraitBoundModifier`

```rust
enum TraitBoundModifier {
    None,
    Maybe(token::Question),
}
```

A modifier on a trait bound, currently only used for the `?` in
`?Sized`.

#### Trait Implementations

##### `impl Clone for crate::TraitBoundModifier`

- <span id="cratetraitboundmodifier-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for crate::TraitBoundModifier`

##### `impl Debug for crate::TraitBoundModifier`

- <span id="cratetraitboundmodifier-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitBoundModifier`

##### `impl Hash for crate::TraitBoundModifier`

- <span id="cratetraitboundmodifier-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::generics::TraitBoundModifier`

- <span id="crategenericstraitboundmodifier-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TraitBoundModifier`

- <span id="cratetraitboundmodifier-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TraitBoundModifier`

##### `impl<T> Spanned for TraitBoundModifier`

- <span id="traitboundmodifier-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::generics::TraitBoundModifier`

- <span id="crategenericstraitboundmodifier-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypeParamBound`

```rust
enum TypeParamBound {
    Trait(TraitBound),
    Lifetime(crate::lifetime::Lifetime),
    PreciseCapture(PreciseCapture),
    Verbatim(proc_macro2::TokenStream),
}
```

A trait or lifetime used as a bound on a type parameter.

#### Implementations

- <span id="crategenericstypeparambound-parse-single"></span>`fn parse_single(input: ParseStream<'_>, allow_precise_capture: bool, allow_const: bool) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

- <span id="crategenericstypeparambound-parse-multiple"></span>`fn parse_multiple(input: ParseStream<'_>, allow_plus: bool, allow_precise_capture: bool, allow_const: bool) -> Result<Punctuated<Self, token::Plus>>` — [`ParseStream`](parse/index.md), [`Result`](#result), [`Punctuated`](punctuated/index.md), [`Plus`](token/index.md)

#### Trait Implementations

##### `impl Clone for crate::TypeParamBound`

- <span id="cratetypeparambound-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypeParamBound`

- <span id="cratetypeparambound-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeParamBound`

##### `impl Hash for crate::TypeParamBound`

- <span id="cratetypeparambound-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::generics::TypeParamBound`

- <span id="crategenericstypeparambound-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypeParamBound`

- <span id="cratetypeparambound-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeParamBound`

##### `impl<T> Spanned for TypeParamBound`

- <span id="typeparambound-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for TypeParamBound`

- <span id="typeparambound-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

### `WherePredicate`

```rust
enum WherePredicate {
    Lifetime(PredicateLifetime),
    Type(PredicateType),
}
```

A single predicate in a `where` clause: `T: Deserialize<'de>`.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Lifetime`**

  A lifetime predicate in a `where` clause: `'a: 'b + 'c`.

- **`Type`**

  A type predicate in a `where` clause: `for<'c> Foo<'c>: Trait<'c>`.

#### Trait Implementations

##### `impl Clone for crate::WherePredicate`

- <span id="cratewherepredicate-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::WherePredicate`

- <span id="cratewherepredicate-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::WherePredicate`

##### `impl Hash for crate::WherePredicate`

- <span id="cratewherepredicate-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::generics::WherePredicate`

- <span id="crategenericswherepredicate-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::WherePredicate`

- <span id="cratewherepredicate-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for WherePredicate`

##### `impl<T> Spanned for WherePredicate`

- <span id="wherepredicate-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for WherePredicate`

- <span id="wherepredicate-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

### `CapturedParam`

```rust
enum CapturedParam {
    Lifetime(crate::lifetime::Lifetime),
    Ident(crate::ident::Ident),
}
```

Single parameter in a precise capturing bound.

#### Variants

- **`Lifetime`**

  A lifetime parameter in precise capturing bound: `fn f<'a>() -> impl
  Trait + use<'a>`.

- **`Ident`**

  A type parameter or const generic parameter in precise capturing
  bound: `fn f<T>() -> impl Trait + use<T>` or `fn f<const K: T>() ->
  impl Trait + use<K>`.

#### Trait Implementations

##### `impl Clone for crate::CapturedParam`

- <span id="cratecapturedparam-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::CapturedParam`

- <span id="cratecapturedparam-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::CapturedParam`

##### `impl Hash for crate::CapturedParam`

- <span id="cratecapturedparam-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::generics::CapturedParam`

- <span id="crategenericscapturedparam-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::CapturedParam`

- <span id="cratecapturedparam-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for CapturedParam`

##### `impl<T> Spanned for CapturedParam`

- <span id="capturedparam-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::generics::CapturedParam`

- <span id="crategenericscapturedparam-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `FnArg`

```rust
enum FnArg {
    Receiver(Receiver),
    Typed(crate::pat::PatType),
}
```

An argument in a function signature: the `n: usize` in `fn f(n: usize)`.

#### Variants

- **`Receiver`**

  The `self` argument of an associated method.

- **`Typed`**

  A function argument accepted by pattern and type.

#### Trait Implementations

##### `impl Clone for crate::FnArg`

- <span id="cratefnarg-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::FnArg`

- <span id="cratefnarg-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FnArg`

##### `impl Hash for crate::FnArg`

- <span id="cratefnarg-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::FnArg`

- <span id="crateitemfnarg-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::FnArg`

- <span id="cratefnarg-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for FnArg`

##### `impl<T> Spanned for FnArg`

- <span id="fnarg-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for FnArg`

- <span id="fnarg-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

### `ForeignItem`

```rust
enum ForeignItem {
    Fn(ForeignItemFn),
    Static(ForeignItemStatic),
    Type(ForeignItemType),
    Macro(ForeignItemMacro),
    Verbatim(proc_macro2::TokenStream),
}
```

An item within an `extern` block.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Fn`**

  A foreign function in an `extern` block.

- **`Static`**

  A foreign static item in an `extern` block: `static ext: u8`.

- **`Type`**

  A foreign type in an `extern` block: `type void`.

- **`Macro`**

  A macro invocation within an extern block.

- **`Verbatim`**

  Tokens in an `extern` block not interpreted by Syn.

#### Trait Implementations

##### `impl Clone for crate::ForeignItem`

- <span id="crateforeignitem-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ForeignItem`

- <span id="crateforeignitem-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItem`

##### `impl Hash for crate::ForeignItem`

- <span id="crateforeignitem-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ForeignItem`

- <span id="crateitemforeignitem-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ForeignItem`

- <span id="crateforeignitem-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ForeignItem`

##### `impl<T> Spanned for ForeignItem`

- <span id="foreignitem-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for ForeignItem`

- <span id="foreignitem-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

### `ImplItem`

```rust
enum ImplItem {
    Const(ImplItemConst),
    Fn(ImplItemFn),
    Type(ImplItemType),
    Macro(ImplItemMacro),
    Verbatim(proc_macro2::TokenStream),
}
```

An item within an impl block.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Const`**

  An associated constant within an impl block.

- **`Fn`**

  An associated function within an impl block.

- **`Type`**

  An associated type within an impl block.

- **`Macro`**

  A macro invocation within an impl block.

- **`Verbatim`**

  Tokens within an impl block not interpreted by Syn.

#### Trait Implementations

##### `impl Clone for crate::ImplItem`

- <span id="crateimplitem-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ImplItem`

- <span id="crateimplitem-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItem`

##### `impl Hash for crate::ImplItem`

- <span id="crateimplitem-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ImplItem`

- <span id="crateitemimplitem-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ImplItem`

- <span id="crateimplitem-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ImplItem`

##### `impl<T> Spanned for ImplItem`

- <span id="implitem-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for ImplItem`

- <span id="implitem-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

### `ImplRestriction`

```rust
enum ImplRestriction {
}
```

Unused, but reserved for RFC 3323 restrictions.

#### Trait Implementations

##### `impl Clone for crate::ImplRestriction`

- <span id="crateimplrestriction-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ImplRestriction`

- <span id="crateimplrestriction-fmt"></span>`fn fmt(&self, _formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplRestriction`

##### `impl Hash for crate::ImplRestriction`

- <span id="crateimplrestriction-hash"></span>`fn hash<H>(&self, _state: &mut H)`

##### `impl PartialEq for crate::ImplRestriction`

- <span id="crateimplrestriction-eq"></span>`fn eq(&self, _other: &Self) -> bool`

### `Item`

```rust
enum Item {
    Const(ItemConst),
    Enum(ItemEnum),
    ExternCrate(ItemExternCrate),
    Fn(ItemFn),
    ForeignMod(ItemForeignMod),
    Impl(ItemImpl),
    Macro(ItemMacro),
    Mod(ItemMod),
    Static(ItemStatic),
    Struct(ItemStruct),
    Trait(ItemTrait),
    TraitAlias(ItemTraitAlias),
    Type(ItemType),
    Union(ItemUnion),
    Use(ItemUse),
    Verbatim(proc_macro2::TokenStream),
}
```

Things that can appear directly inside of a module or scope.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Const`**

  A constant item: `const MAX: u16 = 65535`.

- **`Enum`**

  An enum definition: `enum Foo<A, B> { A(A), B(B) }`.

- **`ExternCrate`**

  An `extern crate` item: `extern crate serde`.

- **`Fn`**

  A free-standing function: `fn process(n: usize) -> Result<()> { ...
  }`.

- **`ForeignMod`**

  A block of foreign items: `extern "C" { ... }`.

- **`Impl`**

  An impl block providing trait or associated items: `impl<A> Trait
  for Data<A> { ... }`.

- **`Macro`**

  A macro invocation, which includes `macro_rules!` definitions.

- **`Mod`**

  A module or module declaration: `mod m` or `mod m { ... }`.

- **`Static`**

  A static item: `static BIKE: Shed = Shed(42)`.

- **`Struct`**

  A struct definition: `struct Foo<A> { x: A }`.

- **`Trait`**

  A trait definition: `pub trait Iterator { ... }`.

- **`TraitAlias`**

  A trait alias: `pub trait SharableIterator = Iterator + Sync`.

- **`Type`**

  A type alias: `type Result<T> = std::result::Result<T, MyError>`.

- **`Union`**

  A union definition: `union Foo<A, B> { x: A, y: B }`.

- **`Use`**

  A use declaration: `use std::collections::HashMap`.

- **`Verbatim`**

  Tokens forming an item not interpreted by Syn.

#### Implementations

- <span id="item-replace-attrs"></span>`fn replace_attrs(&mut self, new: Vec<Attribute>) -> Vec<Attribute>` — [`Attribute`](#attribute)

#### Trait Implementations

##### `impl Clone for crate::Item`

- <span id="crateitem-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Item`

- <span id="crateitem-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Item`

##### `impl Hash for crate::Item`

- <span id="crateitem-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::Item`

- <span id="crateitemitem-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Item`

- <span id="crateitem-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Item`

##### `impl<T> Spanned for Item`

- <span id="item-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Item`

- <span id="item-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

### `StaticMutability`

```rust
enum StaticMutability {
    Mut(token::Mut),
    None,
}
```

The mutability of an `Item::Static` or `ForeignItem::Static`.

#### Trait Implementations

##### `impl Clone for crate::StaticMutability`

- <span id="cratestaticmutability-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::StaticMutability`

- <span id="cratestaticmutability-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::StaticMutability`

##### `impl Hash for crate::StaticMutability`

- <span id="cratestaticmutability-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::StaticMutability`

- <span id="crateitemstaticmutability-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::StaticMutability`

- <span id="cratestaticmutability-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for StaticMutability`

##### `impl<T> Spanned for StaticMutability`

- <span id="staticmutability-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::StaticMutability`

- <span id="crateitemstaticmutability-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TraitItem`

```rust
enum TraitItem {
    Const(TraitItemConst),
    Fn(TraitItemFn),
    Type(TraitItemType),
    Macro(TraitItemMacro),
    Verbatim(proc_macro2::TokenStream),
}
```

An item declaration within the definition of a trait.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Const`**

  An associated constant within the definition of a trait.

- **`Fn`**

  An associated function within the definition of a trait.

- **`Type`**

  An associated type within the definition of a trait.

- **`Macro`**

  A macro invocation within the definition of a trait.

- **`Verbatim`**

  Tokens within the definition of a trait not interpreted by Syn.

#### Trait Implementations

##### `impl Clone for crate::TraitItem`

- <span id="cratetraititem-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TraitItem`

- <span id="cratetraititem-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItem`

##### `impl Hash for crate::TraitItem`

- <span id="cratetraititem-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::TraitItem`

- <span id="crateitemtraititem-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TraitItem`

- <span id="cratetraititem-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TraitItem`

##### `impl<T> Spanned for TraitItem`

- <span id="traititem-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for TraitItem`

- <span id="traititem-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

### `UseTree`

```rust
enum UseTree {
    Path(UsePath),
    Name(UseName),
    Rename(UseRename),
    Glob(UseGlob),
    Group(UseGroup),
}
```

A suffix of an import tree in a `use` item: `Type as Renamed` or `*`.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Path`**

  A path prefix of imports in a `use` item: `std::...`.

- **`Name`**

  An identifier imported by a `use` item: `HashMap`.

- **`Rename`**

  An renamed identifier imported by a `use` item: `HashMap as Map`.

- **`Glob`**

  A glob import in a `use` item: `*`.

- **`Group`**

  A braced group of imports in a `use` item: `{A, B, C}`.

#### Trait Implementations

##### `impl Clone for crate::UseTree`

- <span id="crateusetree-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::UseTree`

- <span id="crateusetree-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseTree`

##### `impl Hash for crate::UseTree`

- <span id="crateusetree-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::UseTree`

- <span id="crateitemusetree-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<UseTree>` — [`ParseStream`](parse/index.md), [`Result`](#result), [`UseTree`](#usetree)

##### `impl PartialEq for crate::UseTree`

- <span id="crateusetree-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for UseTree`

##### `impl<T> Spanned for UseTree`

- <span id="usetree-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for UseTree`

- <span id="usetree-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

### `Lit`

```rust
enum Lit {
    Str(LitStr),
    ByteStr(LitByteStr),
    CStr(LitCStr),
    Byte(LitByte),
    Char(LitChar),
    Int(LitInt),
    Float(LitFloat),
    Bool(LitBool),
    Verbatim(proc_macro2::Literal),
}
```

A Rust literal such as a string or integer or boolean.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Str`**

  A UTF-8 string literal: `"foo"`.

- **`ByteStr`**

  A byte string literal: `b"foo"`.

- **`CStr`**

  A nul-terminated C-string literal: `c"foo"`.

- **`Byte`**

  A byte literal: `b'f'`.

- **`Char`**

  A character literal: `'a'`.

- **`Int`**

  An integer literal: `1` or `1u16`.

- **`Float`**

  A floating point literal: `1f64` or `1.0e10f64`.
  
  Must be finite. May not be infinite or NaN.

- **`Bool`**

  A boolean literal: `true` or `false`.

- **`Verbatim`**

  A raw token literal not interpreted by Syn.

#### Implementations

- <span id="cratelitlit-new"></span>`fn new(token: Literal) -> Self`

- <span id="cratelitlit-from-str"></span>`fn from_str(token: Literal, repr: &str) -> Self`

- <span id="cratelitlit-suffix"></span>`fn suffix(&self) -> &str`

- <span id="cratelitlit-span"></span>`fn span(&self) -> Span`

- <span id="cratelitlit-set-span"></span>`fn set_span(&mut self, span: Span)`

#### Trait Implementations

##### `impl Clone for crate::Lit`

- <span id="cratelit-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Lit`

- <span id="cratelit-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Lit`

##### `impl Hash for crate::Lit`

- <span id="cratelit-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::lit::Lit`

- <span id="cratelitlit-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Lit`

- <span id="cratelit-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for crate::lit::Lit`

##### `impl<T> Spanned for Lit`

- <span id="lit-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Lit`

- <span id="lit-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl Token for crate::lit::Lit`

### `MacroDelimiter`

```rust
enum MacroDelimiter {
    Paren(crate::token::Paren),
    Brace(crate::token::Brace),
    Bracket(crate::token::Bracket),
}
```

A grouping token that surrounds a macro body: `m!(...)` or `m!{...}` or `m![...]`.

#### Implementations

- <span id="macrodelimiter-span"></span>`fn span(&self) -> &DelimSpan`

- <span id="macrodelimiter-is-brace"></span>`fn is_brace(&self) -> bool`

#### Trait Implementations

##### `impl Clone for crate::MacroDelimiter`

- <span id="cratemacrodelimiter-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::MacroDelimiter`

- <span id="cratemacrodelimiter-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::MacroDelimiter`

##### `impl Hash for crate::MacroDelimiter`

- <span id="cratemacrodelimiter-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::MacroDelimiter`

- <span id="cratemacrodelimiter-eq"></span>`fn eq(&self, other: &Self) -> bool`

### `BinOp`

```rust
enum BinOp {
    Add(token::Plus),
    Sub(token::Minus),
    Mul(token::Star),
    Div(token::Slash),
    Rem(token::Percent),
    And(token::AndAnd),
    Or(token::OrOr),
    BitXor(token::Caret),
    BitAnd(token::And),
    BitOr(token::Or),
    Shl(token::Shl),
    Shr(token::Shr),
    Eq(token::EqEq),
    Lt(token::Lt),
    Le(token::Le),
    Ne(token::Ne),
    Ge(token::Ge),
    Gt(token::Gt),
    AddAssign(token::PlusEq),
    SubAssign(token::MinusEq),
    MulAssign(token::StarEq),
    DivAssign(token::SlashEq),
    RemAssign(token::PercentEq),
    BitXorAssign(token::CaretEq),
    BitAndAssign(token::AndEq),
    BitOrAssign(token::OrEq),
    ShlAssign(token::ShlEq),
    ShrAssign(token::ShrEq),
}
```

A binary operator: `+`, `+=`, `&`.

#### Variants

- **`Add`**

  The `+` operator (addition)

- **`Sub`**

  The `-` operator (subtraction)

- **`Mul`**

  The `*` operator (multiplication)

- **`Div`**

  The `/` operator (division)

- **`Rem`**

  The `%` operator (modulus)

- **`And`**

  The `&&` operator (logical and)

- **`Or`**

  The `||` operator (logical or)

- **`BitXor`**

  The `^` operator (bitwise xor)

- **`BitAnd`**

  The `&` operator (bitwise and)

- **`BitOr`**

  The `|` operator (bitwise or)

- **`Shl`**

  The `<<` operator (shift left)

- **`Shr`**

  The `>>` operator (shift right)

- **`Eq`**

  The `==` operator (equality)

- **`Lt`**

  The `<` operator (less than)

- **`Le`**

  The `<=` operator (less than or equal to)

- **`Ne`**

  The `!=` operator (not equal to)

- **`Ge`**

  The `>=` operator (greater than or equal to)

- **`Gt`**

  The `>` operator (greater than)

- **`AddAssign`**

  The `+=` operator

- **`SubAssign`**

  The `-=` operator

- **`MulAssign`**

  The `*=` operator

- **`DivAssign`**

  The `/=` operator

- **`RemAssign`**

  The `%=` operator

- **`BitXorAssign`**

  The `^=` operator

- **`BitAndAssign`**

  The `&=` operator

- **`BitOrAssign`**

  The `|=` operator

- **`ShlAssign`**

  The `<<=` operator

- **`ShrAssign`**

  The `>>=` operator

#### Trait Implementations

##### `impl Clone for crate::BinOp`

- <span id="cratebinop-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for crate::BinOp`

##### `impl Debug for crate::BinOp`

- <span id="cratebinop-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::BinOp`

##### `impl Hash for crate::BinOp`

- <span id="cratebinop-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::op::BinOp`

- <span id="crateopbinop-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::BinOp`

- <span id="cratebinop-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for BinOp`

##### `impl<T> Spanned for BinOp`

- <span id="binop-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::op::BinOp`

- <span id="crateopbinop-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `UnOp`

```rust
enum UnOp {
    Deref(token::Star),
    Not(token::Not),
    Neg(token::Minus),
}
```

A unary operator: `*`, `!`, `-`.

#### Variants

- **`Deref`**

  The `*` operator for dereferencing

- **`Not`**

  The `!` operator for logical inversion

- **`Neg`**

  The `-` operator for negation

#### Trait Implementations

##### `impl Clone for crate::UnOp`

- <span id="crateunop-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for crate::UnOp`

##### `impl Debug for crate::UnOp`

- <span id="crateunop-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UnOp`

##### `impl Hash for crate::UnOp`

- <span id="crateunop-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::op::UnOp`

- <span id="crateopunop-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::UnOp`

- <span id="crateunop-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for UnOp`

##### `impl<T> Spanned for UnOp`

- <span id="unop-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::op::UnOp`

- <span id="crateopunop-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Pat`

```rust
enum Pat {
    Const(PatConst),
    Ident(PatIdent),
    Lit(PatLit),
    Macro(PatMacro),
    Or(PatOr),
    Paren(PatParen),
    Path(PatPath),
    Range(PatRange),
    Reference(PatReference),
    Rest(PatRest),
    Slice(PatSlice),
    Struct(PatStruct),
    Tuple(PatTuple),
    TupleStruct(PatTupleStruct),
    Type(PatType),
    Verbatim(proc_macro2::TokenStream),
    Wild(PatWild),
}
```

A pattern in a local binding, function signature, match expression, or
various other places.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Const`**

  A const block: `const { ... }`.

- **`Ident`**

  A pattern that binds a new variable: `ref mut binding @ SUBPATTERN`.

- **`Lit`**

  A literal pattern: `0`.

- **`Macro`**

  A macro in pattern position.

- **`Or`**

  A pattern that matches any one of a set of cases.

- **`Paren`**

  A parenthesized pattern: `(A | B)`.

- **`Path`**

  A path pattern like `Color::Red`, optionally qualified with a
  self-type.
  
  Unqualified path patterns can legally refer to variants, structs,
  constants or associated constants. Qualified path patterns like
  `<A>::B::C` and `<A as Trait>::B::C` can only legally refer to
  associated constants.

- **`Range`**

  A range pattern: `1..=2`.

- **`Reference`**

  A reference pattern: `&mut var`.

- **`Rest`**

  The dots in a tuple or slice pattern: `[0, 1, ..]`.

- **`Slice`**

  A dynamically sized slice pattern: `[a, b, ref i @ .., y, z]`.

- **`Struct`**

  A struct or struct variant pattern: `Variant { x, y, .. }`.

- **`Tuple`**

  A tuple pattern: `(a, b)`.

- **`TupleStruct`**

  A tuple struct or tuple variant pattern: `Variant(x, y, .., z)`.

- **`Type`**

  A type ascription pattern: `foo: f64`.

- **`Verbatim`**

  Tokens in pattern position not interpreted by Syn.

- **`Wild`**

  A pattern that matches any value: `_`.

#### Implementations

- <span id="cratepatpat-parse-single"></span>`fn parse_single(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

- <span id="cratepatpat-parse-multi"></span>`fn parse_multi(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

- <span id="cratepatpat-parse-multi-with-leading-vert"></span>`fn parse_multi_with_leading_vert(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

#### Trait Implementations

##### `impl Clone for crate::Pat`

- <span id="cratepat-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Pat`

- <span id="cratepat-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Pat`

##### `impl Hash for crate::Pat`

- <span id="cratepat-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::Pat`

- <span id="cratepat-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Pat`

##### `impl<T> Spanned for Pat`

- <span id="pat-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Pat`

- <span id="pat-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

### `GenericArgument`

```rust
enum GenericArgument {
    Lifetime(crate::lifetime::Lifetime),
    Type(crate::ty::Type),
    Const(crate::expr::Expr),
    AssocType(AssocType),
    AssocConst(AssocConst),
    Constraint(Constraint),
}
```

An individual generic argument, like `'a`, `T`, or `Item = T`.

#### Variants

- **`Lifetime`**

  A lifetime argument.

- **`Type`**

  A type argument.

- **`Const`**

  A const expression. Must be inside of a block.
  
  NOTE: Identity expressions are represented as Type arguments, as
  they are indistinguishable syntactically.

- **`AssocType`**

  A binding (equality constraint) on an associated type: the `Item =
  u8` in `Iterator<Item = u8>`.

- **`AssocConst`**

  An equality constraint on an associated constant: the `PANIC =
  false` in `Trait<PANIC = false>`.

- **`Constraint`**

  An associated type bound: `Iterator<Item: Display>`.

#### Trait Implementations

##### `impl Clone for crate::GenericArgument`

- <span id="crategenericargument-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::GenericArgument`

- <span id="crategenericargument-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::GenericArgument`

##### `impl Hash for crate::GenericArgument`

- <span id="crategenericargument-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::path::GenericArgument`

- <span id="cratepathgenericargument-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::GenericArgument`

- <span id="crategenericargument-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for GenericArgument`

##### `impl<T> Spanned for GenericArgument`

- <span id="genericargument-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::path::GenericArgument`

- <span id="cratepathgenericargument-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PathArguments`

```rust
enum PathArguments {
    None,
    AngleBracketed(AngleBracketedGenericArguments),
    Parenthesized(ParenthesizedGenericArguments),
}
```

Angle bracketed or parenthesized arguments of a path segment.

## Angle bracketed

The `<'a, T>` in `std::slice::iter<'a, T>`.

## Parenthesized

The `(A, B) -> C` in `Fn(A, B) -> C`.

#### Variants

- **`AngleBracketed`**

  The `<'a, T>` in `std::slice::iter<'a, T>`.

- **`Parenthesized`**

  The `(A, B) -> C` in `Fn(A, B) -> C`.

#### Implementations

- <span id="patharguments-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="patharguments-is-none"></span>`fn is_none(&self) -> bool`

#### Trait Implementations

##### `impl Clone for crate::PathArguments`

- <span id="cratepatharguments-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PathArguments`

- <span id="cratepatharguments-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PathArguments`

- <span id="patharguments-default"></span>`fn default() -> Self`

##### `impl Eq for crate::PathArguments`

##### `impl Hash for crate::PathArguments`

- <span id="cratepatharguments-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::PathArguments`

- <span id="cratepatharguments-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for PathArguments`

##### `impl<T> Spanned for PathArguments`

- <span id="patharguments-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::path::PathArguments`

- <span id="cratepathpatharguments-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `FieldMutability`

```rust
enum FieldMutability {
    None,
}
```

Unused, but reserved for RFC 3323 restrictions.

#### Trait Implementations

##### `impl Clone for crate::FieldMutability`

- <span id="cratefieldmutability-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::FieldMutability`

- <span id="cratefieldmutability-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldMutability`

##### `impl Hash for crate::FieldMutability`

- <span id="cratefieldmutability-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::FieldMutability`

- <span id="cratefieldmutability-eq"></span>`fn eq(&self, other: &Self) -> bool`

### `Visibility`

```rust
enum Visibility {
    Public(token::Pub),
    Restricted(VisRestricted),
    Inherited,
}
```

The visibility level of an item: inherited or `pub` or
`pub(restricted)`.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Public`**

  A public visibility level: `pub`.

- **`Restricted`**

  A visibility level restricted to some path: `pub(self)` or
  `pub(super)` or `pub(crate)` or `pub(in some::module)`.

- **`Inherited`**

  An inherited visibility, which usually means private.

#### Implementations

- <span id="craterestrictionvisibility-is-inherited"></span>`fn is_inherited(&self) -> bool`

#### Trait Implementations

##### `impl Clone for crate::Visibility`

- <span id="cratevisibility-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Visibility`

- <span id="cratevisibility-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Visibility`

##### `impl Hash for crate::Visibility`

- <span id="cratevisibility-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::restriction::Visibility`

- <span id="craterestrictionvisibility-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Visibility`

- <span id="cratevisibility-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Visibility`

##### `impl<T> Spanned for Visibility`

- <span id="visibility-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::restriction::Visibility`

- <span id="craterestrictionvisibility-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Stmt`

```rust
enum Stmt {
    Local(Local),
    Item(crate::item::Item),
    Expr(crate::expr::Expr, Option<token::Semi>),
    Macro(StmtMacro),
}
```

A statement, usually ending in a semicolon.

#### Variants

- **`Local`**

  A local (let) binding.

- **`Item`**

  An item definition.

- **`Expr`**

  Expression, with or without trailing semicolon.

- **`Macro`**

  A macro invocation in statement position.
  
  Syntactically it's ambiguous which other kind of statement this
  macro would expand to. It can be any of local variable (`let`),
  item, or expression.

#### Trait Implementations

##### `impl Clone for crate::Stmt`

- <span id="cratestmt-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Stmt`

- <span id="cratestmt-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Stmt`

##### `impl Hash for crate::Stmt`

- <span id="cratestmt-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::stmt::Stmt`

- <span id="cratestmtstmt-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Stmt`

- <span id="cratestmt-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Stmt`

##### `impl<T> Spanned for Stmt`

- <span id="stmt-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::stmt::Stmt`

- <span id="cratestmtstmt-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ReturnType`

```rust
enum ReturnType {
    Default,
    Type(token::RArrow, Box<Type>),
}
```

Return type of a function signature.

#### Variants

- **`Default`**

  Return type is not specified.
  
  Functions default to `()` and closures default to type inference.

- **`Type`**

  A particular type is returned.

#### Implementations

- <span id="cratetyreturntype-without-plus"></span>`fn without_plus(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

- <span id="cratetyreturntype-parse"></span>`fn parse(input: ParseStream<'_>, allow_plus: bool) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

#### Trait Implementations

##### `impl Clone for crate::ReturnType`

- <span id="cratereturntype-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ReturnType`

- <span id="cratereturntype-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ReturnType`

##### `impl Hash for crate::ReturnType`

- <span id="cratereturntype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::ReturnType`

- <span id="cratetyreturntype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ReturnType`

- <span id="cratereturntype-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ReturnType`

##### `impl<T> Spanned for ReturnType`

- <span id="returntype-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::ReturnType`

- <span id="cratetyreturntype-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Type`

```rust
enum Type {
    Array(TypeArray),
    BareFn(TypeBareFn),
    Group(TypeGroup),
    ImplTrait(TypeImplTrait),
    Infer(TypeInfer),
    Macro(TypeMacro),
    Never(TypeNever),
    Paren(TypeParen),
    Path(TypePath),
    Ptr(TypePtr),
    Reference(TypeReference),
    Slice(TypeSlice),
    TraitObject(TypeTraitObject),
    Tuple(TypeTuple),
    Verbatim(proc_macro2::TokenStream),
}
```

The possible types that a Rust value could have.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Array`**

  A fixed size array type: `[T; n]`.

- **`BareFn`**

  A bare function type: `fn(usize) -> bool`.

- **`Group`**

  A type contained within invisible delimiters.

- **`ImplTrait`**

  An `impl Bound1 + Bound2 + Bound3` type where `Bound` is a trait or
  a lifetime.

- **`Infer`**

  Indication that a type should be inferred by the compiler: `_`.

- **`Macro`**

  A macro in the type position.

- **`Never`**

  The never type: `!`.

- **`Paren`**

  A parenthesized type equivalent to the inner type.

- **`Path`**

  A path like `std::slice::Iter`, optionally qualified with a
  self-type as in `<Vec<T> as SomeTrait>::Associated`.

- **`Ptr`**

  A raw pointer type: `*const T` or `*mut T`.

- **`Reference`**

  A reference type: `&'a T` or `&'a mut T`.

- **`Slice`**

  A dynamically sized slice type: `[T]`.

- **`TraitObject`**

  A trait object type `dyn Bound1 + Bound2 + Bound3` where `Bound` is a
  trait or a lifetime.

- **`Tuple`**

  A tuple type: `(A, B, C, String)`.

- **`Verbatim`**

  Tokens in type position not interpreted by Syn.

#### Implementations

- <span id="cratetytype-without-plus"></span>`fn without_plus(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

#### Trait Implementations

##### `impl Clone for crate::Type`

- <span id="cratetype-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Type`

- <span id="cratetype-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Type`

##### `impl Hash for crate::Type`

- <span id="cratetype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::Type`

- <span id="cratetytype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Type`

- <span id="cratetype-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Type`

##### `impl<T> Spanned for Type`

- <span id="type-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Type`

- <span id="type-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

## Functions

### `parse`

```rust
fn parse<T: parse::Parse>(tokens: proc_macro::TokenStream) -> Result<T>
```

Parse tokens of source code into the chosen syntax tree node.

This is preferred over parsing a string because tokens are able to preserve
information about where in the user's code they were originally written (the
"span" of the token), possibly allowing the compiler to produce better error
messages.

This function parses a `proc_macro::TokenStream` which is the type used for
interop with the compiler in a procedural macro. To parse a
`proc_macro2::TokenStream`, use `syn::parse2` instead.

This function enforces that the input is fully parsed. If there are any
unparsed tokens at the end of the stream, an error is returned.

### `parse2`

```rust
fn parse2<T: parse::Parse>(tokens: proc_macro2::TokenStream) -> Result<T>
```

Parse a proc-macro2 token stream into the chosen syntax tree node.

This function parses a `proc_macro2::TokenStream` which is commonly useful
when the input comes from a node of the Syn syntax tree, for example the
body tokens of a [`Macro`](#macro) node. When in a procedural macro parsing the
`proc_macro::TokenStream` provided by the compiler, use `syn::parse`
instead.

This function enforces that the input is fully parsed. If there are any
unparsed tokens at the end of the stream, an error is returned.

### `parse_str`

```rust
fn parse_str<T: parse::Parse>(s: &str) -> Result<T>
```

Parse a string of Rust code into the chosen syntax tree node.

This function enforces that the input is fully parsed. If there are any
unparsed tokens at the end of the stream, an error is returned.

# Hygiene

Every span in the resulting syntax tree will be set to resolve at the macro
call site.

# Examples

```rust
use syn::{Expr, Result};

fn run() -> Result<()> {
    let code = "assert_eq!(u8::max_value(), 255)";
    let expr = syn::parse_str::<Expr>(code)?;
    println!("{:#?}", expr);
    Ok(())
}

run().unwrap();
```

### `parse_file`

```rust
fn parse_file(content: &str) -> Result<File>
```

Parse the content of a file of Rust code.

This is different from `syn::parse_str::<File>(content)` in two ways:

- It discards a leading byte order mark `\u{FEFF}` if the file has one.
- It preserves the shebang line of the file, such as `#!/usr/bin/env rustx`.

If present, either of these would be an error using `from_str`.

# Examples

```no_run
use std::error::Error;
use std::fs;
use std::io::Read;

fn run() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("path/to/code.rs")?;
    let ast = syn::parse_file(&content)?;
    if let Some(shebang) = ast.shebang {
        println!("{}", shebang);
    }
    println!("{} items", ast.items.len());

    Ok(())
}

run().unwrap();
```

## Type Aliases

## Macros

### `parenthesized!`

Parse a set of parentheses and expose their content to subsequent parsers.

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

fn main() {
    let input = quote! {
        struct S(A, B);
    };
    syn::parse2::<TupleStruct>(input).unwrap();
}
```

### `braced!`

Parse a set of curly braces and expose their content to subsequent parsers.

# Example

```rust
use quote::quote;

use syn::{braced, token, Ident, Result, Token, Type};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

// Parse a simplified struct syntax like:
//
//     struct S {
//         a: A,
//         b: B,
//     }
struct Struct {
    struct_token: Token![struct],
    ident: Ident,
    brace_token: token::Brace,
    fields: Punctuated<Field, Token![,]>,
}

struct Field {
    name: Ident,
    colon_token: Token![:],
    ty: Type,
}

impl Parse for Struct {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Struct {
            struct_token: input.parse()?,
            ident: input.parse()?,
            brace_token: braced!(content in input),
            fields: content.parse_terminated(Field::parse, Token![,])?,
        })
    }
}

impl Parse for Field {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Field {
            name: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?,
        })
    }
}

fn main() {
    let input = quote! {
        struct S {
            a: A,
            b: B,
        }
    };
    syn::parse2::<Struct>(input).unwrap();
}
```

### `bracketed!`

Parse a set of square brackets and expose their content to subsequent
parsers.

# Example

```rust
use quote::quote;

use proc_macro2::TokenStream;
use syn::{bracketed, token, Result, Token};
use syn::parse::{Parse, ParseStream};

// Parse an outer attribute like:
//
//     #[repr(C, packed)]
struct OuterAttribute {
    pound_token: Token![#],
    bracket_token: token::Bracket,
    content: TokenStream,
}

impl Parse for OuterAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(OuterAttribute {
            pound_token: input.parse()?,
            bracket_token: bracketed!(content in input),
            content: content.parse()?,
        })
    }
}

fn main() {
    let input = quote! {
        #[repr(C, packed)]
    };
    syn::parse2::<OuterAttribute>(input).unwrap();
}
```

### `Token!`

A type-macro that expands to the name of the Rust type representation of a
given token.

As a type, `Token!` is commonly used in the type of struct fields, the type
of a `let` statement, or in turbofish for a `parse` function.

```rust
use syn::{Ident, Token};
use syn::parse::{Parse, ParseStream, Result};

// `struct Foo;`
pub struct UnitStruct {
    struct_token: Token![struct],
    ident: Ident,
    semi_token: Token![;],
}

impl Parse for UnitStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let struct_token: Token![struct] = input.parse()?;
        let ident: Ident = input.parse()?;
        let semi_token = input.parse::<Token![;]>()?;
        Ok(UnitStruct { struct_token, ident, semi_token })
    }
}
```

As an expression, `Token!` is used for peeking tokens or instantiating
tokens from a span.

```rust
use syn::{Ident, Token};
use syn::parse::{Parse, ParseStream, Result};

struct UnitStruct {
    struct_token: Token![struct],
    ident: Ident,
    semi_token: Token![;],
}

impl Parse for UnitStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        unimplemented!()
    }
}

fn make_unit_struct(name: Ident) -> UnitStruct {
    let span = name.span();
    UnitStruct {
        struct_token: Token![struct](span),
        ident: name,
        semi_token: Token![;](span),
    }
}

fn parse(input: ParseStream) -> Result<()> {
if input.peek(Token![struct]) {
    let unit_struct: UnitStruct = input.parse()?;
    /* ... */
}
Ok(())
}
```

See the [token module] documentation for details and examples.


### `custom_keyword!`

Define a type that supports parsing and printing a given identifier as if it
were a keyword.

# Usage

As a convention, it is recommended that this macro be invoked within a
module called `kw` or `keyword` and that the resulting parser be invoked
with a `kw::` or `keyword::` prefix.

```rust
mod kw {
    syn::custom_keyword!(whatever);
}
```

The generated syntax tree node supports the following operations just like
any built-in keyword token.

- [Peeking] — `input.peek(kw::whatever)`

- [Parsing] — `input.parse::<kw::whatever>()?`

- [Printing] — `quote!( ... #whatever_token ... )`

- Construction from a `Span` — `let whatever_token = kw::whatever(sp)`

- Field access to its span — `let sp = whatever_token.span`




# Example

This example parses input that looks like `bool = true` or `str = "value"`.
The key must be either the identifier `bool` or the identifier `str`. If
`bool`, the value may be either `true` or `false`. If `str`, the value may
be any string literal.

The symbols `bool` and `str` are not reserved keywords in Rust so these are
not considered keywords in the `syn::token` module. Like any other
identifier that is not a keyword, these can be declared as custom keywords
by crates that need to use them as such.

```rust
use syn::{LitBool, LitStr, Result, Token};
use syn::parse::{Parse, ParseStream};

mod kw {
    syn::custom_keyword!(bool);
    syn::custom_keyword!(str);
}

enum Argument {
    Bool {
        bool_token: kw::bool,
        eq_token: Token![=],
        value: LitBool,
    },
    Str {
        str_token: kw::str,
        eq_token: Token![=],
        value: LitStr,
    },
}

impl Parse for Argument {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::bool) {
            Ok(Argument::Bool {
                bool_token: input.parse::<kw::bool>()?,
                eq_token: input.parse()?,
                value: input.parse()?,
            })
        } else if lookahead.peek(kw::str) {
            Ok(Argument::Str {
                str_token: input.parse::<kw::str>()?,
                eq_token: input.parse()?,
                value: input.parse()?,
            })
        } else {
            Err(lookahead.error())
        }
    }
}
```

### `custom_punctuation!`

Define a type that supports parsing and printing a multi-character symbol
as if it were a punctuation token.

# Usage

```rust
syn::custom_punctuation!(LeftRightArrow, <=>);
```

The generated syntax tree node supports the following operations just like
any built-in punctuation token.

- [Peeking] — `input.peek(LeftRightArrow)`

- [Parsing] — `input.parse::<LeftRightArrow>()?`

- [Printing] — `quote!( ... #lrarrow ... )`

- Construction from a `Span` — `let lrarrow = LeftRightArrow(sp)`

- Construction from multiple `Span` — `let lrarrow = LeftRightArrow([sp, sp, sp])`

- Field access to its spans — `let spans = lrarrow.spans`




# Example

```rust
use proc_macro2::{TokenStream, TokenTree};
use std::iter;
use syn::parse::{Parse, ParseStream, Peek, Result};
use syn::punctuated::Punctuated;
use syn::Expr;

syn::custom_punctuation!(PathSeparator, </>);

// expr </> expr </> expr ...
struct PathSegments {
    segments: Punctuated<Expr, PathSeparator>,
}

impl Parse for PathSegments {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut segments = Punctuated::new();

        let first = parse_until(input, PathSeparator)?;
        segments.push_value(syn::parse2(first)?);

        while input.peek(PathSeparator) {
            segments.push_punct(input.parse()?);

            let next = parse_until(input, PathSeparator)?;
            segments.push_value(syn::parse2(next)?);
        }

        Ok(PathSegments { segments })
    }
}

fn parse_until<E: Peek>(input: ParseStream, end: E) -> Result<TokenStream> {
    let mut tokens = TokenStream::new();
    while !input.is_empty() && !input.peek(end) {
        let next: TokenTree = input.parse()?;
        tokens.extend(iter::once(next));
    }
    Ok(tokens)
}

fn main() {
    let input = r#" a::b </> c::d::e "#;
    let _: PathSegments = syn::parse_str(input).unwrap();
}
```

### `parse_macro_input!`

Parse the input TokenStream of a macro, triggering a compile error if the
tokens fail to parse.

Refer to the [`parse` module] documentation for more details about parsing
in Syn.

<br>

# Intended usage

This macro must be called from a function that returns
`proc_macro::TokenStream`. Usually this will be your proc macro entry point,
the function that has the #\[proc_macro\] / #\[proc_macro_derive\] /
#\[proc_macro_attribute\] attribute.

```rust
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Result};
use syn::parse::{Parse, ParseStream};

struct MyMacroInput {
    /* ... */
}

impl Parse for MyMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        /* ... */
      Ok(MyMacroInput {})
    }
}

const IGNORE: &str = stringify! {
#[proc_macro]
};
pub fn my_macro(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as MyMacroInput);

    /* ... */
  TokenStream::new()
}
```

<br>

# Usage with Parser

This macro can also be used with the [`Parser` trait] for types that have
multiple ways that they can be parsed.

```rust
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Result};
use syn::parse::ParseStream;

struct MyMacroInput {}

impl MyMacroInput {
    fn parse_alternate(input: ParseStream) -> Result<Self> {
        /* ... */
      Ok(MyMacroInput {})
    }
}

const IGNORE: &str = stringify! {
#[proc_macro]
};
pub fn my_macro(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens with MyMacroInput::parse_alternate);

    /* ... */
  TokenStream::new()
}
```

<br>

# Expansion

`parse_macro_input!($variable as $Type)` expands to something like:

```no_run
extern crate proc_macro;

macro_rules! doc_test {
    ($variable:ident as $Type:ty) => {
match syn::parse::<$Type>($variable) {
    Ok(syntax_tree) => syntax_tree,
    Err(err) => return proc_macro::TokenStream::from(err.to_compile_error()),
}
    };
}

fn test(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let _ = doc_test!(input as syn::Ident);
    proc_macro::TokenStream::new()
}
```

### `parse_quote!`

Quasi-quotation macro that accepts input like the `quote!` macro but uses
type inference to figure out a return type for those tokens.

The return type can be any syntax tree node that implements the [`Parse`](parse/index.md)
trait.

```rust
use quote::quote;
use syn::{parse_quote, Stmt};

fn main() {
    let name = quote!(v);
    let ty = quote!(u8);

    let stmt: Stmt = parse_quote! {
        let #name: #ty = Default::default();
    };

    println!("{:#?}", stmt);
}
```

*This macro is available only if Syn is built with both the `"parsing"` and
`"printing"` features.*

# Example

The following helper function adds a bound `T: HeapSize` to every type
parameter `T` in the input generics.

```rust
use syn::{parse_quote, Generics, GenericParam};

// Add a bound `T: HeapSize` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param.bounds.push(parse_quote!(HeapSize));
        }
    }
    generics
}
```

# Special cases

This macro can parse the following additional types as a special case even
though they do not implement the `Parse` trait.

- [`Attribute`](#attribute) — parses one attribute, allowing either outer like `#[...]`
  or inner like `#![...]`
- `Vec<Attribute>` — parses multiple attributes, including mixed kinds in
  any order
- [`Punctuated<T, P>`](punctuated/index.md) — parses zero or more `T` separated by punctuation
  `P` with optional trailing punctuation
- `Vec<Arm>` — parses arms separated by optional commas according to the
  same grammar as the inside of a `match` expression
- `Vec<Stmt>` — parses the same as `Block::parse_within`
- [`Pat`](#pat), `Box<Pat>` — parses the same as
  `Pat::parse_multi_with_leading_vert`
- [`Field`](#field) — parses a named or unnamed struct field





# Panics

Panics if the tokens fail to parse as the expected syntax tree type. The
caller is responsible for ensuring that the input tokens are syntactically
valid.

### `parse_quote_spanned!`

This macro is [`parse_quote!`](#parse-quote) + `quote_spanned!`.

Please refer to each of their documentation.

# Example

```rust
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_quote_spanned, ReturnType, Signature};

// Changes `fn()` to `fn() -> Pin<Box<dyn Future<Output = ()>>>`,
// and `fn() -> T` to `fn() -> Pin<Box<dyn Future<Output = T>>>`,
// without introducing any call_site() spans.
fn make_ret_pinned_future(sig: &mut Signature) {
    let ret = match &sig.output {
        ReturnType::Default => quote_spanned!(sig.paren_token.span=> ()),
        ReturnType::Type(_, ret) => quote!(#ret),
    };
    sig.output = parse_quote_spanned! {ret.span()=>
        -> ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = #ret>>>
    };
}
```

