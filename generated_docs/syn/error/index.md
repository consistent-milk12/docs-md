*[syn](../index.md) / [error](index.md)*

---

# Module `error`

## Contents

- [Structs](#structs)
  - [`Error`](#error)
  - [`ErrorMessage`](#errormessage)
  - [`SpanRange`](#spanrange)
  - [`IntoIter`](#intoiter)
  - [`Iter`](#iter)
- [Functions](#functions)
  - [`new_at`](#new-at)
  - [`new2`](#new2)
- [Type Aliases](#type-aliases)
  - [`Result`](#result)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Error`](#error) | struct | Error returned when a Syn parser cannot parse the input tokens. |
| [`ErrorMessage`](#errormessage) | struct |  |
| [`SpanRange`](#spanrange) | struct |  |
| [`IntoIter`](#intoiter) | struct |  |
| [`Iter`](#iter) | struct |  |
| [`new_at`](#new-at) | fn |  |
| [`new2`](#new2) | fn |  |
| [`Result`](#result) | type | The result of a Syn parser. |

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

- <span id="error-intoiterator-type-item"></span>`type Item = Error`

- <span id="error-intoiterator-type-intoiter"></span>`type IntoIter = IntoIter`

- <span id="error-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl ToString for Error`

- <span id="error-to-string"></span>`fn to_string(&self) -> String`

### `ErrorMessage`

```rust
struct ErrorMessage {
    span: crate::thread::ThreadBound<SpanRange>,
    message: String,
}
```

*Defined in [`syn-2.0.111/src/error.rs:105-113`](../../../.source_1765521767/syn-2.0.111/src/error.rs#L105-L113)*

#### Implementations

- <span id="errormessage-to-compile-error"></span>`fn to_compile_error(&self, tokens: &mut TokenStream)`

#### Trait Implementations

##### `impl Clone for ErrorMessage`

- <span id="errormessage-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for ErrorMessage`

- <span id="errormessage-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SpanRange`

```rust
struct SpanRange {
    start: proc_macro2::Span,
    end: proc_macro2::Span,
}
```

*Defined in [`syn-2.0.111/src/error.rs:118-121`](../../../.source_1765521767/syn-2.0.111/src/error.rs#L118-L121)*

#### Trait Implementations

##### `impl Clone for SpanRange`

- <span id="spanrange-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for SpanRange`

### `IntoIter`

```rust
struct IntoIter {
    messages: vec::IntoIter<ErrorMessage>,
}
```

*Defined in [`syn-2.0.111/src/error.rs:423-425`](../../../.source_1765521767/syn-2.0.111/src/error.rs#L423-L425)*

#### Trait Implementations

##### `impl IntoIterator for IntoIter`

- <span id="intoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for IntoIter`

- <span id="intoiter-iterator-type-item"></span>`type Item = Error`

- <span id="intoiter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `Iter<'a>`

```rust
struct Iter<'a> {
    messages: slice::Iter<'a, ErrorMessage>,
}
```

*Defined in [`syn-2.0.111/src/error.rs:448-450`](../../../.source_1765521767/syn-2.0.111/src/error.rs#L448-L450)*

#### Trait Implementations

##### `impl IntoIterator for Iter<'a>`

- <span id="iter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Iter<'a>`

- <span id="iter-iterator-type-item"></span>`type Item = Error`

- <span id="iter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Functions

### `new_at`

```rust
fn new_at<T: Display>(scope: proc_macro2::Span, cursor: crate::buffer::Cursor<'_>, message: T) -> Error
```

*Defined in [`syn-2.0.111/src/error.rs:328-335`](../../../.source_1765521767/syn-2.0.111/src/error.rs#L328-L335)*

### `new2`

```rust
fn new2<T: Display>(start: proc_macro2::Span, end: proc_macro2::Span, message: T) -> Error
```

*Defined in [`syn-2.0.111/src/error.rs:338-349`](../../../.source_1765521767/syn-2.0.111/src/error.rs#L338-L349)*

## Type Aliases

### `Result<T>`

```rust
type Result<T> = std::result::Result<T, Error>;
```

*Defined in [`syn-2.0.111/src/error.rs:15`](../../../.source_1765521767/syn-2.0.111/src/error.rs#L15)*

The result of a Syn parser.

