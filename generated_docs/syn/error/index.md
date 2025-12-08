*[syn](../index.md) / [error](index.md)*

---

# Module `error`

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

- `fn new<T: Display>(span: Span, message: T) -> Self`

- `fn new_spanned<T: ToTokens, U: Display>(tokens: T, message: U) -> Self`

- `fn span(self: &Self) -> Span`

- `fn to_compile_error(self: &Self) -> TokenStream`

- `fn into_compile_error(self: Self) -> TokenStream`

- `fn combine(self: &mut Self, another: Error)` — [`Error`](../index.md)

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

### `ErrorMessage`

```rust
struct ErrorMessage {
    span: crate::thread::ThreadBound<SpanRange>,
    message: String,
}
```

#### Implementations

- `fn to_compile_error(self: &Self, tokens: &mut TokenStream)`

#### Trait Implementations

##### `impl Clone for ErrorMessage`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for ErrorMessage`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SpanRange`

```rust
struct SpanRange {
    start: proc_macro2::Span,
    end: proc_macro2::Span,
}
```

#### Trait Implementations

##### `impl Clone for SpanRange`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for SpanRange`

### `IntoIter`

```rust
struct IntoIter {
    messages: vec::IntoIter<ErrorMessage>,
}
```

#### Trait Implementations

##### `impl<I> IntoIterator for IntoIter`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator for IntoIter`

- `type Item = Error`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `Iter<'a>`

```rust
struct Iter<'a> {
    messages: slice::Iter<'a, ErrorMessage>,
}
```

#### Trait Implementations

##### `impl<I> IntoIterator for Iter<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for Iter<'a>`

- `type Item = Error`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

## Functions

### `new_at`

```rust
fn new_at<T: Display>(scope: proc_macro2::Span, cursor: crate::buffer::Cursor<'_>, message: T) -> Error
```

### `new2`

```rust
fn new2<T: Display>(start: proc_macro2::Span, end: proc_macro2::Span, message: T) -> Error
```

## Type Aliases

### `Result<T>`

```rust
type Result<T> = std::result::Result<T, Error>;
```

The result of a Syn parser.

