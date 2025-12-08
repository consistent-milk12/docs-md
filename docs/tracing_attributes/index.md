# Crate `tracing_attributes`

A procedural macro attribute for instrumenting functions with `tracing`.

`tracing` is a framework for instrumenting Rust programs to collect
structured, event-based diagnostic information. This crate provides the
[`#[instrument]`][instrument]() procedural macro attribute.

Note that this macro is also re-exported by the main `tracing` crate.

*Compiler support: [requires `rustc` 1.65+][msrv]*

## Usage

In the `Cargo.toml`:

```toml
[dependencies]
tracing-attributes = "0.1.24"
```

The [`#[instrument]`][instrument]() attribute can now be added to a function
to automatically create and enter `tracing` [`span`](../tracing/span/index.md) when that function is
called. For example:

```rust
use tracing::instrument;

#[instrument]
pub fn my_function(my_arg: usize) {
    // ...
}

fn main() {}
```



## Supported Rust Versions

Tracing is built against the latest stable release. The minimum supported
version is 1.65. The current Tracing version is not guaranteed to build on
Rust versions earlier than the minimum supported version.

Tracing follows the same compiler support policies as the rest of the Tokio
project. The current stable Rust compiler and the three most recent minor
versions before it will always be supported. For example, if the current
stable compiler version is 1.69, the minimum supported version will not be
increased past 1.66, three minor versions prior. Increasing the minimum
supported compiler version is not considered a semver breaking change as
long as doing so complies with this policy.


## Modules

- [`attr`](attr/index.md) - 
- [`expand`](expand/index.md) - 

## Structs

### `MaybeItemFn`

```rust
struct MaybeItemFn {
    outer_attrs: Vec<syn::Attribute>,
    inner_attrs: Vec<syn::Attribute>,
    vis: syn::Visibility,
    sig: syn::Signature,
    brace_token: syn::token::Brace,
    block: proc_macro2::TokenStream,
}
```

This is a more flexible/imprecise `ItemFn` type,
which's block is just a `TokenStream` (it may contain invalid code).

#### Implementations

- `fn as_ref(self: &Self) -> MaybeItemFnRef<'_, TokenStream>` — [`MaybeItemFnRef`](#maybeitemfnref)

#### Trait Implementations

##### `impl Clone for MaybeItemFn`

- `fn clone(self: &Self) -> MaybeItemFn` — [`MaybeItemFn`](#maybeitemfn)

##### `impl Debug for MaybeItemFn`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Parse for MaybeItemFn`

- `fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

### `MaybeItemFnRef<'a, B: ToTokens>`

```rust
struct MaybeItemFnRef<'a, B: ToTokens> {
    outer_attrs: &'a Vec<syn::Attribute>,
    inner_attrs: &'a Vec<syn::Attribute>,
    vis: &'a syn::Visibility,
    sig: &'a syn::Signature,
    brace_token: &'a syn::token::Brace,
    block: &'a B,
}
```

A generic reference type for `MaybeItemFn`,
that takes a generic block type `B` that implements `ToTokens` (eg. `TokenStream`, `Block`).

#### Trait Implementations

##### `impl<'a, B: $crate::clone::Clone + ToTokens> Clone for MaybeItemFnRef<'a, B>`

- `fn clone(self: &Self) -> MaybeItemFnRef<'a, B>` — [`MaybeItemFnRef`](#maybeitemfnref)

##### `impl<'a, B: $crate::fmt::Debug + ToTokens> Debug for MaybeItemFnRef<'a, B>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Functions

### `instrument_speculative`

```rust
fn instrument_speculative(args: attr::InstrumentArgs, item: proc_macro::TokenStream) -> proc_macro::TokenStream
```

Instrument the function, without parsing the function body (instead using the raw tokens).

### `instrument_precise`

```rust
fn instrument_precise(args: attr::InstrumentArgs, item: proc_macro::TokenStream) -> Result<proc_macro::TokenStream, syn::Error>
```

Instrument the function, by fully parsing the function body,
which allows us to rewrite some statements related to async-like patterns.

