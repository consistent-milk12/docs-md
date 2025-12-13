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
  - [`custom_keyword`](#custom-keyword)
  - [`custom_punctuation`](#custom-punctuation)
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
  - [`parse_macro_input`](#parse-macro-input)
  - [`parse_quote`](#parse-quote)
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
  - [`visit`](#visit)
- [Structs](#structs)
  - [`Attribute`](#attribute)
  - [`MetaList`](#metalist)
  - [`MetaNameValue`](#metanamevalue)
  - [`Field`](#field)
  - [`FieldsNamed`](#fieldsnamed)
  - [`FieldsUnnamed`](#fieldsunnamed)
  - [`Variant`](#variant)
  - [`DataEnum`](#dataenum)
  - [`DataStruct`](#datastruct)
  - [`DataUnion`](#dataunion)
  - [`DeriveInput`](#deriveinput)
  - [`Error`](#error)
  - [`Arm`](#arm)
  - [`Label`](#label)
  - [`ExprBinary`](#exprbinary)
  - [`ExprCall`](#exprcall)
  - [`ExprCast`](#exprcast)
  - [`ExprField`](#exprfield)
  - [`ExprIndex`](#exprindex)
  - [`ExprLit`](#exprlit)
  - [`ExprMacro`](#exprmacro)
  - [`ExprMethodCall`](#exprmethodcall)
  - [`ExprParen`](#exprparen)
  - [`ExprPath`](#exprpath)
  - [`ExprReference`](#exprreference)
  - [`ExprStruct`](#exprstruct)
  - [`ExprUnary`](#exprunary)
  - [`FieldValue`](#fieldvalue)
  - [`Index`](#index)
  - [`ExprArray`](#exprarray)
  - [`ExprAssign`](#exprassign)
  - [`ExprAsync`](#exprasync)
  - [`ExprAwait`](#exprawait)
  - [`ExprBlock`](#exprblock)
  - [`ExprBreak`](#exprbreak)
  - [`ExprClosure`](#exprclosure)
  - [`ExprConst`](#exprconst)
  - [`ExprContinue`](#exprcontinue)
  - [`ExprForLoop`](#exprforloop)
  - [`ExprGroup`](#exprgroup)
  - [`ExprIf`](#exprif)
  - [`ExprInfer`](#exprinfer)
  - [`ExprLet`](#exprlet)
  - [`ExprLoop`](#exprloop)
  - [`ExprMatch`](#exprmatch)
  - [`ExprRange`](#exprrange)
  - [`ExprRawAddr`](#exprrawaddr)
  - [`ExprRepeat`](#exprrepeat)
  - [`ExprReturn`](#exprreturn)
  - [`ExprTry`](#exprtry)
  - [`ExprTryBlock`](#exprtryblock)
  - [`ExprTuple`](#exprtuple)
  - [`ExprUnsafe`](#exprunsafe)
  - [`ExprWhile`](#exprwhile)
  - [`ExprYield`](#expryield)
  - [`File`](#file)
  - [`BoundLifetimes`](#boundlifetimes)
  - [`ConstParam`](#constparam)
  - [`Generics`](#generics)
  - [`LifetimeParam`](#lifetimeparam)
  - [`PredicateLifetime`](#predicatelifetime)
  - [`PredicateType`](#predicatetype)
  - [`TraitBound`](#traitbound)
  - [`TypeParam`](#typeparam)
  - [`WhereClause`](#whereclause)
  - [`PreciseCapture`](#precisecapture)
  - [`ImplGenerics`](#implgenerics)
  - [`Turbofish`](#turbofish)
  - [`TypeGenerics`](#typegenerics)
  - [`ForeignItemFn`](#foreignitemfn)
  - [`ForeignItemMacro`](#foreignitemmacro)
  - [`ForeignItemStatic`](#foreignitemstatic)
  - [`ForeignItemType`](#foreignitemtype)
  - [`ImplItemConst`](#implitemconst)
  - [`ImplItemFn`](#implitemfn)
  - [`ImplItemMacro`](#implitemmacro)
  - [`ImplItemType`](#implitemtype)
  - [`ItemConst`](#itemconst)
  - [`ItemEnum`](#itemenum)
  - [`ItemExternCrate`](#itemexterncrate)
  - [`ItemFn`](#itemfn)
  - [`ItemForeignMod`](#itemforeignmod)
  - [`ItemImpl`](#itemimpl)
  - [`ItemMacro`](#itemmacro)
  - [`ItemMod`](#itemmod)
  - [`ItemStatic`](#itemstatic)
  - [`ItemStruct`](#itemstruct)
  - [`ItemTrait`](#itemtrait)
  - [`ItemTraitAlias`](#itemtraitalias)
  - [`ItemType`](#itemtype)
  - [`ItemUnion`](#itemunion)
  - [`ItemUse`](#itemuse)
  - [`Receiver`](#receiver)
  - [`Signature`](#signature)
  - [`TraitItemConst`](#traititemconst)
  - [`TraitItemFn`](#traititemfn)
  - [`TraitItemMacro`](#traititemmacro)
  - [`TraitItemType`](#traititemtype)
  - [`UseGlob`](#useglob)
  - [`UseGroup`](#usegroup)
  - [`UseName`](#usename)
  - [`UsePath`](#usepath)
  - [`UseRename`](#userename)
  - [`Variadic`](#variadic)
  - [`Lifetime`](#lifetime)
  - [`LitBool`](#litbool)
  - [`LitByte`](#litbyte)
  - [`LitByteStr`](#litbytestr)
  - [`LitCStr`](#litcstr)
  - [`LitChar`](#litchar)
  - [`LitFloat`](#litfloat)
  - [`LitInt`](#litint)
  - [`LitStr`](#litstr)
  - [`Macro`](#macro)
  - [`FieldPat`](#fieldpat)
  - [`PatConst`](#patconst)
  - [`PatIdent`](#patident)
  - [`PatLit`](#patlit)
  - [`PatMacro`](#patmacro)
  - [`PatOr`](#pator)
  - [`PatParen`](#patparen)
  - [`PatPath`](#patpath)
  - [`PatRange`](#patrange)
  - [`PatReference`](#patreference)
  - [`PatRest`](#patrest)
  - [`PatSlice`](#patslice)
  - [`PatStruct`](#patstruct)
  - [`PatTuple`](#pattuple)
  - [`PatTupleStruct`](#pattuplestruct)
  - [`PatType`](#pattype)
  - [`PatWild`](#patwild)
  - [`AngleBracketedGenericArguments`](#anglebracketedgenericarguments)
  - [`AssocConst`](#assocconst)
  - [`AssocType`](#assoctype)
  - [`Constraint`](#constraint)
  - [`ParenthesizedGenericArguments`](#parenthesizedgenericarguments)
  - [`Path`](#path)
  - [`PathSegment`](#pathsegment)
  - [`QSelf`](#qself)
  - [`VisRestricted`](#visrestricted)
  - [`Block`](#block)
  - [`Local`](#local)
  - [`LocalInit`](#localinit)
  - [`StmtMacro`](#stmtmacro)
  - [`Abi`](#abi)
  - [`BareFnArg`](#barefnarg)
  - [`BareVariadic`](#barevariadic)
  - [`TypeArray`](#typearray)
  - [`TypeBareFn`](#typebarefn)
  - [`TypeGroup`](#typegroup)
  - [`TypeImplTrait`](#typeimpltrait)
  - [`TypeInfer`](#typeinfer)
  - [`TypeMacro`](#typemacro)
  - [`TypeNever`](#typenever)
  - [`TypeParen`](#typeparen)
  - [`TypePath`](#typepath)
  - [`TypePtr`](#typeptr)
  - [`TypeReference`](#typereference)
  - [`TypeSlice`](#typeslice)
  - [`TypeTraitObject`](#typetraitobject)
  - [`TypeTuple`](#typetuple)
- [Enums](#enums)
  - [`AttrStyle`](#attrstyle)
  - [`Meta`](#meta)
  - [`Fields`](#fields)
  - [`Data`](#data)
  - [`PointerMutability`](#pointermutability)
  - [`RangeLimits`](#rangelimits)
  - [`Expr`](#expr)
  - [`Member`](#member)
  - [`GenericParam`](#genericparam)
  - [`TraitBoundModifier`](#traitboundmodifier)
  - [`TypeParamBound`](#typeparambound)
  - [`WherePredicate`](#wherepredicate)
  - [`CapturedParam`](#capturedparam)
  - [`FnArg`](#fnarg)
  - [`ForeignItem`](#foreignitem)
  - [`ImplItem`](#implitem)
  - [`ImplRestriction`](#implrestriction)
  - [`Item`](#item)
  - [`StaticMutability`](#staticmutability)
  - [`TraitItem`](#traititem)
  - [`UseTree`](#usetree)
  - [`Lit`](#lit)
  - [`MacroDelimiter`](#macrodelimiter)
  - [`BinOp`](#binop)
  - [`UnOp`](#unop)
  - [`Pat`](#pat)
  - [`GenericArgument`](#genericargument)
  - [`PathArguments`](#patharguments)
  - [`FieldMutability`](#fieldmutability)
  - [`Visibility`](#visibility)
  - [`Stmt`](#stmt)
  - [`ReturnType`](#returntype)
  - [`Type`](#type)
- [Functions](#functions)
  - [`parse`](#parse)
  - [`parse2`](#parse2)
  - [`parse_str`](#parse-str)
  - [`parse_file`](#parse-file)
- [Type Aliases](#type-aliases)
  - [`Result`](#result)
- [Macros](#macros)
  - [`parenthesized!`](#parenthesized)
  - [`braced!`](#braced)
  - [`bracketed!`](#bracketed)
  - [`Token!`](#token)
  - [`custom_keyword!`](#custom-keyword)
  - [`custom_punctuation!`](#custom-punctuation)
  - [`parse_macro_input!`](#parse-macro-input)
  - [`parse_quote!`](#parse-quote)
  - [`parse_quote_spanned!`](#parse-quote-spanned)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`macros`](#macros) | mod |  |
| [`group`](#group) | mod |  |
| [`token`](#token) | mod | Tokens representing Rust punctuation, keywords, and delimiters. |
| [`attr`](#attr) | mod |  |
| [`bigint`](#bigint) | mod |  |
| [`buffer`](#buffer) | mod | A stably addressed token buffer supporting efficient traversal based on a cheaply copyable cursor. |
| [`classify`](#classify) | mod |  |
| [`custom_keyword`](#custom-keyword) | mod |  |
| [`custom_punctuation`](#custom-punctuation) | mod |  |
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
| [`parse_macro_input`](#parse-macro-input) | mod |  |
| [`parse_quote`](#parse-quote) | mod |  |
| [`pat`](#pat) | mod |  |
| [`path`](#path) | mod |  |
| [`precedence`](#precedence) | mod |  |
| [`print`](#print) | mod |  |
| [`punctuated`](#punctuated) | mod | A punctuated sequence of syntax tree nodes separated by punctuation. |
| [`restriction`](#restriction) | mod |  |
| [`sealed`](#sealed) | mod |  |
| [`span`](#span) | mod |  |
| [`spanned`](#spanned) | mod | A trait that can provide the `Span` of the complete contents of a syntax tree node. |
| [`stmt`](#stmt) | mod |  |
| [`thread`](#thread) | mod |  |
| [`tt`](#tt) | mod |  |
| [`ty`](#ty) | mod |  |
| [`verbatim`](#verbatim) | mod |  |
| [`whitespace`](#whitespace) | mod |  |
| [`gen`](#gen) | mod |  |
| [`visit`](#visit) | mod |  |
| [`Attribute`](#attribute) | struct |  |
| [`MetaList`](#metalist) | struct |  |
| [`MetaNameValue`](#metanamevalue) | struct |  |
| [`Field`](#field) | struct |  |
| [`FieldsNamed`](#fieldsnamed) | struct |  |
| [`FieldsUnnamed`](#fieldsunnamed) | struct |  |
| [`Variant`](#variant) | struct |  |
| [`DataEnum`](#dataenum) | struct |  |
| [`DataStruct`](#datastruct) | struct |  |
| [`DataUnion`](#dataunion) | struct |  |
| [`DeriveInput`](#deriveinput) | struct |  |
| [`Error`](#error) | struct |  |
| [`Arm`](#arm) | struct |  |
| [`Label`](#label) | struct |  |
| [`ExprBinary`](#exprbinary) | struct |  |
| [`ExprCall`](#exprcall) | struct |  |
| [`ExprCast`](#exprcast) | struct |  |
| [`ExprField`](#exprfield) | struct |  |
| [`ExprIndex`](#exprindex) | struct |  |
| [`ExprLit`](#exprlit) | struct |  |
| [`ExprMacro`](#exprmacro) | struct |  |
| [`ExprMethodCall`](#exprmethodcall) | struct |  |
| [`ExprParen`](#exprparen) | struct |  |
| [`ExprPath`](#exprpath) | struct |  |
| [`ExprReference`](#exprreference) | struct |  |
| [`ExprStruct`](#exprstruct) | struct |  |
| [`ExprUnary`](#exprunary) | struct |  |
| [`FieldValue`](#fieldvalue) | struct |  |
| [`Index`](#index) | struct |  |
| [`ExprArray`](#exprarray) | struct |  |
| [`ExprAssign`](#exprassign) | struct |  |
| [`ExprAsync`](#exprasync) | struct |  |
| [`ExprAwait`](#exprawait) | struct |  |
| [`ExprBlock`](#exprblock) | struct |  |
| [`ExprBreak`](#exprbreak) | struct |  |
| [`ExprClosure`](#exprclosure) | struct |  |
| [`ExprConst`](#exprconst) | struct |  |
| [`ExprContinue`](#exprcontinue) | struct |  |
| [`ExprForLoop`](#exprforloop) | struct |  |
| [`ExprGroup`](#exprgroup) | struct |  |
| [`ExprIf`](#exprif) | struct |  |
| [`ExprInfer`](#exprinfer) | struct |  |
| [`ExprLet`](#exprlet) | struct |  |
| [`ExprLoop`](#exprloop) | struct |  |
| [`ExprMatch`](#exprmatch) | struct |  |
| [`ExprRange`](#exprrange) | struct |  |
| [`ExprRawAddr`](#exprrawaddr) | struct |  |
| [`ExprRepeat`](#exprrepeat) | struct |  |
| [`ExprReturn`](#exprreturn) | struct |  |
| [`ExprTry`](#exprtry) | struct |  |
| [`ExprTryBlock`](#exprtryblock) | struct |  |
| [`ExprTuple`](#exprtuple) | struct |  |
| [`ExprUnsafe`](#exprunsafe) | struct |  |
| [`ExprWhile`](#exprwhile) | struct |  |
| [`ExprYield`](#expryield) | struct |  |
| [`File`](#file) | struct |  |
| [`BoundLifetimes`](#boundlifetimes) | struct |  |
| [`ConstParam`](#constparam) | struct |  |
| [`Generics`](#generics) | struct |  |
| [`LifetimeParam`](#lifetimeparam) | struct |  |
| [`PredicateLifetime`](#predicatelifetime) | struct |  |
| [`PredicateType`](#predicatetype) | struct |  |
| [`TraitBound`](#traitbound) | struct |  |
| [`TypeParam`](#typeparam) | struct |  |
| [`WhereClause`](#whereclause) | struct |  |
| [`PreciseCapture`](#precisecapture) | struct |  |
| [`ImplGenerics`](#implgenerics) | struct |  |
| [`Turbofish`](#turbofish) | struct |  |
| [`TypeGenerics`](#typegenerics) | struct |  |
| [`ForeignItemFn`](#foreignitemfn) | struct |  |
| [`ForeignItemMacro`](#foreignitemmacro) | struct |  |
| [`ForeignItemStatic`](#foreignitemstatic) | struct |  |
| [`ForeignItemType`](#foreignitemtype) | struct |  |
| [`ImplItemConst`](#implitemconst) | struct |  |
| [`ImplItemFn`](#implitemfn) | struct |  |
| [`ImplItemMacro`](#implitemmacro) | struct |  |
| [`ImplItemType`](#implitemtype) | struct |  |
| [`ItemConst`](#itemconst) | struct |  |
| [`ItemEnum`](#itemenum) | struct |  |
| [`ItemExternCrate`](#itemexterncrate) | struct |  |
| [`ItemFn`](#itemfn) | struct |  |
| [`ItemForeignMod`](#itemforeignmod) | struct |  |
| [`ItemImpl`](#itemimpl) | struct |  |
| [`ItemMacro`](#itemmacro) | struct |  |
| [`ItemMod`](#itemmod) | struct |  |
| [`ItemStatic`](#itemstatic) | struct |  |
| [`ItemStruct`](#itemstruct) | struct |  |
| [`ItemTrait`](#itemtrait) | struct |  |
| [`ItemTraitAlias`](#itemtraitalias) | struct |  |
| [`ItemType`](#itemtype) | struct |  |
| [`ItemUnion`](#itemunion) | struct |  |
| [`ItemUse`](#itemuse) | struct |  |
| [`Receiver`](#receiver) | struct |  |
| [`Signature`](#signature) | struct |  |
| [`TraitItemConst`](#traititemconst) | struct |  |
| [`TraitItemFn`](#traititemfn) | struct |  |
| [`TraitItemMacro`](#traititemmacro) | struct |  |
| [`TraitItemType`](#traititemtype) | struct |  |
| [`UseGlob`](#useglob) | struct |  |
| [`UseGroup`](#usegroup) | struct |  |
| [`UseName`](#usename) | struct |  |
| [`UsePath`](#usepath) | struct |  |
| [`UseRename`](#userename) | struct |  |
| [`Variadic`](#variadic) | struct |  |
| [`Lifetime`](#lifetime) | struct |  |
| [`LitBool`](#litbool) | struct |  |
| [`LitByte`](#litbyte) | struct |  |
| [`LitByteStr`](#litbytestr) | struct |  |
| [`LitCStr`](#litcstr) | struct |  |
| [`LitChar`](#litchar) | struct |  |
| [`LitFloat`](#litfloat) | struct |  |
| [`LitInt`](#litint) | struct |  |
| [`LitStr`](#litstr) | struct |  |
| [`Macro`](#macro) | struct |  |
| [`FieldPat`](#fieldpat) | struct |  |
| [`PatConst`](#patconst) | struct |  |
| [`PatIdent`](#patident) | struct |  |
| [`PatLit`](#patlit) | struct |  |
| [`PatMacro`](#patmacro) | struct |  |
| [`PatOr`](#pator) | struct |  |
| [`PatParen`](#patparen) | struct |  |
| [`PatPath`](#patpath) | struct |  |
| [`PatRange`](#patrange) | struct |  |
| [`PatReference`](#patreference) | struct |  |
| [`PatRest`](#patrest) | struct |  |
| [`PatSlice`](#patslice) | struct |  |
| [`PatStruct`](#patstruct) | struct |  |
| [`PatTuple`](#pattuple) | struct |  |
| [`PatTupleStruct`](#pattuplestruct) | struct |  |
| [`PatType`](#pattype) | struct |  |
| [`PatWild`](#patwild) | struct |  |
| [`AngleBracketedGenericArguments`](#anglebracketedgenericarguments) | struct |  |
| [`AssocConst`](#assocconst) | struct |  |
| [`AssocType`](#assoctype) | struct |  |
| [`Constraint`](#constraint) | struct |  |
| [`ParenthesizedGenericArguments`](#parenthesizedgenericarguments) | struct |  |
| [`Path`](#path) | struct |  |
| [`PathSegment`](#pathsegment) | struct |  |
| [`QSelf`](#qself) | struct |  |
| [`VisRestricted`](#visrestricted) | struct |  |
| [`Block`](#block) | struct |  |
| [`Local`](#local) | struct |  |
| [`LocalInit`](#localinit) | struct |  |
| [`StmtMacro`](#stmtmacro) | struct |  |
| [`Abi`](#abi) | struct |  |
| [`BareFnArg`](#barefnarg) | struct |  |
| [`BareVariadic`](#barevariadic) | struct |  |
| [`TypeArray`](#typearray) | struct |  |
| [`TypeBareFn`](#typebarefn) | struct |  |
| [`TypeGroup`](#typegroup) | struct |  |
| [`TypeImplTrait`](#typeimpltrait) | struct |  |
| [`TypeInfer`](#typeinfer) | struct |  |
| [`TypeMacro`](#typemacro) | struct |  |
| [`TypeNever`](#typenever) | struct |  |
| [`TypeParen`](#typeparen) | struct |  |
| [`TypePath`](#typepath) | struct |  |
| [`TypePtr`](#typeptr) | struct |  |
| [`TypeReference`](#typereference) | struct |  |
| [`TypeSlice`](#typeslice) | struct |  |
| [`TypeTraitObject`](#typetraitobject) | struct |  |
| [`TypeTuple`](#typetuple) | struct |  |
| [`AttrStyle`](#attrstyle) | enum |  |
| [`Meta`](#meta) | enum |  |
| [`Fields`](#fields) | enum |  |
| [`Data`](#data) | enum |  |
| [`PointerMutability`](#pointermutability) | enum |  |
| [`RangeLimits`](#rangelimits) | enum |  |
| [`Expr`](#expr) | enum |  |
| [`Member`](#member) | enum |  |
| [`GenericParam`](#genericparam) | enum |  |
| [`TraitBoundModifier`](#traitboundmodifier) | enum |  |
| [`TypeParamBound`](#typeparambound) | enum |  |
| [`WherePredicate`](#wherepredicate) | enum |  |
| [`CapturedParam`](#capturedparam) | enum |  |
| [`FnArg`](#fnarg) | enum |  |
| [`ForeignItem`](#foreignitem) | enum |  |
| [`ImplItem`](#implitem) | enum |  |
| [`ImplRestriction`](#implrestriction) | enum |  |
| [`Item`](#item) | enum |  |
| [`StaticMutability`](#staticmutability) | enum |  |
| [`TraitItem`](#traititem) | enum |  |
| [`UseTree`](#usetree) | enum |  |
| [`Lit`](#lit) | enum |  |
| [`MacroDelimiter`](#macrodelimiter) | enum |  |
| [`BinOp`](#binop) | enum |  |
| [`UnOp`](#unop) | enum |  |
| [`Pat`](#pat) | enum |  |
| [`GenericArgument`](#genericargument) | enum |  |
| [`PathArguments`](#patharguments) | enum |  |
| [`FieldMutability`](#fieldmutability) | enum |  |
| [`Visibility`](#visibility) | enum |  |
| [`Stmt`](#stmt) | enum |  |
| [`ReturnType`](#returntype) | enum |  |
| [`Type`](#type) | enum |  |
| [`parse`](#parse) | fn | Parse tokens of source code into the chosen syntax tree node. |
| [`parse2`](#parse2) | fn | Parse a proc-macro2 token stream into the chosen syntax tree node. |
| [`parse_str`](#parse-str) | fn | Parse a string of Rust code into the chosen syntax tree node. |
| [`parse_file`](#parse-file) | fn | Parse the content of a file of Rust code. |
| [`Result`](#result) | type |  |
| [`parenthesized!`](#parenthesized) | macro | Parse a set of parentheses and expose their content to subsequent parsers. |
| [`braced!`](#braced) | macro | Parse a set of curly braces and expose their content to subsequent parsers. |
| [`bracketed!`](#bracketed) | macro | Parse a set of square brackets and expose their content to subsequent parsers. |
| [`Token!`](#token) | macro | A type-macro that expands to the name of the Rust type representation of a given token. |
| [`custom_keyword!`](#custom-keyword) | macro | Define a type that supports parsing and printing a given identifier as if it were a keyword. |
| [`custom_punctuation!`](#custom-punctuation) | macro | Define a type that supports parsing and printing a multi-character symbol as if it were a punctuation token. |
| [`parse_macro_input!`](#parse-macro-input) | macro | Parse the input TokenStream of a macro, triggering a compile error if the tokens fail to parse. |
| [`parse_quote!`](#parse-quote) | macro | Quasi-quotation macro that accepts input like the [`quote!`] macro but uses type inference to figure out a return type for those tokens. |
| [`parse_quote_spanned!`](#parse-quote-spanned) | macro | This macro is [`parse_quote!`] + [`quote_spanned!`][quote::quote_spanned]. |

## Modules

- [`macros`](macros/index.md)
- [`group`](group/index.md)
- [`token`](token/index.md) — Tokens representing Rust punctuation, keywords, and delimiters.
- [`attr`](attr/index.md)
- [`bigint`](bigint/index.md)
- [`buffer`](buffer/index.md) — A stably addressed token buffer supporting efficient traversal based on a
- [`classify`](classify/index.md)
- [`custom_keyword`](custom_keyword/index.md)
- [`custom_punctuation`](custom_punctuation/index.md)
- [`data`](data/index.md)
- [`derive`](derive/index.md)
- [`drops`](drops/index.md)
- [`error`](error/index.md)
- [`expr`](expr/index.md)
- [`ext`](ext/index.md) — Extension traits to provide parsing methods on foreign types.
- [`file`](file/index.md)
- [`fixup`](fixup/index.md)
- [`generics`](generics/index.md)
- [`ident`](ident/index.md)
- [`item`](item/index.md)
- [`lifetime`](lifetime/index.md)
- [`lit`](lit/index.md)
- [`lookahead`](lookahead/index.md)
- [`mac`](mac/index.md)
- [`meta`](meta/index.md) — Facility for interpreting structured content inside of an `Attribute`.
- [`op`](op/index.md)
- [`parse`](parse/index.md) — Parsing interface for parsing a token stream into a syntax tree node.
- [`parse_macro_input`](parse_macro_input/index.md)
- [`parse_quote`](parse_quote/index.md)
- [`pat`](pat/index.md)
- [`path`](path/index.md)
- [`precedence`](precedence/index.md)
- [`print`](print/index.md)
- [`punctuated`](punctuated/index.md) — A punctuated sequence of syntax tree nodes separated by punctuation.
- [`restriction`](restriction/index.md)
- [`sealed`](sealed/index.md)
- [`span`](span/index.md)
- [`spanned`](spanned/index.md) — A trait that can provide the `Span` of the complete contents of a syntax
- [`stmt`](stmt/index.md)
- [`thread`](thread/index.md)
- [`tt`](tt/index.md)
- [`ty`](ty/index.md)
- [`verbatim`](verbatim/index.md)
- [`whitespace`](whitespace/index.md)
- [`gen`](gen/index.md)
- [`visit`](visit/index.md) — Syntax tree traversal to walk a shared borrow of a syntax tree.

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

*Defined in [`syn-2.0.111/src/attr.rs:19-179`](../../.source_1765521767/syn-2.0.111/src/attr.rs#L19-L179)*

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

- <span id="attribute-path"></span>`fn path(&self) -> &Path` — [`Path`](path/index.md#path)

  Returns the path that identifies the interpretation of this attribute.

  

  For example this would return the `test` in `#[test]`, the `derive` in

  `#[derive(Copy)]`, and the `path` in `#[path = "sys/windows.rs"]`.

- <span id="attribute-parse-args"></span>`fn parse_args<T: Parse>(&self) -> Result<T>` — [`Result`](error/index.md#result)

  Parse the arguments to the attribute as a syntax tree.

  

  This is similar to pulling out the `TokenStream` from `Meta::List` and

  doing `syn::parse2::<T>(meta_list.tokens)`, except that using

  `parse_args` the error message has a more useful span when `tokens` is

  empty.

  

  The surrounding delimiters are *not* included in the input to the

  parser.

  

  ```text

  #[my_attr(value < 5)]

            ^^^^^^^^^ what gets parsed

  ```

  

  # Example

  

  ```rust

  use syn::{parse_quote, Attribute, Expr};

  

  let attr: Attribute = parse_quote! {

      #[precondition(value < 5)]

  };

  

  if attr.path().is_ident("precondition") {

      let precondition: Expr = attr.parse_args()?;

      // ...

  }

  anyhow::Ok(())

  ```

- <span id="attribute-parse-args-with"></span>`fn parse_args_with<F: Parser>(&self, parser: F) -> Result<<F as >::Output>` — [`Result`](error/index.md#result), [`Parser`](parse/index.md#parser)

  Parse the arguments to the attribute using the given parser.

  

  # Example

  

  ```rust

  use syn::{parse_quote, Attribute};

  

  let attr: Attribute = parse_quote! {

      #[inception { #[brrrrrrraaaaawwwwrwrrrmrmrmmrmrmmmmm] }]

  };

  

  let bwom = attr.parse_args_with(Attribute::parse_outer)?;

  

  // Attribute does not have a Parse impl, so we couldn't directly do:

  // let bwom: Attribute = attr.parse_args()?;

  anyhow::Ok(())

  ```

- <span id="attribute-parse-nested-meta"></span>`fn parse_nested_meta(&self, logic: impl FnMut(ParseNestedMeta<'_>) -> Result<()>) -> Result<()>` — [`ParseNestedMeta`](meta/index.md#parsenestedmeta), [`Result`](error/index.md#result)

  Parse the arguments to the attribute, expecting it to follow the

  conventional structure used by most of Rust's built-in attributes.

  

  The [*Meta Item Attribute Syntax*][`syntax`](../regex_automata/util/syntax/index.md) section in the Rust reference

  explains the convention in more detail. Not all attributes follow this

  convention, so `parse_args()` is available if you

  need to parse arbitrarily goofy attribute syntax.

  

  # Example

  

  We'll parse a struct, and then parse some of Rust's `#[repr]` attribute

  syntax.

  

  ```rust

  use syn::{parenthesized, parse_quote, token, ItemStruct, LitInt};

  

  let input: ItemStruct = parse_quote! {

      #[repr(C, align(4))]

      pub struct MyStruct(u16, u32);

  };

  

  let mut repr_c = false;

  let mut repr_transparent = false;

  let mut repr_align = None::<usize>;

  let mut repr_packed = None::<usize>;

  for attr in &input.attrs {

      if attr.path().is_ident("repr") {

          attr.parse_nested_meta(|meta| {

              // #[repr(C)]

              if meta.path.is_ident("C") {

                  repr_c = true;

                  return Ok(());

              }

  

              // #[repr(transparent)]

              if meta.path.is_ident("transparent") {

                  repr_transparent = true;

                  return Ok(());

              }

  

              // #[repr(align(N))]

              if meta.path.is_ident("align") {

                  let content;

                  parenthesized!(content in meta.input);

                  let lit: LitInt = content.parse()?;

                  let n: usize = lit.base10_parse()?;

                  repr_align = Some(n);

                  return Ok(());

              }

  

              // #[repr(packed)] or #[repr(packed(N))], omitted N means 1

              if meta.path.is_ident("packed") {

                  if meta.input.peek(token::Paren) {

                      let content;

                      parenthesized!(content in meta.input);

                      let lit: LitInt = content.parse()?;

                      let n: usize = lit.base10_parse()?;

                      repr_packed = Some(n);

                  } else {

                      repr_packed = Some(1);

                  }

                  return Ok(());

              }

  

              Err(meta.error("unrecognized repr"))

          })?;

      }

  }

  anyhow::Ok(())

  ```

  

  # Alternatives

  

  In some cases, for attributes which have nested layers of structured

  content, the following less flexible approach might be more convenient:

  

  ```rust

  use syn::{parse_quote, ItemStruct};

  

  let input: ItemStruct = parse_quote! {

      #[repr(C, align(4))]

      pub struct MyStruct(u16, u32);

  };

  

  use syn::punctuated::Punctuated;

  use syn::{parenthesized, token, Error, LitInt, Meta, Token};

  

  let mut repr_c = false;

  let mut repr_transparent = false;

  let mut repr_align = None::<usize>;

  let mut repr_packed = None::<usize>;

  for attr in &input.attrs {

      if attr.path().is_ident("repr") {

          let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

          for meta in nested {

              match meta {

                  // #[repr(C)]

                  Meta::Path(path) if path.is_ident("C") => {

                      repr_c = true;

                  }

  

                  // #[repr(align(N))]

                  Meta::List(meta) if meta.path.is_ident("align") => {

                      let lit: LitInt = meta.parse_args()?;

                      let n: usize = lit.base10_parse()?;

                      repr_align = Some(n);

                  }

  

                  /* ... */

  

                  _ => {

                      return Err(Error::new_spanned(meta, "unrecognized repr"));

                  }

              }

          }

      }

  }

  Ok(())

  ```

- <span id="attribute-parse-outer"></span>`fn parse_outer(input: ParseStream<'_>) -> Result<Vec<Self>>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

  Parses zero or more outer attributes from the stream.

  

  # Example

  

  See

  [*Parsing from tokens to Attribute*](#parsing-from-tokens-to-attribute).

- <span id="attribute-parse-inner"></span>`fn parse_inner(input: ParseStream<'_>) -> Result<Vec<Self>>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

  Parses zero or more inner attributes from the stream.

  

  # Example

  

  See

  [*Parsing from tokens to Attribute*](#parsing-from-tokens-to-attribute).

#### Trait Implementations

##### `impl Any for Attribute`

- <span id="attribute-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Attribute`

- <span id="attribute-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Attribute`

- <span id="attribute-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Attribute`

- <span id="crateattribute-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Attribute`

- <span id="attribute-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Attribute`

- <span id="crateattribute-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Attribute`

##### `impl<T> From for Attribute`

- <span id="attribute-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Attribute`

- <span id="crateattribute-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Attribute`

- <span id="attribute-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::Attribute`

- <span id="crateattribute-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Attribute`

##### `impl Spanned for Attribute`

- <span id="attribute-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Attribute`

- <span id="attribute-toowned-type-owned"></span>`type Owned = T`

- <span id="attribute-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="attribute-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::attr::Attribute`

- <span id="crateattrattribute-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Attribute`

- <span id="attribute-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="attribute-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Attribute`

- <span id="attribute-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="attribute-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MetaList`

```rust
struct MetaList {
    pub path: crate::path::Path,
    pub delimiter: crate::mac::MacroDelimiter,
    pub tokens: proc_macro2::TokenStream,
}
```

*Defined in [`syn-2.0.111/src/attr.rs:484-492`](../../.source_1765521767/syn-2.0.111/src/attr.rs#L484-L492)*

A structured list within an attribute, like `derive(Copy, Clone)`.

#### Implementations

- <span id="metalist-parse-args"></span>`fn parse_args<T: Parse>(&self) -> Result<T>` — [`Result`](error/index.md#result)

  See `Attribute::parse_args`.

- <span id="metalist-parse-args-with"></span>`fn parse_args_with<F: Parser>(&self, parser: F) -> Result<<F as >::Output>` — [`Result`](error/index.md#result), [`Parser`](parse/index.md#parser)

  See `Attribute::parse_args_with`.

- <span id="metalist-parse-nested-meta"></span>`fn parse_nested_meta(&self, logic: impl FnMut(ParseNestedMeta<'_>) -> Result<()>) -> Result<()>` — [`ParseNestedMeta`](meta/index.md#parsenestedmeta), [`Result`](error/index.md#result)

  See `Attribute::parse_nested_meta`.

#### Trait Implementations

##### `impl Any for MetaList`

- <span id="metalist-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MetaList`

- <span id="metalist-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MetaList`

- <span id="metalist-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::MetaList`

- <span id="cratemetalist-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for MetaList`

- <span id="metalist-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::MetaList`

- <span id="cratemetalist-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::MetaList`

##### `impl<T> From for MetaList`

- <span id="metalist-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::MetaList`

- <span id="cratemetalist-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for MetaList`

- <span id="metalist-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::attr::MetaList`

- <span id="crateattrmetalist-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::MetaList`

- <span id="cratemetalist-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for MetaList`

##### `impl Spanned for MetaList`

- <span id="metalist-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for MetaList`

- <span id="metalist-toowned-type-owned"></span>`type Owned = T`

- <span id="metalist-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="metalist-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::attr::MetaList`

- <span id="crateattrmetalist-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for MetaList`

- <span id="metalist-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="metalist-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MetaList`

- <span id="metalist-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="metalist-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MetaNameValue`

```rust
struct MetaNameValue {
    pub path: crate::path::Path,
    pub eq_token: token::Eq,
    pub value: crate::expr::Expr,
}
```

*Defined in [`syn-2.0.111/src/attr.rs:494-502`](../../.source_1765521767/syn-2.0.111/src/attr.rs#L494-L502)*

A name-value pair within an attribute, like `feature = "nightly"`.

#### Implementations

- <span id="cratemetanamevalue-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for MetaNameValue`

- <span id="metanamevalue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MetaNameValue`

- <span id="metanamevalue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MetaNameValue`

- <span id="metanamevalue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::MetaNameValue`

- <span id="cratemetanamevalue-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for MetaNameValue`

- <span id="metanamevalue-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::MetaNameValue`

- <span id="cratemetanamevalue-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::MetaNameValue`

##### `impl<T> From for MetaNameValue`

- <span id="metanamevalue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::MetaNameValue`

- <span id="cratemetanamevalue-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for MetaNameValue`

- <span id="metanamevalue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::attr::MetaNameValue`

- <span id="crateattrmetanamevalue-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::MetaNameValue`

- <span id="cratemetanamevalue-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for MetaNameValue`

##### `impl Spanned for MetaNameValue`

- <span id="metanamevalue-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for MetaNameValue`

- <span id="metanamevalue-toowned-type-owned"></span>`type Owned = T`

- <span id="metanamevalue-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="metanamevalue-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::attr::MetaNameValue`

- <span id="crateattrmetanamevalue-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for MetaNameValue`

- <span id="metanamevalue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="metanamevalue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MetaNameValue`

- <span id="metanamevalue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="metanamevalue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/data.rs:181-200`](../../.source_1765521767/syn-2.0.111/src/data.rs#L181-L200)*

A field of a struct or enum variant.

#### Fields

- **`ident`**: `Option<crate::ident::Ident>`

  Name of the field, if any.
  
  Fields of tuple structs have no names.

#### Implementations

- <span id="cratedatafield-parse-named"></span>`fn parse_named(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

  Parses a named (braced struct) field.

- <span id="cratedatafield-parse-unnamed"></span>`fn parse_unnamed(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

  Parses an unnamed (tuple struct) field.

#### Trait Implementations

##### `impl Any for Field`

- <span id="field-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Field`

- <span id="field-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Field`

- <span id="field-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Field`

- <span id="cratefield-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Field`

- <span id="field-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Field`

- <span id="cratefield-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Field`

##### `impl<T> From for Field`

- <span id="field-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Field`

- <span id="cratefield-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Field`

- <span id="field-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::Field`

- <span id="cratefield-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Field`

##### `impl Spanned for Field`

- <span id="field-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Field`

- <span id="field-toowned-type-owned"></span>`type Owned = T`

- <span id="field-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="field-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::data::Field`

- <span id="cratedatafield-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Field`

- <span id="field-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="field-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Field`

- <span id="field-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="field-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FieldsNamed`

```rust
struct FieldsNamed {
    pub brace_token: token::Brace,
    pub named: crate::punctuated::Punctuated<Field, token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/data.rs:48-56`](../../.source_1765521767/syn-2.0.111/src/data.rs#L48-L56)*

Named fields of a struct or struct variant such as `Point { x: f64,
y: f64 }`.

#### Implementations

- <span id="cratefieldsnamed-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for FieldsNamed`

- <span id="fieldsnamed-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FieldsNamed`

- <span id="fieldsnamed-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FieldsNamed`

- <span id="fieldsnamed-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::FieldsNamed`

- <span id="cratefieldsnamed-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for FieldsNamed`

- <span id="fieldsnamed-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::FieldsNamed`

- <span id="cratefieldsnamed-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldsNamed`

##### `impl<T> From for FieldsNamed`

- <span id="fieldsnamed-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::FieldsNamed`

- <span id="cratefieldsnamed-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for FieldsNamed`

- <span id="fieldsnamed-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::data::FieldsNamed`

- <span id="cratedatafieldsnamed-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::FieldsNamed`

- <span id="cratefieldsnamed-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for FieldsNamed`

##### `impl Spanned for FieldsNamed`

- <span id="fieldsnamed-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for FieldsNamed`

- <span id="fieldsnamed-toowned-type-owned"></span>`type Owned = T`

- <span id="fieldsnamed-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fieldsnamed-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::data::FieldsNamed`

- <span id="cratedatafieldsnamed-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for FieldsNamed`

- <span id="fieldsnamed-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fieldsnamed-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FieldsNamed`

- <span id="fieldsnamed-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fieldsnamed-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FieldsUnnamed`

```rust
struct FieldsUnnamed {
    pub paren_token: token::Paren,
    pub unnamed: crate::punctuated::Punctuated<Field, token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/data.rs:58-65`](../../.source_1765521767/syn-2.0.111/src/data.rs#L58-L65)*

Unnamed fields of a tuple struct or tuple variant such as `Some(T)`.

#### Implementations

- <span id="cratefieldsunnamed-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for FieldsUnnamed`

- <span id="fieldsunnamed-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FieldsUnnamed`

- <span id="fieldsunnamed-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FieldsUnnamed`

- <span id="fieldsunnamed-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::FieldsUnnamed`

- <span id="cratefieldsunnamed-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for FieldsUnnamed`

- <span id="fieldsunnamed-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::FieldsUnnamed`

- <span id="cratefieldsunnamed-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldsUnnamed`

##### `impl<T> From for FieldsUnnamed`

- <span id="fieldsunnamed-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::FieldsUnnamed`

- <span id="cratefieldsunnamed-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for FieldsUnnamed`

- <span id="fieldsunnamed-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::data::FieldsUnnamed`

- <span id="cratedatafieldsunnamed-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::FieldsUnnamed`

- <span id="cratefieldsunnamed-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for FieldsUnnamed`

##### `impl Spanned for FieldsUnnamed`

- <span id="fieldsunnamed-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for FieldsUnnamed`

- <span id="fieldsunnamed-toowned-type-owned"></span>`type Owned = T`

- <span id="fieldsunnamed-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fieldsunnamed-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::data::FieldsUnnamed`

- <span id="cratedatafieldsunnamed-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for FieldsUnnamed`

- <span id="fieldsunnamed-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fieldsunnamed-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FieldsUnnamed`

- <span id="fieldsunnamed-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fieldsunnamed-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Variant`

```rust
struct Variant {
    pub attrs: Vec<crate::attr::Attribute>,
    pub ident: crate::ident::Ident,
    pub fields: Fields,
    pub discriminant: Option<(token::Eq, crate::expr::Expr)>,
}
```

*Defined in [`syn-2.0.111/src/data.rs:9-24`](../../.source_1765521767/syn-2.0.111/src/data.rs#L9-L24)*

An enum variant.

#### Fields

- **`ident`**: `crate::ident::Ident`

  Name of the variant.

- **`fields`**: `Fields`

  Content stored in the variant.

- **`discriminant`**: `Option<(token::Eq, crate::expr::Expr)>`

  Explicit discriminant: `Variant = 1`

#### Trait Implementations

##### `impl Any for Variant`

- <span id="variant-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Variant`

- <span id="variant-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Variant`

- <span id="variant-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Variant`

- <span id="cratevariant-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Variant`

- <span id="variant-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Variant`

- <span id="cratevariant-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Variant`

##### `impl<T> From for Variant`

- <span id="variant-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Variant`

- <span id="cratevariant-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Variant`

- <span id="variant-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::data::Variant`

- <span id="cratedatavariant-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::Variant`

- <span id="cratevariant-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Variant`

##### `impl Spanned for Variant`

- <span id="variant-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Variant`

- <span id="variant-toowned-type-owned"></span>`type Owned = T`

- <span id="variant-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="variant-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::data::Variant`

- <span id="cratedatavariant-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Variant`

- <span id="variant-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="variant-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Variant`

- <span id="variant-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="variant-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DataEnum`

```rust
struct DataEnum {
    pub enum_token: token::Enum,
    pub brace_token: token::Brace,
    pub variants: crate::punctuated::Punctuated<crate::data::Variant, token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/derive.rs:47-55`](../../.source_1765521767/syn-2.0.111/src/derive.rs#L47-L55)*

An enum input to a `proc_macro_derive` macro.

#### Implementations

- <span id="cratedataenum-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for DataEnum`

- <span id="dataenum-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DataEnum`

- <span id="dataenum-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DataEnum`

- <span id="dataenum-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::DataEnum`

- <span id="cratedataenum-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for DataEnum`

- <span id="dataenum-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::DataEnum`

- <span id="cratedataenum-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::DataEnum`

##### `impl<T> From for DataEnum`

- <span id="dataenum-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::DataEnum`

- <span id="cratedataenum-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for DataEnum`

- <span id="dataenum-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::DataEnum`

- <span id="cratedataenum-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl ToOwned for DataEnum`

- <span id="dataenum-toowned-type-owned"></span>`type Owned = T`

- <span id="dataenum-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dataenum-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DataEnum`

- <span id="dataenum-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dataenum-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DataEnum`

- <span id="dataenum-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dataenum-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DataStruct`

```rust
struct DataStruct {
    pub struct_token: token::Struct,
    pub fields: crate::data::Fields,
    pub semi_token: Option<token::Semi>,
}
```

*Defined in [`syn-2.0.111/src/derive.rs:37-45`](../../.source_1765521767/syn-2.0.111/src/derive.rs#L37-L45)*

A struct input to a `proc_macro_derive` macro.

#### Implementations

- <span id="cratedatastruct-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for DataStruct`

- <span id="datastruct-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DataStruct`

- <span id="datastruct-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DataStruct`

- <span id="datastruct-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::DataStruct`

- <span id="cratedatastruct-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for DataStruct`

- <span id="datastruct-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::DataStruct`

- <span id="cratedatastruct-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::DataStruct`

##### `impl<T> From for DataStruct`

- <span id="datastruct-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::DataStruct`

- <span id="cratedatastruct-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for DataStruct`

- <span id="datastruct-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::DataStruct`

- <span id="cratedatastruct-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl ToOwned for DataStruct`

- <span id="datastruct-toowned-type-owned"></span>`type Owned = T`

- <span id="datastruct-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="datastruct-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DataStruct`

- <span id="datastruct-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="datastruct-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DataStruct`

- <span id="datastruct-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="datastruct-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DataUnion`

```rust
struct DataUnion {
    pub union_token: token::Union,
    pub fields: crate::data::FieldsNamed,
}
```

*Defined in [`syn-2.0.111/src/derive.rs:57-64`](../../.source_1765521767/syn-2.0.111/src/derive.rs#L57-L64)*

An untagged union input to a `proc_macro_derive` macro.

#### Implementations

- <span id="cratedataunion-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for DataUnion`

- <span id="dataunion-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DataUnion`

- <span id="dataunion-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DataUnion`

- <span id="dataunion-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::DataUnion`

- <span id="cratedataunion-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for DataUnion`

- <span id="dataunion-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::DataUnion`

- <span id="cratedataunion-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::DataUnion`

##### `impl<T> From for DataUnion`

- <span id="dataunion-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::DataUnion`

- <span id="cratedataunion-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for DataUnion`

- <span id="dataunion-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::DataUnion`

- <span id="cratedataunion-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl ToOwned for DataUnion`

- <span id="dataunion-toowned-type-owned"></span>`type Owned = T`

- <span id="dataunion-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dataunion-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DataUnion`

- <span id="dataunion-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dataunion-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DataUnion`

- <span id="dataunion-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dataunion-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/derive.rs:9-19`](../../.source_1765521767/syn-2.0.111/src/derive.rs#L9-L19)*

Data structure sent to a `proc_macro_derive` macro.

#### Trait Implementations

##### `impl Any for DeriveInput`

- <span id="deriveinput-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DeriveInput`

- <span id="deriveinput-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DeriveInput`

- <span id="deriveinput-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::DeriveInput`

- <span id="cratederiveinput-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for DeriveInput`

- <span id="deriveinput-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::DeriveInput`

- <span id="cratederiveinput-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::DeriveInput`

##### `impl<T> From for DeriveInput`

- <span id="deriveinput-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::DeriveInput`

- <span id="cratederiveinput-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for DeriveInput`

- <span id="deriveinput-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::derive::DeriveInput`

- <span id="cratederivederiveinput-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::DeriveInput`

- <span id="cratederiveinput-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for DeriveInput`

##### `impl Spanned for DeriveInput`

- <span id="deriveinput-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for DeriveInput`

- <span id="deriveinput-toowned-type-owned"></span>`type Owned = T`

- <span id="deriveinput-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="deriveinput-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::derive::DeriveInput`

- <span id="cratederivederiveinput-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for DeriveInput`

- <span id="deriveinput-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="deriveinput-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DeriveInput`

- <span id="deriveinput-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="deriveinput-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Error`

```rust
struct Error {
    messages: Vec<ErrorMessage>,
}
```

*Defined in [`syn-2.0.111/src/error.rs:101-103`](../../.source_1765521767/syn-2.0.111/src/error.rs#L101-L103)*

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

- <span id="error-combine"></span>`fn combine(&mut self, another: Error)` — [`Error`](error/index.md#error)

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

*Defined in [`syn-2.0.111/src/expr.rs:1119-1146`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L1119-L1146)*

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

- <span id="crateexprarm-parse-multiple"></span>`fn parse_multiple(input: ParseStream<'_>) -> Result<Vec<Self>>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

#### Trait Implementations

##### `impl Any for Arm`

- <span id="arm-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Arm`

- <span id="arm-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Arm`

- <span id="arm-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Arm`

- <span id="cratearm-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Arm`

- <span id="arm-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Arm`

- <span id="cratearm-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Arm`

##### `impl<T> From for Arm`

- <span id="arm-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Arm`

- <span id="cratearm-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Arm`

- <span id="arm-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::Arm`

- <span id="crateexprarm-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Arm>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result), [`Arm`](expr/index.md#arm)

##### `impl PartialEq for crate::Arm`

- <span id="cratearm-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Arm`

##### `impl Spanned for Arm`

- <span id="arm-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Arm`

- <span id="arm-toowned-type-owned"></span>`type Owned = T`

- <span id="arm-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="arm-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::Arm`

- <span id="crateexprarm-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Arm`

- <span id="arm-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="arm-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Arm`

- <span id="arm-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="arm-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Label`

```rust
struct Label {
    pub name: crate::lifetime::Lifetime,
    pub colon_token: token::Colon,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:1109-1116`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L1109-L1116)*

A lifetime labeling a `for`, `while`, or `loop`.

#### Trait Implementations

##### `impl Any for Label`

- <span id="label-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Label`

- <span id="label-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Label`

- <span id="label-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Label`

- <span id="cratelabel-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Label`

- <span id="label-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Label`

- <span id="cratelabel-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Label`

##### `impl<T> From for Label`

- <span id="label-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Label`

- <span id="cratelabel-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Label`

- <span id="label-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::Label`

- <span id="crateexprlabel-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::Label`

- <span id="cratelabel-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Label`

##### `impl Spanned for Label`

- <span id="label-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Label`

- <span id="label-toowned-type-owned"></span>`type Owned = T`

- <span id="label-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="label-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::Label`

- <span id="crateexprlabel-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Label`

- <span id="label-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="label-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Label`

- <span id="label-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="label-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprBinary`

```rust
struct ExprBinary {
    pub attrs: Vec<crate::attr::Attribute>,
    pub left: Box<Expr>,
    pub op: crate::op::BinOp,
    pub right: Box<Expr>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:312-321`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L312-L321)*

A binary operation: `a + b`, `a += b`.

#### Implementations

- <span id="crateexprbinary-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprBinary`

- <span id="exprbinary-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprBinary`

- <span id="exprbinary-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprBinary`

- <span id="exprbinary-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprBinary`

- <span id="crateexprbinary-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprBinary`

- <span id="exprbinary-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprBinary`

- <span id="crateexprbinary-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprBinary`

##### `impl<T> From for ExprBinary`

- <span id="exprbinary-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprBinary`

- <span id="crateexprbinary-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprBinary`

- <span id="exprbinary-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprBinary`

- <span id="crateexprexprbinary-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprBinary`

- <span id="crateexprbinary-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprBinary`

##### `impl Spanned for ExprBinary`

- <span id="exprbinary-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprBinary`

- <span id="exprbinary-toowned-type-owned"></span>`type Owned = T`

- <span id="exprbinary-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprbinary-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprBinary`

- <span id="crateexprexprbinary-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprBinary`

- <span id="exprbinary-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprbinary-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprBinary`

- <span id="exprbinary-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprbinary-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprCall`

```rust
struct ExprCall {
    pub attrs: Vec<crate::attr::Attribute>,
    pub func: Box<Expr>,
    pub paren_token: token::Paren,
    pub args: crate::punctuated::Punctuated<Expr, token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:345-354`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L345-L354)*

A function call expression: `invoke(a, b)`.

#### Implementations

- <span id="crateexprcall-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprCall`

- <span id="exprcall-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprCall`

- <span id="exprcall-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprCall`

- <span id="exprcall-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprCall`

- <span id="crateexprcall-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprCall`

- <span id="exprcall-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprCall`

- <span id="crateexprcall-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprCall`

##### `impl<T> From for ExprCall`

- <span id="exprcall-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprCall`

- <span id="crateexprcall-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprCall`

- <span id="exprcall-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprCall`

- <span id="crateexprexprcall-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprCall`

- <span id="crateexprcall-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprCall`

##### `impl Spanned for ExprCall`

- <span id="exprcall-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprCall`

- <span id="exprcall-toowned-type-owned"></span>`type Owned = T`

- <span id="exprcall-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprcall-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprCall`

- <span id="crateexprexprcall-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprCall`

- <span id="exprcall-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprcall-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprCall`

- <span id="exprcall-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprcall-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprCast`

```rust
struct ExprCast {
    pub attrs: Vec<crate::attr::Attribute>,
    pub expr: Box<Expr>,
    pub as_token: token::As,
    pub ty: Box<crate::ty::Type>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:356-365`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L356-L365)*

A cast expression: `foo as f64`.

#### Implementations

- <span id="crateexprcast-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprCast`

- <span id="exprcast-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprCast`

- <span id="exprcast-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprCast`

- <span id="exprcast-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprCast`

- <span id="crateexprcast-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprCast`

- <span id="exprcast-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprCast`

- <span id="crateexprcast-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprCast`

##### `impl<T> From for ExprCast`

- <span id="exprcast-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprCast`

- <span id="crateexprcast-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprCast`

- <span id="exprcast-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprCast`

- <span id="crateexprexprcast-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprCast`

- <span id="crateexprcast-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprCast`

##### `impl Spanned for ExprCast`

- <span id="exprcast-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprCast`

- <span id="exprcast-toowned-type-owned"></span>`type Owned = T`

- <span id="exprcast-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprcast-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprCast`

- <span id="crateexprexprcast-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprCast`

- <span id="exprcast-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprcast-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprCast`

- <span id="exprcast-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprcast-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprField`

```rust
struct ExprField {
    pub attrs: Vec<crate::attr::Attribute>,
    pub base: Box<Expr>,
    pub dot_token: token::Dot,
    pub member: Member,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:405-415`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L405-L415)*

Access of a named struct field (`obj.k`) or unnamed tuple struct
field (`obj.0`).

#### Implementations

- <span id="crateexprfield-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprField`

- <span id="exprfield-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprField`

- <span id="exprfield-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprField`

- <span id="exprfield-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprField`

- <span id="crateexprfield-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprField`

- <span id="exprfield-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprField`

- <span id="crateexprfield-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprField`

##### `impl<T> From for ExprField`

- <span id="exprfield-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprField`

- <span id="crateexprfield-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprField`

- <span id="exprfield-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprField`

- <span id="crateexprexprfield-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprField`

- <span id="crateexprfield-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprField`

##### `impl Spanned for ExprField`

- <span id="exprfield-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprField`

- <span id="exprfield-toowned-type-owned"></span>`type Owned = T`

- <span id="exprfield-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprfield-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprField`

- <span id="crateexprexprfield-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprField`

- <span id="exprfield-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprfield-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprField`

- <span id="exprfield-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprfield-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprIndex`

```rust
struct ExprIndex {
    pub attrs: Vec<crate::attr::Attribute>,
    pub expr: Box<Expr>,
    pub bracket_token: token::Bracket,
    pub index: Box<Expr>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:461-470`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L461-L470)*

A square bracketed indexing expression: `vector[2]`.

#### Implementations

- <span id="crateexprindex-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprIndex`

- <span id="exprindex-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprIndex`

- <span id="exprindex-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprIndex`

- <span id="exprindex-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprIndex`

- <span id="crateexprindex-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprIndex`

- <span id="exprindex-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprIndex`

- <span id="crateexprindex-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprIndex`

##### `impl<T> From for ExprIndex`

- <span id="exprindex-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprIndex`

- <span id="crateexprindex-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprIndex`

- <span id="exprindex-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprIndex`

- <span id="crateexprexprindex-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprIndex`

- <span id="crateexprindex-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprIndex`

##### `impl Spanned for ExprIndex`

- <span id="exprindex-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprIndex`

- <span id="exprindex-toowned-type-owned"></span>`type Owned = T`

- <span id="exprindex-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprindex-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprIndex`

- <span id="crateexprexprindex-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprIndex`

- <span id="exprindex-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprindex-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprIndex`

- <span id="exprindex-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprindex-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprLit`

```rust
struct ExprLit {
    pub attrs: Vec<crate::attr::Attribute>,
    pub lit: crate::lit::Lit,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:493-500`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L493-L500)*

A literal in place of an expression: `1`, `"foo"`.

#### Implementations

- <span id="crateexprlit-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprLit`

- <span id="exprlit-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprLit`

- <span id="exprlit-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprLit`

- <span id="exprlit-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprLit`

- <span id="crateexprlit-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprLit`

- <span id="exprlit-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprLit`

- <span id="crateexprlit-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprLit`

##### `impl<T> From for ExprLit`

- <span id="exprlit-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprLit`

- <span id="crateexprlit-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprLit`

- <span id="exprlit-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprLit`

- <span id="crateexprexprlit-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprLit`

- <span id="crateexprlit-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprLit`

##### `impl Spanned for ExprLit`

- <span id="exprlit-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprLit`

- <span id="exprlit-toowned-type-owned"></span>`type Owned = T`

- <span id="exprlit-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprlit-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprLit`

- <span id="crateexprexprlit-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprLit`

- <span id="exprlit-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprlit-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprLit`

- <span id="exprlit-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprlit-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprMacro`

```rust
struct ExprMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:513-520`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L513-L520)*

A macro invocation expression: `format!("{}", q)`.

#### Implementations

- <span id="crateexprmacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprMacro`

- <span id="exprmacro-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprMacro`

- <span id="exprmacro-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprMacro`

- <span id="exprmacro-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprMacro`

- <span id="crateexprmacro-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprMacro`

- <span id="exprmacro-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprMacro`

- <span id="crateexprmacro-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprMacro`

##### `impl<T> From for ExprMacro`

- <span id="exprmacro-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprMacro`

- <span id="crateexprmacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprMacro`

- <span id="exprmacro-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprMacro`

- <span id="crateexprexprmacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprMacro`

- <span id="crateexprmacro-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprMacro`

##### `impl Spanned for ExprMacro`

- <span id="exprmacro-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprMacro`

- <span id="exprmacro-toowned-type-owned"></span>`type Owned = T`

- <span id="exprmacro-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprmacro-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprMacro`

- <span id="crateexprexprmacro-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprMacro`

- <span id="exprmacro-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprmacro-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprMacro`

- <span id="exprmacro-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprmacro-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/expr.rs:534-546`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L534-L546)*

A method call expression: `x.foo::<T>(a, b)`.

#### Implementations

- <span id="crateexprmethodcall-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprMethodCall`

- <span id="exprmethodcall-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprMethodCall`

- <span id="exprmethodcall-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprMethodCall`

- <span id="exprmethodcall-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprMethodCall`

- <span id="crateexprmethodcall-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprMethodCall`

- <span id="exprmethodcall-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprMethodCall`

- <span id="crateexprmethodcall-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprMethodCall`

##### `impl<T> From for ExprMethodCall`

- <span id="exprmethodcall-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprMethodCall`

- <span id="crateexprmethodcall-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprMethodCall`

- <span id="exprmethodcall-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprMethodCall`

- <span id="crateexprexprmethodcall-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprMethodCall`

- <span id="crateexprmethodcall-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprMethodCall`

##### `impl Spanned for ExprMethodCall`

- <span id="exprmethodcall-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprMethodCall`

- <span id="exprmethodcall-toowned-type-owned"></span>`type Owned = T`

- <span id="exprmethodcall-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprmethodcall-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprMethodCall`

- <span id="crateexprexprmethodcall-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprMethodCall`

- <span id="exprmethodcall-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprmethodcall-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprMethodCall`

- <span id="exprmethodcall-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprmethodcall-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprParen`

```rust
struct ExprParen {
    pub attrs: Vec<crate::attr::Attribute>,
    pub paren_token: token::Paren,
    pub expr: Box<Expr>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:548-556`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L548-L556)*

A parenthesized expression: `(a + b)`.

#### Implementations

- <span id="crateexprparen-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprParen`

- <span id="exprparen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprParen`

- <span id="exprparen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprParen`

- <span id="exprparen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprParen`

- <span id="crateexprparen-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprParen`

- <span id="exprparen-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprParen`

- <span id="crateexprparen-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprParen`

##### `impl<T> From for ExprParen`

- <span id="exprparen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprParen`

- <span id="crateexprparen-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprParen`

- <span id="exprparen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprParen`

- <span id="crateexprexprparen-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprParen`

- <span id="crateexprparen-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprParen`

##### `impl Spanned for ExprParen`

- <span id="exprparen-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprParen`

- <span id="exprparen-toowned-type-owned"></span>`type Owned = T`

- <span id="exprparen-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprparen-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprParen`

- <span id="crateexprexprparen-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprParen`

- <span id="exprparen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprparen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprParen`

- <span id="exprparen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprparen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprPath`

```rust
struct ExprPath {
    pub attrs: Vec<crate::attr::Attribute>,
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:558-569`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L558-L569)*

A path like `std::mem::replace` possibly containing generic
parameters and a qualified self-type.

A plain identifier like `x` is a path of length 1.

#### Implementations

- <span id="crateexprpath-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprPath`

- <span id="exprpath-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprPath`

- <span id="exprpath-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprPath`

- <span id="exprpath-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprPath`

- <span id="crateexprpath-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprPath`

- <span id="exprpath-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprPath`

- <span id="crateexprpath-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprPath`

##### `impl<T> From for ExprPath`

- <span id="exprpath-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprPath`

- <span id="crateexprpath-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprPath`

- <span id="exprpath-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprPath`

- <span id="crateexprexprpath-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprPath`

- <span id="crateexprpath-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprPath`

##### `impl Spanned for ExprPath`

- <span id="exprpath-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprPath`

- <span id="exprpath-toowned-type-owned"></span>`type Owned = T`

- <span id="exprpath-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprpath-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprPath`

- <span id="crateexprexprpath-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprPath`

- <span id="exprpath-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprpath-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprPath`

- <span id="exprpath-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprpath-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprReference`

```rust
struct ExprReference {
    pub attrs: Vec<crate::attr::Attribute>,
    pub and_token: token::And,
    pub mutability: Option<token::Mut>,
    pub expr: Box<Expr>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:594-603`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L594-L603)*

A referencing operation: `&a` or `&mut a`.

#### Implementations

- <span id="crateexprreference-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprReference`

- <span id="exprreference-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprReference`

- <span id="exprreference-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprReference`

- <span id="exprreference-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprReference`

- <span id="crateexprreference-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprReference`

- <span id="exprreference-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprReference`

- <span id="crateexprreference-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprReference`

##### `impl<T> From for ExprReference`

- <span id="exprreference-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprReference`

- <span id="crateexprreference-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprReference`

- <span id="exprreference-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprReference`

- <span id="crateexprexprreference-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprReference`

- <span id="crateexprreference-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprReference`

##### `impl Spanned for ExprReference`

- <span id="exprreference-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprReference`

- <span id="exprreference-toowned-type-owned"></span>`type Owned = T`

- <span id="exprreference-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprreference-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprReference`

- <span id="crateexprexprreference-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprReference`

- <span id="exprreference-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprreference-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprReference`

- <span id="exprreference-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprreference-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/expr.rs:627-642`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L627-L642)*

A struct literal expression: `Point { x: 1, y: 1 }`.

The `rest` provides the value of the remaining fields as in `S { a:
1, b: 1, ..rest }`.

#### Implementations

- <span id="crateexprstruct-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprStruct`

- <span id="exprstruct-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprStruct`

- <span id="exprstruct-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprStruct`

- <span id="exprstruct-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprStruct`

- <span id="crateexprstruct-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprStruct`

- <span id="exprstruct-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprStruct`

- <span id="crateexprstruct-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprStruct`

##### `impl<T> From for ExprStruct`

- <span id="exprstruct-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprStruct`

- <span id="crateexprstruct-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprStruct`

- <span id="exprstruct-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprStruct`

- <span id="crateexprexprstruct-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprStruct`

- <span id="crateexprstruct-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprStruct`

##### `impl Spanned for ExprStruct`

- <span id="exprstruct-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprStruct`

- <span id="exprstruct-toowned-type-owned"></span>`type Owned = T`

- <span id="exprstruct-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprstruct-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprStruct`

- <span id="crateexprexprstruct-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprStruct`

- <span id="exprstruct-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprstruct-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprStruct`

- <span id="exprstruct-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprstruct-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprUnary`

```rust
struct ExprUnary {
    pub attrs: Vec<crate::attr::Attribute>,
    pub op: crate::op::UnOp,
    pub expr: Box<Expr>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:674-682`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L674-L682)*

A unary operation: `!x`, `*x`.

#### Implementations

- <span id="crateexprunary-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprUnary`

- <span id="exprunary-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprUnary`

- <span id="exprunary-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprUnary`

- <span id="exprunary-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprUnary`

- <span id="crateexprunary-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprUnary`

- <span id="exprunary-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprUnary`

- <span id="crateexprunary-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprUnary`

##### `impl<T> From for ExprUnary`

- <span id="exprunary-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprUnary`

- <span id="crateexprunary-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprUnary`

- <span id="exprunary-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprUnary`

- <span id="crateexprexprunary-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprUnary`

- <span id="crateexprunary-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprUnary`

##### `impl Spanned for ExprUnary`

- <span id="exprunary-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprUnary`

- <span id="exprunary-toowned-type-owned"></span>`type Owned = T`

- <span id="exprunary-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprunary-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprUnary`

- <span id="crateexprexprunary-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprUnary`

- <span id="exprunary-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprunary-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprUnary`

- <span id="exprunary-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprunary-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FieldValue`

```rust
struct FieldValue {
    pub attrs: Vec<crate::attr::Attribute>,
    pub member: Member,
    pub colon_token: Option<token::Colon>,
    pub expr: Expr,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:1093-1106`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L1093-L1106)*

A field-value pair in a struct literal.

#### Fields

- **`colon_token`**: `Option<token::Colon>`

  The colon in `Struct { x: x }`. If written in shorthand like
  `Struct { x }`, there is no colon.

#### Trait Implementations

##### `impl Any for FieldValue`

- <span id="fieldvalue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FieldValue`

- <span id="fieldvalue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FieldValue`

- <span id="fieldvalue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::FieldValue`

- <span id="cratefieldvalue-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for FieldValue`

- <span id="fieldvalue-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::FieldValue`

- <span id="cratefieldvalue-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldValue`

##### `impl<T> From for FieldValue`

- <span id="fieldvalue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::FieldValue`

- <span id="cratefieldvalue-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for FieldValue`

- <span id="fieldvalue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::FieldValue`

- <span id="crateexprfieldvalue-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::FieldValue`

- <span id="cratefieldvalue-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for FieldValue`

##### `impl Spanned for FieldValue`

- <span id="fieldvalue-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for FieldValue`

- <span id="fieldvalue-toowned-type-owned"></span>`type Owned = T`

- <span id="fieldvalue-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fieldvalue-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::FieldValue`

- <span id="crateexprfieldvalue-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for FieldValue`

- <span id="fieldvalue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fieldvalue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FieldValue`

- <span id="fieldvalue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fieldvalue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Index`

```rust
struct Index {
    pub index: u32,
    pub span: proc_macro2::Span,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:1049-1056`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L1049-L1056)*

The index of an unnamed tuple struct field.

#### Trait Implementations

##### `impl Any for Index`

- <span id="index-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Index`

- <span id="index-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Index`

- <span id="index-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Index`

- <span id="crateindex-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Index`

- <span id="index-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Index`

- <span id="crateindex-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Index`

##### `impl<T> From for Index`

- <span id="index-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Index`

- <span id="index-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl IdentFragment for Index`

- <span id="index-identfragment-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="index-identfragment-span"></span>`fn span(&self) -> Option<Span>`

##### `impl<U> Into for Index`

- <span id="index-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::Index`

- <span id="crateexprindex-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for Index`

- <span id="index-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Index`

##### `impl Spanned for Index`

- <span id="index-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Index`

- <span id="index-toowned-type-owned"></span>`type Owned = T`

- <span id="index-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="index-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::Index`

- <span id="crateexprindex-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Index`

- <span id="index-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="index-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Index`

- <span id="index-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="index-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprArray`

```rust
struct ExprArray {
    pub attrs: Vec<crate::attr::Attribute>,
    pub bracket_token: token::Bracket,
    pub elems: crate::punctuated::Punctuated<Expr, token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:269-277`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L269-L277)*

A slice literal expression: `[a, b, c, d]`.

#### Implementations

- <span id="crateexprarray-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprArray`

- <span id="exprarray-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprArray`

- <span id="exprarray-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprArray`

- <span id="exprarray-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprArray`

- <span id="crateexprarray-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprArray`

- <span id="exprarray-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprArray`

- <span id="crateexprarray-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprArray`

##### `impl<T> From for ExprArray`

- <span id="exprarray-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprArray`

- <span id="crateexprarray-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprArray`

- <span id="exprarray-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprArray`

- <span id="crateexprexprarray-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprArray`

- <span id="crateexprarray-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprArray`

##### `impl Spanned for ExprArray`

- <span id="exprarray-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprArray`

- <span id="exprarray-toowned-type-owned"></span>`type Owned = T`

- <span id="exprarray-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprarray-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprArray`

- <span id="crateexprexprarray-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprArray`

- <span id="exprarray-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprarray-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprArray`

- <span id="exprarray-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprarray-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprAssign`

```rust
struct ExprAssign {
    pub attrs: Vec<crate::attr::Attribute>,
    pub left: Box<Expr>,
    pub eq_token: token::Eq,
    pub right: Box<Expr>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:279-288`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L279-L288)*

An assignment expression: `a = compute()`.

#### Implementations

- <span id="crateexprassign-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprAssign`

- <span id="exprassign-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprAssign`

- <span id="exprassign-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprAssign`

- <span id="exprassign-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprAssign`

- <span id="crateexprassign-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprAssign`

- <span id="exprassign-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprAssign`

- <span id="crateexprassign-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprAssign`

##### `impl<T> From for ExprAssign`

- <span id="exprassign-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprAssign`

- <span id="crateexprassign-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprAssign`

- <span id="exprassign-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprAssign`

- <span id="crateexprexprassign-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprAssign`

- <span id="crateexprassign-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprAssign`

##### `impl Spanned for ExprAssign`

- <span id="exprassign-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprAssign`

- <span id="exprassign-toowned-type-owned"></span>`type Owned = T`

- <span id="exprassign-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprassign-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprAssign`

- <span id="crateexprexprassign-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprAssign`

- <span id="exprassign-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprassign-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprAssign`

- <span id="exprassign-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprassign-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprAsync`

```rust
struct ExprAsync {
    pub attrs: Vec<crate::attr::Attribute>,
    pub async_token: token::Async,
    pub capture: Option<token::Move>,
    pub block: crate::stmt::Block,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:290-299`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L290-L299)*

An async block: `async { ... }`.

#### Implementations

- <span id="crateexprasync-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprAsync`

- <span id="exprasync-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprAsync`

- <span id="exprasync-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprAsync`

- <span id="exprasync-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprAsync`

- <span id="crateexprasync-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprAsync`

- <span id="exprasync-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprAsync`

- <span id="crateexprasync-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprAsync`

##### `impl<T> From for ExprAsync`

- <span id="exprasync-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprAsync`

- <span id="crateexprasync-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprAsync`

- <span id="exprasync-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprAsync`

- <span id="crateexprexprasync-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprAsync`

- <span id="crateexprasync-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprAsync`

##### `impl Spanned for ExprAsync`

- <span id="exprasync-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprAsync`

- <span id="exprasync-toowned-type-owned"></span>`type Owned = T`

- <span id="exprasync-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprasync-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprAsync`

- <span id="crateexprexprasync-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprAsync`

- <span id="exprasync-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprasync-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprAsync`

- <span id="exprasync-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprasync-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprAwait`

```rust
struct ExprAwait {
    pub attrs: Vec<crate::attr::Attribute>,
    pub base: Box<Expr>,
    pub dot_token: token::Dot,
    pub await_token: token::Await,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:301-310`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L301-L310)*

An await expression: `fut.await`.

#### Implementations

- <span id="crateexprawait-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprAwait`

- <span id="exprawait-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprAwait`

- <span id="exprawait-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprAwait`

- <span id="exprawait-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprAwait`

- <span id="crateexprawait-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprAwait`

- <span id="exprawait-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprAwait`

- <span id="crateexprawait-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprAwait`

##### `impl<T> From for ExprAwait`

- <span id="exprawait-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprAwait`

- <span id="crateexprawait-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprAwait`

- <span id="exprawait-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprAwait`

- <span id="crateexprexprawait-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprAwait`

- <span id="crateexprawait-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprAwait`

##### `impl Spanned for ExprAwait`

- <span id="exprawait-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprAwait`

- <span id="exprawait-toowned-type-owned"></span>`type Owned = T`

- <span id="exprawait-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprawait-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprAwait`

- <span id="crateexprexprawait-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprAwait`

- <span id="exprawait-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprawait-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprAwait`

- <span id="exprawait-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprawait-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprBlock`

```rust
struct ExprBlock {
    pub attrs: Vec<crate::attr::Attribute>,
    pub label: Option<Label>,
    pub block: crate::stmt::Block,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:323-331`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L323-L331)*

A blocked scope: `{ ... }`.

#### Implementations

- <span id="crateexprblock-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprBlock`

- <span id="exprblock-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprBlock`

- <span id="exprblock-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprBlock`

- <span id="exprblock-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprBlock`

- <span id="crateexprblock-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprBlock`

- <span id="exprblock-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprBlock`

- <span id="crateexprblock-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprBlock`

##### `impl<T> From for ExprBlock`

- <span id="exprblock-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprBlock`

- <span id="crateexprblock-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprBlock`

- <span id="exprblock-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprBlock`

- <span id="crateexprexprblock-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprBlock`

- <span id="crateexprblock-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprBlock`

##### `impl Spanned for ExprBlock`

- <span id="exprblock-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprBlock`

- <span id="exprblock-toowned-type-owned"></span>`type Owned = T`

- <span id="exprblock-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprblock-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprBlock`

- <span id="crateexprexprblock-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprBlock`

- <span id="exprblock-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprblock-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprBlock`

- <span id="exprblock-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprblock-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprBreak`

```rust
struct ExprBreak {
    pub attrs: Vec<crate::attr::Attribute>,
    pub break_token: token::Break,
    pub label: Option<crate::lifetime::Lifetime>,
    pub expr: Option<Box<Expr>>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:333-343`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L333-L343)*

A `break`, with an optional label to break and an optional
expression.

#### Implementations

- <span id="crateexprbreak-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprBreak`

- <span id="exprbreak-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprBreak`

- <span id="exprbreak-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprBreak`

- <span id="exprbreak-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprBreak`

- <span id="crateexprbreak-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprBreak`

- <span id="exprbreak-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprBreak`

- <span id="crateexprbreak-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprBreak`

##### `impl<T> From for ExprBreak`

- <span id="exprbreak-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprBreak`

- <span id="crateexprbreak-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprBreak`

- <span id="exprbreak-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprBreak`

- <span id="crateexprexprbreak-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprBreak`

- <span id="crateexprbreak-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprBreak`

##### `impl Spanned for ExprBreak`

- <span id="exprbreak-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprBreak`

- <span id="exprbreak-toowned-type-owned"></span>`type Owned = T`

- <span id="exprbreak-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprbreak-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprBreak`

- <span id="crateexprexprbreak-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprBreak`

- <span id="exprbreak-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprbreak-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprBreak`

- <span id="exprbreak-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprbreak-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/expr.rs:367-383`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L367-L383)*

A closure expression: `|a, b| a + b`.

#### Implementations

- <span id="crateexprclosure-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprClosure`

- <span id="exprclosure-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprClosure`

- <span id="exprclosure-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprClosure`

- <span id="exprclosure-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprClosure`

- <span id="crateexprclosure-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprClosure`

- <span id="exprclosure-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprClosure`

- <span id="crateexprclosure-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprClosure`

##### `impl<T> From for ExprClosure`

- <span id="exprclosure-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprClosure`

- <span id="crateexprclosure-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprClosure`

- <span id="exprclosure-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprClosure`

- <span id="crateexprexprclosure-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprClosure`

- <span id="crateexprclosure-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprClosure`

##### `impl Spanned for ExprClosure`

- <span id="exprclosure-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprClosure`

- <span id="exprclosure-toowned-type-owned"></span>`type Owned = T`

- <span id="exprclosure-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprclosure-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprClosure`

- <span id="crateexprexprclosure-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprClosure`

- <span id="exprclosure-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprclosure-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprClosure`

- <span id="exprclosure-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprclosure-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprConst`

```rust
struct ExprConst {
    pub attrs: Vec<crate::attr::Attribute>,
    pub const_token: token::Const,
    pub block: crate::stmt::Block,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:385-393`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L385-L393)*

A const block: `const { ... }`.

#### Implementations

- <span id="crateexprconst-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprConst`

- <span id="exprconst-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprConst`

- <span id="exprconst-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprConst`

- <span id="exprconst-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprConst`

- <span id="crateexprconst-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprConst`

- <span id="exprconst-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprConst`

- <span id="crateexprconst-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprConst`

##### `impl<T> From for ExprConst`

- <span id="exprconst-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprConst`

- <span id="crateexprconst-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprConst`

- <span id="exprconst-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprConst`

- <span id="crateexprexprconst-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprConst`

- <span id="crateexprconst-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprConst`

##### `impl Spanned for ExprConst`

- <span id="exprconst-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprConst`

- <span id="exprconst-toowned-type-owned"></span>`type Owned = T`

- <span id="exprconst-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprconst-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprConst`

- <span id="crateexprexprconst-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprConst`

- <span id="exprconst-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprconst-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprConst`

- <span id="exprconst-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprconst-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprContinue`

```rust
struct ExprContinue {
    pub attrs: Vec<crate::attr::Attribute>,
    pub continue_token: token::Continue,
    pub label: Option<crate::lifetime::Lifetime>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:395-403`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L395-L403)*

A `continue`, with an optional label.

#### Implementations

- <span id="crateexprcontinue-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprContinue`

- <span id="exprcontinue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprContinue`

- <span id="exprcontinue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprContinue`

- <span id="exprcontinue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprContinue`

- <span id="crateexprcontinue-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprContinue`

- <span id="exprcontinue-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprContinue`

- <span id="crateexprcontinue-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprContinue`

##### `impl<T> From for ExprContinue`

- <span id="exprcontinue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprContinue`

- <span id="crateexprcontinue-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprContinue`

- <span id="exprcontinue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprContinue`

- <span id="crateexprexprcontinue-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprContinue`

- <span id="crateexprcontinue-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprContinue`

##### `impl Spanned for ExprContinue`

- <span id="exprcontinue-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprContinue`

- <span id="exprcontinue-toowned-type-owned"></span>`type Owned = T`

- <span id="exprcontinue-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprcontinue-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprContinue`

- <span id="crateexprexprcontinue-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprContinue`

- <span id="exprcontinue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprcontinue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprContinue`

- <span id="exprcontinue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprcontinue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/expr.rs:417-429`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L417-L429)*

A for loop: `for pat in expr { ... }`.

#### Implementations

- <span id="crateexprforloop-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprForLoop`

- <span id="exprforloop-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprForLoop`

- <span id="exprforloop-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprForLoop`

- <span id="exprforloop-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprForLoop`

- <span id="crateexprforloop-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprForLoop`

- <span id="exprforloop-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprForLoop`

- <span id="crateexprforloop-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprForLoop`

##### `impl<T> From for ExprForLoop`

- <span id="exprforloop-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprForLoop`

- <span id="crateexprforloop-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprForLoop`

- <span id="exprforloop-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprForLoop`

- <span id="crateexprexprforloop-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprForLoop`

- <span id="crateexprforloop-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprForLoop`

##### `impl Spanned for ExprForLoop`

- <span id="exprforloop-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprForLoop`

- <span id="exprforloop-toowned-type-owned"></span>`type Owned = T`

- <span id="exprforloop-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprforloop-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprForLoop`

- <span id="crateexprexprforloop-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprForLoop`

- <span id="exprforloop-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprforloop-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprForLoop`

- <span id="exprforloop-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprforloop-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprGroup`

```rust
struct ExprGroup {
    pub attrs: Vec<crate::attr::Attribute>,
    pub group_token: token::Group,
    pub expr: Box<Expr>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:431-443`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L431-L443)*

An expression contained within invisible delimiters.

This variant is important for faithfully representing the precedence
of expressions and is related to `None`-delimited spans in a
`TokenStream`.

#### Implementations

- <span id="crateexprgroup-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprGroup`

- <span id="exprgroup-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprGroup`

- <span id="exprgroup-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprGroup`

- <span id="exprgroup-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprGroup`

- <span id="crateexprgroup-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprGroup`

- <span id="exprgroup-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprGroup`

- <span id="crateexprgroup-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprGroup`

##### `impl<T> From for ExprGroup`

- <span id="exprgroup-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprGroup`

- <span id="crateexprgroup-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprGroup`

- <span id="exprgroup-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::ExprGroup`

- <span id="crateexprgroup-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprGroup`

##### `impl Spanned for ExprGroup`

- <span id="exprgroup-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprGroup`

- <span id="exprgroup-toowned-type-owned"></span>`type Owned = T`

- <span id="exprgroup-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprgroup-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprGroup`

- <span id="crateexprexprgroup-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprGroup`

- <span id="exprgroup-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprgroup-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprGroup`

- <span id="exprgroup-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprgroup-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/expr.rs:445-459`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L445-L459)*

An `if` expression with an optional `else` block: `if expr { ... }
else { ... }`.

The `else` branch expression may only be an `If` or `Block`
expression, not any of the other types of expression.

#### Implementations

- <span id="crateexprif-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprIf`

- <span id="exprif-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprIf`

- <span id="exprif-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprIf`

- <span id="exprif-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprIf`

- <span id="crateexprif-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprIf`

- <span id="exprif-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprIf`

- <span id="crateexprif-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprIf`

##### `impl<T> From for ExprIf`

- <span id="exprif-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprIf`

- <span id="crateexprif-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprIf`

- <span id="exprif-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprIf`

- <span id="crateexprexprif-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprIf`

- <span id="crateexprif-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprIf`

##### `impl Spanned for ExprIf`

- <span id="exprif-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprIf`

- <span id="exprif-toowned-type-owned"></span>`type Owned = T`

- <span id="exprif-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprif-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprIf`

- <span id="crateexprexprif-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprIf`

- <span id="exprif-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprif-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprIf`

- <span id="exprif-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprif-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprInfer`

```rust
struct ExprInfer {
    pub attrs: Vec<crate::attr::Attribute>,
    pub underscore_token: token::Underscore,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:472-479`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L472-L479)*

The inferred value of a const generic argument, denoted `_`.

#### Implementations

- <span id="crateexprinfer-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprInfer`

- <span id="exprinfer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprInfer`

- <span id="exprinfer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprInfer`

- <span id="exprinfer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprInfer`

- <span id="crateexprinfer-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprInfer`

- <span id="exprinfer-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprInfer`

- <span id="crateexprinfer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprInfer`

##### `impl<T> From for ExprInfer`

- <span id="exprinfer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprInfer`

- <span id="crateexprinfer-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprInfer`

- <span id="exprinfer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprInfer`

- <span id="crateexprexprinfer-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprInfer`

- <span id="crateexprinfer-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprInfer`

##### `impl Spanned for ExprInfer`

- <span id="exprinfer-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprInfer`

- <span id="exprinfer-toowned-type-owned"></span>`type Owned = T`

- <span id="exprinfer-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprinfer-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprInfer`

- <span id="crateexprexprinfer-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprInfer`

- <span id="exprinfer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprinfer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprInfer`

- <span id="exprinfer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprinfer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/expr.rs:481-491`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L481-L491)*

A `let` guard: `let Some(x) = opt`.

#### Implementations

- <span id="crateexprlet-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprLet`

- <span id="exprlet-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprLet`

- <span id="exprlet-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprLet`

- <span id="exprlet-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprLet`

- <span id="crateexprlet-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprLet`

- <span id="exprlet-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprLet`

- <span id="crateexprlet-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprLet`

##### `impl<T> From for ExprLet`

- <span id="exprlet-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprLet`

- <span id="crateexprlet-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprLet`

- <span id="exprlet-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprLet`

- <span id="crateexprexprlet-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprLet`

- <span id="crateexprlet-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprLet`

##### `impl Spanned for ExprLet`

- <span id="exprlet-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprLet`

- <span id="exprlet-toowned-type-owned"></span>`type Owned = T`

- <span id="exprlet-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprlet-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprLet`

- <span id="crateexprexprlet-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprLet`

- <span id="exprlet-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprlet-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprLet`

- <span id="exprlet-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprlet-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprLoop`

```rust
struct ExprLoop {
    pub attrs: Vec<crate::attr::Attribute>,
    pub label: Option<Label>,
    pub loop_token: token::Loop,
    pub body: crate::stmt::Block,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:502-511`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L502-L511)*

Conditionless loop: `loop { ... }`.

#### Implementations

- <span id="crateexprloop-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprLoop`

- <span id="exprloop-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprLoop`

- <span id="exprloop-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprLoop`

- <span id="exprloop-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprLoop`

- <span id="crateexprloop-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprLoop`

- <span id="exprloop-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprLoop`

- <span id="crateexprloop-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprLoop`

##### `impl<T> From for ExprLoop`

- <span id="exprloop-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprLoop`

- <span id="crateexprloop-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprLoop`

- <span id="exprloop-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprLoop`

- <span id="crateexprexprloop-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprLoop`

- <span id="crateexprloop-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprLoop`

##### `impl Spanned for ExprLoop`

- <span id="exprloop-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprLoop`

- <span id="exprloop-toowned-type-owned"></span>`type Owned = T`

- <span id="exprloop-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprloop-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprLoop`

- <span id="crateexprexprloop-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprLoop`

- <span id="exprloop-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprloop-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprLoop`

- <span id="exprloop-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprloop-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/expr.rs:522-532`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L522-L532)*

A `match` expression: `match n { Some(n) => {}, None => {} }`.

#### Implementations

- <span id="crateexprmatch-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprMatch`

- <span id="exprmatch-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprMatch`

- <span id="exprmatch-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprMatch`

- <span id="exprmatch-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprMatch`

- <span id="crateexprmatch-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprMatch`

- <span id="exprmatch-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprMatch`

- <span id="crateexprmatch-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprMatch`

##### `impl<T> From for ExprMatch`

- <span id="exprmatch-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprMatch`

- <span id="crateexprmatch-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprMatch`

- <span id="exprmatch-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprMatch`

- <span id="crateexprexprmatch-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprMatch`

- <span id="crateexprmatch-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprMatch`

##### `impl Spanned for ExprMatch`

- <span id="exprmatch-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprMatch`

- <span id="exprmatch-toowned-type-owned"></span>`type Owned = T`

- <span id="exprmatch-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprmatch-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprMatch`

- <span id="crateexprexprmatch-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprMatch`

- <span id="exprmatch-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprmatch-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprMatch`

- <span id="exprmatch-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprmatch-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprRange`

```rust
struct ExprRange {
    pub attrs: Vec<crate::attr::Attribute>,
    pub start: Option<Box<Expr>>,
    pub limits: RangeLimits,
    pub end: Option<Box<Expr>>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:571-580`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L571-L580)*

A range expression: `1..2`, `1..`, `..2`, `1..=2`, `..=2`.

#### Implementations

- <span id="crateexprrange-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprRange`

- <span id="exprrange-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprRange`

- <span id="exprrange-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprRange`

- <span id="exprrange-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprRange`

- <span id="crateexprrange-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprRange`

- <span id="exprrange-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprRange`

- <span id="crateexprrange-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprRange`

##### `impl<T> From for ExprRange`

- <span id="exprrange-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprRange`

- <span id="crateexprrange-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprRange`

- <span id="exprrange-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprRange`

- <span id="crateexprexprrange-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprRange`

- <span id="crateexprrange-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprRange`

##### `impl Spanned for ExprRange`

- <span id="exprrange-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprRange`

- <span id="exprrange-toowned-type-owned"></span>`type Owned = T`

- <span id="exprrange-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprrange-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprRange`

- <span id="crateexprexprrange-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprRange`

- <span id="exprrange-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprrange-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprRange`

- <span id="exprrange-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprrange-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/expr.rs:582-592`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L582-L592)*

Address-of operation: `&raw const place` or `&raw mut place`.

#### Implementations

- <span id="crateexprrawaddr-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprRawAddr`

- <span id="exprrawaddr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprRawAddr`

- <span id="exprrawaddr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprRawAddr`

- <span id="exprrawaddr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprRawAddr`

- <span id="crateexprrawaddr-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprRawAddr`

- <span id="exprrawaddr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprRawAddr`

- <span id="crateexprrawaddr-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprRawAddr`

##### `impl<T> From for ExprRawAddr`

- <span id="exprrawaddr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprRawAddr`

- <span id="crateexprrawaddr-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprRawAddr`

- <span id="exprrawaddr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprRawAddr`

- <span id="crateexprexprrawaddr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprRawAddr`

- <span id="crateexprrawaddr-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprRawAddr`

##### `impl Spanned for ExprRawAddr`

- <span id="exprrawaddr-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprRawAddr`

- <span id="exprrawaddr-toowned-type-owned"></span>`type Owned = T`

- <span id="exprrawaddr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprrawaddr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprRawAddr`

- <span id="crateexprexprrawaddr-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprRawAddr`

- <span id="exprrawaddr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprrawaddr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprRawAddr`

- <span id="exprrawaddr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprrawaddr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/expr.rs:605-615`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L605-L615)*

An array literal constructed from one repeated element: `[0u8; N]`.

#### Implementations

- <span id="crateexprrepeat-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprRepeat`

- <span id="exprrepeat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprRepeat`

- <span id="exprrepeat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprRepeat`

- <span id="exprrepeat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprRepeat`

- <span id="crateexprrepeat-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprRepeat`

- <span id="exprrepeat-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprRepeat`

- <span id="crateexprrepeat-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprRepeat`

##### `impl<T> From for ExprRepeat`

- <span id="exprrepeat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprRepeat`

- <span id="crateexprrepeat-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprRepeat`

- <span id="exprrepeat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprRepeat`

- <span id="crateexprexprrepeat-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprRepeat`

- <span id="crateexprrepeat-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprRepeat`

##### `impl Spanned for ExprRepeat`

- <span id="exprrepeat-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprRepeat`

- <span id="exprrepeat-toowned-type-owned"></span>`type Owned = T`

- <span id="exprrepeat-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprrepeat-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprRepeat`

- <span id="crateexprexprrepeat-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprRepeat`

- <span id="exprrepeat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprrepeat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprRepeat`

- <span id="exprrepeat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprrepeat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprReturn`

```rust
struct ExprReturn {
    pub attrs: Vec<crate::attr::Attribute>,
    pub return_token: token::Return,
    pub expr: Option<Box<Expr>>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:617-625`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L617-L625)*

A `return`, with an optional value to be returned.

#### Implementations

- <span id="crateexprreturn-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprReturn`

- <span id="exprreturn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprReturn`

- <span id="exprreturn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprReturn`

- <span id="exprreturn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprReturn`

- <span id="crateexprreturn-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprReturn`

- <span id="exprreturn-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprReturn`

- <span id="crateexprreturn-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprReturn`

##### `impl<T> From for ExprReturn`

- <span id="exprreturn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprReturn`

- <span id="crateexprreturn-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprReturn`

- <span id="exprreturn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprReturn`

- <span id="crateexprexprreturn-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprReturn`

- <span id="crateexprreturn-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprReturn`

##### `impl Spanned for ExprReturn`

- <span id="exprreturn-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprReturn`

- <span id="exprreturn-toowned-type-owned"></span>`type Owned = T`

- <span id="exprreturn-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprreturn-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprReturn`

- <span id="crateexprexprreturn-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprReturn`

- <span id="exprreturn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprreturn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprReturn`

- <span id="exprreturn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprreturn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprTry`

```rust
struct ExprTry {
    pub attrs: Vec<crate::attr::Attribute>,
    pub expr: Box<Expr>,
    pub question_token: token::Question,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:644-652`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L644-L652)*

A try-expression: `expr?`.

#### Implementations

- <span id="crateexprtry-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprTry`

- <span id="exprtry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprTry`

- <span id="exprtry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprTry`

- <span id="exprtry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprTry`

- <span id="crateexprtry-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprTry`

- <span id="exprtry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprTry`

- <span id="crateexprtry-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprTry`

##### `impl<T> From for ExprTry`

- <span id="exprtry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprTry`

- <span id="crateexprtry-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprTry`

- <span id="exprtry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprTry`

- <span id="crateexprexprtry-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprTry`

- <span id="crateexprtry-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprTry`

##### `impl Spanned for ExprTry`

- <span id="exprtry-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprTry`

- <span id="exprtry-toowned-type-owned"></span>`type Owned = T`

- <span id="exprtry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprtry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprTry`

- <span id="crateexprexprtry-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprTry`

- <span id="exprtry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprtry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprTry`

- <span id="exprtry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprtry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprTryBlock`

```rust
struct ExprTryBlock {
    pub attrs: Vec<crate::attr::Attribute>,
    pub try_token: token::Try,
    pub block: crate::stmt::Block,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:654-662`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L654-L662)*

A try block: `try { ... }`.

#### Implementations

- <span id="crateexprtryblock-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprTryBlock`

- <span id="exprtryblock-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprTryBlock`

- <span id="exprtryblock-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprTryBlock`

- <span id="exprtryblock-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprTryBlock`

- <span id="crateexprtryblock-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprTryBlock`

- <span id="exprtryblock-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprTryBlock`

- <span id="crateexprtryblock-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprTryBlock`

##### `impl<T> From for ExprTryBlock`

- <span id="exprtryblock-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprTryBlock`

- <span id="crateexprtryblock-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprTryBlock`

- <span id="exprtryblock-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprTryBlock`

- <span id="crateexprexprtryblock-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprTryBlock`

- <span id="crateexprtryblock-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprTryBlock`

##### `impl Spanned for ExprTryBlock`

- <span id="exprtryblock-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprTryBlock`

- <span id="exprtryblock-toowned-type-owned"></span>`type Owned = T`

- <span id="exprtryblock-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprtryblock-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprTryBlock`

- <span id="crateexprexprtryblock-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprTryBlock`

- <span id="exprtryblock-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprtryblock-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprTryBlock`

- <span id="exprtryblock-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprtryblock-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprTuple`

```rust
struct ExprTuple {
    pub attrs: Vec<crate::attr::Attribute>,
    pub paren_token: token::Paren,
    pub elems: crate::punctuated::Punctuated<Expr, token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:664-672`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L664-L672)*

A tuple expression: `(a, b, c, d)`.

#### Implementations

- <span id="crateexprtuple-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprTuple`

- <span id="exprtuple-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprTuple`

- <span id="exprtuple-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprTuple`

- <span id="exprtuple-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprTuple`

- <span id="crateexprtuple-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprTuple`

- <span id="exprtuple-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprTuple`

- <span id="crateexprtuple-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprTuple`

##### `impl<T> From for ExprTuple`

- <span id="exprtuple-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprTuple`

- <span id="crateexprtuple-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprTuple`

- <span id="exprtuple-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprTuple`

- <span id="crateexprexprtuple-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprTuple`

- <span id="crateexprtuple-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprTuple`

##### `impl Spanned for ExprTuple`

- <span id="exprtuple-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprTuple`

- <span id="exprtuple-toowned-type-owned"></span>`type Owned = T`

- <span id="exprtuple-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprtuple-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprTuple`

- <span id="crateexprexprtuple-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprTuple`

- <span id="exprtuple-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprtuple-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprTuple`

- <span id="exprtuple-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprtuple-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprUnsafe`

```rust
struct ExprUnsafe {
    pub attrs: Vec<crate::attr::Attribute>,
    pub unsafe_token: token::Unsafe,
    pub block: crate::stmt::Block,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:684-692`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L684-L692)*

An unsafe block: `unsafe { ... }`.

#### Implementations

- <span id="crateexprunsafe-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprUnsafe`

- <span id="exprunsafe-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprUnsafe`

- <span id="exprunsafe-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprUnsafe`

- <span id="exprunsafe-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprUnsafe`

- <span id="crateexprunsafe-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprUnsafe`

- <span id="exprunsafe-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprUnsafe`

- <span id="crateexprunsafe-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprUnsafe`

##### `impl<T> From for ExprUnsafe`

- <span id="exprunsafe-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprUnsafe`

- <span id="crateexprunsafe-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprUnsafe`

- <span id="exprunsafe-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprUnsafe`

- <span id="crateexprexprunsafe-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprUnsafe`

- <span id="crateexprunsafe-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprUnsafe`

##### `impl Spanned for ExprUnsafe`

- <span id="exprunsafe-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprUnsafe`

- <span id="exprunsafe-toowned-type-owned"></span>`type Owned = T`

- <span id="exprunsafe-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprunsafe-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprUnsafe`

- <span id="crateexprexprunsafe-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprUnsafe`

- <span id="exprunsafe-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprunsafe-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprUnsafe`

- <span id="exprunsafe-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprunsafe-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/expr.rs:694-704`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L694-L704)*

A while loop: `while expr { ... }`.

#### Implementations

- <span id="crateexprwhile-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprWhile`

- <span id="exprwhile-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprWhile`

- <span id="exprwhile-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprWhile`

- <span id="exprwhile-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprWhile`

- <span id="crateexprwhile-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprWhile`

- <span id="exprwhile-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprWhile`

- <span id="crateexprwhile-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprWhile`

##### `impl<T> From for ExprWhile`

- <span id="exprwhile-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprWhile`

- <span id="crateexprwhile-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprWhile`

- <span id="exprwhile-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprWhile`

- <span id="crateexprexprwhile-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprWhile`

- <span id="crateexprwhile-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprWhile`

##### `impl Spanned for ExprWhile`

- <span id="exprwhile-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprWhile`

- <span id="exprwhile-toowned-type-owned"></span>`type Owned = T`

- <span id="exprwhile-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprwhile-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprWhile`

- <span id="crateexprexprwhile-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprWhile`

- <span id="exprwhile-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprwhile-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprWhile`

- <span id="exprwhile-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprwhile-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprYield`

```rust
struct ExprYield {
    pub attrs: Vec<crate::attr::Attribute>,
    pub yield_token: token::Yield,
    pub expr: Option<Box<Expr>>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:706-714`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L706-L714)*

A yield expression: `yield expr`.

#### Implementations

- <span id="crateexpryield-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprYield`

- <span id="expryield-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprYield`

- <span id="expryield-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprYield`

- <span id="expryield-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprYield`

- <span id="crateexpryield-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprYield`

- <span id="expryield-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprYield`

- <span id="crateexpryield-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprYield`

##### `impl<T> From for ExprYield`

- <span id="expryield-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprYield`

- <span id="crateexpryield-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprYield`

- <span id="expryield-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprYield`

- <span id="crateexprexpryield-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprYield`

- <span id="crateexpryield-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprYield`

##### `impl Spanned for ExprYield`

- <span id="expryield-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprYield`

- <span id="expryield-toowned-type-owned"></span>`type Owned = T`

- <span id="expryield-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="expryield-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprYield`

- <span id="crateexprexpryield-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprYield`

- <span id="expryield-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="expryield-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprYield`

- <span id="expryield-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="expryield-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `File`

```rust
struct File {
    pub shebang: Option<String>,
    pub attrs: Vec<crate::attr::Attribute>,
    pub items: Vec<crate::item::Item>,
}
```

*Defined in [`syn-2.0.111/src/file.rs:4-84`](../../.source_1765521767/syn-2.0.111/src/file.rs#L4-L84)*

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

##### `impl Any for File`

- <span id="file-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for File`

- <span id="file-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for File`

- <span id="file-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::File`

- <span id="cratefile-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for File`

- <span id="file-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::File`

- <span id="cratefile-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::File`

##### `impl<T> From for File`

- <span id="file-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::File`

- <span id="cratefile-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for File`

- <span id="file-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::file::File`

- <span id="cratefilefile-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::File`

- <span id="cratefile-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for File`

##### `impl Spanned for File`

- <span id="file-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for File`

- <span id="file-toowned-type-owned"></span>`type Owned = T`

- <span id="file-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="file-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::file::File`

- <span id="cratefilefile-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for File`

- <span id="file-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="file-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for File`

- <span id="file-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="file-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BoundLifetimes`

```rust
struct BoundLifetimes {
    pub for_token: token::For,
    pub lt_token: token::Lt,
    pub lifetimes: crate::punctuated::Punctuated<GenericParam, token::Comma>,
    pub gt_token: token::Gt,
}
```

*Defined in [`syn-2.0.111/src/generics.rs:352-361`](../../.source_1765521767/syn-2.0.111/src/generics.rs#L352-L361)*

A set of bound lifetimes: `for<'a, 'b, 'c>`.

#### Trait Implementations

##### `impl Any for BoundLifetimes`

- <span id="boundlifetimes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BoundLifetimes`

- <span id="boundlifetimes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BoundLifetimes`

- <span id="boundlifetimes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::BoundLifetimes`

- <span id="crateboundlifetimes-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for BoundLifetimes`

- <span id="boundlifetimes-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::BoundLifetimes`

- <span id="crateboundlifetimes-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BoundLifetimes`

- <span id="boundlifetimes-default"></span>`fn default() -> Self`

##### `impl Eq for crate::BoundLifetimes`

##### `impl<T> From for BoundLifetimes`

- <span id="boundlifetimes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::BoundLifetimes`

- <span id="crateboundlifetimes-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for BoundLifetimes`

- <span id="boundlifetimes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::generics::BoundLifetimes`

- <span id="crategenericsboundlifetimes-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::BoundLifetimes`

- <span id="crateboundlifetimes-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for BoundLifetimes`

##### `impl Spanned for BoundLifetimes`

- <span id="boundlifetimes-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for BoundLifetimes`

- <span id="boundlifetimes-toowned-type-owned"></span>`type Owned = T`

- <span id="boundlifetimes-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="boundlifetimes-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::BoundLifetimes`

- <span id="crategenericsboundlifetimes-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for BoundLifetimes`

- <span id="boundlifetimes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="boundlifetimes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BoundLifetimes`

- <span id="boundlifetimes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="boundlifetimes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/generics.rs:80-92`](../../.source_1765521767/syn-2.0.111/src/generics.rs#L80-L92)*

A const generic parameter: `const LENGTH: usize`.

#### Trait Implementations

##### `impl Any for ConstParam`

- <span id="constparam-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ConstParam`

- <span id="constparam-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ConstParam`

- <span id="constparam-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ConstParam`

- <span id="crateconstparam-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ConstParam`

- <span id="constparam-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ConstParam`

- <span id="crateconstparam-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ConstParam`

##### `impl<T> From for ConstParam`

- <span id="constparam-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ConstParam`

- <span id="crateconstparam-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ConstParam`

- <span id="constparam-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::generics::ConstParam`

- <span id="crategenericsconstparam-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ConstParam`

- <span id="crateconstparam-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ConstParam`

##### `impl Spanned for ConstParam`

- <span id="constparam-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ConstParam`

- <span id="constparam-toowned-type-owned"></span>`type Owned = T`

- <span id="constparam-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="constparam-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::ConstParam`

- <span id="crategenericsconstparam-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ConstParam`

- <span id="constparam-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="constparam-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ConstParam`

- <span id="constparam-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="constparam-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Generics`

```rust
struct Generics {
    pub lt_token: Option<token::Lt>,
    pub params: crate::punctuated::Punctuated<GenericParam, token::Comma>,
    pub gt_token: Option<token::Gt>,
    pub where_clause: Option<WhereClause>,
}
```

*Defined in [`syn-2.0.111/src/generics.rs:15-32`](../../.source_1765521767/syn-2.0.111/src/generics.rs#L15-L32)*

Lifetimes and type parameters attached to a declaration of a function,
enum, trait, etc.

This struct represents two distinct optional syntactic elements,
[generic parameters] and [where clause]. In some locations of the
grammar, there may be other tokens in between these two things.



#### Implementations

- <span id="generics-lifetimes"></span>`fn lifetimes(&self) -> Lifetimes<'_>` — [`Lifetimes`](generics/index.md#lifetimes)

  Iterator over the lifetime parameters in `self.params`.

- <span id="generics-lifetimes-mut"></span>`fn lifetimes_mut(&mut self) -> LifetimesMut<'_>` — [`LifetimesMut`](generics/index.md#lifetimesmut)

  Iterator over the lifetime parameters in `self.params`.

- <span id="generics-type-params"></span>`fn type_params(&self) -> TypeParams<'_>` — [`TypeParams`](generics/index.md#typeparams)

  Iterator over the type parameters in `self.params`.

- <span id="generics-type-params-mut"></span>`fn type_params_mut(&mut self) -> TypeParamsMut<'_>` — [`TypeParamsMut`](generics/index.md#typeparamsmut)

  Iterator over the type parameters in `self.params`.

- <span id="generics-const-params"></span>`fn const_params(&self) -> ConstParams<'_>` — [`ConstParams`](generics/index.md#constparams)

  Iterator over the constant parameters in `self.params`.

- <span id="generics-const-params-mut"></span>`fn const_params_mut(&mut self) -> ConstParamsMut<'_>` — [`ConstParamsMut`](generics/index.md#constparamsmut)

  Iterator over the constant parameters in `self.params`.

- <span id="generics-make-where-clause"></span>`fn make_where_clause(&mut self) -> &mut WhereClause` — [`WhereClause`](generics/index.md#whereclause)

  Initializes an empty `where`-clause if there is not one present already.

- <span id="generics-split-for-impl"></span>`fn split_for_impl(&self) -> (ImplGenerics<'_>, TypeGenerics<'_>, Option<&WhereClause>)` — [`ImplGenerics`](generics/index.md#implgenerics), [`TypeGenerics`](generics/index.md#typegenerics), [`WhereClause`](generics/index.md#whereclause)

  Split a type's generics into the pieces required for impl'ing a trait

  for that type.

  

  ```rust

  use proc_macro2::{Span, Ident};

  use quote::quote;

  

  let generics: syn::Generics = Default::default();

  let name = Ident::new("MyType", Span::call_site());

  

  let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

  quote! {

      impl #impl_generics MyTrait for #name #ty_generics #where_clause {

          // ...

      }

  }

  ;

  ```

#### Trait Implementations

##### `impl Any for Generics`

- <span id="generics-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Generics`

- <span id="generics-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Generics`

- <span id="generics-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Generics`

- <span id="crategenerics-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Generics`

- <span id="generics-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Generics`

- <span id="crategenerics-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Generics`

- <span id="generics-default"></span>`fn default() -> Self`

##### `impl Eq for crate::Generics`

##### `impl<T> From for Generics`

- <span id="generics-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Generics`

- <span id="crategenerics-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Generics`

- <span id="generics-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::generics::Generics`

- <span id="crategenericsgenerics-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::Generics`

- <span id="crategenerics-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Generics`

##### `impl Spanned for Generics`

- <span id="generics-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Generics`

- <span id="generics-toowned-type-owned"></span>`type Owned = T`

- <span id="generics-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="generics-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::Generics`

- <span id="crategenericsgenerics-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Generics`

- <span id="generics-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="generics-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Generics`

- <span id="generics-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="generics-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LifetimeParam`

```rust
struct LifetimeParam {
    pub attrs: Vec<crate::attr::Attribute>,
    pub lifetime: crate::lifetime::Lifetime,
    pub colon_token: Option<token::Colon>,
    pub bounds: crate::punctuated::Punctuated<crate::lifetime::Lifetime, token::Plus>,
}
```

*Defined in [`syn-2.0.111/src/generics.rs:56-65`](../../.source_1765521767/syn-2.0.111/src/generics.rs#L56-L65)*

A lifetime definition: `'a: 'b + 'c + 'd`.

#### Implementations

- <span id="lifetimeparam-new"></span>`fn new(lifetime: Lifetime) -> Self` — [`Lifetime`](lifetime/index.md#lifetime)

#### Trait Implementations

##### `impl Any for LifetimeParam`

- <span id="lifetimeparam-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LifetimeParam`

- <span id="lifetimeparam-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LifetimeParam`

- <span id="lifetimeparam-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::LifetimeParam`

- <span id="cratelifetimeparam-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for LifetimeParam`

- <span id="lifetimeparam-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::LifetimeParam`

- <span id="cratelifetimeparam-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LifetimeParam`

##### `impl<T> From for LifetimeParam`

- <span id="lifetimeparam-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::LifetimeParam`

- <span id="cratelifetimeparam-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for LifetimeParam`

- <span id="lifetimeparam-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::generics::LifetimeParam`

- <span id="crategenericslifetimeparam-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::LifetimeParam`

- <span id="cratelifetimeparam-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for LifetimeParam`

##### `impl Spanned for LifetimeParam`

- <span id="lifetimeparam-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for LifetimeParam`

- <span id="lifetimeparam-toowned-type-owned"></span>`type Owned = T`

- <span id="lifetimeparam-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="lifetimeparam-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::LifetimeParam`

- <span id="crategenericslifetimeparam-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for LifetimeParam`

- <span id="lifetimeparam-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lifetimeparam-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LifetimeParam`

- <span id="lifetimeparam-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lifetimeparam-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PredicateLifetime`

```rust
struct PredicateLifetime {
    pub lifetime: crate::lifetime::Lifetime,
    pub colon_token: token::Colon,
    pub bounds: crate::punctuated::Punctuated<crate::lifetime::Lifetime, token::Plus>,
}
```

*Defined in [`syn-2.0.111/src/generics.rs:490-498`](../../.source_1765521767/syn-2.0.111/src/generics.rs#L490-L498)*

A lifetime predicate in a `where` clause: `'a: 'b + 'c`.

#### Trait Implementations

##### `impl Any for PredicateLifetime`

- <span id="predicatelifetime-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PredicateLifetime`

- <span id="predicatelifetime-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PredicateLifetime`

- <span id="predicatelifetime-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PredicateLifetime`

- <span id="cratepredicatelifetime-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PredicateLifetime`

- <span id="predicatelifetime-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PredicateLifetime`

- <span id="cratepredicatelifetime-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PredicateLifetime`

##### `impl<T> From for PredicateLifetime`

- <span id="predicatelifetime-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PredicateLifetime`

- <span id="cratepredicatelifetime-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PredicateLifetime`

- <span id="predicatelifetime-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::PredicateLifetime`

- <span id="cratepredicatelifetime-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PredicateLifetime`

##### `impl Spanned for PredicateLifetime`

- <span id="predicatelifetime-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PredicateLifetime`

- <span id="predicatelifetime-toowned-type-owned"></span>`type Owned = T`

- <span id="predicatelifetime-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="predicatelifetime-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::PredicateLifetime`

- <span id="crategenericspredicatelifetime-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PredicateLifetime`

- <span id="predicatelifetime-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="predicatelifetime-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PredicateLifetime`

- <span id="predicatelifetime-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="predicatelifetime-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PredicateType`

```rust
struct PredicateType {
    pub lifetimes: Option<BoundLifetimes>,
    pub bounded_ty: crate::ty::Type,
    pub colon_token: token::Colon,
    pub bounds: crate::punctuated::Punctuated<TypeParamBound, token::Plus>,
}
```

*Defined in [`syn-2.0.111/src/generics.rs:500-512`](../../.source_1765521767/syn-2.0.111/src/generics.rs#L500-L512)*

A type predicate in a `where` clause: `for<'c> Foo<'c>: Trait<'c>`.

#### Fields

- **`lifetimes`**: `Option<BoundLifetimes>`

  Any lifetimes from a `for` binding

- **`bounded_ty`**: `crate::ty::Type`

  The type being bounded

- **`bounds`**: `crate::punctuated::Punctuated<TypeParamBound, token::Plus>`

  Trait and lifetime bounds (`Clone+Send+'static`)

#### Trait Implementations

##### `impl Any for PredicateType`

- <span id="predicatetype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PredicateType`

- <span id="predicatetype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PredicateType`

- <span id="predicatetype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PredicateType`

- <span id="cratepredicatetype-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PredicateType`

- <span id="predicatetype-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PredicateType`

- <span id="cratepredicatetype-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PredicateType`

##### `impl<T> From for PredicateType`

- <span id="predicatetype-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PredicateType`

- <span id="cratepredicatetype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PredicateType`

- <span id="predicatetype-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::PredicateType`

- <span id="cratepredicatetype-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PredicateType`

##### `impl Spanned for PredicateType`

- <span id="predicatetype-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PredicateType`

- <span id="predicatetype-toowned-type-owned"></span>`type Owned = T`

- <span id="predicatetype-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="predicatetype-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::PredicateType`

- <span id="crategenericspredicatetype-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PredicateType`

- <span id="predicatetype-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="predicatetype-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PredicateType`

- <span id="predicatetype-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="predicatetype-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TraitBound`

```rust
struct TraitBound {
    pub paren_token: Option<token::Paren>,
    pub modifier: TraitBoundModifier,
    pub lifetimes: Option<BoundLifetimes>,
    pub path: crate::path::Path,
}
```

*Defined in [`syn-2.0.111/src/generics.rs:410-421`](../../.source_1765521767/syn-2.0.111/src/generics.rs#L410-L421)*

A trait used as a bound on a type parameter.

#### Fields

- **`lifetimes`**: `Option<BoundLifetimes>`

  The `for<'a>` in `for<'a> Foo<&'a T>`

- **`path`**: `crate::path::Path`

  The `Foo<&'a T>` in `for<'a> Foo<&'a T>`

#### Implementations

- <span id="crategenericstraitbound-do-parse"></span>`fn do_parse(input: ParseStream<'_>, allow_const: bool) -> Result<Option<Self>>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

#### Trait Implementations

##### `impl Any for TraitBound`

- <span id="traitbound-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TraitBound`

- <span id="traitbound-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TraitBound`

- <span id="traitbound-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TraitBound`

- <span id="cratetraitbound-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TraitBound`

- <span id="traitbound-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TraitBound`

- <span id="cratetraitbound-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitBound`

##### `impl<T> From for TraitBound`

- <span id="traitbound-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TraitBound`

- <span id="cratetraitbound-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TraitBound`

- <span id="traitbound-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::generics::TraitBound`

- <span id="crategenericstraitbound-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::TraitBound`

- <span id="cratetraitbound-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TraitBound`

##### `impl Spanned for TraitBound`

- <span id="traitbound-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TraitBound`

- <span id="traitbound-toowned-type-owned"></span>`type Owned = T`

- <span id="traitbound-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="traitbound-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::TraitBound`

- <span id="crategenericstraitbound-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TraitBound`

- <span id="traitbound-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="traitbound-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TraitBound`

- <span id="traitbound-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="traitbound-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/generics.rs:67-78`](../../.source_1765521767/syn-2.0.111/src/generics.rs#L67-L78)*

A generic type parameter: `T: Into<String>`.

#### Trait Implementations

##### `impl Any for TypeParam`

- <span id="typeparam-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeParam`

- <span id="typeparam-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeParam`

- <span id="typeparam-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypeParam`

- <span id="cratetypeparam-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeParam`

- <span id="typeparam-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypeParam`

- <span id="cratetypeparam-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeParam`

##### `impl<T> From for TypeParam`

- <span id="typeparam-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypeParam`

- <span id="cratetypeparam-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TypeParam`

- <span id="typeparam-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::generics::TypeParam`

- <span id="crategenericstypeparam-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::TypeParam`

- <span id="cratetypeparam-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeParam`

##### `impl Spanned for TypeParam`

- <span id="typeparam-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeParam`

- <span id="typeparam-toowned-type-owned"></span>`type Owned = T`

- <span id="typeparam-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typeparam-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::TypeParam`

- <span id="crategenericstypeparam-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypeParam`

- <span id="typeparam-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typeparam-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeParam`

- <span id="typeparam-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typeparam-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WhereClause`

```rust
struct WhereClause {
    pub where_token: token::Where,
    pub predicates: crate::punctuated::Punctuated<WherePredicate, token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/generics.rs:461-469`](../../.source_1765521767/syn-2.0.111/src/generics.rs#L461-L469)*

A `where` clause in a definition: `where T: Deserialize<'de>, D:
'static`.

#### Trait Implementations

##### `impl Any for WhereClause`

- <span id="whereclause-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WhereClause`

- <span id="whereclause-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WhereClause`

- <span id="whereclause-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::WhereClause`

- <span id="cratewhereclause-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for WhereClause`

- <span id="whereclause-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::WhereClause`

- <span id="cratewhereclause-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::WhereClause`

##### `impl<T> From for WhereClause`

- <span id="whereclause-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::WhereClause`

- <span id="cratewhereclause-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for WhereClause`

- <span id="whereclause-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::generics::WhereClause`

- <span id="crategenericswhereclause-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::WhereClause`

- <span id="cratewhereclause-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for WhereClause`

##### `impl Spanned for WhereClause`

- <span id="whereclause-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for WhereClause`

- <span id="whereclause-toowned-type-owned"></span>`type Owned = T`

- <span id="whereclause-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="whereclause-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::WhereClause`

- <span id="crategenericswhereclause-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for WhereClause`

- <span id="whereclause-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="whereclause-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WhereClause`

- <span id="whereclause-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="whereclause-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PreciseCapture`

```rust
struct PreciseCapture {
    pub use_token: token::Use,
    pub lt_token: token::Lt,
    pub params: crate::punctuated::Punctuated<CapturedParam, token::Comma>,
    pub gt_token: token::Gt,
}
```

*Defined in [`syn-2.0.111/src/generics.rs:433-443`](../../.source_1765521767/syn-2.0.111/src/generics.rs#L433-L443)*

Precise capturing bound: the 'use&lt;&hellip;&gt;' in `impl Trait +
use<'a, T>`.

#### Trait Implementations

##### `impl Any for PreciseCapture`

- <span id="precisecapture-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PreciseCapture`

- <span id="precisecapture-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PreciseCapture`

- <span id="precisecapture-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PreciseCapture`

- <span id="crateprecisecapture-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PreciseCapture`

- <span id="precisecapture-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PreciseCapture`

- <span id="crateprecisecapture-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PreciseCapture`

##### `impl<T> From for PreciseCapture`

- <span id="precisecapture-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PreciseCapture`

- <span id="crateprecisecapture-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PreciseCapture`

- <span id="precisecapture-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::generics::PreciseCapture`

- <span id="crategenericsprecisecapture-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::PreciseCapture`

- <span id="crateprecisecapture-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PreciseCapture`

##### `impl Spanned for PreciseCapture`

- <span id="precisecapture-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PreciseCapture`

- <span id="precisecapture-toowned-type-owned"></span>`type Owned = T`

- <span id="precisecapture-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="precisecapture-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::PreciseCapture`

- <span id="crategenericsprecisecapture-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PreciseCapture`

- <span id="precisecapture-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="precisecapture-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PreciseCapture`

- <span id="precisecapture-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="precisecapture-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ImplGenerics<'a>`

```rust
struct ImplGenerics<'a>(&'a Generics);
```

*Defined in [`syn-2.0.111/src/generics.rs:275`](../../.source_1765521767/syn-2.0.111/src/generics.rs#L275)*

Returned by `Generics::split_for_impl`.

#### Trait Implementations

##### `impl Any for ImplGenerics<'a>`

- <span id="implgenerics-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImplGenerics<'a>`

- <span id="implgenerics-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImplGenerics<'a>`

- <span id="implgenerics-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ImplGenerics<'a>`

- <span id="implgenerics-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ImplGenerics<'a>`

- <span id="implgenerics-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ImplGenerics<'a>`

- <span id="implgenerics-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ImplGenerics<'a>`

##### `impl<T> From for ImplGenerics<'a>`

- <span id="implgenerics-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for ImplGenerics<'a>`

- <span id="implgenerics-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<U> Into for ImplGenerics<'a>`

- <span id="implgenerics-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ImplGenerics<'a>`

- <span id="implgenerics-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ImplGenerics<'a>`

##### `impl Spanned for ImplGenerics<'a>`

- <span id="implgenerics-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ImplGenerics<'a>`

- <span id="implgenerics-toowned-type-owned"></span>`type Owned = T`

- <span id="implgenerics-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="implgenerics-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::ImplGenerics<'a>`

- <span id="crategenericsimplgenerics-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ImplGenerics<'a>`

- <span id="implgenerics-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="implgenerics-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImplGenerics<'a>`

- <span id="implgenerics-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="implgenerics-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Turbofish<'a>`

```rust
struct Turbofish<'a>(&'a Generics);
```

*Defined in [`syn-2.0.111/src/generics.rs:291`](../../.source_1765521767/syn-2.0.111/src/generics.rs#L291)*

Returned by `TypeGenerics::as_turbofish`.

#### Trait Implementations

##### `impl Any for Turbofish<'a>`

- <span id="turbofish-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Turbofish<'a>`

- <span id="turbofish-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Turbofish<'a>`

- <span id="turbofish-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Turbofish<'a>`

- <span id="turbofish-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Turbofish<'a>`

- <span id="turbofish-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Turbofish<'a>`

- <span id="turbofish-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Turbofish<'a>`

##### `impl<T> From for Turbofish<'a>`

- <span id="turbofish-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Turbofish<'a>`

- <span id="turbofish-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<U> Into for Turbofish<'a>`

- <span id="turbofish-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Turbofish<'a>`

- <span id="turbofish-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Turbofish<'a>`

##### `impl Spanned for Turbofish<'a>`

- <span id="turbofish-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Turbofish<'a>`

- <span id="turbofish-toowned-type-owned"></span>`type Owned = T`

- <span id="turbofish-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="turbofish-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::Turbofish<'a>`

- <span id="crategenericsturbofish-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Turbofish<'a>`

- <span id="turbofish-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="turbofish-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Turbofish<'a>`

- <span id="turbofish-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="turbofish-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypeGenerics<'a>`

```rust
struct TypeGenerics<'a>(&'a Generics);
```

*Defined in [`syn-2.0.111/src/generics.rs:283`](../../.source_1765521767/syn-2.0.111/src/generics.rs#L283)*

Returned by `Generics::split_for_impl`.

#### Implementations

- <span id="typegenerics-as-turbofish"></span>`fn as_turbofish(&self) -> Turbofish<'a>` — [`Turbofish`](generics/index.md#turbofish)

  Turn a type's generics like `<X, Y>` into a turbofish like `::<X, Y>`.

#### Trait Implementations

##### `impl Any for TypeGenerics<'a>`

- <span id="typegenerics-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeGenerics<'a>`

- <span id="typegenerics-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeGenerics<'a>`

- <span id="typegenerics-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TypeGenerics<'a>`

- <span id="typegenerics-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeGenerics<'a>`

- <span id="typegenerics-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for TypeGenerics<'a>`

- <span id="typegenerics-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TypeGenerics<'a>`

##### `impl<T> From for TypeGenerics<'a>`

- <span id="typegenerics-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for TypeGenerics<'a>`

- <span id="typegenerics-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<U> Into for TypeGenerics<'a>`

- <span id="typegenerics-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for TypeGenerics<'a>`

- <span id="typegenerics-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeGenerics<'a>`

##### `impl Spanned for TypeGenerics<'a>`

- <span id="typegenerics-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeGenerics<'a>`

- <span id="typegenerics-toowned-type-owned"></span>`type Owned = T`

- <span id="typegenerics-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typegenerics-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::TypeGenerics<'a>`

- <span id="crategenericstypegenerics-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypeGenerics<'a>`

- <span id="typegenerics-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typegenerics-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeGenerics<'a>`

- <span id="typegenerics-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typegenerics-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ForeignItemFn`

```rust
struct ForeignItemFn {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub sig: Signature,
    pub semi_token: token::Semi,
}
```

*Defined in [`syn-2.0.111/src/item.rs:542-551`](../../.source_1765521767/syn-2.0.111/src/item.rs#L542-L551)*

A foreign function in an `extern` block.

#### Implementations

- <span id="crateforeignitemfn-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ForeignItemFn`

- <span id="foreignitemfn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ForeignItemFn`

- <span id="foreignitemfn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ForeignItemFn`

- <span id="foreignitemfn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ForeignItemFn`

- <span id="crateforeignitemfn-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ForeignItemFn`

- <span id="foreignitemfn-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ForeignItemFn`

- <span id="crateforeignitemfn-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItemFn`

##### `impl<T> From for ForeignItemFn`

- <span id="foreignitemfn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ForeignItemFn`

- <span id="crateforeignitemfn-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ForeignItemFn`

- <span id="foreignitemfn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ForeignItemFn`

- <span id="crateitemforeignitemfn-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ForeignItemFn`

- <span id="crateforeignitemfn-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ForeignItemFn`

##### `impl Spanned for ForeignItemFn`

- <span id="foreignitemfn-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ForeignItemFn`

- <span id="foreignitemfn-toowned-type-owned"></span>`type Owned = T`

- <span id="foreignitemfn-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="foreignitemfn-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ForeignItemFn`

- <span id="crateitemforeignitemfn-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ForeignItemFn`

- <span id="foreignitemfn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="foreignitemfn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ForeignItemFn`

- <span id="foreignitemfn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="foreignitemfn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ForeignItemMacro`

```rust
struct ForeignItemMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<token::Semi>,
}
```

*Defined in [`syn-2.0.111/src/item.rs:581-589`](../../.source_1765521767/syn-2.0.111/src/item.rs#L581-L589)*

A macro invocation within an extern block.

#### Implementations

- <span id="crateforeignitemmacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ForeignItemMacro`

- <span id="foreignitemmacro-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ForeignItemMacro`

- <span id="foreignitemmacro-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ForeignItemMacro`

- <span id="foreignitemmacro-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ForeignItemMacro`

- <span id="crateforeignitemmacro-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ForeignItemMacro`

- <span id="foreignitemmacro-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ForeignItemMacro`

- <span id="crateforeignitemmacro-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItemMacro`

##### `impl<T> From for ForeignItemMacro`

- <span id="foreignitemmacro-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ForeignItemMacro`

- <span id="crateforeignitemmacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ForeignItemMacro`

- <span id="foreignitemmacro-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ForeignItemMacro`

- <span id="crateitemforeignitemmacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ForeignItemMacro`

- <span id="crateforeignitemmacro-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ForeignItemMacro`

##### `impl Spanned for ForeignItemMacro`

- <span id="foreignitemmacro-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ForeignItemMacro`

- <span id="foreignitemmacro-toowned-type-owned"></span>`type Owned = T`

- <span id="foreignitemmacro-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="foreignitemmacro-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ForeignItemMacro`

- <span id="crateitemforeignitemmacro-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ForeignItemMacro`

- <span id="foreignitemmacro-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="foreignitemmacro-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ForeignItemMacro`

- <span id="foreignitemmacro-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="foreignitemmacro-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:553-566`](../../.source_1765521767/syn-2.0.111/src/item.rs#L553-L566)*

A foreign static item in an `extern` block: `static ext: u8`.

#### Implementations

- <span id="crateforeignitemstatic-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ForeignItemStatic`

- <span id="foreignitemstatic-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ForeignItemStatic`

- <span id="foreignitemstatic-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ForeignItemStatic`

- <span id="foreignitemstatic-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ForeignItemStatic`

- <span id="crateforeignitemstatic-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ForeignItemStatic`

- <span id="foreignitemstatic-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ForeignItemStatic`

- <span id="crateforeignitemstatic-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItemStatic`

##### `impl<T> From for ForeignItemStatic`

- <span id="foreignitemstatic-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ForeignItemStatic`

- <span id="crateforeignitemstatic-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ForeignItemStatic`

- <span id="foreignitemstatic-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ForeignItemStatic`

- <span id="crateitemforeignitemstatic-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ForeignItemStatic`

- <span id="crateforeignitemstatic-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ForeignItemStatic`

##### `impl Spanned for ForeignItemStatic`

- <span id="foreignitemstatic-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ForeignItemStatic`

- <span id="foreignitemstatic-toowned-type-owned"></span>`type Owned = T`

- <span id="foreignitemstatic-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="foreignitemstatic-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ForeignItemStatic`

- <span id="crateitemforeignitemstatic-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ForeignItemStatic`

- <span id="foreignitemstatic-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="foreignitemstatic-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ForeignItemStatic`

- <span id="foreignitemstatic-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="foreignitemstatic-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:568-579`](../../.source_1765521767/syn-2.0.111/src/item.rs#L568-L579)*

A foreign type in an `extern` block: `type void`.

#### Implementations

- <span id="crateforeignitemtype-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ForeignItemType`

- <span id="foreignitemtype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ForeignItemType`

- <span id="foreignitemtype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ForeignItemType`

- <span id="foreignitemtype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ForeignItemType`

- <span id="crateforeignitemtype-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ForeignItemType`

- <span id="foreignitemtype-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ForeignItemType`

- <span id="crateforeignitemtype-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItemType`

##### `impl<T> From for ForeignItemType`

- <span id="foreignitemtype-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ForeignItemType`

- <span id="crateforeignitemtype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ForeignItemType`

- <span id="foreignitemtype-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ForeignItemType`

- <span id="crateitemforeignitemtype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ForeignItemType`

- <span id="crateforeignitemtype-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ForeignItemType`

##### `impl Spanned for ForeignItemType`

- <span id="foreignitemtype-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ForeignItemType`

- <span id="foreignitemtype-toowned-type-owned"></span>`type Owned = T`

- <span id="foreignitemtype-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="foreignitemtype-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ForeignItemType`

- <span id="crateitemforeignitemtype-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ForeignItemType`

- <span id="foreignitemtype-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="foreignitemtype-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ForeignItemType`

- <span id="foreignitemtype-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="foreignitemtype-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:734-750`](../../.source_1765521767/syn-2.0.111/src/item.rs#L734-L750)*

An associated constant within an impl block.

#### Implementations

- <span id="crateimplitemconst-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ImplItemConst`

- <span id="implitemconst-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImplItemConst`

- <span id="implitemconst-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImplItemConst`

- <span id="implitemconst-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ImplItemConst`

- <span id="crateimplitemconst-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ImplItemConst`

- <span id="implitemconst-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ImplItemConst`

- <span id="crateimplitemconst-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItemConst`

##### `impl<T> From for ImplItemConst`

- <span id="implitemconst-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ImplItemConst`

- <span id="crateimplitemconst-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ImplItemConst`

- <span id="implitemconst-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ImplItemConst`

- <span id="crateitemimplitemconst-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ImplItemConst`

- <span id="crateimplitemconst-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ImplItemConst`

##### `impl Spanned for ImplItemConst`

- <span id="implitemconst-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ImplItemConst`

- <span id="implitemconst-toowned-type-owned"></span>`type Owned = T`

- <span id="implitemconst-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="implitemconst-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ImplItemConst`

- <span id="crateitemimplitemconst-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ImplItemConst`

- <span id="implitemconst-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="implitemconst-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImplItemConst`

- <span id="implitemconst-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="implitemconst-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:752-762`](../../.source_1765521767/syn-2.0.111/src/item.rs#L752-L762)*

An associated function within an impl block.

#### Implementations

- <span id="crateimplitemfn-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ImplItemFn`

- <span id="implitemfn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImplItemFn`

- <span id="implitemfn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImplItemFn`

- <span id="implitemfn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ImplItemFn`

- <span id="crateimplitemfn-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ImplItemFn`

- <span id="implitemfn-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ImplItemFn`

- <span id="crateimplitemfn-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItemFn`

##### `impl<T> From for ImplItemFn`

- <span id="implitemfn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ImplItemFn`

- <span id="crateimplitemfn-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ImplItemFn`

- <span id="implitemfn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ImplItemFn`

- <span id="crateitemimplitemfn-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ImplItemFn`

- <span id="crateimplitemfn-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ImplItemFn`

##### `impl Spanned for ImplItemFn`

- <span id="implitemfn-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ImplItemFn`

- <span id="implitemfn-toowned-type-owned"></span>`type Owned = T`

- <span id="implitemfn-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="implitemfn-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ImplItemFn`

- <span id="crateitemimplitemfn-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ImplItemFn`

- <span id="implitemfn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="implitemfn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImplItemFn`

- <span id="implitemfn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="implitemfn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ImplItemMacro`

```rust
struct ImplItemMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<token::Semi>,
}
```

*Defined in [`syn-2.0.111/src/item.rs:780-788`](../../.source_1765521767/syn-2.0.111/src/item.rs#L780-L788)*

A macro invocation within an impl block.

#### Implementations

- <span id="crateimplitemmacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ImplItemMacro`

- <span id="implitemmacro-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImplItemMacro`

- <span id="implitemmacro-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImplItemMacro`

- <span id="implitemmacro-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ImplItemMacro`

- <span id="crateimplitemmacro-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ImplItemMacro`

- <span id="implitemmacro-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ImplItemMacro`

- <span id="crateimplitemmacro-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItemMacro`

##### `impl<T> From for ImplItemMacro`

- <span id="implitemmacro-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ImplItemMacro`

- <span id="crateimplitemmacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ImplItemMacro`

- <span id="implitemmacro-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ImplItemMacro`

- <span id="crateitemimplitemmacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ImplItemMacro`

- <span id="crateimplitemmacro-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ImplItemMacro`

##### `impl Spanned for ImplItemMacro`

- <span id="implitemmacro-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ImplItemMacro`

- <span id="implitemmacro-toowned-type-owned"></span>`type Owned = T`

- <span id="implitemmacro-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="implitemmacro-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ImplItemMacro`

- <span id="crateitemimplitemmacro-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ImplItemMacro`

- <span id="implitemmacro-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="implitemmacro-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImplItemMacro`

- <span id="implitemmacro-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="implitemmacro-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:764-778`](../../.source_1765521767/syn-2.0.111/src/item.rs#L764-L778)*

An associated type within an impl block.

#### Implementations

- <span id="crateimplitemtype-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ImplItemType`

- <span id="implitemtype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImplItemType`

- <span id="implitemtype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImplItemType`

- <span id="implitemtype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ImplItemType`

- <span id="crateimplitemtype-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ImplItemType`

- <span id="implitemtype-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ImplItemType`

- <span id="crateimplitemtype-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItemType`

##### `impl<T> From for ImplItemType`

- <span id="implitemtype-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ImplItemType`

- <span id="crateimplitemtype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ImplItemType`

- <span id="implitemtype-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ImplItemType`

- <span id="crateitemimplitemtype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ImplItemType`

- <span id="crateimplitemtype-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ImplItemType`

##### `impl Spanned for ImplItemType`

- <span id="implitemtype-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ImplItemType`

- <span id="implitemtype-toowned-type-owned"></span>`type Owned = T`

- <span id="implitemtype-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="implitemtype-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ImplItemType`

- <span id="crateitemimplitemtype-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ImplItemType`

- <span id="implitemtype-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="implitemtype-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImplItemType`

- <span id="implitemtype-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="implitemtype-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:101-116`](../../.source_1765521767/syn-2.0.111/src/item.rs#L101-L116)*

A constant item: `const MAX: u16 = 65535`.

#### Implementations

- <span id="crateitemconst-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemConst`

- <span id="itemconst-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemConst`

- <span id="itemconst-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemConst`

- <span id="itemconst-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemConst`

- <span id="crateitemconst-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemConst`

- <span id="itemconst-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemConst`

- <span id="crateitemconst-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemConst`

##### `impl<T> From for ItemConst`

- <span id="itemconst-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemConst`

- <span id="crateitemconst-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemConst`

- <span id="itemconst-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemConst`

- <span id="crateitemitemconst-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ItemConst`

- <span id="crateitemconst-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemConst`

##### `impl Spanned for ItemConst`

- <span id="itemconst-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemConst`

- <span id="itemconst-toowned-type-owned"></span>`type Owned = T`

- <span id="itemconst-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemconst-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemConst`

- <span id="crateitemitemconst-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemConst`

- <span id="itemconst-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemconst-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemConst`

- <span id="itemconst-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemconst-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:118-130`](../../.source_1765521767/syn-2.0.111/src/item.rs#L118-L130)*

An enum definition: `enum Foo<A, B> { A(A), B(B) }`.

#### Implementations

- <span id="crateitemenum-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemEnum`

- <span id="itemenum-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemEnum`

- <span id="itemenum-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemEnum`

- <span id="itemenum-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemEnum`

- <span id="crateitemenum-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemEnum`

- <span id="itemenum-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemEnum`

- <span id="crateitemenum-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemEnum`

##### `impl<T> From for ItemEnum`

- <span id="itemenum-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemEnum`

- <span id="crateitemenum-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemEnum`

- <span id="itemenum-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemEnum`

- <span id="crateitemitemenum-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ItemEnum`

- <span id="crateitemenum-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemEnum`

##### `impl Spanned for ItemEnum`

- <span id="itemenum-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemEnum`

- <span id="itemenum-toowned-type-owned"></span>`type Owned = T`

- <span id="itemenum-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemenum-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemEnum`

- <span id="crateitemitemenum-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemEnum`

- <span id="itemenum-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemenum-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemEnum`

- <span id="itemenum-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemenum-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:132-144`](../../.source_1765521767/syn-2.0.111/src/item.rs#L132-L144)*

An `extern crate` item: `extern crate serde`.

#### Implementations

- <span id="crateitemexterncrate-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemExternCrate`

- <span id="itemexterncrate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemExternCrate`

- <span id="itemexterncrate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemExternCrate`

- <span id="itemexterncrate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemExternCrate`

- <span id="crateitemexterncrate-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemExternCrate`

- <span id="itemexterncrate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemExternCrate`

- <span id="crateitemexterncrate-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemExternCrate`

##### `impl<T> From for ItemExternCrate`

- <span id="itemexterncrate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemExternCrate`

- <span id="crateitemexterncrate-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemExternCrate`

- <span id="itemexterncrate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemExternCrate`

- <span id="crateitemitemexterncrate-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ItemExternCrate`

- <span id="crateitemexterncrate-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemExternCrate`

##### `impl Spanned for ItemExternCrate`

- <span id="itemexterncrate-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemExternCrate`

- <span id="itemexterncrate-toowned-type-owned"></span>`type Owned = T`

- <span id="itemexterncrate-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemexterncrate-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemExternCrate`

- <span id="crateitemitemexterncrate-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemExternCrate`

- <span id="itemexterncrate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemexterncrate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemExternCrate`

- <span id="itemexterncrate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemexterncrate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ItemFn`

```rust
struct ItemFn {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub sig: Signature,
    pub block: Box<crate::stmt::Block>,
}
```

*Defined in [`syn-2.0.111/src/item.rs:146-155`](../../.source_1765521767/syn-2.0.111/src/item.rs#L146-L155)*

A free-standing function: `fn process(n: usize) -> Result<()> { ... }`.

#### Implementations

- <span id="crateitemfn-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemFn`

- <span id="itemfn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemFn`

- <span id="itemfn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemFn`

- <span id="itemfn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemFn`

- <span id="crateitemfn-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemFn`

- <span id="itemfn-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemFn`

- <span id="crateitemfn-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemFn`

##### `impl<T> From for ItemFn`

- <span id="itemfn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemFn`

- <span id="crateitemfn-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemFn`

- <span id="itemfn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemFn`

- <span id="crateitemitemfn-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ItemFn`

- <span id="crateitemfn-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemFn`

##### `impl Spanned for ItemFn`

- <span id="itemfn-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemFn`

- <span id="itemfn-toowned-type-owned"></span>`type Owned = T`

- <span id="itemfn-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemfn-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemFn`

- <span id="crateitemitemfn-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemFn`

- <span id="itemfn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemfn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemFn`

- <span id="itemfn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemfn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:157-167`](../../.source_1765521767/syn-2.0.111/src/item.rs#L157-L167)*

A block of foreign items: `extern "C" { ... }`.

#### Implementations

- <span id="crateitemforeignmod-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemForeignMod`

- <span id="itemforeignmod-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemForeignMod`

- <span id="itemforeignmod-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemForeignMod`

- <span id="itemforeignmod-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemForeignMod`

- <span id="crateitemforeignmod-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemForeignMod`

- <span id="itemforeignmod-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemForeignMod`

- <span id="crateitemforeignmod-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemForeignMod`

##### `impl<T> From for ItemForeignMod`

- <span id="itemforeignmod-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemForeignMod`

- <span id="crateitemforeignmod-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemForeignMod`

- <span id="itemforeignmod-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemForeignMod`

- <span id="crateitemitemforeignmod-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ItemForeignMod`

- <span id="crateitemforeignmod-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemForeignMod`

##### `impl Spanned for ItemForeignMod`

- <span id="itemforeignmod-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemForeignMod`

- <span id="itemforeignmod-toowned-type-owned"></span>`type Owned = T`

- <span id="itemforeignmod-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemforeignmod-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemForeignMod`

- <span id="crateitemitemforeignmod-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemForeignMod`

- <span id="itemforeignmod-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemforeignmod-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemForeignMod`

- <span id="itemforeignmod-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemforeignmod-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:169-186`](../../.source_1765521767/syn-2.0.111/src/item.rs#L169-L186)*

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

##### `impl Any for ItemImpl`

- <span id="itemimpl-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemImpl`

- <span id="itemimpl-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemImpl`

- <span id="itemimpl-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemImpl`

- <span id="crateitemimpl-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemImpl`

- <span id="itemimpl-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemImpl`

- <span id="crateitemimpl-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemImpl`

##### `impl<T> From for ItemImpl`

- <span id="itemimpl-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemImpl`

- <span id="crateitemimpl-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemImpl`

- <span id="itemimpl-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemImpl`

- <span id="crateitemitemimpl-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ItemImpl`

- <span id="crateitemimpl-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemImpl`

##### `impl Spanned for ItemImpl`

- <span id="itemimpl-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemImpl`

- <span id="itemimpl-toowned-type-owned"></span>`type Owned = T`

- <span id="itemimpl-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemimpl-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemImpl`

- <span id="crateitemitemimpl-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemImpl`

- <span id="itemimpl-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemimpl-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemImpl`

- <span id="itemimpl-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemimpl-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ItemMacro`

```rust
struct ItemMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub ident: Option<crate::ident::Ident>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<token::Semi>,
}
```

*Defined in [`syn-2.0.111/src/item.rs:188-198`](../../.source_1765521767/syn-2.0.111/src/item.rs#L188-L198)*

A macro invocation, which includes `macro_rules!` definitions.

#### Fields

- **`ident`**: `Option<crate::ident::Ident>`

  The `example` in `macro_rules! example { ... }`.

#### Implementations

- <span id="crateitemmacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemMacro`

- <span id="itemmacro-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemMacro`

- <span id="itemmacro-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemMacro`

- <span id="itemmacro-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemMacro`

- <span id="crateitemmacro-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemMacro`

- <span id="itemmacro-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemMacro`

- <span id="crateitemmacro-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemMacro`

##### `impl<T> From for ItemMacro`

- <span id="itemmacro-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemMacro`

- <span id="crateitemmacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemMacro`

- <span id="itemmacro-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemMacro`

- <span id="crateitemitemmacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ItemMacro`

- <span id="crateitemmacro-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemMacro`

##### `impl Spanned for ItemMacro`

- <span id="itemmacro-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemMacro`

- <span id="itemmacro-toowned-type-owned"></span>`type Owned = T`

- <span id="itemmacro-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemmacro-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemMacro`

- <span id="crateitemitemmacro-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemMacro`

- <span id="itemmacro-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemmacro-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemMacro`

- <span id="itemmacro-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemmacro-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:200-212`](../../.source_1765521767/syn-2.0.111/src/item.rs#L200-L212)*

A module or module declaration: `mod m` or `mod m { ... }`.

#### Implementations

- <span id="crateitemmod-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemMod`

- <span id="itemmod-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemMod`

- <span id="itemmod-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemMod`

- <span id="itemmod-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemMod`

- <span id="crateitemmod-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemMod`

- <span id="itemmod-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemMod`

- <span id="crateitemmod-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemMod`

##### `impl<T> From for ItemMod`

- <span id="itemmod-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemMod`

- <span id="crateitemmod-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemMod`

- <span id="itemmod-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemMod`

- <span id="crateitemitemmod-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ItemMod`

- <span id="crateitemmod-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemMod`

##### `impl Spanned for ItemMod`

- <span id="itemmod-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemMod`

- <span id="itemmod-toowned-type-owned"></span>`type Owned = T`

- <span id="itemmod-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemmod-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemMod`

- <span id="crateitemitemmod-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemMod`

- <span id="itemmod-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemmod-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemMod`

- <span id="itemmod-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemmod-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:214-229`](../../.source_1765521767/syn-2.0.111/src/item.rs#L214-L229)*

A static item: `static BIKE: Shed = Shed(42)`.

#### Implementations

- <span id="crateitemstatic-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemStatic`

- <span id="itemstatic-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemStatic`

- <span id="itemstatic-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemStatic`

- <span id="itemstatic-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemStatic`

- <span id="crateitemstatic-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemStatic`

- <span id="itemstatic-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemStatic`

- <span id="crateitemstatic-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemStatic`

##### `impl<T> From for ItemStatic`

- <span id="itemstatic-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemStatic`

- <span id="crateitemstatic-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemStatic`

- <span id="itemstatic-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemStatic`

- <span id="crateitemitemstatic-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ItemStatic`

- <span id="crateitemstatic-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemStatic`

##### `impl Spanned for ItemStatic`

- <span id="itemstatic-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemStatic`

- <span id="itemstatic-toowned-type-owned"></span>`type Owned = T`

- <span id="itemstatic-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemstatic-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemStatic`

- <span id="crateitemitemstatic-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemStatic`

- <span id="itemstatic-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemstatic-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemStatic`

- <span id="itemstatic-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemstatic-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:231-243`](../../.source_1765521767/syn-2.0.111/src/item.rs#L231-L243)*

A struct definition: `struct Foo<A> { x: A }`.

#### Implementations

- <span id="crateitemstruct-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemStruct`

- <span id="itemstruct-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemStruct`

- <span id="itemstruct-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemStruct`

- <span id="itemstruct-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemStruct`

- <span id="crateitemstruct-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemStruct`

- <span id="itemstruct-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemStruct`

- <span id="crateitemstruct-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemStruct`

##### `impl<T> From for ItemStruct`

- <span id="itemstruct-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemStruct`

- <span id="crateitemstruct-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemStruct`

- <span id="itemstruct-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemStruct`

- <span id="crateitemitemstruct-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ItemStruct`

- <span id="crateitemstruct-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemStruct`

##### `impl Spanned for ItemStruct`

- <span id="itemstruct-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemStruct`

- <span id="itemstruct-toowned-type-owned"></span>`type Owned = T`

- <span id="itemstruct-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemstruct-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemStruct`

- <span id="crateitemitemstruct-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemStruct`

- <span id="itemstruct-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemstruct-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemStruct`

- <span id="itemstruct-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemstruct-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:245-262`](../../.source_1765521767/syn-2.0.111/src/item.rs#L245-L262)*

A trait definition: `pub trait Iterator { ... }`.

#### Implementations

- <span id="crateitemtrait-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemTrait`

- <span id="itemtrait-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemTrait`

- <span id="itemtrait-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemTrait`

- <span id="itemtrait-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemTrait`

- <span id="crateitemtrait-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemTrait`

- <span id="itemtrait-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemTrait`

- <span id="crateitemtrait-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemTrait`

##### `impl<T> From for ItemTrait`

- <span id="itemtrait-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemTrait`

- <span id="crateitemtrait-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemTrait`

- <span id="itemtrait-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemTrait`

- <span id="crateitemitemtrait-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ItemTrait`

- <span id="crateitemtrait-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemTrait`

##### `impl Spanned for ItemTrait`

- <span id="itemtrait-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemTrait`

- <span id="itemtrait-toowned-type-owned"></span>`type Owned = T`

- <span id="itemtrait-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemtrait-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemTrait`

- <span id="crateitemitemtrait-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemTrait`

- <span id="itemtrait-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemtrait-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemTrait`

- <span id="itemtrait-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemtrait-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:264-277`](../../.source_1765521767/syn-2.0.111/src/item.rs#L264-L277)*

A trait alias: `pub trait SharableIterator = Iterator + Sync`.

#### Implementations

- <span id="crateitemtraitalias-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemTraitAlias`

- <span id="itemtraitalias-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemTraitAlias`

- <span id="itemtraitalias-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemTraitAlias`

- <span id="itemtraitalias-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemTraitAlias`

- <span id="crateitemtraitalias-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemTraitAlias`

- <span id="itemtraitalias-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemTraitAlias`

- <span id="crateitemtraitalias-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemTraitAlias`

##### `impl<T> From for ItemTraitAlias`

- <span id="itemtraitalias-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemTraitAlias`

- <span id="crateitemtraitalias-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemTraitAlias`

- <span id="itemtraitalias-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemTraitAlias`

- <span id="crateitemitemtraitalias-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ItemTraitAlias`

- <span id="crateitemtraitalias-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemTraitAlias`

##### `impl Spanned for ItemTraitAlias`

- <span id="itemtraitalias-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemTraitAlias`

- <span id="itemtraitalias-toowned-type-owned"></span>`type Owned = T`

- <span id="itemtraitalias-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemtraitalias-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemTraitAlias`

- <span id="crateitemitemtraitalias-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemTraitAlias`

- <span id="itemtraitalias-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemtraitalias-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemTraitAlias`

- <span id="itemtraitalias-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemtraitalias-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:279-292`](../../.source_1765521767/syn-2.0.111/src/item.rs#L279-L292)*

A type alias: `type Result<T> = std::result::Result<T, MyError>`.

#### Implementations

- <span id="crateitemtype-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemType`

- <span id="itemtype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemType`

- <span id="itemtype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemType`

- <span id="itemtype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemType`

- <span id="crateitemtype-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemType`

- <span id="itemtype-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemType`

- <span id="crateitemtype-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemType`

##### `impl<T> From for ItemType`

- <span id="itemtype-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemType`

- <span id="crateitemtype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemType`

- <span id="itemtype-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemType`

- <span id="crateitemitemtype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ItemType`

- <span id="crateitemtype-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemType`

##### `impl Spanned for ItemType`

- <span id="itemtype-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemType`

- <span id="itemtype-toowned-type-owned"></span>`type Owned = T`

- <span id="itemtype-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemtype-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemType`

- <span id="crateitemitemtype-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemType`

- <span id="itemtype-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemtype-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemType`

- <span id="itemtype-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemtype-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:294-305`](../../.source_1765521767/syn-2.0.111/src/item.rs#L294-L305)*

A union definition: `union Foo<A, B> { x: A, y: B }`.

#### Implementations

- <span id="crateitemunion-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemUnion`

- <span id="itemunion-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemUnion`

- <span id="itemunion-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemUnion`

- <span id="itemunion-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemUnion`

- <span id="crateitemunion-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemUnion`

- <span id="itemunion-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemUnion`

- <span id="crateitemunion-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemUnion`

##### `impl<T> From for ItemUnion`

- <span id="itemunion-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemUnion`

- <span id="crateitemunion-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemUnion`

- <span id="itemunion-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemUnion`

- <span id="crateitemitemunion-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ItemUnion`

- <span id="crateitemunion-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemUnion`

##### `impl Spanned for ItemUnion`

- <span id="itemunion-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemUnion`

- <span id="itemunion-toowned-type-owned"></span>`type Owned = T`

- <span id="itemunion-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemunion-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemUnion`

- <span id="crateitemitemunion-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemUnion`

- <span id="itemunion-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemunion-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemUnion`

- <span id="itemunion-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemunion-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:307-318`](../../.source_1765521767/syn-2.0.111/src/item.rs#L307-L318)*

A use declaration: `use std::collections::HashMap`.

#### Implementations

- <span id="crateitemuse-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemUse`

- <span id="itemuse-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemUse`

- <span id="itemuse-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemUse`

- <span id="itemuse-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemUse`

- <span id="crateitemuse-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemUse`

- <span id="itemuse-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemUse`

- <span id="crateitemuse-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemUse`

##### `impl<T> From for ItemUse`

- <span id="itemuse-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemUse`

- <span id="crateitemuse-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemUse`

- <span id="itemuse-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemUse`

- <span id="crateitemitemuse-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ItemUse`

- <span id="crateitemuse-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemUse`

##### `impl Spanned for ItemUse`

- <span id="itemuse-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemUse`

- <span id="itemuse-toowned-type-owned"></span>`type Owned = T`

- <span id="itemuse-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemuse-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemUse`

- <span id="crateitemitemuse-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemUse`

- <span id="itemuse-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemuse-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemUse`

- <span id="itemuse-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemuse-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:832-849`](../../.source_1765521767/syn-2.0.111/src/item.rs#L832-L849)*

The `self` argument of an associated method.

If `colon_token` is present, the receiver is written with an explicit
type such as `self: Box<Self>`. If `colon_token` is absent, the receiver
is written in shorthand such as `self` or `&self` or `&mut self`. In the
shorthand case, the type in `ty` is reconstructed as one of `Self`,
`&Self`, or `&mut Self`.

#### Implementations

- <span id="receiver-lifetime"></span>`fn lifetime(&self) -> Option<&Lifetime>` — [`Lifetime`](lifetime/index.md#lifetime)

#### Trait Implementations

##### `impl Any for Receiver`

- <span id="receiver-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Receiver`

- <span id="receiver-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Receiver`

- <span id="receiver-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Receiver`

- <span id="cratereceiver-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Receiver`

- <span id="receiver-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Receiver`

- <span id="cratereceiver-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Receiver`

##### `impl<T> From for Receiver`

- <span id="receiver-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Receiver`

- <span id="cratereceiver-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Receiver`

- <span id="receiver-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::Receiver`

- <span id="crateitemreceiver-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::Receiver`

- <span id="cratereceiver-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Receiver`

##### `impl Spanned for Receiver`

- <span id="receiver-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Receiver`

- <span id="receiver-toowned-type-owned"></span>`type Owned = T`

- <span id="receiver-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="receiver-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::Receiver`

- <span id="crateitemreceiver-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Receiver`

- <span id="receiver-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="receiver-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Receiver`

- <span id="receiver-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="receiver-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:790-807`](../../.source_1765521767/syn-2.0.111/src/item.rs#L790-L807)*

A function signature in a trait or implementation: `unsafe fn
initialize(&self)`.

#### Implementations

- <span id="signature-receiver"></span>`fn receiver(&self) -> Option<&Receiver>` — [`Receiver`](item/index.md#receiver)

  A method's `self` receiver, such as `&self` or `self: Box<Self>`.

#### Trait Implementations

##### `impl Any for Signature`

- <span id="signature-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Signature`

- <span id="signature-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Signature`

- <span id="signature-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Signature`

- <span id="cratesignature-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Signature`

- <span id="signature-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Signature`

- <span id="cratesignature-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Signature`

##### `impl<T> From for Signature`

- <span id="signature-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Signature`

- <span id="cratesignature-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Signature`

- <span id="signature-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::Signature`

- <span id="crateitemsignature-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::Signature`

- <span id="cratesignature-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Signature`

##### `impl Spanned for Signature`

- <span id="signature-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Signature`

- <span id="signature-toowned-type-owned"></span>`type Owned = T`

- <span id="signature-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="signature-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::Signature`

- <span id="crateitemsignature-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Signature`

- <span id="signature-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="signature-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Signature`

- <span id="signature-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="signature-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:637-650`](../../.source_1765521767/syn-2.0.111/src/item.rs#L637-L650)*

An associated constant within the definition of a trait.

#### Implementations

- <span id="cratetraititemconst-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TraitItemConst`

- <span id="traititemconst-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TraitItemConst`

- <span id="traititemconst-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TraitItemConst`

- <span id="traititemconst-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TraitItemConst`

- <span id="cratetraititemconst-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TraitItemConst`

- <span id="traititemconst-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TraitItemConst`

- <span id="cratetraititemconst-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItemConst`

##### `impl<T> From for TraitItemConst`

- <span id="traititemconst-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TraitItemConst`

- <span id="cratetraititemconst-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TraitItemConst`

- <span id="traititemconst-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::TraitItemConst`

- <span id="crateitemtraititemconst-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::TraitItemConst`

- <span id="cratetraititemconst-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TraitItemConst`

##### `impl Spanned for TraitItemConst`

- <span id="traititemconst-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TraitItemConst`

- <span id="traititemconst-toowned-type-owned"></span>`type Owned = T`

- <span id="traititemconst-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="traititemconst-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::TraitItemConst`

- <span id="crateitemtraititemconst-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TraitItemConst`

- <span id="traititemconst-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="traititemconst-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TraitItemConst`

- <span id="traititemconst-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="traititemconst-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TraitItemFn`

```rust
struct TraitItemFn {
    pub attrs: Vec<crate::attr::Attribute>,
    pub sig: Signature,
    pub default: Option<crate::stmt::Block>,
    pub semi_token: Option<token::Semi>,
}
```

*Defined in [`syn-2.0.111/src/item.rs:652-661`](../../.source_1765521767/syn-2.0.111/src/item.rs#L652-L661)*

An associated function within the definition of a trait.

#### Implementations

- <span id="cratetraititemfn-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TraitItemFn`

- <span id="traititemfn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TraitItemFn`

- <span id="traititemfn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TraitItemFn`

- <span id="traititemfn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TraitItemFn`

- <span id="cratetraititemfn-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TraitItemFn`

- <span id="traititemfn-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TraitItemFn`

- <span id="cratetraititemfn-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItemFn`

##### `impl<T> From for TraitItemFn`

- <span id="traititemfn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TraitItemFn`

- <span id="cratetraititemfn-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TraitItemFn`

- <span id="traititemfn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::TraitItemFn`

- <span id="crateitemtraititemfn-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::TraitItemFn`

- <span id="cratetraititemfn-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TraitItemFn`

##### `impl Spanned for TraitItemFn`

- <span id="traititemfn-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TraitItemFn`

- <span id="traititemfn-toowned-type-owned"></span>`type Owned = T`

- <span id="traititemfn-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="traititemfn-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::TraitItemFn`

- <span id="crateitemtraititemfn-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TraitItemFn`

- <span id="traititemfn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="traititemfn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TraitItemFn`

- <span id="traititemfn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="traititemfn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TraitItemMacro`

```rust
struct TraitItemMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<token::Semi>,
}
```

*Defined in [`syn-2.0.111/src/item.rs:678-686`](../../.source_1765521767/syn-2.0.111/src/item.rs#L678-L686)*

A macro invocation within the definition of a trait.

#### Implementations

- <span id="cratetraititemmacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TraitItemMacro`

- <span id="traititemmacro-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TraitItemMacro`

- <span id="traititemmacro-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TraitItemMacro`

- <span id="traititemmacro-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TraitItemMacro`

- <span id="cratetraititemmacro-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TraitItemMacro`

- <span id="traititemmacro-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TraitItemMacro`

- <span id="cratetraititemmacro-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItemMacro`

##### `impl<T> From for TraitItemMacro`

- <span id="traititemmacro-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TraitItemMacro`

- <span id="cratetraititemmacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TraitItemMacro`

- <span id="traititemmacro-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::TraitItemMacro`

- <span id="crateitemtraititemmacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::TraitItemMacro`

- <span id="cratetraititemmacro-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TraitItemMacro`

##### `impl Spanned for TraitItemMacro`

- <span id="traititemmacro-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TraitItemMacro`

- <span id="traititemmacro-toowned-type-owned"></span>`type Owned = T`

- <span id="traititemmacro-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="traititemmacro-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::TraitItemMacro`

- <span id="crateitemtraititemmacro-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TraitItemMacro`

- <span id="traititemmacro-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="traititemmacro-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TraitItemMacro`

- <span id="traititemmacro-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="traititemmacro-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:663-676`](../../.source_1765521767/syn-2.0.111/src/item.rs#L663-L676)*

An associated type within the definition of a trait.

#### Implementations

- <span id="cratetraititemtype-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TraitItemType`

- <span id="traititemtype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TraitItemType`

- <span id="traititemtype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TraitItemType`

- <span id="traititemtype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TraitItemType`

- <span id="cratetraititemtype-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TraitItemType`

- <span id="traititemtype-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TraitItemType`

- <span id="cratetraititemtype-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItemType`

##### `impl<T> From for TraitItemType`

- <span id="traititemtype-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TraitItemType`

- <span id="cratetraititemtype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TraitItemType`

- <span id="traititemtype-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::TraitItemType`

- <span id="crateitemtraititemtype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::TraitItemType`

- <span id="cratetraititemtype-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TraitItemType`

##### `impl Spanned for TraitItemType`

- <span id="traititemtype-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TraitItemType`

- <span id="traititemtype-toowned-type-owned"></span>`type Owned = T`

- <span id="traititemtype-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="traititemtype-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::TraitItemType`

- <span id="crateitemtraititemtype-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TraitItemType`

- <span id="traititemtype-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="traititemtype-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TraitItemType`

- <span id="traititemtype-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="traititemtype-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UseGlob`

```rust
struct UseGlob {
    pub star_token: token::Star,
}
```

*Defined in [`syn-2.0.111/src/item.rs:479-485`](../../.source_1765521767/syn-2.0.111/src/item.rs#L479-L485)*

A glob import in a `use` item: `*`.

#### Trait Implementations

##### `impl Any for UseGlob`

- <span id="useglob-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UseGlob`

- <span id="useglob-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UseGlob`

- <span id="useglob-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::UseGlob`

- <span id="crateuseglob-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for UseGlob`

- <span id="useglob-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::UseGlob`

- <span id="crateuseglob-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseGlob`

##### `impl<T> From for UseGlob`

- <span id="useglob-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::UseGlob`

- <span id="crateuseglob-hash"></span>`fn hash<H>(&self, _state: &mut H)`

##### `impl<U> Into for UseGlob`

- <span id="useglob-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::UseGlob`

- <span id="crateuseglob-partialeq-eq"></span>`fn eq(&self, _other: &Self) -> bool`

##### `impl Sealed for UseGlob`

##### `impl Spanned for UseGlob`

- <span id="useglob-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for UseGlob`

- <span id="useglob-toowned-type-owned"></span>`type Owned = T`

- <span id="useglob-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="useglob-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::UseGlob`

- <span id="crateitemuseglob-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for UseGlob`

- <span id="useglob-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="useglob-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UseGlob`

- <span id="useglob-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="useglob-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UseGroup`

```rust
struct UseGroup {
    pub brace_token: token::Brace,
    pub items: crate::punctuated::Punctuated<UseTree, token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/item.rs:487-494`](../../.source_1765521767/syn-2.0.111/src/item.rs#L487-L494)*

A braced group of imports in a `use` item: `{A, B, C}`.

#### Trait Implementations

##### `impl Any for UseGroup`

- <span id="usegroup-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UseGroup`

- <span id="usegroup-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UseGroup`

- <span id="usegroup-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::UseGroup`

- <span id="crateusegroup-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for UseGroup`

- <span id="usegroup-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::UseGroup`

- <span id="crateusegroup-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseGroup`

##### `impl<T> From for UseGroup`

- <span id="usegroup-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::UseGroup`

- <span id="crateusegroup-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for UseGroup`

- <span id="usegroup-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::UseGroup`

- <span id="crateusegroup-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for UseGroup`

##### `impl Spanned for UseGroup`

- <span id="usegroup-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for UseGroup`

- <span id="usegroup-toowned-type-owned"></span>`type Owned = T`

- <span id="usegroup-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="usegroup-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::UseGroup`

- <span id="crateitemusegroup-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for UseGroup`

- <span id="usegroup-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="usegroup-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UseGroup`

- <span id="usegroup-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="usegroup-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UseName`

```rust
struct UseName {
    pub ident: crate::ident::Ident,
}
```

*Defined in [`syn-2.0.111/src/item.rs:461-467`](../../.source_1765521767/syn-2.0.111/src/item.rs#L461-L467)*

An identifier imported by a `use` item: `HashMap`.

#### Trait Implementations

##### `impl Any for UseName`

- <span id="usename-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UseName`

- <span id="usename-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UseName`

- <span id="usename-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::UseName`

- <span id="crateusename-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for UseName`

- <span id="usename-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::UseName`

- <span id="crateusename-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseName`

##### `impl<T> From for UseName`

- <span id="usename-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::UseName`

- <span id="crateusename-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for UseName`

- <span id="usename-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::UseName`

- <span id="crateusename-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for UseName`

##### `impl Spanned for UseName`

- <span id="usename-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for UseName`

- <span id="usename-toowned-type-owned"></span>`type Owned = T`

- <span id="usename-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="usename-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::UseName`

- <span id="crateitemusename-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for UseName`

- <span id="usename-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="usename-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UseName`

- <span id="usename-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="usename-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UsePath`

```rust
struct UsePath {
    pub ident: crate::ident::Ident,
    pub colon2_token: token::PathSep,
    pub tree: Box<UseTree>,
}
```

*Defined in [`syn-2.0.111/src/item.rs:451-459`](../../.source_1765521767/syn-2.0.111/src/item.rs#L451-L459)*

A path prefix of imports in a `use` item: `std::...`.

#### Trait Implementations

##### `impl Any for UsePath`

- <span id="usepath-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UsePath`

- <span id="usepath-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UsePath`

- <span id="usepath-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::UsePath`

- <span id="crateusepath-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for UsePath`

- <span id="usepath-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::UsePath`

- <span id="crateusepath-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UsePath`

##### `impl<T> From for UsePath`

- <span id="usepath-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::UsePath`

- <span id="crateusepath-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for UsePath`

- <span id="usepath-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::UsePath`

- <span id="crateusepath-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for UsePath`

##### `impl Spanned for UsePath`

- <span id="usepath-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for UsePath`

- <span id="usepath-toowned-type-owned"></span>`type Owned = T`

- <span id="usepath-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="usepath-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::UsePath`

- <span id="crateitemusepath-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for UsePath`

- <span id="usepath-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="usepath-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UsePath`

- <span id="usepath-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="usepath-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UseRename`

```rust
struct UseRename {
    pub ident: crate::ident::Ident,
    pub as_token: token::As,
    pub rename: crate::ident::Ident,
}
```

*Defined in [`syn-2.0.111/src/item.rs:469-477`](../../.source_1765521767/syn-2.0.111/src/item.rs#L469-L477)*

An renamed identifier imported by a `use` item: `HashMap as Map`.

#### Trait Implementations

##### `impl Any for UseRename`

- <span id="userename-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UseRename`

- <span id="userename-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UseRename`

- <span id="userename-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::UseRename`

- <span id="crateuserename-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for UseRename`

- <span id="userename-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::UseRename`

- <span id="crateuserename-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseRename`

##### `impl<T> From for UseRename`

- <span id="userename-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::UseRename`

- <span id="crateuserename-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for UseRename`

- <span id="userename-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::UseRename`

- <span id="crateuserename-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for UseRename`

##### `impl Spanned for UseRename`

- <span id="userename-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for UseRename`

- <span id="userename-toowned-type-owned"></span>`type Owned = T`

- <span id="userename-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="userename-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::UseRename`

- <span id="crateitemuserename-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for UseRename`

- <span id="userename-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="userename-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UseRename`

- <span id="userename-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="userename-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Variadic`

```rust
struct Variadic {
    pub attrs: Vec<crate::attr::Attribute>,
    pub pat: Option<(Box<crate::pat::Pat>, token::Colon)>,
    pub dots: token::DotDotDot,
    pub comma: Option<token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/item.rs:857-876`](../../.source_1765521767/syn-2.0.111/src/item.rs#L857-L876)*

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

##### `impl Any for Variadic`

- <span id="variadic-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Variadic`

- <span id="variadic-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Variadic`

- <span id="variadic-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Variadic`

- <span id="cratevariadic-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Variadic`

- <span id="variadic-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Variadic`

- <span id="cratevariadic-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Variadic`

##### `impl<T> From for Variadic`

- <span id="variadic-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Variadic`

- <span id="cratevariadic-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Variadic`

- <span id="variadic-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::Variadic`

- <span id="cratevariadic-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Variadic`

##### `impl Spanned for Variadic`

- <span id="variadic-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Variadic`

- <span id="variadic-toowned-type-owned"></span>`type Owned = T`

- <span id="variadic-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="variadic-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::Variadic`

- <span id="crateitemvariadic-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Variadic`

- <span id="variadic-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="variadic-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Variadic`

- <span id="variadic-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="variadic-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Lifetime`

```rust
struct Lifetime {
    pub apostrophe: proc_macro2::Span,
    pub ident: proc_macro2::Ident,
}
```

*Defined in [`syn-2.0.111/src/lifetime.rs:18-21`](../../.source_1765521767/syn-2.0.111/src/lifetime.rs#L18-L21)*

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

  # Panics

  

  Panics if the lifetime does not conform to the bulleted rules above.

  

  # Invocation

  

  ```rust

  use proc_macro2::Span;

  use syn::Lifetime;

  

  fn f() -> Lifetime {

  Lifetime::new("'a", Span::call_site())

  }

  ```

- <span id="lifetime-span"></span>`fn span(&self) -> Span`

- <span id="lifetime-set-span"></span>`fn set_span(&mut self, span: Span)`

#### Trait Implementations

##### `impl Any for Lifetime`

- <span id="lifetime-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Lifetime`

- <span id="lifetime-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Lifetime`

- <span id="lifetime-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Lifetime`

- <span id="lifetime-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Lifetime`

- <span id="lifetime-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Lifetime`

- <span id="cratelifetime-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Lifetime`

- <span id="lifetime-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Lifetime`

##### `impl<T> From for Lifetime`

- <span id="lifetime-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Lifetime`

- <span id="lifetime-hash"></span>`fn hash<H: Hasher>(&self, h: &mut H)`

##### `impl<U> Into for Lifetime`

- <span id="lifetime-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Lifetime`

- <span id="lifetime-ord-cmp"></span>`fn cmp(&self, other: &Lifetime) -> Ordering` — [`Lifetime`](lifetime/index.md#lifetime)

##### `impl Parse for crate::lifetime::Lifetime`

- <span id="cratelifetimelifetime-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for Lifetime`

- <span id="lifetime-partialeq-eq"></span>`fn eq(&self, other: &Lifetime) -> bool` — [`Lifetime`](lifetime/index.md#lifetime)

##### `impl PartialOrd for Lifetime`

- <span id="lifetime-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Lifetime) -> Option<Ordering>` — [`Lifetime`](lifetime/index.md#lifetime)

##### `impl Sealed for Lifetime`

##### `impl Spanned for Lifetime`

- <span id="lifetime-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Lifetime`

- <span id="lifetime-toowned-type-owned"></span>`type Owned = T`

- <span id="lifetime-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="lifetime-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Lifetime`

- <span id="lifetime-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for crate::lifetime::Lifetime`

- <span id="cratelifetimelifetime-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lifetime::Lifetime`

##### `impl<U> TryFrom for Lifetime`

- <span id="lifetime-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lifetime-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Lifetime`

- <span id="lifetime-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lifetime-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LitBool`

```rust
struct LitBool {
    pub value: bool,
    pub span: proc_macro2::Span,
}
```

*Defined in [`syn-2.0.111/src/lit.rs:126-132`](../../.source_1765521767/syn-2.0.111/src/lit.rs#L126-L132)*

A boolean literal: `true` or `false`.

#### Implementations

- <span id="cratelitlitbool-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for LitBool`

- <span id="litbool-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LitBool`

- <span id="litbool-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LitBool`

- <span id="litbool-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::LitBool`

- <span id="cratelitbool-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for LitBool`

- <span id="litbool-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::lit::LitBool`

- <span id="cratelitlitbool-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitBool`

##### `impl<T> From for LitBool`

- <span id="litbool-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::LitBool`

- <span id="cratelitbool-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for LitBool`

- <span id="litbool-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::lit::LitBool`

- <span id="cratelitlitbool-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::LitBool`

- <span id="cratelitbool-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for LitBool`

##### `impl Spanned for LitBool`

- <span id="litbool-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for LitBool`

- <span id="litbool-toowned-type-owned"></span>`type Owned = T`

- <span id="litbool-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="litbool-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::lit::LitBool`

- <span id="cratelitlitbool-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitBool`

##### `impl<U> TryFrom for LitBool`

- <span id="litbool-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="litbool-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LitBool`

- <span id="litbool-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="litbool-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LitByte`

```rust
struct LitByte {
    repr: Box<LitRepr>,
}
```

*Defined in [`syn-2.0.111/src/lit.rs:79-84`](../../.source_1765521767/syn-2.0.111/src/lit.rs#L79-L84)*

A byte literal: `b'f'`.

#### Implementations

- <span id="cratelitlitbyte-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for LitByte`

- <span id="litbyte-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LitByte`

- <span id="litbyte-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LitByte`

- <span id="litbyte-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LitByte`

- <span id="litbyte-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for LitByte`

- <span id="litbyte-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::lit::LitByte`

- <span id="cratelitlitbyte-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitByte`

##### `impl<T> From for LitByte`

- <span id="litbyte-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for LitByte`

- <span id="litbyte-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for LitByte`

- <span id="litbyte-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::lit::LitByte`

- <span id="cratelitlitbyte-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for LitByte`

- <span id="litbyte-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for LitByte`

##### `impl Spanned for LitByte`

- <span id="litbyte-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for LitByte`

- <span id="litbyte-toowned-type-owned"></span>`type Owned = T`

- <span id="litbyte-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="litbyte-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::lit::LitByte`

- <span id="cratelitlitbyte-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitByte`

##### `impl<U> TryFrom for LitByte`

- <span id="litbyte-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="litbyte-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LitByte`

- <span id="litbyte-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="litbyte-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LitByteStr`

```rust
struct LitByteStr {
    repr: Box<LitRepr>,
}
```

*Defined in [`syn-2.0.111/src/lit.rs:65-70`](../../.source_1765521767/syn-2.0.111/src/lit.rs#L65-L70)*

A byte string literal: `b"foo"`.

#### Implementations

- <span id="cratelitlitbytestr-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for LitByteStr`

- <span id="litbytestr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LitByteStr`

- <span id="litbytestr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LitByteStr`

- <span id="litbytestr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LitByteStr`

- <span id="litbytestr-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for LitByteStr`

- <span id="litbytestr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::lit::LitByteStr`

- <span id="cratelitlitbytestr-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitByteStr`

##### `impl<T> From for LitByteStr`

- <span id="litbytestr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for LitByteStr`

- <span id="litbytestr-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for LitByteStr`

- <span id="litbytestr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::lit::LitByteStr`

- <span id="cratelitlitbytestr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for LitByteStr`

- <span id="litbytestr-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for LitByteStr`

##### `impl Spanned for LitByteStr`

- <span id="litbytestr-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for LitByteStr`

- <span id="litbytestr-toowned-type-owned"></span>`type Owned = T`

- <span id="litbytestr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="litbytestr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::lit::LitByteStr`

- <span id="cratelitlitbytestr-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitByteStr`

##### `impl<U> TryFrom for LitByteStr`

- <span id="litbytestr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="litbytestr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LitByteStr`

- <span id="litbytestr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="litbytestr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LitCStr`

```rust
struct LitCStr {
    repr: Box<LitRepr>,
}
```

*Defined in [`syn-2.0.111/src/lit.rs:72-77`](../../.source_1765521767/syn-2.0.111/src/lit.rs#L72-L77)*

A nul-terminated C-string literal: `c"foo"`.

#### Implementations

- <span id="cratelitlitcstr-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for LitCStr`

- <span id="litcstr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LitCStr`

- <span id="litcstr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LitCStr`

- <span id="litcstr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LitCStr`

- <span id="litcstr-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for LitCStr`

- <span id="litcstr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::lit::LitCStr`

- <span id="cratelitlitcstr-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitCStr`

##### `impl<T> From for LitCStr`

- <span id="litcstr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for LitCStr`

- <span id="litcstr-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for LitCStr`

- <span id="litcstr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::lit::LitCStr`

- <span id="cratelitlitcstr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for LitCStr`

- <span id="litcstr-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for LitCStr`

##### `impl Spanned for LitCStr`

- <span id="litcstr-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for LitCStr`

- <span id="litcstr-toowned-type-owned"></span>`type Owned = T`

- <span id="litcstr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="litcstr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::lit::LitCStr`

- <span id="cratelitlitcstr-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitCStr`

##### `impl<U> TryFrom for LitCStr`

- <span id="litcstr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="litcstr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LitCStr`

- <span id="litcstr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="litcstr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LitChar`

```rust
struct LitChar {
    repr: Box<LitRepr>,
}
```

*Defined in [`syn-2.0.111/src/lit.rs:86-91`](../../.source_1765521767/syn-2.0.111/src/lit.rs#L86-L91)*

A character literal: `'a'`.

#### Implementations

- <span id="cratelitlitchar-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for LitChar`

- <span id="litchar-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LitChar`

- <span id="litchar-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LitChar`

- <span id="litchar-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LitChar`

- <span id="litchar-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for LitChar`

- <span id="litchar-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::lit::LitChar`

- <span id="cratelitlitchar-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitChar`

##### `impl<T> From for LitChar`

- <span id="litchar-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for LitChar`

- <span id="litchar-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for LitChar`

- <span id="litchar-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::lit::LitChar`

- <span id="cratelitlitchar-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for LitChar`

- <span id="litchar-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for LitChar`

##### `impl Spanned for LitChar`

- <span id="litchar-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for LitChar`

- <span id="litchar-toowned-type-owned"></span>`type Owned = T`

- <span id="litchar-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="litchar-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::lit::LitChar`

- <span id="cratelitlitchar-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitChar`

##### `impl<U> TryFrom for LitChar`

- <span id="litchar-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="litchar-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LitChar`

- <span id="litchar-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="litchar-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LitFloat`

```rust
struct LitFloat {
    repr: Box<LitFloatRepr>,
}
```

*Defined in [`syn-2.0.111/src/lit.rs:111-118`](../../.source_1765521767/syn-2.0.111/src/lit.rs#L111-L118)*

A floating point literal: `1f64` or `1.0e10f64`.

Must be finite. May not be infinite or NaN.

#### Implementations

- <span id="cratelitlitfloat-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for LitFloat`

- <span id="litfloat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LitFloat`

- <span id="litfloat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LitFloat`

- <span id="litfloat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LitFloat`

- <span id="litfloat-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for LitFloat`

- <span id="litfloat-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::lit::LitFloat`

- <span id="cratelitlitfloat-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for LitFloat`

- <span id="litfloat-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitFloat`

##### `impl<T> From for LitFloat`

- <span id="litfloat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for LitFloat`

- <span id="litfloat-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for LitFloat`

- <span id="litfloat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::lit::LitFloat`

- <span id="cratelitlitfloat-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for LitFloat`

- <span id="litfloat-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for LitFloat`

##### `impl Spanned for LitFloat`

- <span id="litfloat-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for LitFloat`

- <span id="litfloat-toowned-type-owned"></span>`type Owned = T`

- <span id="litfloat-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="litfloat-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for LitFloat`

- <span id="litfloat-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for crate::lit::LitFloat`

- <span id="cratelitlitfloat-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitFloat`

##### `impl<U> TryFrom for LitFloat`

- <span id="litfloat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="litfloat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LitFloat`

- <span id="litfloat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="litfloat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LitInt`

```rust
struct LitInt {
    repr: Box<LitIntRepr>,
}
```

*Defined in [`syn-2.0.111/src/lit.rs:98-103`](../../.source_1765521767/syn-2.0.111/src/lit.rs#L98-L103)*

An integer literal: `1` or `1u16`.

#### Implementations

- <span id="cratelitlitint-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for LitInt`

- <span id="litint-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LitInt`

- <span id="litint-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LitInt`

- <span id="litint-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LitInt`

- <span id="litint-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for LitInt`

- <span id="litint-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::lit::LitInt`

- <span id="cratelitlitint-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for LitInt`

- <span id="litint-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitInt`

##### `impl<T> From for LitInt`

- <span id="litint-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for LitInt`

- <span id="litint-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for LitInt`

- <span id="litint-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::lit::LitInt`

- <span id="cratelitlitint-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for LitInt`

- <span id="litint-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for LitInt`

##### `impl Spanned for LitInt`

- <span id="litint-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for LitInt`

- <span id="litint-toowned-type-owned"></span>`type Owned = T`

- <span id="litint-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="litint-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for LitInt`

- <span id="litint-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for crate::lit::LitInt`

- <span id="cratelitlitint-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitInt`

##### `impl<U> TryFrom for LitInt`

- <span id="litint-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="litint-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LitInt`

- <span id="litint-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="litint-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LitStr`

```rust
struct LitStr {
    repr: Box<LitRepr>,
}
```

*Defined in [`syn-2.0.111/src/lit.rs:58-63`](../../.source_1765521767/syn-2.0.111/src/lit.rs#L58-L63)*

A UTF-8 string literal: `"foo"`.

#### Implementations

- <span id="cratelitlitstr-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for LitStr`

- <span id="litstr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LitStr`

- <span id="litstr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LitStr`

- <span id="litstr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LitStr`

- <span id="litstr-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for LitStr`

- <span id="litstr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::lit::LitStr`

- <span id="cratelitlitstr-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitStr`

##### `impl<T> From for LitStr`

- <span id="litstr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for LitStr`

- <span id="litstr-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for LitStr`

- <span id="litstr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::lit::LitStr`

- <span id="cratelitlitstr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for LitStr`

- <span id="litstr-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for LitStr`

##### `impl Spanned for LitStr`

- <span id="litstr-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for LitStr`

- <span id="litstr-toowned-type-owned"></span>`type Owned = T`

- <span id="litstr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="litstr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::lit::LitStr`

- <span id="cratelitlitstr-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitStr`

##### `impl<U> TryFrom for LitStr`

- <span id="litstr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="litstr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LitStr`

- <span id="litstr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="litstr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Macro`

```rust
struct Macro {
    pub path: crate::path::Path,
    pub bang_token: token::Not,
    pub delimiter: MacroDelimiter,
    pub tokens: proc_macro2::TokenStream,
}
```

*Defined in [`syn-2.0.111/src/mac.rs:14-23`](../../.source_1765521767/syn-2.0.111/src/mac.rs#L14-L23)*

A macro invocation: `println!("{}", mac)`.

#### Implementations

- <span id="macro-parse-body"></span>`fn parse_body<T: Parse>(&self) -> Result<T>` — [`Result`](error/index.md#result)

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

- <span id="macro-parse-body-with"></span>`fn parse_body_with<F: Parser>(&self, parser: F) -> Result<<F as >::Output>` — [`Result`](error/index.md#result), [`Parser`](parse/index.md#parser)

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

- <span id="cratemacmacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

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

### `FieldPat`

```rust
struct FieldPat {
    pub attrs: Vec<crate::attr::Attribute>,
    pub member: crate::expr::Member,
    pub colon_token: Option<token::Colon>,
    pub pat: Box<Pat>,
}
```

*Defined in [`syn-2.0.111/src/pat.rs:224-236`](../../.source_1765521767/syn-2.0.111/src/pat.rs#L224-L236)*

A single field in a struct pattern.

Patterns like the fields of Foo `{ x, ref y, ref mut z }` are treated
the same as `x: x, y: ref y, z: ref mut z` but there is no colon token.

#### Trait Implementations

##### `impl Any for FieldPat`

- <span id="fieldpat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FieldPat`

- <span id="fieldpat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FieldPat`

- <span id="fieldpat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::FieldPat`

- <span id="cratefieldpat-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for FieldPat`

- <span id="fieldpat-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::FieldPat`

- <span id="cratefieldpat-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldPat`

##### `impl<T> From for FieldPat`

- <span id="fieldpat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::FieldPat`

- <span id="cratefieldpat-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for FieldPat`

- <span id="fieldpat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::FieldPat`

- <span id="cratefieldpat-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for FieldPat`

##### `impl Spanned for FieldPat`

- <span id="fieldpat-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for FieldPat`

- <span id="fieldpat-toowned-type-owned"></span>`type Owned = T`

- <span id="fieldpat-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fieldpat-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::pat::FieldPat`

- <span id="cratepatfieldpat-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for FieldPat`

- <span id="fieldpat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fieldpat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FieldPat`

- <span id="fieldpat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fieldpat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatConst`

```rust
struct PatConst {
    pub attrs: Vec<crate::attr::Attribute>,
    pub const_token: token::Const,
    pub block: crate::stmt::Block,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:385-393`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L385-L393)*

A const block: `const { ... }`.

#### Implementations

- <span id="crateexprconst-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprConst`

- <span id="exprconst-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprConst`

- <span id="exprconst-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprConst`

- <span id="exprconst-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprConst`

- <span id="crateexprconst-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprConst`

- <span id="exprconst-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprConst`

- <span id="crateexprconst-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprConst`

##### `impl<T> From for ExprConst`

- <span id="exprconst-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprConst`

- <span id="crateexprconst-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprConst`

- <span id="exprconst-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprConst`

- <span id="crateexprexprconst-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprConst`

- <span id="crateexprconst-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprConst`

##### `impl Spanned for ExprConst`

- <span id="exprconst-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprConst`

- <span id="exprconst-toowned-type-owned"></span>`type Owned = T`

- <span id="exprconst-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprconst-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprConst`

- <span id="crateexprexprconst-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprConst`

- <span id="exprconst-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprconst-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprConst`

- <span id="exprconst-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprconst-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/pat.rs:104-117`](../../.source_1765521767/syn-2.0.111/src/pat.rs#L104-L117)*

A pattern that binds a new variable: `ref mut binding @ SUBPATTERN`.

It may also be a unit struct or struct variant (e.g. `None`), or a
constant; these cannot be distinguished syntactically.

#### Implementations

- <span id="cratepatident-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for PatIdent`

- <span id="patident-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatIdent`

- <span id="patident-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatIdent`

- <span id="patident-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PatIdent`

- <span id="cratepatident-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PatIdent`

- <span id="patident-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PatIdent`

- <span id="cratepatident-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatIdent`

##### `impl<T> From for PatIdent`

- <span id="patident-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PatIdent`

- <span id="cratepatident-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PatIdent`

- <span id="patident-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::PatIdent`

- <span id="cratepatident-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatIdent`

##### `impl Spanned for PatIdent`

- <span id="patident-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PatIdent`

- <span id="patident-toowned-type-owned"></span>`type Owned = T`

- <span id="patident-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="patident-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::pat::PatIdent`

- <span id="cratepatpatident-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PatIdent`

- <span id="patident-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="patident-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatIdent`

- <span id="patident-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="patident-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatLit`

```rust
struct PatLit {
    pub attrs: Vec<crate::attr::Attribute>,
    pub lit: crate::lit::Lit,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:493-500`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L493-L500)*

A literal in place of an expression: `1`, `"foo"`.

#### Implementations

- <span id="crateexprlit-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprLit`

- <span id="exprlit-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprLit`

- <span id="exprlit-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprLit`

- <span id="exprlit-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprLit`

- <span id="crateexprlit-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprLit`

- <span id="exprlit-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprLit`

- <span id="crateexprlit-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprLit`

##### `impl<T> From for ExprLit`

- <span id="exprlit-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprLit`

- <span id="crateexprlit-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprLit`

- <span id="exprlit-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprLit`

- <span id="crateexprexprlit-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprLit`

- <span id="crateexprlit-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprLit`

##### `impl Spanned for ExprLit`

- <span id="exprlit-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprLit`

- <span id="exprlit-toowned-type-owned"></span>`type Owned = T`

- <span id="exprlit-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprlit-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprLit`

- <span id="crateexprexprlit-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprLit`

- <span id="exprlit-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprlit-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprLit`

- <span id="exprlit-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprlit-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatMacro`

```rust
struct PatMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:513-520`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L513-L520)*

A macro invocation expression: `format!("{}", q)`.

#### Implementations

- <span id="crateexprmacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprMacro`

- <span id="exprmacro-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprMacro`

- <span id="exprmacro-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprMacro`

- <span id="exprmacro-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprMacro`

- <span id="crateexprmacro-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprMacro`

- <span id="exprmacro-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprMacro`

- <span id="crateexprmacro-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprMacro`

##### `impl<T> From for ExprMacro`

- <span id="exprmacro-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprMacro`

- <span id="crateexprmacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprMacro`

- <span id="exprmacro-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprMacro`

- <span id="crateexprexprmacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprMacro`

- <span id="crateexprmacro-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprMacro`

##### `impl Spanned for ExprMacro`

- <span id="exprmacro-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprMacro`

- <span id="exprmacro-toowned-type-owned"></span>`type Owned = T`

- <span id="exprmacro-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprmacro-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprMacro`

- <span id="crateexprexprmacro-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprMacro`

- <span id="exprmacro-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprmacro-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprMacro`

- <span id="exprmacro-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprmacro-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatOr`

```rust
struct PatOr {
    pub attrs: Vec<crate::attr::Attribute>,
    pub leading_vert: Option<token::Or>,
    pub cases: crate::punctuated::Punctuated<Pat, token::Or>,
}
```

*Defined in [`syn-2.0.111/src/pat.rs:119-127`](../../.source_1765521767/syn-2.0.111/src/pat.rs#L119-L127)*

A pattern that matches any one of a set of cases.

#### Implementations

- <span id="cratepator-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for PatOr`

- <span id="pator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatOr`

- <span id="pator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatOr`

- <span id="pator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PatOr`

- <span id="cratepator-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PatOr`

- <span id="pator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PatOr`

- <span id="cratepator-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatOr`

##### `impl<T> From for PatOr`

- <span id="pator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PatOr`

- <span id="cratepator-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PatOr`

- <span id="pator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::PatOr`

- <span id="cratepator-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatOr`

##### `impl Spanned for PatOr`

- <span id="pator-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PatOr`

- <span id="pator-toowned-type-owned"></span>`type Owned = T`

- <span id="pator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::pat::PatOr`

- <span id="cratepatpator-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PatOr`

- <span id="pator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatOr`

- <span id="pator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatParen`

```rust
struct PatParen {
    pub attrs: Vec<crate::attr::Attribute>,
    pub paren_token: token::Paren,
    pub pat: Box<Pat>,
}
```

*Defined in [`syn-2.0.111/src/pat.rs:129-137`](../../.source_1765521767/syn-2.0.111/src/pat.rs#L129-L137)*

A parenthesized pattern: `(A | B)`.

#### Implementations

- <span id="cratepatparen-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for PatParen`

- <span id="patparen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatParen`

- <span id="patparen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatParen`

- <span id="patparen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PatParen`

- <span id="cratepatparen-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PatParen`

- <span id="patparen-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PatParen`

- <span id="cratepatparen-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatParen`

##### `impl<T> From for PatParen`

- <span id="patparen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PatParen`

- <span id="cratepatparen-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PatParen`

- <span id="patparen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::PatParen`

- <span id="cratepatparen-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatParen`

##### `impl Spanned for PatParen`

- <span id="patparen-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PatParen`

- <span id="patparen-toowned-type-owned"></span>`type Owned = T`

- <span id="patparen-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="patparen-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::pat::PatParen`

- <span id="cratepatpatparen-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PatParen`

- <span id="patparen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="patparen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatParen`

- <span id="patparen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="patparen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatPath`

```rust
struct PatPath {
    pub attrs: Vec<crate::attr::Attribute>,
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:558-569`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L558-L569)*

A path like `std::mem::replace` possibly containing generic
parameters and a qualified self-type.

A plain identifier like `x` is a path of length 1.

#### Implementations

- <span id="crateexprpath-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprPath`

- <span id="exprpath-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprPath`

- <span id="exprpath-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprPath`

- <span id="exprpath-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprPath`

- <span id="crateexprpath-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprPath`

- <span id="exprpath-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprPath`

- <span id="crateexprpath-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprPath`

##### `impl<T> From for ExprPath`

- <span id="exprpath-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprPath`

- <span id="crateexprpath-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprPath`

- <span id="exprpath-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprPath`

- <span id="crateexprexprpath-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprPath`

- <span id="crateexprpath-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprPath`

##### `impl Spanned for ExprPath`

- <span id="exprpath-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprPath`

- <span id="exprpath-toowned-type-owned"></span>`type Owned = T`

- <span id="exprpath-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprpath-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprPath`

- <span id="crateexprexprpath-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprPath`

- <span id="exprpath-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprpath-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprPath`

- <span id="exprpath-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprpath-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatRange`

```rust
struct PatRange {
    pub attrs: Vec<crate::attr::Attribute>,
    pub start: Option<Box<Expr>>,
    pub limits: RangeLimits,
    pub end: Option<Box<Expr>>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:571-580`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L571-L580)*

A range expression: `1..2`, `1..`, `..2`, `1..=2`, `..=2`.

#### Implementations

- <span id="crateexprrange-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprRange`

- <span id="exprrange-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprRange`

- <span id="exprrange-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprRange`

- <span id="exprrange-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprRange`

- <span id="crateexprrange-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprRange`

- <span id="exprrange-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprRange`

- <span id="crateexprrange-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprRange`

##### `impl<T> From for ExprRange`

- <span id="exprrange-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprRange`

- <span id="crateexprrange-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprRange`

- <span id="exprrange-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprRange`

- <span id="crateexprexprrange-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ExprRange`

- <span id="crateexprrange-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprRange`

##### `impl Spanned for ExprRange`

- <span id="exprrange-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprRange`

- <span id="exprrange-toowned-type-owned"></span>`type Owned = T`

- <span id="exprrange-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprrange-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprRange`

- <span id="crateexprexprrange-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprRange`

- <span id="exprrange-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprrange-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprRange`

- <span id="exprrange-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprrange-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatReference`

```rust
struct PatReference {
    pub attrs: Vec<crate::attr::Attribute>,
    pub and_token: token::And,
    pub mutability: Option<token::Mut>,
    pub pat: Box<Pat>,
}
```

*Defined in [`syn-2.0.111/src/pat.rs:139-148`](../../.source_1765521767/syn-2.0.111/src/pat.rs#L139-L148)*

A reference pattern: `&mut var`.

#### Implementations

- <span id="cratepatreference-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for PatReference`

- <span id="patreference-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatReference`

- <span id="patreference-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatReference`

- <span id="patreference-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PatReference`

- <span id="cratepatreference-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PatReference`

- <span id="patreference-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PatReference`

- <span id="cratepatreference-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatReference`

##### `impl<T> From for PatReference`

- <span id="patreference-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PatReference`

- <span id="cratepatreference-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PatReference`

- <span id="patreference-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::PatReference`

- <span id="cratepatreference-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatReference`

##### `impl Spanned for PatReference`

- <span id="patreference-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PatReference`

- <span id="patreference-toowned-type-owned"></span>`type Owned = T`

- <span id="patreference-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="patreference-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::pat::PatReference`

- <span id="cratepatpatreference-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PatReference`

- <span id="patreference-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="patreference-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatReference`

- <span id="patreference-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="patreference-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatRest`

```rust
struct PatRest {
    pub attrs: Vec<crate::attr::Attribute>,
    pub dot2_token: token::DotDot,
}
```

*Defined in [`syn-2.0.111/src/pat.rs:150-157`](../../.source_1765521767/syn-2.0.111/src/pat.rs#L150-L157)*

The dots in a tuple or slice pattern: `[0, 1, ..]`.

#### Implementations

- <span id="cratepatrest-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for PatRest`

- <span id="patrest-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatRest`

- <span id="patrest-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatRest`

- <span id="patrest-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PatRest`

- <span id="cratepatrest-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PatRest`

- <span id="patrest-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PatRest`

- <span id="cratepatrest-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatRest`

##### `impl<T> From for PatRest`

- <span id="patrest-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PatRest`

- <span id="cratepatrest-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PatRest`

- <span id="patrest-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::PatRest`

- <span id="cratepatrest-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatRest`

##### `impl Spanned for PatRest`

- <span id="patrest-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PatRest`

- <span id="patrest-toowned-type-owned"></span>`type Owned = T`

- <span id="patrest-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="patrest-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::pat::PatRest`

- <span id="cratepatpatrest-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PatRest`

- <span id="patrest-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="patrest-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatRest`

- <span id="patrest-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="patrest-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatSlice`

```rust
struct PatSlice {
    pub attrs: Vec<crate::attr::Attribute>,
    pub bracket_token: token::Bracket,
    pub elems: crate::punctuated::Punctuated<Pat, token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/pat.rs:159-167`](../../.source_1765521767/syn-2.0.111/src/pat.rs#L159-L167)*

A dynamically sized slice pattern: `[a, b, ref i @ .., y, z]`.

#### Implementations

- <span id="cratepatslice-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for PatSlice`

- <span id="patslice-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatSlice`

- <span id="patslice-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatSlice`

- <span id="patslice-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PatSlice`

- <span id="cratepatslice-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PatSlice`

- <span id="patslice-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PatSlice`

- <span id="cratepatslice-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatSlice`

##### `impl<T> From for PatSlice`

- <span id="patslice-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PatSlice`

- <span id="cratepatslice-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PatSlice`

- <span id="patslice-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::PatSlice`

- <span id="cratepatslice-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatSlice`

##### `impl Spanned for PatSlice`

- <span id="patslice-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PatSlice`

- <span id="patslice-toowned-type-owned"></span>`type Owned = T`

- <span id="patslice-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="patslice-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::pat::PatSlice`

- <span id="cratepatpatslice-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PatSlice`

- <span id="patslice-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="patslice-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatSlice`

- <span id="patslice-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="patslice-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/pat.rs:169-180`](../../.source_1765521767/syn-2.0.111/src/pat.rs#L169-L180)*

A struct or struct variant pattern: `Variant { x, y, .. }`.

#### Implementations

- <span id="cratepatstruct-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for PatStruct`

- <span id="patstruct-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatStruct`

- <span id="patstruct-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatStruct`

- <span id="patstruct-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PatStruct`

- <span id="cratepatstruct-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PatStruct`

- <span id="patstruct-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PatStruct`

- <span id="cratepatstruct-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatStruct`

##### `impl<T> From for PatStruct`

- <span id="patstruct-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PatStruct`

- <span id="cratepatstruct-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PatStruct`

- <span id="patstruct-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::PatStruct`

- <span id="cratepatstruct-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatStruct`

##### `impl Spanned for PatStruct`

- <span id="patstruct-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PatStruct`

- <span id="patstruct-toowned-type-owned"></span>`type Owned = T`

- <span id="patstruct-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="patstruct-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::pat::PatStruct`

- <span id="cratepatpatstruct-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PatStruct`

- <span id="patstruct-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="patstruct-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatStruct`

- <span id="patstruct-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="patstruct-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatTuple`

```rust
struct PatTuple {
    pub attrs: Vec<crate::attr::Attribute>,
    pub paren_token: token::Paren,
    pub elems: crate::punctuated::Punctuated<Pat, token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/pat.rs:182-190`](../../.source_1765521767/syn-2.0.111/src/pat.rs#L182-L190)*

A tuple pattern: `(a, b)`.

#### Implementations

- <span id="cratepattuple-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for PatTuple`

- <span id="pattuple-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatTuple`

- <span id="pattuple-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatTuple`

- <span id="pattuple-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PatTuple`

- <span id="cratepattuple-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PatTuple`

- <span id="pattuple-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PatTuple`

- <span id="cratepattuple-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatTuple`

##### `impl<T> From for PatTuple`

- <span id="pattuple-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PatTuple`

- <span id="cratepattuple-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PatTuple`

- <span id="pattuple-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::PatTuple`

- <span id="cratepattuple-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatTuple`

##### `impl Spanned for PatTuple`

- <span id="pattuple-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PatTuple`

- <span id="pattuple-toowned-type-owned"></span>`type Owned = T`

- <span id="pattuple-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pattuple-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::pat::PatTuple`

- <span id="cratepatpattuple-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PatTuple`

- <span id="pattuple-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pattuple-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatTuple`

- <span id="pattuple-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pattuple-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/pat.rs:192-202`](../../.source_1765521767/syn-2.0.111/src/pat.rs#L192-L202)*

A tuple struct or tuple variant pattern: `Variant(x, y, .., z)`.

#### Implementations

- <span id="cratepattuplestruct-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for PatTupleStruct`

- <span id="pattuplestruct-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatTupleStruct`

- <span id="pattuplestruct-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatTupleStruct`

- <span id="pattuplestruct-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PatTupleStruct`

- <span id="cratepattuplestruct-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PatTupleStruct`

- <span id="pattuplestruct-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PatTupleStruct`

- <span id="cratepattuplestruct-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatTupleStruct`

##### `impl<T> From for PatTupleStruct`

- <span id="pattuplestruct-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PatTupleStruct`

- <span id="cratepattuplestruct-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PatTupleStruct`

- <span id="pattuplestruct-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::PatTupleStruct`

- <span id="cratepattuplestruct-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatTupleStruct`

##### `impl Spanned for PatTupleStruct`

- <span id="pattuplestruct-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PatTupleStruct`

- <span id="pattuplestruct-toowned-type-owned"></span>`type Owned = T`

- <span id="pattuplestruct-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pattuplestruct-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::pat::PatTupleStruct`

- <span id="cratepatpattuplestruct-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PatTupleStruct`

- <span id="pattuplestruct-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pattuplestruct-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatTupleStruct`

- <span id="pattuplestruct-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pattuplestruct-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatType`

```rust
struct PatType {
    pub attrs: Vec<crate::attr::Attribute>,
    pub pat: Box<Pat>,
    pub colon_token: token::Colon,
    pub ty: Box<crate::ty::Type>,
}
```

*Defined in [`syn-2.0.111/src/pat.rs:204-213`](../../.source_1765521767/syn-2.0.111/src/pat.rs#L204-L213)*

A type ascription pattern: `foo: f64`.

#### Implementations

- <span id="cratepattype-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for PatType`

- <span id="pattype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatType`

- <span id="pattype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatType`

- <span id="pattype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PatType`

- <span id="cratepattype-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PatType`

- <span id="pattype-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PatType`

- <span id="cratepattype-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatType`

##### `impl<T> From for PatType`

- <span id="pattype-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PatType`

- <span id="cratepattype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PatType`

- <span id="pattype-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::pat::PatType`

- <span id="cratepatpattype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::PatType`

- <span id="cratepattype-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatType`

##### `impl Spanned for PatType`

- <span id="pattype-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PatType`

- <span id="pattype-toowned-type-owned"></span>`type Owned = T`

- <span id="pattype-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pattype-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::pat::PatType`

- <span id="cratepatpattype-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PatType`

- <span id="pattype-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pattype-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatType`

- <span id="pattype-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pattype-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatWild`

```rust
struct PatWild {
    pub attrs: Vec<crate::attr::Attribute>,
    pub underscore_token: token::Underscore,
}
```

*Defined in [`syn-2.0.111/src/pat.rs:215-222`](../../.source_1765521767/syn-2.0.111/src/pat.rs#L215-L222)*

A pattern that matches any value: `_`.

#### Implementations

- <span id="cratepatwild-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for PatWild`

- <span id="patwild-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatWild`

- <span id="patwild-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatWild`

- <span id="patwild-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PatWild`

- <span id="cratepatwild-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PatWild`

- <span id="patwild-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PatWild`

- <span id="cratepatwild-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatWild`

##### `impl<T> From for PatWild`

- <span id="patwild-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PatWild`

- <span id="cratepatwild-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PatWild`

- <span id="patwild-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::PatWild`

- <span id="cratepatwild-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatWild`

##### `impl Spanned for PatWild`

- <span id="patwild-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PatWild`

- <span id="patwild-toowned-type-owned"></span>`type Owned = T`

- <span id="patwild-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="patwild-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::pat::PatWild`

- <span id="cratepatpatwild-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PatWild`

- <span id="patwild-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="patwild-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatWild`

- <span id="patwild-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="patwild-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AngleBracketedGenericArguments`

```rust
struct AngleBracketedGenericArguments {
    pub colon2_token: Option<token::PathSep>,
    pub lt_token: token::Lt,
    pub args: crate::punctuated::Punctuated<GenericArgument, token::Comma>,
    pub gt_token: token::Gt,
}
```

*Defined in [`syn-2.0.111/src/path.rs:196-206`](../../.source_1765521767/syn-2.0.111/src/path.rs#L196-L206)*

Angle bracketed arguments of a path segment: the `<K, V>` in `HashMap<K,
V>`.

#### Implementations

- <span id="cratepathanglebracketedgenericarguments-parse-turbofish"></span>`fn parse_turbofish(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

  Parse `::<…>` with mandatory leading `::`.

  

  The ordinary [`Parse`](parse/index.md) impl for `AngleBracketedGenericArguments`

  parses optional leading `::`.

- <span id="cratepathanglebracketedgenericarguments-do-parse"></span>`fn do_parse(colon2_token: Option<token::PathSep>, input: ParseStream<'_>) -> Result<Self>` — [`PathSep`](token/index.md#pathsep), [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

#### Trait Implementations

##### `impl Any for AngleBracketedGenericArguments`

- <span id="anglebracketedgenericarguments-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AngleBracketedGenericArguments`

- <span id="anglebracketedgenericarguments-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AngleBracketedGenericArguments`

- <span id="anglebracketedgenericarguments-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::AngleBracketedGenericArguments`

- <span id="crateanglebracketedgenericarguments-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for AngleBracketedGenericArguments`

- <span id="anglebracketedgenericarguments-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::AngleBracketedGenericArguments`

- <span id="crateanglebracketedgenericarguments-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::AngleBracketedGenericArguments`

##### `impl<T> From for AngleBracketedGenericArguments`

- <span id="anglebracketedgenericarguments-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::AngleBracketedGenericArguments`

- <span id="crateanglebracketedgenericarguments-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for AngleBracketedGenericArguments`

- <span id="anglebracketedgenericarguments-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::path::AngleBracketedGenericArguments`

- <span id="cratepathanglebracketedgenericarguments-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::AngleBracketedGenericArguments`

- <span id="crateanglebracketedgenericarguments-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for AngleBracketedGenericArguments`

##### `impl Spanned for AngleBracketedGenericArguments`

- <span id="anglebracketedgenericarguments-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for AngleBracketedGenericArguments`

- <span id="anglebracketedgenericarguments-toowned-type-owned"></span>`type Owned = T`

- <span id="anglebracketedgenericarguments-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="anglebracketedgenericarguments-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::path::AngleBracketedGenericArguments`

- <span id="cratepathanglebracketedgenericarguments-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for AngleBracketedGenericArguments`

- <span id="anglebracketedgenericarguments-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="anglebracketedgenericarguments-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AngleBracketedGenericArguments`

- <span id="anglebracketedgenericarguments-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="anglebracketedgenericarguments-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AssocConst`

```rust
struct AssocConst {
    pub ident: crate::ident::Ident,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub eq_token: token::Eq,
    pub value: crate::expr::Expr,
}
```

*Defined in [`syn-2.0.111/src/path.rs:220-230`](../../.source_1765521767/syn-2.0.111/src/path.rs#L220-L230)*

An equality constraint on an associated constant: the `PANIC = false` in
`Trait<PANIC = false>`.

#### Trait Implementations

##### `impl Any for AssocConst`

- <span id="assocconst-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AssocConst`

- <span id="assocconst-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AssocConst`

- <span id="assocconst-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::AssocConst`

- <span id="crateassocconst-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for AssocConst`

- <span id="assocconst-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::AssocConst`

- <span id="crateassocconst-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::AssocConst`

##### `impl<T> From for AssocConst`

- <span id="assocconst-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::AssocConst`

- <span id="crateassocconst-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for AssocConst`

- <span id="assocconst-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::AssocConst`

- <span id="crateassocconst-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for AssocConst`

##### `impl Spanned for AssocConst`

- <span id="assocconst-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for AssocConst`

- <span id="assocconst-toowned-type-owned"></span>`type Owned = T`

- <span id="assocconst-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="assocconst-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::path::AssocConst`

- <span id="cratepathassocconst-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for AssocConst`

- <span id="assocconst-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="assocconst-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AssocConst`

- <span id="assocconst-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="assocconst-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AssocType`

```rust
struct AssocType {
    pub ident: crate::ident::Ident,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub eq_token: token::Eq,
    pub ty: crate::ty::Type,
}
```

*Defined in [`syn-2.0.111/src/path.rs:208-218`](../../.source_1765521767/syn-2.0.111/src/path.rs#L208-L218)*

A binding (equality constraint) on an associated type: the `Item = u8`
in `Iterator<Item = u8>`.

#### Trait Implementations

##### `impl Any for AssocType`

- <span id="assoctype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AssocType`

- <span id="assoctype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AssocType`

- <span id="assoctype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::AssocType`

- <span id="crateassoctype-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for AssocType`

- <span id="assoctype-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::AssocType`

- <span id="crateassoctype-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::AssocType`

##### `impl<T> From for AssocType`

- <span id="assoctype-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::AssocType`

- <span id="crateassoctype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for AssocType`

- <span id="assoctype-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::AssocType`

- <span id="crateassoctype-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for AssocType`

##### `impl Spanned for AssocType`

- <span id="assoctype-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for AssocType`

- <span id="assoctype-toowned-type-owned"></span>`type Owned = T`

- <span id="assoctype-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="assoctype-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::path::AssocType`

- <span id="cratepathassoctype-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for AssocType`

- <span id="assoctype-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="assoctype-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AssocType`

- <span id="assoctype-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="assoctype-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Constraint`

```rust
struct Constraint {
    pub ident: crate::ident::Ident,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub colon_token: token::Colon,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, token::Plus>,
}
```

*Defined in [`syn-2.0.111/src/path.rs:232-241`](../../.source_1765521767/syn-2.0.111/src/path.rs#L232-L241)*

An associated type bound: `Iterator<Item: Display>`.

#### Trait Implementations

##### `impl Any for Constraint`

- <span id="constraint-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Constraint`

- <span id="constraint-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Constraint`

- <span id="constraint-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Constraint`

- <span id="crateconstraint-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Constraint`

- <span id="constraint-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Constraint`

- <span id="crateconstraint-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Constraint`

##### `impl<T> From for Constraint`

- <span id="constraint-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Constraint`

- <span id="crateconstraint-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Constraint`

- <span id="constraint-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::Constraint`

- <span id="crateconstraint-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Constraint`

##### `impl Spanned for Constraint`

- <span id="constraint-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Constraint`

- <span id="constraint-toowned-type-owned"></span>`type Owned = T`

- <span id="constraint-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="constraint-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::path::Constraint`

- <span id="cratepathconstraint-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Constraint`

- <span id="constraint-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="constraint-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Constraint`

- <span id="constraint-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="constraint-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ParenthesizedGenericArguments`

```rust
struct ParenthesizedGenericArguments {
    pub paren_token: token::Paren,
    pub inputs: crate::punctuated::Punctuated<crate::ty::Type, token::Comma>,
    pub output: crate::ty::ReturnType,
}
```

*Defined in [`syn-2.0.111/src/path.rs:243-254`](../../.source_1765521767/syn-2.0.111/src/path.rs#L243-L254)*

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

##### `impl Any for ParenthesizedGenericArguments`

- <span id="parenthesizedgenericarguments-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ParenthesizedGenericArguments`

- <span id="parenthesizedgenericarguments-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ParenthesizedGenericArguments`

- <span id="parenthesizedgenericarguments-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ParenthesizedGenericArguments`

- <span id="crateparenthesizedgenericarguments-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ParenthesizedGenericArguments`

- <span id="parenthesizedgenericarguments-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ParenthesizedGenericArguments`

- <span id="crateparenthesizedgenericarguments-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ParenthesizedGenericArguments`

##### `impl<T> From for ParenthesizedGenericArguments`

- <span id="parenthesizedgenericarguments-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ParenthesizedGenericArguments`

- <span id="crateparenthesizedgenericarguments-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ParenthesizedGenericArguments`

- <span id="parenthesizedgenericarguments-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::path::ParenthesizedGenericArguments`

- <span id="cratepathparenthesizedgenericarguments-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ParenthesizedGenericArguments`

- <span id="crateparenthesizedgenericarguments-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ParenthesizedGenericArguments`

##### `impl Spanned for ParenthesizedGenericArguments`

- <span id="parenthesizedgenericarguments-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ParenthesizedGenericArguments`

- <span id="parenthesizedgenericarguments-toowned-type-owned"></span>`type Owned = T`

- <span id="parenthesizedgenericarguments-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="parenthesizedgenericarguments-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::path::ParenthesizedGenericArguments`

- <span id="cratepathparenthesizedgenericarguments-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ParenthesizedGenericArguments`

- <span id="parenthesizedgenericarguments-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parenthesizedgenericarguments-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ParenthesizedGenericArguments`

- <span id="parenthesizedgenericarguments-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parenthesizedgenericarguments-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Path`

```rust
struct Path {
    pub leading_colon: Option<token::PathSep>,
    pub segments: crate::punctuated::Punctuated<PathSegment, token::PathSep>,
}
```

*Defined in [`syn-2.0.111/src/path.rs:11-18`](../../.source_1765521767/syn-2.0.111/src/path.rs#L11-L18)*

A path at which a named item is exported (e.g. `std::collections::HashMap`).

#### Implementations

- <span id="cratepathpath-parse-mod-style"></span>`fn parse_mod_style(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

  Parse a `Path` containing no path arguments on any of its segments.

  

  # Example

  

  ```rust

  use syn::{Path, Result, Token};

  use syn::parse::{Parse, ParseStream};

  

  // A simplified single `use` statement like:

  //

  //     use std::collections::HashMap;

  //

  // Note that generic parameters are not allowed in a `use` statement

  // so the following must not be accepted.

  //

  //     use a::<b>::c;

  struct SingleUse {

      use_token: Token![use],

      path: Path,

  }

  

  impl Parse for SingleUse {

      fn parse(input: ParseStream) -> Result<Self> {

          Ok(SingleUse {

              use_token: input.parse()?,

              path: input.call(Path::parse_mod_style)?,

          })

      }

  }

  ```

- <span id="cratepathpath-parse-helper"></span>`fn parse_helper(input: ParseStream<'_>, expr_style: bool) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

- <span id="cratepathpath-parse-rest"></span>`fn parse_rest(input: ParseStream<'_>, path: &mut Self, expr_style: bool) -> Result<()>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

- <span id="cratepathpath-is-mod-style"></span>`fn is_mod_style(&self) -> bool`

#### Trait Implementations

##### `impl Any for Path`

- <span id="path-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Path`

- <span id="path-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Path`

- <span id="path-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Path`

- <span id="cratepath-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Path`

- <span id="path-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Path`

- <span id="cratepath-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Path`

##### `impl<T> From for Path`

- <span id="path-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Path`

- <span id="cratepath-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Path`

- <span id="path-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::path::Path`

- <span id="cratepathpath-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::Path`

- <span id="cratepath-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialEq for syn::Path`

##### `impl Sealed for Path`

##### `impl Spanned for Path`

- <span id="path-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Path`

- <span id="path-toowned-type-owned"></span>`type Owned = T`

- <span id="path-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="path-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::path::Path`

- <span id="cratepathpath-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Path`

- <span id="path-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="path-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Path`

- <span id="path-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="path-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PathSegment`

```rust
struct PathSegment {
    pub ident: crate::ident::Ident,
    pub arguments: PathArguments,
}
```

*Defined in [`syn-2.0.111/src/path.rs:107-114`](../../.source_1765521767/syn-2.0.111/src/path.rs#L107-L114)*

A segment of a path together with any path arguments on that segment.

#### Implementations

- <span id="cratepathpathsegment-parse-helper"></span>`fn parse_helper(input: ParseStream<'_>, expr_style: bool) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

#### Trait Implementations

##### `impl Any for PathSegment`

- <span id="pathsegment-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PathSegment`

- <span id="pathsegment-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PathSegment`

- <span id="pathsegment-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PathSegment`

- <span id="cratepathsegment-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PathSegment`

- <span id="pathsegment-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PathSegment`

- <span id="cratepathsegment-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PathSegment`

##### `impl<T> From for PathSegment`

- <span id="pathsegment-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PathSegment`

- <span id="cratepathsegment-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PathSegment`

- <span id="pathsegment-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::path::PathSegment`

- <span id="cratepathpathsegment-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::PathSegment`

- <span id="cratepathsegment-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PathSegment`

##### `impl Spanned for PathSegment`

- <span id="pathsegment-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PathSegment`

- <span id="pathsegment-toowned-type-owned"></span>`type Owned = T`

- <span id="pathsegment-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pathsegment-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::path::PathSegment`

- <span id="cratepathpathsegment-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PathSegment`

- <span id="pathsegment-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pathsegment-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PathSegment`

- <span id="pathsegment-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pathsegment-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/path.rs:256-281`](../../.source_1765521767/syn-2.0.111/src/path.rs#L256-L281)*

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

##### `impl Any for QSelf`

- <span id="qself-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for QSelf`

- <span id="qself-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for QSelf`

- <span id="qself-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::QSelf`

- <span id="crateqself-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for QSelf`

- <span id="qself-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::QSelf`

- <span id="crateqself-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::QSelf`

##### `impl<T> From for QSelf`

- <span id="qself-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::QSelf`

- <span id="crateqself-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for QSelf`

- <span id="qself-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::QSelf`

- <span id="crateqself-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for crate::QSelf`

##### `impl Spanned for crate::path::QSelf`

- <span id="cratepathqself-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for QSelf`

- <span id="qself-toowned-type-owned"></span>`type Owned = T`

- <span id="qself-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="qself-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for QSelf`

- <span id="qself-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="qself-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for QSelf`

- <span id="qself-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="qself-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `VisRestricted`

```rust
struct VisRestricted {
    pub pub_token: token::Pub,
    pub paren_token: token::Paren,
    pub in_token: Option<token::In>,
    pub path: Box<crate::path::Path>,
}
```

*Defined in [`syn-2.0.111/src/restriction.rs:27-37`](../../.source_1765521767/syn-2.0.111/src/restriction.rs#L27-L37)*

A visibility level restricted to some path: `pub(self)` or
`pub(super)` or `pub(crate)` or `pub(in some::module)`.

#### Implementations

- <span id="cratevisrestricted-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for VisRestricted`

- <span id="visrestricted-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VisRestricted`

- <span id="visrestricted-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VisRestricted`

- <span id="visrestricted-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::VisRestricted`

- <span id="cratevisrestricted-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for VisRestricted`

- <span id="visrestricted-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::VisRestricted`

- <span id="cratevisrestricted-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::VisRestricted`

##### `impl<T> From for VisRestricted`

- <span id="visrestricted-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::VisRestricted`

- <span id="cratevisrestricted-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for VisRestricted`

- <span id="visrestricted-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::VisRestricted`

- <span id="cratevisrestricted-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for VisRestricted`

##### `impl Spanned for VisRestricted`

- <span id="visrestricted-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for VisRestricted`

- <span id="visrestricted-toowned-type-owned"></span>`type Owned = T`

- <span id="visrestricted-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="visrestricted-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::restriction::VisRestricted`

- <span id="craterestrictionvisrestricted-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for VisRestricted`

- <span id="visrestricted-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="visrestricted-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VisRestricted`

- <span id="visrestricted-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="visrestricted-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Block`

```rust
struct Block {
    pub brace_token: token::Brace,
    pub stmts: Vec<Stmt>,
}
```

*Defined in [`syn-2.0.111/src/stmt.rs:8-16`](../../.source_1765521767/syn-2.0.111/src/stmt.rs#L8-L16)*

A braced block containing Rust statements.

#### Fields

- **`stmts`**: `Vec<Stmt>`

  Statements in a block

#### Implementations

- <span id="cratestmtblock-parse-within"></span>`fn parse_within(input: ParseStream<'_>) -> Result<Vec<Stmt>>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result), [`Stmt`](stmt/index.md#stmt)

  Parse the body of a block as zero or more statements, possibly

  including one trailing expression.

  

  # Example

  

  ```rust

  use syn::{braced, token, Attribute, Block, Ident, Result, Stmt, Token};

  use syn::parse::{Parse, ParseStream};

  

  // Parse a function with no generics or parameter list.

  //

  //     fn playground {

  //         let mut x = 1;

  //         x += 1;

  //         println!("{}", x);

  //     }

  struct MiniFunction {

      attrs: Vec<Attribute>,

      fn_token: Token![fn],

      name: Ident,

      brace_token: token::Brace,

      stmts: Vec<Stmt>,

  }

  

  impl Parse for MiniFunction {

      fn parse(input: ParseStream) -> Result<Self> {

          let outer_attrs = input.call(Attribute::parse_outer)?;

          let fn_token: Token![fn] = input.parse()?;

          let name: Ident = input.parse()?;

  

          let content;

          let brace_token = braced!(content in input);

          let inner_attrs = content.call(Attribute::parse_inner)?;

          let stmts = content.call(Block::parse_within)?;

  

          Ok(MiniFunction {

              attrs: {

                  let mut attrs = outer_attrs;

                  attrs.extend(inner_attrs);

                  attrs

              },

              fn_token,

              name,

              brace_token,

              stmts,

          })

      }

  }

  ```

#### Trait Implementations

##### `impl Any for Block`

- <span id="block-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Block`

- <span id="block-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Block`

- <span id="block-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Block`

- <span id="crateblock-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Block`

- <span id="block-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Block`

- <span id="crateblock-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Block`

##### `impl<T> From for Block`

- <span id="block-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Block`

- <span id="crateblock-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Block`

- <span id="block-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::stmt::Block`

- <span id="cratestmtblock-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::Block`

- <span id="crateblock-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Block`

##### `impl Spanned for Block`

- <span id="block-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Block`

- <span id="block-toowned-type-owned"></span>`type Owned = T`

- <span id="block-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="block-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::stmt::Block`

- <span id="cratestmtblock-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Block`

- <span id="block-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="block-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Block`

- <span id="block-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="block-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/stmt.rs:40-50`](../../.source_1765521767/syn-2.0.111/src/stmt.rs#L40-L50)*

A local `let` binding: `let x: u64 = s.parse()?;`.

#### Implementations

- <span id="cratelocal-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for Local`

- <span id="local-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Local`

- <span id="local-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Local`

- <span id="local-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Local`

- <span id="cratelocal-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Local`

- <span id="local-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Local`

- <span id="cratelocal-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Local`

##### `impl<T> From for Local`

- <span id="local-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Local`

- <span id="cratelocal-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Local`

- <span id="local-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::Local`

- <span id="cratelocal-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Local`

##### `impl Spanned for Local`

- <span id="local-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Local`

- <span id="local-toowned-type-owned"></span>`type Owned = T`

- <span id="local-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="local-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::stmt::Local`

- <span id="cratestmtlocal-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Local`

- <span id="local-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="local-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Local`

- <span id="local-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="local-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LocalInit`

```rust
struct LocalInit {
    pub eq_token: token::Eq,
    pub expr: Box<crate::expr::Expr>,
    pub diverge: Option<(token::Else, Box<crate::expr::Expr>)>,
}
```

*Defined in [`syn-2.0.111/src/stmt.rs:52-64`](../../.source_1765521767/syn-2.0.111/src/stmt.rs#L52-L64)*

The expression assigned in a local `let` binding, including optional
diverging `else` block.

`LocalInit` represents `= s.parse()?` in `let x: u64 = s.parse()?` and
`= r else { return }` in `let Ok(x) = r else { return }`.

#### Trait Implementations

##### `impl Any for LocalInit`

- <span id="localinit-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LocalInit`

- <span id="localinit-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LocalInit`

- <span id="localinit-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::LocalInit`

- <span id="cratelocalinit-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for LocalInit`

- <span id="localinit-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::LocalInit`

- <span id="cratelocalinit-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LocalInit`

##### `impl<T> From for LocalInit`

- <span id="localinit-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::LocalInit`

- <span id="cratelocalinit-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for LocalInit`

- <span id="localinit-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::LocalInit`

- <span id="cratelocalinit-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl ToOwned for LocalInit`

- <span id="localinit-toowned-type-owned"></span>`type Owned = T`

- <span id="localinit-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="localinit-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LocalInit`

- <span id="localinit-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="localinit-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LocalInit`

- <span id="localinit-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="localinit-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StmtMacro`

```rust
struct StmtMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<token::Semi>,
}
```

*Defined in [`syn-2.0.111/src/stmt.rs:66-78`](../../.source_1765521767/syn-2.0.111/src/stmt.rs#L66-L78)*

A macro invocation in statement position.

Syntactically it's ambiguous which other kind of statement this macro
would expand to. It can be any of local variable (`let`), item, or
expression.

#### Implementations

- <span id="cratestmtmacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for StmtMacro`

- <span id="stmtmacro-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StmtMacro`

- <span id="stmtmacro-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StmtMacro`

- <span id="stmtmacro-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::StmtMacro`

- <span id="cratestmtmacro-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for StmtMacro`

- <span id="stmtmacro-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::StmtMacro`

- <span id="cratestmtmacro-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::StmtMacro`

##### `impl<T> From for StmtMacro`

- <span id="stmtmacro-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::StmtMacro`

- <span id="cratestmtmacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for StmtMacro`

- <span id="stmtmacro-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::StmtMacro`

- <span id="cratestmtmacro-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for StmtMacro`

##### `impl Spanned for StmtMacro`

- <span id="stmtmacro-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for StmtMacro`

- <span id="stmtmacro-toowned-type-owned"></span>`type Owned = T`

- <span id="stmtmacro-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="stmtmacro-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::stmt::StmtMacro`

- <span id="cratestmtstmtmacro-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for StmtMacro`

- <span id="stmtmacro-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stmtmacro-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StmtMacro`

- <span id="stmtmacro-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stmtmacro-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Abi`

```rust
struct Abi {
    pub extern_token: token::Extern,
    pub name: Option<crate::lit::LitStr>,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:230-237`](../../.source_1765521767/syn-2.0.111/src/ty.rs#L230-L237)*

The binary interface of a function: `extern "C"`.

#### Trait Implementations

##### `impl Any for Abi`

- <span id="abi-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Abi`

- <span id="abi-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Abi`

- <span id="abi-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Abi`

- <span id="crateabi-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Abi`

- <span id="abi-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Abi`

- <span id="crateabi-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Abi`

##### `impl<T> From for Abi`

- <span id="abi-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Abi`

- <span id="crateabi-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Abi`

- <span id="abi-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::Abi`

- <span id="cratetyabi-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::Abi`

- <span id="crateabi-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Abi`

##### `impl Spanned for Abi`

- <span id="abi-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Abi`

- <span id="abi-toowned-type-owned"></span>`type Owned = T`

- <span id="abi-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="abi-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::Abi`

- <span id="cratetyabi-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Abi`

- <span id="abi-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="abi-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Abi`

- <span id="abi-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="abi-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BareFnArg`

```rust
struct BareFnArg {
    pub attrs: Vec<crate::attr::Attribute>,
    pub name: Option<(crate::ident::Ident, token::Colon)>,
    pub ty: Type,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:239-247`](../../.source_1765521767/syn-2.0.111/src/ty.rs#L239-L247)*

An argument in a function type: the `usize` in `fn(usize) -> bool`.

#### Trait Implementations

##### `impl Any for BareFnArg`

- <span id="barefnarg-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BareFnArg`

- <span id="barefnarg-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BareFnArg`

- <span id="barefnarg-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::BareFnArg`

- <span id="cratebarefnarg-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for BareFnArg`

- <span id="barefnarg-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::BareFnArg`

- <span id="cratebarefnarg-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::BareFnArg`

##### `impl<T> From for BareFnArg`

- <span id="barefnarg-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::BareFnArg`

- <span id="cratebarefnarg-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for BareFnArg`

- <span id="barefnarg-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::BareFnArg`

- <span id="cratetybarefnarg-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::BareFnArg`

- <span id="cratebarefnarg-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for BareFnArg`

##### `impl Spanned for BareFnArg`

- <span id="barefnarg-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for BareFnArg`

- <span id="barefnarg-toowned-type-owned"></span>`type Owned = T`

- <span id="barefnarg-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="barefnarg-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::BareFnArg`

- <span id="cratetybarefnarg-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for BareFnArg`

- <span id="barefnarg-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="barefnarg-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BareFnArg`

- <span id="barefnarg-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="barefnarg-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BareVariadic`

```rust
struct BareVariadic {
    pub attrs: Vec<crate::attr::Attribute>,
    pub name: Option<(crate::ident::Ident, token::Colon)>,
    pub dots: token::DotDotDot,
    pub comma: Option<token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:249-258`](../../.source_1765521767/syn-2.0.111/src/ty.rs#L249-L258)*

The variadic argument of a function pointer like `fn(usize, ...)`.

#### Trait Implementations

##### `impl Any for BareVariadic`

- <span id="barevariadic-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BareVariadic`

- <span id="barevariadic-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BareVariadic`

- <span id="barevariadic-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::BareVariadic`

- <span id="cratebarevariadic-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for BareVariadic`

- <span id="barevariadic-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::BareVariadic`

- <span id="cratebarevariadic-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::BareVariadic`

##### `impl<T> From for BareVariadic`

- <span id="barevariadic-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::BareVariadic`

- <span id="cratebarevariadic-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for BareVariadic`

- <span id="barevariadic-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::BareVariadic`

- <span id="cratebarevariadic-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for BareVariadic`

##### `impl Spanned for BareVariadic`

- <span id="barevariadic-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for BareVariadic`

- <span id="barevariadic-toowned-type-owned"></span>`type Owned = T`

- <span id="barevariadic-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="barevariadic-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::BareVariadic`

- <span id="cratetybarevariadic-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for BareVariadic`

- <span id="barevariadic-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="barevariadic-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BareVariadic`

- <span id="barevariadic-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="barevariadic-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypeArray`

```rust
struct TypeArray {
    pub bracket_token: token::Bracket,
    pub elem: Box<Type>,
    pub semi_token: token::Semi,
    pub len: crate::expr::Expr,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:92-101`](../../.source_1765521767/syn-2.0.111/src/ty.rs#L92-L101)*

A fixed size array type: `[T; n]`.

#### Implementations

- <span id="cratetypearray-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TypeArray`

- <span id="typearray-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeArray`

- <span id="typearray-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeArray`

- <span id="typearray-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypeArray`

- <span id="cratetypearray-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeArray`

- <span id="typearray-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypeArray`

- <span id="cratetypearray-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeArray`

##### `impl<T> From for TypeArray`

- <span id="typearray-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypeArray`

- <span id="cratetypearray-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TypeArray`

- <span id="typearray-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::TypeArray`

- <span id="cratetytypearray-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::TypeArray`

- <span id="cratetypearray-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeArray`

##### `impl Spanned for TypeArray`

- <span id="typearray-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeArray`

- <span id="typearray-toowned-type-owned"></span>`type Owned = T`

- <span id="typearray-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typearray-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::TypeArray`

- <span id="cratetytypearray-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypeArray`

- <span id="typearray-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typearray-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeArray`

- <span id="typearray-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typearray-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/ty.rs:103-116`](../../.source_1765521767/syn-2.0.111/src/ty.rs#L103-L116)*

A bare function type: `fn(usize) -> bool`.

#### Implementations

- <span id="cratetypebarefn-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TypeBareFn`

- <span id="typebarefn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeBareFn`

- <span id="typebarefn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeBareFn`

- <span id="typebarefn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypeBareFn`

- <span id="cratetypebarefn-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeBareFn`

- <span id="typebarefn-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypeBareFn`

- <span id="cratetypebarefn-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeBareFn`

##### `impl<T> From for TypeBareFn`

- <span id="typebarefn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypeBareFn`

- <span id="cratetypebarefn-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TypeBareFn`

- <span id="typebarefn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::TypeBareFn`

- <span id="cratetytypebarefn-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::TypeBareFn`

- <span id="cratetypebarefn-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeBareFn`

##### `impl Spanned for TypeBareFn`

- <span id="typebarefn-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeBareFn`

- <span id="typebarefn-toowned-type-owned"></span>`type Owned = T`

- <span id="typebarefn-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typebarefn-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::TypeBareFn`

- <span id="cratetytypebarefn-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypeBareFn`

- <span id="typebarefn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typebarefn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeBareFn`

- <span id="typebarefn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typebarefn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypeGroup`

```rust
struct TypeGroup {
    pub group_token: token::Group,
    pub elem: Box<Type>,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:118-125`](../../.source_1765521767/syn-2.0.111/src/ty.rs#L118-L125)*

A type contained within invisible delimiters.

#### Implementations

- <span id="cratetypegroup-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TypeGroup`

- <span id="typegroup-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeGroup`

- <span id="typegroup-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeGroup`

- <span id="typegroup-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypeGroup`

- <span id="cratetypegroup-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeGroup`

- <span id="typegroup-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypeGroup`

- <span id="cratetypegroup-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeGroup`

##### `impl<T> From for TypeGroup`

- <span id="typegroup-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypeGroup`

- <span id="cratetypegroup-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TypeGroup`

- <span id="typegroup-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::TypeGroup`

- <span id="cratetytypegroup-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::TypeGroup`

- <span id="cratetypegroup-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeGroup`

##### `impl Spanned for TypeGroup`

- <span id="typegroup-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeGroup`

- <span id="typegroup-toowned-type-owned"></span>`type Owned = T`

- <span id="typegroup-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typegroup-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::TypeGroup`

- <span id="cratetytypegroup-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypeGroup`

- <span id="typegroup-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typegroup-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeGroup`

- <span id="typegroup-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typegroup-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypeImplTrait`

```rust
struct TypeImplTrait {
    pub impl_token: token::Impl,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, token::Plus>,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:127-135`](../../.source_1765521767/syn-2.0.111/src/ty.rs#L127-L135)*

An `impl Bound1 + Bound2 + Bound3` type where `Bound` is a trait or
a lifetime.

#### Implementations

- <span id="cratetytypeimpltrait-without-plus"></span>`fn without_plus(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

- <span id="cratetytypeimpltrait-parse"></span>`fn parse(input: ParseStream<'_>, allow_plus: bool) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

#### Trait Implementations

##### `impl Any for TypeImplTrait`

- <span id="typeimpltrait-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeImplTrait`

- <span id="typeimpltrait-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeImplTrait`

- <span id="typeimpltrait-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypeImplTrait`

- <span id="cratetypeimpltrait-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeImplTrait`

- <span id="typeimpltrait-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypeImplTrait`

- <span id="cratetypeimpltrait-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeImplTrait`

##### `impl<T> From for TypeImplTrait`

- <span id="typeimpltrait-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypeImplTrait`

- <span id="cratetypeimpltrait-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TypeImplTrait`

- <span id="typeimpltrait-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::TypeImplTrait`

- <span id="cratetytypeimpltrait-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::TypeImplTrait`

- <span id="cratetypeimpltrait-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeImplTrait`

##### `impl Spanned for TypeImplTrait`

- <span id="typeimpltrait-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeImplTrait`

- <span id="typeimpltrait-toowned-type-owned"></span>`type Owned = T`

- <span id="typeimpltrait-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typeimpltrait-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::TypeImplTrait`

- <span id="cratetytypeimpltrait-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypeImplTrait`

- <span id="typeimpltrait-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typeimpltrait-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeImplTrait`

- <span id="typeimpltrait-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typeimpltrait-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypeInfer`

```rust
struct TypeInfer {
    pub underscore_token: token::Underscore,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:137-143`](../../.source_1765521767/syn-2.0.111/src/ty.rs#L137-L143)*

Indication that a type should be inferred by the compiler: `_`.

#### Implementations

- <span id="cratetypeinfer-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TypeInfer`

- <span id="typeinfer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeInfer`

- <span id="typeinfer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeInfer`

- <span id="typeinfer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypeInfer`

- <span id="cratetypeinfer-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeInfer`

- <span id="typeinfer-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypeInfer`

- <span id="cratetypeinfer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeInfer`

##### `impl<T> From for TypeInfer`

- <span id="typeinfer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypeInfer`

- <span id="cratetypeinfer-hash"></span>`fn hash<H>(&self, _state: &mut H)`

##### `impl<U> Into for TypeInfer`

- <span id="typeinfer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::TypeInfer`

- <span id="cratetytypeinfer-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::TypeInfer`

- <span id="cratetypeinfer-partialeq-eq"></span>`fn eq(&self, _other: &Self) -> bool`

##### `impl Sealed for TypeInfer`

##### `impl Spanned for TypeInfer`

- <span id="typeinfer-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeInfer`

- <span id="typeinfer-toowned-type-owned"></span>`type Owned = T`

- <span id="typeinfer-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typeinfer-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::TypeInfer`

- <span id="cratetytypeinfer-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypeInfer`

- <span id="typeinfer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typeinfer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeInfer`

- <span id="typeinfer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typeinfer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypeMacro`

```rust
struct TypeMacro {
    pub mac: crate::mac::Macro,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:145-151`](../../.source_1765521767/syn-2.0.111/src/ty.rs#L145-L151)*

A macro in the type position.

#### Implementations

- <span id="cratetypemacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TypeMacro`

- <span id="typemacro-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeMacro`

- <span id="typemacro-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeMacro`

- <span id="typemacro-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypeMacro`

- <span id="cratetypemacro-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeMacro`

- <span id="typemacro-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypeMacro`

- <span id="cratetypemacro-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeMacro`

##### `impl<T> From for TypeMacro`

- <span id="typemacro-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypeMacro`

- <span id="cratetypemacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TypeMacro`

- <span id="typemacro-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::TypeMacro`

- <span id="cratetytypemacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::TypeMacro`

- <span id="cratetypemacro-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeMacro`

##### `impl Spanned for TypeMacro`

- <span id="typemacro-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeMacro`

- <span id="typemacro-toowned-type-owned"></span>`type Owned = T`

- <span id="typemacro-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typemacro-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::TypeMacro`

- <span id="cratetytypemacro-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypeMacro`

- <span id="typemacro-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typemacro-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeMacro`

- <span id="typemacro-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typemacro-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypeNever`

```rust
struct TypeNever {
    pub bang_token: token::Not,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:153-159`](../../.source_1765521767/syn-2.0.111/src/ty.rs#L153-L159)*

The never type: `!`.

#### Implementations

- <span id="cratetypenever-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TypeNever`

- <span id="typenever-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeNever`

- <span id="typenever-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeNever`

- <span id="typenever-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypeNever`

- <span id="cratetypenever-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeNever`

- <span id="typenever-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypeNever`

- <span id="cratetypenever-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeNever`

##### `impl<T> From for TypeNever`

- <span id="typenever-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypeNever`

- <span id="cratetypenever-hash"></span>`fn hash<H>(&self, _state: &mut H)`

##### `impl<U> Into for TypeNever`

- <span id="typenever-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::TypeNever`

- <span id="cratetytypenever-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::TypeNever`

- <span id="cratetypenever-partialeq-eq"></span>`fn eq(&self, _other: &Self) -> bool`

##### `impl Sealed for TypeNever`

##### `impl Spanned for TypeNever`

- <span id="typenever-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeNever`

- <span id="typenever-toowned-type-owned"></span>`type Owned = T`

- <span id="typenever-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typenever-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::TypeNever`

- <span id="cratetytypenever-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypeNever`

- <span id="typenever-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typenever-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeNever`

- <span id="typenever-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typenever-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypeParen`

```rust
struct TypeParen {
    pub paren_token: token::Paren,
    pub elem: Box<Type>,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:161-168`](../../.source_1765521767/syn-2.0.111/src/ty.rs#L161-L168)*

A parenthesized type equivalent to the inner type.

#### Implementations

- <span id="cratetytypeparen-parse"></span>`fn parse(input: ParseStream<'_>, allow_plus: bool) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

#### Trait Implementations

##### `impl Any for TypeParen`

- <span id="typeparen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeParen`

- <span id="typeparen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeParen`

- <span id="typeparen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypeParen`

- <span id="cratetypeparen-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeParen`

- <span id="typeparen-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypeParen`

- <span id="cratetypeparen-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeParen`

##### `impl<T> From for TypeParen`

- <span id="typeparen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypeParen`

- <span id="cratetypeparen-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TypeParen`

- <span id="typeparen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::TypeParen`

- <span id="cratetytypeparen-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::TypeParen`

- <span id="cratetypeparen-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeParen`

##### `impl Spanned for TypeParen`

- <span id="typeparen-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeParen`

- <span id="typeparen-toowned-type-owned"></span>`type Owned = T`

- <span id="typeparen-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typeparen-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::TypeParen`

- <span id="cratetytypeparen-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypeParen`

- <span id="typeparen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typeparen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeParen`

- <span id="typeparen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typeparen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypePath`

```rust
struct TypePath {
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:170-178`](../../.source_1765521767/syn-2.0.111/src/ty.rs#L170-L178)*

A path like `std::slice::Iter`, optionally qualified with a
self-type as in `<Vec<T> as SomeTrait>::Associated`.

#### Implementations

- <span id="cratetypepath-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TypePath`

- <span id="typepath-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypePath`

- <span id="typepath-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypePath`

- <span id="typepath-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypePath`

- <span id="cratetypepath-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypePath`

- <span id="typepath-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypePath`

- <span id="cratetypepath-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypePath`

##### `impl<T> From for TypePath`

- <span id="typepath-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypePath`

- <span id="cratetypepath-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TypePath`

- <span id="typepath-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::TypePath`

- <span id="cratetytypepath-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::TypePath`

- <span id="cratetypepath-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypePath`

##### `impl Spanned for TypePath`

- <span id="typepath-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypePath`

- <span id="typepath-toowned-type-owned"></span>`type Owned = T`

- <span id="typepath-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typepath-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::TypePath`

- <span id="cratetytypepath-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypePath`

- <span id="typepath-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typepath-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypePath`

- <span id="typepath-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typepath-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypePtr`

```rust
struct TypePtr {
    pub star_token: token::Star,
    pub const_token: Option<token::Const>,
    pub mutability: Option<token::Mut>,
    pub elem: Box<Type>,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:180-189`](../../.source_1765521767/syn-2.0.111/src/ty.rs#L180-L189)*

A raw pointer type: `*const T` or `*mut T`.

#### Implementations

- <span id="cratetypeptr-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TypePtr`

- <span id="typeptr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypePtr`

- <span id="typeptr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypePtr`

- <span id="typeptr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypePtr`

- <span id="cratetypeptr-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypePtr`

- <span id="typeptr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypePtr`

- <span id="cratetypeptr-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypePtr`

##### `impl<T> From for TypePtr`

- <span id="typeptr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypePtr`

- <span id="cratetypeptr-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TypePtr`

- <span id="typeptr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::TypePtr`

- <span id="cratetytypeptr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::TypePtr`

- <span id="cratetypeptr-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypePtr`

##### `impl Spanned for TypePtr`

- <span id="typeptr-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypePtr`

- <span id="typeptr-toowned-type-owned"></span>`type Owned = T`

- <span id="typeptr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typeptr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::TypePtr`

- <span id="cratetytypeptr-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypePtr`

- <span id="typeptr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typeptr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypePtr`

- <span id="typeptr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typeptr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypeReference`

```rust
struct TypeReference {
    pub and_token: token::And,
    pub lifetime: Option<crate::lifetime::Lifetime>,
    pub mutability: Option<token::Mut>,
    pub elem: Box<Type>,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:191-200`](../../.source_1765521767/syn-2.0.111/src/ty.rs#L191-L200)*

A reference type: `&'a T` or `&'a mut T`.

#### Implementations

- <span id="cratetypereference-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TypeReference`

- <span id="typereference-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeReference`

- <span id="typereference-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeReference`

- <span id="typereference-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypeReference`

- <span id="cratetypereference-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeReference`

- <span id="typereference-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypeReference`

- <span id="cratetypereference-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeReference`

##### `impl<T> From for TypeReference`

- <span id="typereference-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypeReference`

- <span id="cratetypereference-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TypeReference`

- <span id="typereference-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::TypeReference`

- <span id="cratetytypereference-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::TypeReference`

- <span id="cratetypereference-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeReference`

##### `impl Spanned for TypeReference`

- <span id="typereference-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeReference`

- <span id="typereference-toowned-type-owned"></span>`type Owned = T`

- <span id="typereference-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typereference-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::TypeReference`

- <span id="cratetytypereference-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypeReference`

- <span id="typereference-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typereference-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeReference`

- <span id="typereference-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typereference-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypeSlice`

```rust
struct TypeSlice {
    pub bracket_token: token::Bracket,
    pub elem: Box<Type>,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:202-209`](../../.source_1765521767/syn-2.0.111/src/ty.rs#L202-L209)*

A dynamically sized slice type: `[T]`.

#### Implementations

- <span id="cratetypeslice-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TypeSlice`

- <span id="typeslice-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeSlice`

- <span id="typeslice-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeSlice`

- <span id="typeslice-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypeSlice`

- <span id="cratetypeslice-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeSlice`

- <span id="typeslice-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypeSlice`

- <span id="cratetypeslice-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeSlice`

##### `impl<T> From for TypeSlice`

- <span id="typeslice-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypeSlice`

- <span id="cratetypeslice-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TypeSlice`

- <span id="typeslice-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::TypeSlice`

- <span id="cratetytypeslice-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::TypeSlice`

- <span id="cratetypeslice-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeSlice`

##### `impl Spanned for TypeSlice`

- <span id="typeslice-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeSlice`

- <span id="typeslice-toowned-type-owned"></span>`type Owned = T`

- <span id="typeslice-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typeslice-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::TypeSlice`

- <span id="cratetytypeslice-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypeSlice`

- <span id="typeslice-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typeslice-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeSlice`

- <span id="typeslice-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typeslice-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypeTraitObject`

```rust
struct TypeTraitObject {
    pub dyn_token: Option<token::Dyn>,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, token::Plus>,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:211-219`](../../.source_1765521767/syn-2.0.111/src/ty.rs#L211-L219)*

A trait object type `dyn Bound1 + Bound2 + Bound3` where `Bound` is a
trait or a lifetime.

#### Implementations

- <span id="cratetytypetraitobject-without-plus"></span>`fn without_plus(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

- <span id="cratetytypetraitobject-parse"></span>`fn parse(input: ParseStream<'_>, allow_plus: bool) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

- <span id="cratetytypetraitobject-parse-bounds"></span>`fn parse_bounds(dyn_span: Span, input: ParseStream<'_>, allow_plus: bool) -> Result<Punctuated<TypeParamBound, token::Plus>>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result), [`Punctuated`](punctuated/index.md#punctuated), [`TypeParamBound`](generics/index.md#typeparambound), [`Plus`](token/index.md#plus)

#### Trait Implementations

##### `impl Any for TypeTraitObject`

- <span id="typetraitobject-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeTraitObject`

- <span id="typetraitobject-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeTraitObject`

- <span id="typetraitobject-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypeTraitObject`

- <span id="cratetypetraitobject-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeTraitObject`

- <span id="typetraitobject-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypeTraitObject`

- <span id="cratetypetraitobject-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeTraitObject`

##### `impl<T> From for TypeTraitObject`

- <span id="typetraitobject-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypeTraitObject`

- <span id="cratetypetraitobject-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TypeTraitObject`

- <span id="typetraitobject-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::TypeTraitObject`

- <span id="cratetytypetraitobject-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::TypeTraitObject`

- <span id="cratetypetraitobject-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeTraitObject`

##### `impl Spanned for TypeTraitObject`

- <span id="typetraitobject-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeTraitObject`

- <span id="typetraitobject-toowned-type-owned"></span>`type Owned = T`

- <span id="typetraitobject-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typetraitobject-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::TypeTraitObject`

- <span id="cratetytypetraitobject-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypeTraitObject`

- <span id="typetraitobject-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typetraitobject-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeTraitObject`

- <span id="typetraitobject-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typetraitobject-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypeTuple`

```rust
struct TypeTuple {
    pub paren_token: token::Paren,
    pub elems: crate::punctuated::Punctuated<Type, token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:221-228`](../../.source_1765521767/syn-2.0.111/src/ty.rs#L221-L228)*

A tuple type: `(A, B, C, String)`.

#### Implementations

- <span id="cratetypetuple-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TypeTuple`

- <span id="typetuple-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeTuple`

- <span id="typetuple-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeTuple`

- <span id="typetuple-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypeTuple`

- <span id="cratetypetuple-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeTuple`

- <span id="typetuple-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypeTuple`

- <span id="cratetypetuple-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeTuple`

##### `impl<T> From for TypeTuple`

- <span id="typetuple-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypeTuple`

- <span id="cratetypetuple-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TypeTuple`

- <span id="typetuple-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::TypeTuple`

- <span id="cratetytypetuple-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::TypeTuple`

- <span id="cratetypetuple-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeTuple`

##### `impl Spanned for TypeTuple`

- <span id="typetuple-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeTuple`

- <span id="typetuple-toowned-type-owned"></span>`type Owned = T`

- <span id="typetuple-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typetuple-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::TypeTuple`

- <span id="cratetytypetuple-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypeTuple`

- <span id="typetuple-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typetuple-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeTuple`

- <span id="typetuple-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typetuple-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `AttrStyle`

```rust
enum AttrStyle {
    Outer,
    Inner(token::Not),
}
```

*Defined in [`syn-2.0.111/src/attr.rs:429-449`](../../.source_1765521767/syn-2.0.111/src/attr.rs#L429-L449)*

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

##### `impl Any for AttrStyle`

- <span id="attrstyle-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AttrStyle`

- <span id="attrstyle-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AttrStyle`

- <span id="attrstyle-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::AttrStyle`

- <span id="crateattrstyle-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for AttrStyle`

- <span id="attrstyle-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for crate::AttrStyle`

##### `impl Debug for crate::AttrStyle`

- <span id="crateattrstyle-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::AttrStyle`

##### `impl<T> From for AttrStyle`

- <span id="attrstyle-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::AttrStyle`

- <span id="crateattrstyle-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for AttrStyle`

- <span id="attrstyle-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::AttrStyle`

- <span id="crateattrstyle-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl ToOwned for AttrStyle`

- <span id="attrstyle-toowned-type-owned"></span>`type Owned = T`

- <span id="attrstyle-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="attrstyle-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AttrStyle`

- <span id="attrstyle-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="attrstyle-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AttrStyle`

- <span id="attrstyle-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="attrstyle-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Meta`

```rust
enum Meta {
    Path(crate::path::Path),
    List(MetaList),
    NameValue(MetaNameValue),
}
```

*Defined in [`syn-2.0.111/src/attr.rs:451-482`](../../.source_1765521767/syn-2.0.111/src/attr.rs#L451-L482)*

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

- <span id="meta-path"></span>`fn path(&self) -> &Path` — [`Path`](path/index.md#path)

  Returns the path that begins this structured meta item.

  

  For example this would return the `test` in `#[test]`, the `derive` in

  `#[derive(Copy)]`, and the `path` in `#[path = "sys/windows.rs"]`.

- <span id="meta-require-path-only"></span>`fn require_path_only(&self) -> Result<&Path>` — [`Result`](error/index.md#result), [`Path`](path/index.md#path)

  Error if this is a `Meta::List` or `Meta::NameValue`.

- <span id="meta-require-list"></span>`fn require_list(&self) -> Result<&MetaList>` — [`Result`](error/index.md#result), [`MetaList`](attr/index.md#metalist)

  Error if this is a `Meta::Path` or `Meta::NameValue`.

- <span id="meta-require-name-value"></span>`fn require_name_value(&self) -> Result<&MetaNameValue>` — [`Result`](error/index.md#result), [`MetaNameValue`](attr/index.md#metanamevalue)

  Error if this is a `Meta::Path` or `Meta::List`.

#### Trait Implementations

##### `impl Any for Meta`

- <span id="meta-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Meta`

- <span id="meta-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Meta`

- <span id="meta-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Meta`

- <span id="cratemeta-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Meta`

- <span id="meta-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Meta`

- <span id="cratemeta-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Meta`

##### `impl<T> From for Meta`

- <span id="meta-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Meta`

- <span id="cratemeta-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Meta`

- <span id="meta-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::attr::Meta`

- <span id="crateattrmeta-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::Meta`

- <span id="cratemeta-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Meta`

##### `impl Spanned for Meta`

- <span id="meta-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Meta`

- <span id="meta-toowned-type-owned"></span>`type Owned = T`

- <span id="meta-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="meta-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::attr::Meta`

- <span id="crateattrmeta-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Meta`

- <span id="meta-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="meta-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Meta`

- <span id="meta-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="meta-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Fields`

```rust
enum Fields {
    Named(FieldsNamed),
    Unnamed(FieldsUnnamed),
    Unit,
}
```

*Defined in [`syn-2.0.111/src/data.rs:26-46`](../../.source_1765521767/syn-2.0.111/src/data.rs#L26-L46)*

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

- <span id="fields-iter"></span>`fn iter(&self) -> punctuated::Iter<'_, Field>` — [`Iter`](punctuated/index.md#iter), [`Field`](data/index.md#field)

  Get an iterator over the borrowed [`Field`](data/index.md) items in this object. This

  iterator can be used to iterate over a named or unnamed struct or

  variant's fields uniformly.

- <span id="fields-iter-mut"></span>`fn iter_mut(&mut self) -> punctuated::IterMut<'_, Field>` — [`IterMut`](punctuated/index.md#itermut), [`Field`](data/index.md#field)

  Get an iterator over the mutably borrowed [`Field`](data/index.md) items in this

  object. This iterator can be used to iterate over a named or unnamed

  struct or variant's fields uniformly.

- <span id="fields-len"></span>`fn len(&self) -> usize`

  Returns the number of fields.

- <span id="fields-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns `true` if there are zero fields.

- <span id="fields-members"></span>`fn members(&self) -> Members<'_>` — [`Members`](data/index.md#members)

  Get an iterator over the fields of a struct or variant as [`Member`](expr/index.md)s.

  This iterator can be used to iterate over a named or unnamed struct or

  variant's fields uniformly.

  

  # Example

  

  The following is a simplistic [`Clone`](../fs_err/index.md) derive for structs. (A more

  complete implementation would additionally want to infer trait bounds on

  the generic type parameters.)

  

  ```rust

  use quote::quote;

  

  fn derive_clone(input: &syn::ItemStruct) -> proc_macro2::TokenStream {

      let ident = &input.ident;

      let members = input.fields.members();

      let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

      quote! {

          impl #impl_generics Clone for #ident #ty_generics #where_clause {

              fn clone(&self) -> Self {

                  Self {

                      #(#members: self.#members.clone()),*

                  }

              }

          }

      }

  }

  ```

  

  For structs with named fields, it produces an expression like `Self { a:

  self.a.clone() }`. For structs with unnamed fields, `Self { 0:

  self.0.clone() }`. And for unit structs, `Self {}`.

#### Trait Implementations

##### `impl Any for Fields`

- <span id="fields-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Fields`

- <span id="fields-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Fields`

- <span id="fields-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Fields`

- <span id="cratefields-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Fields`

- <span id="fields-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Fields`

- <span id="cratefields-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Fields`

##### `impl<T> From for Fields`

- <span id="fields-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Fields`

- <span id="cratefields-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Fields`

- <span id="fields-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Fields`

- <span id="fields-intoiterator-type-item"></span>`type Item = Field`

- <span id="fields-intoiterator-type-intoiter"></span>`type IntoIter = IntoIter<Field>`

- <span id="fields-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl PartialEq for crate::Fields`

- <span id="cratefields-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Fields`

##### `impl Spanned for Fields`

- <span id="fields-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Fields`

- <span id="fields-toowned-type-owned"></span>`type Owned = T`

- <span id="fields-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fields-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Fields`

- <span id="fields-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl<U> TryFrom for Fields`

- <span id="fields-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fields-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Fields`

- <span id="fields-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fields-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Data`

```rust
enum Data {
    Struct(DataStruct),
    Enum(DataEnum),
    Union(DataUnion),
}
```

*Defined in [`syn-2.0.111/src/derive.rs:21-35`](../../.source_1765521767/syn-2.0.111/src/derive.rs#L21-L35)*

The storage of a struct, enum or union data structure.

# Syntax tree enum

This type is a [syntax tree enum].


#### Trait Implementations

##### `impl Any for Data`

- <span id="data-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Data`

- <span id="data-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Data`

- <span id="data-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Data`

- <span id="cratedata-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Data`

- <span id="data-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Data`

- <span id="cratedata-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Data`

##### `impl<T> From for Data`

- <span id="data-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Data`

- <span id="cratedata-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Data`

- <span id="data-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::Data`

- <span id="cratedata-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl ToOwned for Data`

- <span id="data-toowned-type-owned"></span>`type Owned = T`

- <span id="data-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="data-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Data`

- <span id="data-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="data-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Data`

- <span id="data-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="data-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PointerMutability`

```rust
enum PointerMutability {
    Const(token::Const),
    Mut(token::Mut),
}
```

*Defined in [`syn-2.0.111/src/expr.rs:1161-1169`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L1161-L1169)*

Mutability of a raw pointer (`*const T`, `*mut T`), in which non-mutable
isn't the implicit default.

#### Trait Implementations

##### `impl Any for PointerMutability`

- <span id="pointermutability-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PointerMutability`

- <span id="pointermutability-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PointerMutability`

- <span id="pointermutability-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PointerMutability`

- <span id="cratepointermutability-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PointerMutability`

- <span id="pointermutability-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PointerMutability`

- <span id="cratepointermutability-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PointerMutability`

##### `impl<T> From for PointerMutability`

- <span id="pointermutability-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PointerMutability`

- <span id="cratepointermutability-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PointerMutability`

- <span id="pointermutability-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::PointerMutability`

- <span id="crateexprpointermutability-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::PointerMutability`

- <span id="cratepointermutability-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PointerMutability`

##### `impl Spanned for PointerMutability`

- <span id="pointermutability-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PointerMutability`

- <span id="pointermutability-toowned-type-owned"></span>`type Owned = T`

- <span id="pointermutability-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pointermutability-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::PointerMutability`

- <span id="crateexprpointermutability-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PointerMutability`

- <span id="pointermutability-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pointermutability-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PointerMutability`

- <span id="pointermutability-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pointermutability-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RangeLimits`

```rust
enum RangeLimits {
    HalfOpen(token::DotDot),
    Closed(token::DotDotEq),
}
```

*Defined in [`syn-2.0.111/src/expr.rs:1149-1158`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L1149-L1158)*

Limit types of a range, inclusive or exclusive.

#### Variants

- **`HalfOpen`**

  Inclusive at the beginning, exclusive at the end.

- **`Closed`**

  Inclusive at the beginning and end.

#### Implementations

- <span id="crateexprrangelimits-parse-obsolete"></span>`fn parse_obsolete(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

#### Trait Implementations

##### `impl Any for RangeLimits`

- <span id="rangelimits-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RangeLimits`

- <span id="rangelimits-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RangeLimits`

- <span id="rangelimits-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::RangeLimits`

- <span id="craterangelimits-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for RangeLimits`

- <span id="rangelimits-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for crate::RangeLimits`

##### `impl Debug for crate::RangeLimits`

- <span id="craterangelimits-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::RangeLimits`

##### `impl<T> From for RangeLimits`

- <span id="rangelimits-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::RangeLimits`

- <span id="craterangelimits-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for RangeLimits`

- <span id="rangelimits-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::RangeLimits`

- <span id="crateexprrangelimits-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::RangeLimits`

- <span id="craterangelimits-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for RangeLimits`

##### `impl Spanned for RangeLimits`

- <span id="rangelimits-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for RangeLimits`

- <span id="rangelimits-toowned-type-owned"></span>`type Owned = T`

- <span id="rangelimits-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rangelimits-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::RangeLimits`

- <span id="crateexprrangelimits-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for RangeLimits`

- <span id="rangelimits-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rangelimits-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RangeLimits`

- <span id="rangelimits-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rangelimits-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/expr.rs:35-267`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L35-L267)*

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

- <span id="expr-const-placeholder"></span>`const PLACEHOLDER: Self`

- <span id="expr-parse-without-eager-brace"></span>`fn parse_without_eager_brace(input: ParseStream<'_>) -> Result<Expr>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result), [`Expr`](expr/index.md#expr)

  An alternative to the primary `Expr::parse` parser (from the [`Parse`](parse/index.md)

  trait) for ambiguous syntactic positions in which a trailing brace

  should not be taken as part of the expression.

  

  Rust grammar has an ambiguity where braces sometimes turn a path

  expression into a struct initialization and sometimes do not. In the

  following code, the expression `S {}` is one expression. Presumably

  there is an empty struct `struct S {}` defined somewhere which it is

  instantiating.

  

  ```rust

  struct S;

  impl std::ops::Deref for S {

      type Target = bool;

      fn deref(&self) -> &Self::Target {

          &true

      }

  }

  let _ = *S {};

  

  // parsed by rustc as: `*(S {})`

  ```

  

  We would want to parse the above using `Expr::parse` after the `=`

  token.

  

  But in the following, `S {}` is *not* a struct init expression.

  

  ```rust

  const S: &bool = &true;

  if *S {} {}

  

  // parsed by rustc as:

  //

  //    if (*S) {

  //        /* empty block */

  //    }

  //    {

  //        /* another empty block */

  //    }

  ```

  

  For that reason we would want to parse if-conditions using

  `Expr::parse_without_eager_brace` after the `if` token. Same for similar

  syntactic positions such as the condition expr after a `while` token or

  the expr at the top of a `match`.

  

  The Rust grammar's choices around which way this ambiguity is resolved

  at various syntactic positions is fairly arbitrary. Really either parse

  behavior could work in most positions, and language designers just

  decide each case based on which is more likely to be what the programmer

  had in mind most of the time.

  

  ```rust

  struct S;

  fn doc() -> S {

  if return S {} {}

  unreachable!()

  }

  

  // parsed by rustc as:

  //

  //    if (return (S {})) {

  //    }

  //

  // but could equally well have been this other arbitrary choice:

  //

  //    if (return S) {

  //    }

  //    {}

  ```

  

  Note the grammar ambiguity on trailing braces is distinct from

  precedence and is not captured by assigning a precedence level to the

  braced struct init expr in relation to other operators. This can be

  illustrated by `return 0..S {}` vs `match 0..S {}`. The former parses as

  `return (0..(S {}))` implying tighter precedence for struct init than

  `..`, while the latter parses as `match (0..S) {}` implying tighter

  precedence for `..` than struct init, a contradiction.

- <span id="expr-parse-with-earlier-boundary-rule"></span>`fn parse_with_earlier_boundary_rule(input: ParseStream<'_>) -> Result<Expr>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result), [`Expr`](expr/index.md#expr)

  An alternative to the primary `Expr::parse` parser (from the [`Parse`](parse/index.md)

  trait) for syntactic positions in which expression boundaries are placed

  more eagerly than done by the typical expression grammar. This includes

  expressions at the head of a statement or in the right-hand side of a

  `match` arm.

  

  Compare the following cases:

  

  1.

    ```rust

    let result = ();

    let guard = false;

    let cond = true;

    let f = true;

    let g = f;

  

    let _ = match result {

        () if guard => if cond { f } else { g }

        () => false,

    };

    ```

  

  2.

    ```rust

    let cond = true;

    let f = ();

    let g = f;

  

    let _ = || {

        if cond { f } else { g }

        ()

    };

    ```

  

  3.

    ```rust

    let cond = true;

    let f = || ();

    let g = f;

  

    let _ = [if cond { f } else { g } ()];

    ```

  

  The same sequence of tokens `if cond { f } else { g } ()` appears in

  expression position 3 times. The first two syntactic positions use eager

  placement of expression boundaries, and parse as `Expr::If`, with the

  adjacent `()` becoming `Pat::Tuple` or `Expr::Tuple`. In contrast, the

  third case uses standard expression boundaries and parses as

  `Expr::Call`.

  

  As with `parse_without_eager_brace`, this ambiguity in the Rust

  grammar is independent of precedence.

- <span id="expr-peek"></span>`fn peek(input: ParseStream<'_>) -> bool` — [`ParseStream`](parse/index.md#parsestream)

  Returns whether the next token in the parse stream is one that might

  possibly form the beginning of an expr.

  

  This classification is a load-bearing part of the grammar of some Rust

  expressions, notably `return` and `break`. For example `return < …` will

  never parse `<` as a binary operator regardless of what comes after,

  because `<` is a legal starting token for an expression and so it's

  required to be continued as a return value, such as `return <Struct as

  Trait>::CONST`. Meanwhile `return > …` treats the `>` as a binary

  operator because it cannot be a starting token for any Rust expression.

- <span id="expr-replace-attrs"></span>`fn replace_attrs(&mut self, new: Vec<Attribute>) -> Vec<Attribute>` — [`Attribute`](attr/index.md#attribute)

#### Trait Implementations

##### `impl Any for Expr`

- <span id="expr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Expr`

- <span id="expr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Expr`

- <span id="expr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Expr`

- <span id="crateexpr-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Expr`

- <span id="expr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Expr`

- <span id="crateexpr-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Expr`

##### `impl<T> From for Expr`

- <span id="expr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Expr`

- <span id="crateexpr-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Expr`

- <span id="expr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::Expr`

- <span id="crateexprexpr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::Expr`

- <span id="crateexpr-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Expr`

##### `impl Spanned for Expr`

- <span id="expr-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Expr`

- <span id="expr-toowned-type-owned"></span>`type Owned = T`

- <span id="expr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="expr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Expr`

- <span id="expr-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl<U> TryFrom for Expr`

- <span id="expr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="expr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Expr`

- <span id="expr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="expr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Member`

```rust
enum Member {
    Named(crate::ident::Ident),
    Unnamed(Index),
}
```

*Defined in [`syn-2.0.111/src/expr.rs:971-981`](../../.source_1765521767/syn-2.0.111/src/expr.rs#L971-L981)*

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

##### `impl Any for Member`

- <span id="member-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Member`

- <span id="member-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Member`

- <span id="member-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Member`

- <span id="cratemember-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Member`

- <span id="member-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Member`

- <span id="cratemember-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Member`

##### `impl<T> From for Member`

- <span id="member-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Member`

- <span id="member-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl IdentFragment for Member`

- <span id="member-identfragment-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="member-identfragment-span"></span>`fn span(&self) -> Option<Span>`

##### `impl<U> Into for Member`

- <span id="member-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::Member`

- <span id="crateexprmember-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for Member`

- <span id="member-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Member`

##### `impl Spanned for Member`

- <span id="member-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Member`

- <span id="member-toowned-type-owned"></span>`type Owned = T`

- <span id="member-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="member-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::Member`

- <span id="crateexprmember-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Member`

- <span id="member-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="member-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Member`

- <span id="member-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="member-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `GenericParam`

```rust
enum GenericParam {
    Lifetime(LifetimeParam),
    Type(TypeParam),
    Const(ConstParam),
}
```

*Defined in [`syn-2.0.111/src/generics.rs:34-54`](../../.source_1765521767/syn-2.0.111/src/generics.rs#L34-L54)*

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

##### `impl Any for GenericParam`

- <span id="genericparam-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GenericParam`

- <span id="genericparam-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GenericParam`

- <span id="genericparam-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::GenericParam`

- <span id="crategenericparam-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for GenericParam`

- <span id="genericparam-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::GenericParam`

- <span id="crategenericparam-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::GenericParam`

##### `impl<T> From for GenericParam`

- <span id="genericparam-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::GenericParam`

- <span id="crategenericparam-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for GenericParam`

- <span id="genericparam-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::generics::GenericParam`

- <span id="crategenericsgenericparam-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::GenericParam`

- <span id="crategenericparam-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for GenericParam`

##### `impl Spanned for GenericParam`

- <span id="genericparam-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for GenericParam`

- <span id="genericparam-toowned-type-owned"></span>`type Owned = T`

- <span id="genericparam-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="genericparam-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for GenericParam`

- <span id="genericparam-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl<U> TryFrom for GenericParam`

- <span id="genericparam-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="genericparam-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GenericParam`

- <span id="genericparam-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="genericparam-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TraitBoundModifier`

```rust
enum TraitBoundModifier {
    None,
    Maybe(token::Question),
}
```

*Defined in [`syn-2.0.111/src/generics.rs:423-431`](../../.source_1765521767/syn-2.0.111/src/generics.rs#L423-L431)*

A modifier on a trait bound, currently only used for the `?` in
`?Sized`.

#### Trait Implementations

##### `impl Any for TraitBoundModifier`

- <span id="traitboundmodifier-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TraitBoundModifier`

- <span id="traitboundmodifier-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TraitBoundModifier`

- <span id="traitboundmodifier-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TraitBoundModifier`

- <span id="cratetraitboundmodifier-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TraitBoundModifier`

- <span id="traitboundmodifier-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for crate::TraitBoundModifier`

##### `impl Debug for crate::TraitBoundModifier`

- <span id="cratetraitboundmodifier-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitBoundModifier`

##### `impl<T> From for TraitBoundModifier`

- <span id="traitboundmodifier-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TraitBoundModifier`

- <span id="cratetraitboundmodifier-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TraitBoundModifier`

- <span id="traitboundmodifier-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::generics::TraitBoundModifier`

- <span id="crategenericstraitboundmodifier-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::TraitBoundModifier`

- <span id="cratetraitboundmodifier-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TraitBoundModifier`

##### `impl Spanned for TraitBoundModifier`

- <span id="traitboundmodifier-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TraitBoundModifier`

- <span id="traitboundmodifier-toowned-type-owned"></span>`type Owned = T`

- <span id="traitboundmodifier-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="traitboundmodifier-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::TraitBoundModifier`

- <span id="crategenericstraitboundmodifier-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TraitBoundModifier`

- <span id="traitboundmodifier-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="traitboundmodifier-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TraitBoundModifier`

- <span id="traitboundmodifier-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="traitboundmodifier-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypeParamBound`

```rust
enum TypeParamBound {
    Trait(TraitBound),
    Lifetime(crate::lifetime::Lifetime),
    PreciseCapture(PreciseCapture),
    Verbatim(proc_macro2::TokenStream),
}
```

*Defined in [`syn-2.0.111/src/generics.rs:398-408`](../../.source_1765521767/syn-2.0.111/src/generics.rs#L398-L408)*

A trait or lifetime used as a bound on a type parameter.

#### Implementations

- <span id="crategenericstypeparambound-parse-single"></span>`fn parse_single(input: ParseStream<'_>, allow_precise_capture: bool, allow_const: bool) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

- <span id="crategenericstypeparambound-parse-multiple"></span>`fn parse_multiple(input: ParseStream<'_>, allow_plus: bool, allow_precise_capture: bool, allow_const: bool) -> Result<Punctuated<Self, token::Plus>>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result), [`Punctuated`](punctuated/index.md#punctuated), [`Plus`](token/index.md#plus)

#### Trait Implementations

##### `impl Any for TypeParamBound`

- <span id="typeparambound-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeParamBound`

- <span id="typeparambound-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeParamBound`

- <span id="typeparambound-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypeParamBound`

- <span id="cratetypeparambound-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeParamBound`

- <span id="typeparambound-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypeParamBound`

- <span id="cratetypeparambound-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeParamBound`

##### `impl<T> From for TypeParamBound`

- <span id="typeparambound-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypeParamBound`

- <span id="cratetypeparambound-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TypeParamBound`

- <span id="typeparambound-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::generics::TypeParamBound`

- <span id="crategenericstypeparambound-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::TypeParamBound`

- <span id="cratetypeparambound-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeParamBound`

##### `impl Spanned for TypeParamBound`

- <span id="typeparambound-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeParamBound`

- <span id="typeparambound-toowned-type-owned"></span>`type Owned = T`

- <span id="typeparambound-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typeparambound-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for TypeParamBound`

- <span id="typeparambound-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl<U> TryFrom for TypeParamBound`

- <span id="typeparambound-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typeparambound-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeParamBound`

- <span id="typeparambound-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typeparambound-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WherePredicate`

```rust
enum WherePredicate {
    Lifetime(PredicateLifetime),
    Type(PredicateType),
}
```

*Defined in [`syn-2.0.111/src/generics.rs:471-488`](../../.source_1765521767/syn-2.0.111/src/generics.rs#L471-L488)*

A single predicate in a `where` clause: `T: Deserialize<'de>`.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Lifetime`**

  A lifetime predicate in a `where` clause: `'a: 'b + 'c`.

- **`Type`**

  A type predicate in a `where` clause: `for<'c> Foo<'c>: Trait<'c>`.

#### Trait Implementations

##### `impl Any for WherePredicate`

- <span id="wherepredicate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WherePredicate`

- <span id="wherepredicate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WherePredicate`

- <span id="wherepredicate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::WherePredicate`

- <span id="cratewherepredicate-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for WherePredicate`

- <span id="wherepredicate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::WherePredicate`

- <span id="cratewherepredicate-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::WherePredicate`

##### `impl<T> From for WherePredicate`

- <span id="wherepredicate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::WherePredicate`

- <span id="cratewherepredicate-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for WherePredicate`

- <span id="wherepredicate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::generics::WherePredicate`

- <span id="crategenericswherepredicate-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::WherePredicate`

- <span id="cratewherepredicate-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for WherePredicate`

##### `impl Spanned for WherePredicate`

- <span id="wherepredicate-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for WherePredicate`

- <span id="wherepredicate-toowned-type-owned"></span>`type Owned = T`

- <span id="wherepredicate-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="wherepredicate-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for WherePredicate`

- <span id="wherepredicate-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl<U> TryFrom for WherePredicate`

- <span id="wherepredicate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="wherepredicate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WherePredicate`

- <span id="wherepredicate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="wherepredicate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CapturedParam`

```rust
enum CapturedParam {
    Lifetime(crate::lifetime::Lifetime),
    Ident(crate::ident::Ident),
}
```

*Defined in [`syn-2.0.111/src/generics.rs:446-459`](../../.source_1765521767/syn-2.0.111/src/generics.rs#L446-L459)*

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

##### `impl Any for CapturedParam`

- <span id="capturedparam-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CapturedParam`

- <span id="capturedparam-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CapturedParam`

- <span id="capturedparam-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::CapturedParam`

- <span id="cratecapturedparam-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for CapturedParam`

- <span id="capturedparam-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::CapturedParam`

- <span id="cratecapturedparam-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::CapturedParam`

##### `impl<T> From for CapturedParam`

- <span id="capturedparam-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::CapturedParam`

- <span id="cratecapturedparam-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for CapturedParam`

- <span id="capturedparam-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::generics::CapturedParam`

- <span id="crategenericscapturedparam-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::CapturedParam`

- <span id="cratecapturedparam-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for CapturedParam`

##### `impl Spanned for CapturedParam`

- <span id="capturedparam-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for CapturedParam`

- <span id="capturedparam-toowned-type-owned"></span>`type Owned = T`

- <span id="capturedparam-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="capturedparam-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::CapturedParam`

- <span id="crategenericscapturedparam-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for CapturedParam`

- <span id="capturedparam-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="capturedparam-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CapturedParam`

- <span id="capturedparam-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="capturedparam-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FnArg`

```rust
enum FnArg {
    Receiver(Receiver),
    Typed(crate::pat::PatType),
}
```

*Defined in [`syn-2.0.111/src/item.rs:820-830`](../../.source_1765521767/syn-2.0.111/src/item.rs#L820-L830)*

An argument in a function signature: the `n: usize` in `fn f(n: usize)`.

#### Variants

- **`Receiver`**

  The `self` argument of an associated method.

- **`Typed`**

  A function argument accepted by pattern and type.

#### Trait Implementations

##### `impl Any for FnArg`

- <span id="fnarg-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FnArg`

- <span id="fnarg-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FnArg`

- <span id="fnarg-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::FnArg`

- <span id="cratefnarg-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for FnArg`

- <span id="fnarg-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::FnArg`

- <span id="cratefnarg-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FnArg`

##### `impl<T> From for FnArg`

- <span id="fnarg-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::FnArg`

- <span id="cratefnarg-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for FnArg`

- <span id="fnarg-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::FnArg`

- <span id="crateitemfnarg-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::FnArg`

- <span id="cratefnarg-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for FnArg`

##### `impl Spanned for FnArg`

- <span id="fnarg-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for FnArg`

- <span id="fnarg-toowned-type-owned"></span>`type Owned = T`

- <span id="fnarg-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fnarg-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for FnArg`

- <span id="fnarg-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl<U> TryFrom for FnArg`

- <span id="fnarg-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fnarg-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FnArg`

- <span id="fnarg-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fnarg-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:496-540`](../../.source_1765521767/syn-2.0.111/src/item.rs#L496-L540)*

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

##### `impl Any for ForeignItem`

- <span id="foreignitem-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ForeignItem`

- <span id="foreignitem-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ForeignItem`

- <span id="foreignitem-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ForeignItem`

- <span id="crateforeignitem-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ForeignItem`

- <span id="foreignitem-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ForeignItem`

- <span id="crateforeignitem-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItem`

##### `impl<T> From for ForeignItem`

- <span id="foreignitem-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ForeignItem`

- <span id="crateforeignitem-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ForeignItem`

- <span id="foreignitem-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ForeignItem`

- <span id="crateitemforeignitem-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ForeignItem`

- <span id="crateforeignitem-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ForeignItem`

##### `impl Spanned for ForeignItem`

- <span id="foreignitem-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ForeignItem`

- <span id="foreignitem-toowned-type-owned"></span>`type Owned = T`

- <span id="foreignitem-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="foreignitem-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for ForeignItem`

- <span id="foreignitem-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl<U> TryFrom for ForeignItem`

- <span id="foreignitem-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="foreignitem-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ForeignItem`

- <span id="foreignitem-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="foreignitem-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:688-732`](../../.source_1765521767/syn-2.0.111/src/item.rs#L688-L732)*

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

##### `impl Any for ImplItem`

- <span id="implitem-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImplItem`

- <span id="implitem-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImplItem`

- <span id="implitem-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ImplItem`

- <span id="crateimplitem-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ImplItem`

- <span id="implitem-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ImplItem`

- <span id="crateimplitem-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItem`

##### `impl<T> From for ImplItem`

- <span id="implitem-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ImplItem`

- <span id="crateimplitem-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ImplItem`

- <span id="implitem-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ImplItem`

- <span id="crateitemimplitem-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ImplItem`

- <span id="crateimplitem-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ImplItem`

##### `impl Spanned for ImplItem`

- <span id="implitem-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ImplItem`

- <span id="implitem-toowned-type-owned"></span>`type Owned = T`

- <span id="implitem-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="implitem-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for ImplItem`

- <span id="implitem-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl<U> TryFrom for ImplItem`

- <span id="implitem-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="implitem-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImplItem`

- <span id="implitem-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="implitem-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ImplRestriction`

```rust
enum ImplRestriction {
}
```

*Defined in [`syn-2.0.111/src/item.rs:888-903`](../../.source_1765521767/syn-2.0.111/src/item.rs#L888-L903)*

Unused, but reserved for RFC 3323 restrictions.

#### Trait Implementations

##### `impl Any for ImplRestriction`

- <span id="implrestriction-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImplRestriction`

- <span id="implrestriction-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImplRestriction`

- <span id="implrestriction-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ImplRestriction`

- <span id="crateimplrestriction-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ImplRestriction`

- <span id="implrestriction-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ImplRestriction`

- <span id="crateimplrestriction-debug-fmt"></span>`fn fmt(&self, _formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplRestriction`

##### `impl<T> From for ImplRestriction`

- <span id="implrestriction-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ImplRestriction`

- <span id="crateimplrestriction-hash"></span>`fn hash<H>(&self, _state: &mut H)`

##### `impl<U> Into for ImplRestriction`

- <span id="implrestriction-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::ImplRestriction`

- <span id="crateimplrestriction-partialeq-eq"></span>`fn eq(&self, _other: &Self) -> bool`

##### `impl ToOwned for ImplRestriction`

- <span id="implrestriction-toowned-type-owned"></span>`type Owned = T`

- <span id="implrestriction-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="implrestriction-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ImplRestriction`

- <span id="implrestriction-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="implrestriction-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImplRestriction`

- <span id="implrestriction-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="implrestriction-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:20-99`](../../.source_1765521767/syn-2.0.111/src/item.rs#L20-L99)*

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

- <span id="item-replace-attrs"></span>`fn replace_attrs(&mut self, new: Vec<Attribute>) -> Vec<Attribute>` — [`Attribute`](attr/index.md#attribute)

#### Trait Implementations

##### `impl Any for Item`

- <span id="item-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Item`

- <span id="item-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Item`

- <span id="item-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Item`

- <span id="crateitem-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Item`

- <span id="item-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Item`

- <span id="crateitem-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Item`

##### `impl<T> From for Item`

- <span id="item-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Item`

- <span id="crateitem-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Item`

- <span id="item-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::Item`

- <span id="crateitemitem-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::Item`

- <span id="crateitem-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Item`

##### `impl Spanned for Item`

- <span id="item-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Item`

- <span id="item-toowned-type-owned"></span>`type Owned = T`

- <span id="item-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="item-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Item`

- <span id="item-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl<U> TryFrom for Item`

- <span id="item-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="item-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Item`

- <span id="item-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="item-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StaticMutability`

```rust
enum StaticMutability {
    Mut(token::Mut),
    None,
}
```

*Defined in [`syn-2.0.111/src/item.rs:878-886`](../../.source_1765521767/syn-2.0.111/src/item.rs#L878-L886)*

The mutability of an `Item::Static` or `ForeignItem::Static`.

#### Trait Implementations

##### `impl Any for StaticMutability`

- <span id="staticmutability-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StaticMutability`

- <span id="staticmutability-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StaticMutability`

- <span id="staticmutability-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::StaticMutability`

- <span id="cratestaticmutability-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for StaticMutability`

- <span id="staticmutability-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::StaticMutability`

- <span id="cratestaticmutability-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::StaticMutability`

##### `impl<T> From for StaticMutability`

- <span id="staticmutability-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::StaticMutability`

- <span id="cratestaticmutability-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for StaticMutability`

- <span id="staticmutability-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::StaticMutability`

- <span id="crateitemstaticmutability-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::StaticMutability`

- <span id="cratestaticmutability-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for StaticMutability`

##### `impl Spanned for StaticMutability`

- <span id="staticmutability-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for StaticMutability`

- <span id="staticmutability-toowned-type-owned"></span>`type Owned = T`

- <span id="staticmutability-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="staticmutability-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::StaticMutability`

- <span id="crateitemstaticmutability-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for StaticMutability`

- <span id="staticmutability-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="staticmutability-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StaticMutability`

- <span id="staticmutability-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="staticmutability-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:591-635`](../../.source_1765521767/syn-2.0.111/src/item.rs#L591-L635)*

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

##### `impl Any for TraitItem`

- <span id="traititem-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TraitItem`

- <span id="traititem-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TraitItem`

- <span id="traititem-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TraitItem`

- <span id="cratetraititem-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TraitItem`

- <span id="traititem-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TraitItem`

- <span id="cratetraititem-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItem`

##### `impl<T> From for TraitItem`

- <span id="traititem-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TraitItem`

- <span id="cratetraititem-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TraitItem`

- <span id="traititem-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::TraitItem`

- <span id="crateitemtraititem-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::TraitItem`

- <span id="cratetraititem-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TraitItem`

##### `impl Spanned for TraitItem`

- <span id="traititem-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TraitItem`

- <span id="traititem-toowned-type-owned"></span>`type Owned = T`

- <span id="traititem-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="traititem-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for TraitItem`

- <span id="traititem-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl<U> TryFrom for TraitItem`

- <span id="traititem-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="traititem-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TraitItem`

- <span id="traititem-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="traititem-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:424-449`](../../.source_1765521767/syn-2.0.111/src/item.rs#L424-L449)*

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

##### `impl Any for UseTree`

- <span id="usetree-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UseTree`

- <span id="usetree-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UseTree`

- <span id="usetree-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::UseTree`

- <span id="crateusetree-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for UseTree`

- <span id="usetree-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::UseTree`

- <span id="crateusetree-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseTree`

##### `impl<T> From for UseTree`

- <span id="usetree-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::UseTree`

- <span id="crateusetree-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for UseTree`

- <span id="usetree-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::UseTree`

- <span id="crateitemusetree-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<UseTree>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result), [`UseTree`](item/index.md#usetree)

##### `impl PartialEq for crate::UseTree`

- <span id="crateusetree-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for UseTree`

##### `impl Spanned for UseTree`

- <span id="usetree-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for UseTree`

- <span id="usetree-toowned-type-owned"></span>`type Owned = T`

- <span id="usetree-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="usetree-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for UseTree`

- <span id="usetree-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl<U> TryFrom for UseTree`

- <span id="usetree-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="usetree-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UseTree`

- <span id="usetree-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="usetree-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/lit.rs:17-56`](../../.source_1765521767/syn-2.0.111/src/lit.rs#L17-L56)*

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

  Interpret a Syn literal from a proc-macro2 literal.

- <span id="cratelitlit-from-str"></span>`fn from_str(token: Literal, repr: &str) -> Self`

- <span id="cratelitlit-suffix"></span>`fn suffix(&self) -> &str`

- <span id="cratelitlit-span"></span>`fn span(&self) -> Span`

- <span id="cratelitlit-set-span"></span>`fn set_span(&mut self, span: Span)`

#### Trait Implementations

##### `impl Any for Lit`

- <span id="lit-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Lit`

- <span id="lit-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Lit`

- <span id="lit-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Lit`

- <span id="cratelit-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Lit`

- <span id="lit-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Lit`

- <span id="cratelit-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Lit`

##### `impl<T> From for Lit`

- <span id="lit-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Lit`

- <span id="cratelit-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Lit`

- <span id="lit-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::lit::Lit`

- <span id="cratelitlit-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::Lit`

- <span id="cratelit-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Lit`

##### `impl Spanned for Lit`

- <span id="lit-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Lit`

- <span id="lit-toowned-type-owned"></span>`type Owned = T`

- <span id="lit-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="lit-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Lit`

- <span id="lit-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl Token for crate::lit::Lit`

##### `impl<U> TryFrom for Lit`

- <span id="lit-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lit-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Lit`

- <span id="lit-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lit-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MacroDelimiter`

```rust
enum MacroDelimiter {
    Paren(crate::token::Paren),
    Brace(crate::token::Brace),
    Bracket(crate::token::Bracket),
}
```

*Defined in [`syn-2.0.111/src/mac.rs:25-33`](../../.source_1765521767/syn-2.0.111/src/mac.rs#L25-L33)*

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

*Defined in [`syn-2.0.111/src/op.rs:1-63`](../../.source_1765521767/syn-2.0.111/src/op.rs#L1-L63)*

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

##### `impl Any for BinOp`

- <span id="binop-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BinOp`

- <span id="binop-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BinOp`

- <span id="binop-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::BinOp`

- <span id="cratebinop-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for BinOp`

- <span id="binop-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for crate::BinOp`

##### `impl Debug for crate::BinOp`

- <span id="cratebinop-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::BinOp`

##### `impl<T> From for BinOp`

- <span id="binop-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::BinOp`

- <span id="cratebinop-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for BinOp`

- <span id="binop-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::op::BinOp`

- <span id="crateopbinop-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::BinOp`

- <span id="cratebinop-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for BinOp`

##### `impl Spanned for BinOp`

- <span id="binop-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for BinOp`

- <span id="binop-toowned-type-owned"></span>`type Owned = T`

- <span id="binop-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="binop-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::op::BinOp`

- <span id="crateopbinop-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for BinOp`

- <span id="binop-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="binop-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BinOp`

- <span id="binop-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="binop-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UnOp`

```rust
enum UnOp {
    Deref(token::Star),
    Not(token::Not),
    Neg(token::Minus),
}
```

*Defined in [`syn-2.0.111/src/op.rs:65-77`](../../.source_1765521767/syn-2.0.111/src/op.rs#L65-L77)*

A unary operator: `*`, `!`, `-`.

#### Variants

- **`Deref`**

  The `*` operator for dereferencing

- **`Not`**

  The `!` operator for logical inversion

- **`Neg`**

  The `-` operator for negation

#### Trait Implementations

##### `impl Any for UnOp`

- <span id="unop-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnOp`

- <span id="unop-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnOp`

- <span id="unop-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::UnOp`

- <span id="crateunop-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for UnOp`

- <span id="unop-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for crate::UnOp`

##### `impl Debug for crate::UnOp`

- <span id="crateunop-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UnOp`

##### `impl<T> From for UnOp`

- <span id="unop-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::UnOp`

- <span id="crateunop-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for UnOp`

- <span id="unop-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::op::UnOp`

- <span id="crateopunop-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::UnOp`

- <span id="crateunop-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for UnOp`

##### `impl Spanned for UnOp`

- <span id="unop-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for UnOp`

- <span id="unop-toowned-type-owned"></span>`type Owned = T`

- <span id="unop-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unop-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::op::UnOp`

- <span id="crateopunop-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for UnOp`

- <span id="unop-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unop-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UnOp`

- <span id="unop-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unop-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/pat.rs:15-102`](../../.source_1765521767/syn-2.0.111/src/pat.rs#L15-L102)*

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

- <span id="cratepatpat-parse-single"></span>`fn parse_single(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

  Parse a pattern that does _not_ involve `|` at the top level.

  

  This parser matches the behavior of the `$:pat_param` macro_rules

  matcher, and on editions prior to Rust 2021, the behavior of

  `$:pat`.

  

  In Rust syntax, some examples of where this syntax would occur are

  in the argument pattern of functions and closures. Patterns using

  `|` are not allowed to occur in these positions.

  

  ```compile_fail

  fn f(Some(_) | None: Option<T>) {

      let _ = |Some(_) | None: Option<T>| {};

      //       ^^^^^^^^^^^^^^^^^^^^^^^^^??? :(

  }

  ```

  

  ```console

  error: top-level or-patterns are not allowed in function parameters

   --> src/main.rs:1:6

    |

  1 | fn f(Some(_) | None: Option<T>) {

    |      ^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Some(_) | None)`

  ```

- <span id="cratepatpat-parse-multi"></span>`fn parse_multi(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

  Parse a pattern, possibly involving `|`, but not a leading `|`.

- <span id="cratepatpat-parse-multi-with-leading-vert"></span>`fn parse_multi_with_leading_vert(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

  Parse a pattern, possibly involving `|`, possibly including a

  leading `|`.

  

  This parser matches the behavior of the Rust 2021 edition's `$:pat`

  macro_rules matcher.

  

  In Rust syntax, an example of where this syntax would occur is in

  the pattern of a `match` arm, where the language permits an optional

  leading `|`, although it is not idiomatic to write one there in

  handwritten code.

  

  ```rust

  let wat = None;

  match wat {

      | None | Some(false) => {}

      | Some(true) => {}

  }

  ```

  

  The compiler accepts it only to facilitate some situations in

  macro-generated code where a macro author might need to write:

  

  ```rust

  macro_rules! doc {

      ($value:expr, ($($conditions1:pat),*), ($($conditions2:pat),*), $then:expr) => {

  match $value {

      $(| $conditions1)* $(| $conditions2)* => $then

  }

      };

  }

  

  doc!(true, (true), (false), {});

  doc!(true, (), (true, false), {});

  doc!(true, (true, false), (), {});

  ```

  

  Expressing the same thing correctly in the case that either one (but

  not both) of `$conditions1` and `$conditions2` might be empty,

  without leading `|`, is complex.

  

  Use `Pat::parse_multi` instead if you are not intending to support

  macro-generated macro input.

#### Trait Implementations

##### `impl Any for Pat`

- <span id="pat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Pat`

- <span id="pat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Pat`

- <span id="pat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Pat`

- <span id="cratepat-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Pat`

- <span id="pat-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Pat`

- <span id="cratepat-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Pat`

##### `impl<T> From for Pat`

- <span id="pat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Pat`

- <span id="cratepat-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Pat`

- <span id="pat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::Pat`

- <span id="cratepat-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Pat`

##### `impl Spanned for Pat`

- <span id="pat-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Pat`

- <span id="pat-toowned-type-owned"></span>`type Owned = T`

- <span id="pat-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pat-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Pat`

- <span id="pat-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl<U> TryFrom for Pat`

- <span id="pat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Pat`

- <span id="pat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/path.rs:171-194`](../../.source_1765521767/syn-2.0.111/src/path.rs#L171-L194)*

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

##### `impl Any for GenericArgument`

- <span id="genericargument-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GenericArgument`

- <span id="genericargument-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GenericArgument`

- <span id="genericargument-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::GenericArgument`

- <span id="crategenericargument-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for GenericArgument`

- <span id="genericargument-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::GenericArgument`

- <span id="crategenericargument-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::GenericArgument`

##### `impl<T> From for GenericArgument`

- <span id="genericargument-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::GenericArgument`

- <span id="crategenericargument-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for GenericArgument`

- <span id="genericargument-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::path::GenericArgument`

- <span id="cratepathgenericargument-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::GenericArgument`

- <span id="crategenericargument-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for GenericArgument`

##### `impl Spanned for GenericArgument`

- <span id="genericargument-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for GenericArgument`

- <span id="genericargument-toowned-type-owned"></span>`type Owned = T`

- <span id="genericargument-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="genericargument-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::path::GenericArgument`

- <span id="cratepathgenericargument-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for GenericArgument`

- <span id="genericargument-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="genericargument-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GenericArgument`

- <span id="genericargument-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="genericargument-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PathArguments`

```rust
enum PathArguments {
    None,
    AngleBracketed(AngleBracketedGenericArguments),
    Parenthesized(ParenthesizedGenericArguments),
}
```

*Defined in [`syn-2.0.111/src/path.rs:128-146`](../../.source_1765521767/syn-2.0.111/src/path.rs#L128-L146)*

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

##### `impl Any for PathArguments`

- <span id="patharguments-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PathArguments`

- <span id="patharguments-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PathArguments`

- <span id="patharguments-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PathArguments`

- <span id="cratepatharguments-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PathArguments`

- <span id="patharguments-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PathArguments`

- <span id="cratepatharguments-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PathArguments`

- <span id="patharguments-default"></span>`fn default() -> Self`

##### `impl Eq for crate::PathArguments`

##### `impl<T> From for PathArguments`

- <span id="patharguments-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PathArguments`

- <span id="cratepatharguments-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PathArguments`

- <span id="patharguments-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::PathArguments`

- <span id="cratepatharguments-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PathArguments`

##### `impl Spanned for PathArguments`

- <span id="patharguments-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PathArguments`

- <span id="patharguments-toowned-type-owned"></span>`type Owned = T`

- <span id="patharguments-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="patharguments-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::path::PathArguments`

- <span id="cratepathpatharguments-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PathArguments`

- <span id="patharguments-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="patharguments-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PathArguments`

- <span id="patharguments-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="patharguments-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FieldMutability`

```rust
enum FieldMutability {
    None,
}
```

*Defined in [`syn-2.0.111/src/restriction.rs:39-57`](../../.source_1765521767/syn-2.0.111/src/restriction.rs#L39-L57)*

Unused, but reserved for RFC 3323 restrictions.

#### Trait Implementations

##### `impl Any for FieldMutability`

- <span id="fieldmutability-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FieldMutability`

- <span id="fieldmutability-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FieldMutability`

- <span id="fieldmutability-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::FieldMutability`

- <span id="cratefieldmutability-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for FieldMutability`

- <span id="fieldmutability-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::FieldMutability`

- <span id="cratefieldmutability-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldMutability`

##### `impl<T> From for FieldMutability`

- <span id="fieldmutability-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::FieldMutability`

- <span id="cratefieldmutability-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for FieldMutability`

- <span id="fieldmutability-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::FieldMutability`

- <span id="cratefieldmutability-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl ToOwned for FieldMutability`

- <span id="fieldmutability-toowned-type-owned"></span>`type Owned = T`

- <span id="fieldmutability-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fieldmutability-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FieldMutability`

- <span id="fieldmutability-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fieldmutability-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FieldMutability`

- <span id="fieldmutability-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fieldmutability-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Visibility`

```rust
enum Visibility {
    Public(token::Pub),
    Restricted(VisRestricted),
    Inherited,
}
```

*Defined in [`syn-2.0.111/src/restriction.rs:4-25`](../../.source_1765521767/syn-2.0.111/src/restriction.rs#L4-L25)*

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

##### `impl Any for Visibility`

- <span id="visibility-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Visibility`

- <span id="visibility-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Visibility`

- <span id="visibility-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Visibility`

- <span id="cratevisibility-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Visibility`

- <span id="visibility-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Visibility`

- <span id="cratevisibility-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Visibility`

##### `impl<T> From for Visibility`

- <span id="visibility-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Visibility`

- <span id="cratevisibility-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Visibility`

- <span id="visibility-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::restriction::Visibility`

- <span id="craterestrictionvisibility-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::Visibility`

- <span id="cratevisibility-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Visibility`

##### `impl Spanned for Visibility`

- <span id="visibility-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Visibility`

- <span id="visibility-toowned-type-owned"></span>`type Owned = T`

- <span id="visibility-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="visibility-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::restriction::Visibility`

- <span id="craterestrictionvisibility-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Visibility`

- <span id="visibility-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="visibility-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Visibility`

- <span id="visibility-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="visibility-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Stmt`

```rust
enum Stmt {
    Local(Local),
    Item(crate::item::Item),
    Expr(crate::expr::Expr, Option<token::Semi>),
    Macro(StmtMacro),
}
```

*Defined in [`syn-2.0.111/src/stmt.rs:18-38`](../../.source_1765521767/syn-2.0.111/src/stmt.rs#L18-L38)*

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

##### `impl Any for Stmt`

- <span id="stmt-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Stmt`

- <span id="stmt-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Stmt`

- <span id="stmt-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Stmt`

- <span id="cratestmt-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Stmt`

- <span id="stmt-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Stmt`

- <span id="cratestmt-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Stmt`

##### `impl<T> From for Stmt`

- <span id="stmt-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Stmt`

- <span id="cratestmt-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Stmt`

- <span id="stmt-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::stmt::Stmt`

- <span id="cratestmtstmt-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::Stmt`

- <span id="cratestmt-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Stmt`

##### `impl Spanned for Stmt`

- <span id="stmt-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Stmt`

- <span id="stmt-toowned-type-owned"></span>`type Owned = T`

- <span id="stmt-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="stmt-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::stmt::Stmt`

- <span id="cratestmtstmt-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Stmt`

- <span id="stmt-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stmt-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Stmt`

- <span id="stmt-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stmt-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ReturnType`

```rust
enum ReturnType {
    Default,
    Type(token::RArrow, Box<Type>),
}
```

*Defined in [`syn-2.0.111/src/ty.rs:260-271`](../../.source_1765521767/syn-2.0.111/src/ty.rs#L260-L271)*

Return type of a function signature.

#### Variants

- **`Default`**

  Return type is not specified.
  
  Functions default to `()` and closures default to type inference.

- **`Type`**

  A particular type is returned.

#### Implementations

- <span id="cratetyreturntype-without-plus"></span>`fn without_plus(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

- <span id="cratetyreturntype-parse"></span>`fn parse(input: ParseStream<'_>, allow_plus: bool) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

#### Trait Implementations

##### `impl Any for ReturnType`

- <span id="returntype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReturnType`

- <span id="returntype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReturnType`

- <span id="returntype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ReturnType`

- <span id="cratereturntype-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ReturnType`

- <span id="returntype-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ReturnType`

- <span id="cratereturntype-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ReturnType`

##### `impl<T> From for ReturnType`

- <span id="returntype-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ReturnType`

- <span id="cratereturntype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ReturnType`

- <span id="returntype-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::ReturnType`

- <span id="cratetyreturntype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::ReturnType`

- <span id="cratereturntype-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ReturnType`

##### `impl Spanned for ReturnType`

- <span id="returntype-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ReturnType`

- <span id="returntype-toowned-type-owned"></span>`type Owned = T`

- <span id="returntype-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="returntype-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::ReturnType`

- <span id="cratetyreturntype-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ReturnType`

- <span id="returntype-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="returntype-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReturnType`

- <span id="returntype-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="returntype-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/ty.rs:13-90`](../../.source_1765521767/syn-2.0.111/src/ty.rs#L13-L90)*

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

- <span id="cratetytype-without-plus"></span>`fn without_plus(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

  In some positions, types may not contain the `+` character, to

  disambiguate them. For example in the expression `1 as T`, T may not

  contain a `+` character.

  

  This parser does not allow a `+`, while the default parser does.

#### Trait Implementations

##### `impl Any for Type`

- <span id="type-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Type`

- <span id="type-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Type`

- <span id="type-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Type`

- <span id="cratetype-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Type`

- <span id="type-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Type`

- <span id="cratetype-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Type`

##### `impl<T> From for Type`

- <span id="type-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Type`

- <span id="cratetype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Type`

- <span id="type-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::Type`

- <span id="cratetytype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md#parsestream), [`Result`](error/index.md#result)

##### `impl PartialEq for crate::Type`

- <span id="cratetype-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Type`

##### `impl Spanned for Type`

- <span id="type-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Type`

- <span id="type-toowned-type-owned"></span>`type Owned = T`

- <span id="type-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="type-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Type`

- <span id="type-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl<U> TryFrom for Type`

- <span id="type-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="type-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Type`

- <span id="type-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="type-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `parse`

```rust
fn parse<T: parse::Parse>(tokens: proc_macro::TokenStream) -> Result<T>
```

*Defined in [`syn-2.0.111/src/lib.rs:902-904`](../../.source_1765521767/syn-2.0.111/src/lib.rs#L902-L904)*

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

*Defined in [`syn-2.0.111/src/lib.rs:920-922`](../../.source_1765521767/syn-2.0.111/src/lib.rs#L920-L922)*

Parse a proc-macro2 token stream into the chosen syntax tree node.

This function parses a `proc_macro2::TokenStream` which is commonly useful
when the input comes from a node of the Syn syntax tree, for example the
body tokens of a [`Macro`](mac/index.md) node. When in a procedural macro parsing the
`proc_macro::TokenStream` provided by the compiler, use `syn::parse`
instead.

This function enforces that the input is fully parsed. If there are any
unparsed tokens at the end of the stream, an error is returned.

### `parse_str`

```rust
fn parse_str<T: parse::Parse>(s: &str) -> Result<T>
```

*Defined in [`syn-2.0.111/src/lib.rs:950-952`](../../.source_1765521767/syn-2.0.111/src/lib.rs#L950-L952)*

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

*Defined in [`syn-2.0.111/src/lib.rs:985-1009`](../../.source_1765521767/syn-2.0.111/src/lib.rs#L985-L1009)*

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

### `Result<T>`

```rust
type Result<T> = std::result::Result<T, Error>;
```

*Defined in [`syn-2.0.111/src/error.rs:15`](../../.source_1765521767/syn-2.0.111/src/error.rs#L15)*

The result of a Syn parser.

## Macros

### `parenthesized!`

*Defined in [`syn-2.0.111/src/group.rs:146-159`](../../.source_1765521767/syn-2.0.111/src/group.rs#L146-L159)*

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

*Defined in [`syn-2.0.111/src/group.rs:225-238`](../../.source_1765521767/syn-2.0.111/src/group.rs#L225-L238)*

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

*Defined in [`syn-2.0.111/src/group.rs:281-294`](../../.source_1765521767/syn-2.0.111/src/group.rs#L281-L294)*

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

*Defined in [`syn-2.0.111/src/token.rs:871-972`](../../.source_1765521767/syn-2.0.111/src/token.rs#L871-L972)*

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

*Defined in [`syn-2.0.111/src/custom_keyword.rs:90-123`](../../.source_1765521767/syn-2.0.111/src/custom_keyword.rs#L90-L123)*

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

*Defined in [`syn-2.0.111/src/custom_punctuation.rs:79-110`](../../.source_1765521767/syn-2.0.111/src/custom_punctuation.rs#L79-L110)*

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

*Defined in [`syn-2.0.111/src/parse_macro_input.rs:108-128`](../../.source_1765521767/syn-2.0.111/src/parse_macro_input.rs#L108-L128)*

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

*Defined in [`syn-2.0.111/src/parse_quote.rs:80-84`](../../.source_1765521767/syn-2.0.111/src/parse_quote.rs#L80-L84)*

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

- [`Attribute`](attr/index.md) — parses one attribute, allowing either outer like `#[...]`
  or inner like `#![...]`
- `Vec<Attribute>` — parses multiple attributes, including mixed kinds in
  any order
- [`Punctuated<T, P>`](punctuated/index.md) — parses zero or more `T` separated by punctuation
  `P` with optional trailing punctuation
- `Vec<Arm>` — parses arms separated by optional commas according to the
  same grammar as the inside of a `match` expression
- `Vec<Stmt>` — parses the same as `Block::parse_within`
- [`Pat`](pat/index.md), `Box<Pat>` — parses the same as
  `Pat::parse_multi_with_leading_vert`
- [`Field`](data/index.md) — parses a named or unnamed struct field





# Panics

Panics if the tokens fail to parse as the expected syntax tree type. The
caller is responsible for ensuring that the input tokens are syntactically
valid.

### `parse_quote_spanned!`

*Defined in [`syn-2.0.111/src/parse_quote.rs:112-116`](../../.source_1765521767/syn-2.0.111/src/parse_quote.rs#L112-L116)*

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

