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
to automatically create and enter `tracing` [`span`](../syn/span/index.md) when that function is
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


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`attr`](#attr) | mod |  |
| [`expand`](#expand) | mod |  |
| [`MaybeItemFn`](#maybeitemfn) | struct | This is a more flexible/imprecise `ItemFn` type, which's block is just a `TokenStream` (it may contain invalid code). |
| [`MaybeItemFnRef`](#maybeitemfnref) | struct | A generic reference type for `MaybeItemFn`, that takes a generic block type `B` that implements `ToTokens` (eg. |
| [`instrument_speculative`](#instrument-speculative) | fn | Instrument the function, without parsing the function body (instead using the raw tokens). |
| [`instrument_precise`](#instrument-precise) | fn | Instrument the function, by fully parsing the function body, which allows us to rewrite some statements related to async-like patterns. |

## Modules

- [`attr`](attr/index.md)
- [`expand`](expand/index.md)

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

*Defined in [`tracing-attributes-0.1.31/src/lib.rs:638-645`](../../.source_1765521767/tracing-attributes-0.1.31/src/lib.rs#L638-L645)*

This is a more flexible/imprecise `ItemFn` type,
which's block is just a `TokenStream` (it may contain invalid code).

#### Implementations

- <span id="maybeitemfn-as-ref"></span>`fn as_ref(&self) -> MaybeItemFnRef<'_, TokenStream>` — [`MaybeItemFnRef`](#maybeitemfnref)

#### Trait Implementations

##### `impl Any for MaybeItemFn`

- <span id="maybeitemfn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MaybeItemFn`

- <span id="maybeitemfn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MaybeItemFn`

- <span id="maybeitemfn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MaybeItemFn`

- <span id="maybeitemfn-clone"></span>`fn clone(&self) -> MaybeItemFn` — [`MaybeItemFn`](#maybeitemfn)

##### `impl CloneToUninit for MaybeItemFn`

- <span id="maybeitemfn-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for MaybeItemFn`

- <span id="maybeitemfn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MaybeItemFn`

- <span id="maybeitemfn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MaybeItemFn`

- <span id="maybeitemfn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for MaybeItemFn`

- <span id="maybeitemfn-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

##### `impl ToOwned for MaybeItemFn`

- <span id="maybeitemfn-toowned-type-owned"></span>`type Owned = T`

- <span id="maybeitemfn-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="maybeitemfn-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MaybeItemFn`

- <span id="maybeitemfn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="maybeitemfn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MaybeItemFn`

- <span id="maybeitemfn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="maybeitemfn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`tracing-attributes-0.1.31/src/lib.rs:710-717`](../../.source_1765521767/tracing-attributes-0.1.31/src/lib.rs#L710-L717)*

A generic reference type for `MaybeItemFn`,
that takes a generic block type `B` that implements `ToTokens` (eg. `TokenStream`, `Block`).

#### Trait Implementations

##### `impl Any for MaybeItemFnRef<'a, B>`

- <span id="maybeitemfnref-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MaybeItemFnRef<'a, B>`

- <span id="maybeitemfnref-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MaybeItemFnRef<'a, B>`

- <span id="maybeitemfnref-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<B: clone::Clone + ToTokens> Clone for MaybeItemFnRef<'a, B>`

- <span id="maybeitemfnref-clone"></span>`fn clone(&self) -> MaybeItemFnRef<'a, B>` — [`MaybeItemFnRef`](#maybeitemfnref)

##### `impl CloneToUninit for MaybeItemFnRef<'a, B>`

- <span id="maybeitemfnref-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<B: fmt::Debug + ToTokens> Debug for MaybeItemFnRef<'a, B>`

- <span id="maybeitemfnref-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MaybeItemFnRef<'a, B>`

- <span id="maybeitemfnref-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MaybeItemFnRef<'a, B>`

- <span id="maybeitemfnref-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for MaybeItemFnRef<'a, B>`

- <span id="maybeitemfnref-toowned-type-owned"></span>`type Owned = T`

- <span id="maybeitemfnref-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="maybeitemfnref-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MaybeItemFnRef<'a, B>`

- <span id="maybeitemfnref-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="maybeitemfnref-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MaybeItemFnRef<'a, B>`

- <span id="maybeitemfnref-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="maybeitemfnref-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `instrument_speculative`

```rust
fn instrument_speculative(args: attr::InstrumentArgs, item: proc_macro::TokenStream) -> proc_macro::TokenStream
```

*Defined in [`tracing-attributes-0.1.31/src/lib.rs:587-600`](../../.source_1765521767/tracing-attributes-0.1.31/src/lib.rs#L587-L600)*

Instrument the function, without parsing the function body (instead using the raw tokens).

### `instrument_precise`

```rust
fn instrument_precise(args: attr::InstrumentArgs, item: proc_macro::TokenStream) -> Result<proc_macro::TokenStream, syn::Error>
```

*Defined in [`tracing-attributes-0.1.31/src/lib.rs:604-633`](../../.source_1765521767/tracing-attributes-0.1.31/src/lib.rs#L604-L633)*

Instrument the function, by fully parsing the function body,
which allows us to rewrite some statements related to async-like patterns.

