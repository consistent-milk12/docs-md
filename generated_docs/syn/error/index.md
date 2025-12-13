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

- <span id="error-combine"></span>`fn combine(&mut self, another: Error)` — [`Error`](#error)

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

##### `impl Any for ErrorMessage`

- <span id="errormessage-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ErrorMessage`

- <span id="errormessage-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ErrorMessage`

- <span id="errormessage-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ErrorMessage`

- <span id="errormessage-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ErrorMessage`

- <span id="errormessage-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ErrorMessage`

- <span id="errormessage-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ErrorMessage`

- <span id="errormessage-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ErrorMessage`

- <span id="errormessage-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ErrorMessage`

- <span id="errormessage-toowned-type-owned"></span>`type Owned = T`

- <span id="errormessage-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="errormessage-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ErrorMessage`

- <span id="errormessage-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="errormessage-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ErrorMessage`

- <span id="errormessage-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="errormessage-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SpanRange`

```rust
struct SpanRange {
    start: proc_macro2::Span,
    end: proc_macro2::Span,
}
```

*Defined in [`syn-2.0.111/src/error.rs:118-121`](../../../.source_1765521767/syn-2.0.111/src/error.rs#L118-L121)*

#### Trait Implementations

##### `impl Any for SpanRange`

- <span id="spanrange-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SpanRange`

- <span id="spanrange-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SpanRange`

- <span id="spanrange-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SpanRange`

- <span id="spanrange-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for SpanRange`

- <span id="spanrange-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SpanRange`

##### `impl<T> From for SpanRange`

- <span id="spanrange-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SpanRange`

- <span id="spanrange-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for SpanRange`

- <span id="spanrange-toowned-type-owned"></span>`type Owned = T`

- <span id="spanrange-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="spanrange-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SpanRange`

- <span id="spanrange-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="spanrange-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SpanRange`

- <span id="spanrange-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="spanrange-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IntoIter`

```rust
struct IntoIter {
    messages: vec::IntoIter<ErrorMessage>,
}
```

*Defined in [`syn-2.0.111/src/error.rs:423-425`](../../../.source_1765521767/syn-2.0.111/src/error.rs#L423-L425)*

#### Trait Implementations

##### `impl Any for IntoIter`

- <span id="intoiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IntoIter`

- <span id="intoiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntoIter`

- <span id="intoiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for IntoIter`

- <span id="intoiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IntoIter`

- <span id="intoiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for IntoIter`

- <span id="intoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for IntoIter`

- <span id="intoiter-iterator-type-item"></span>`type Item = Error`

- <span id="intoiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for IntoIter`

- <span id="intoiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="intoiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IntoIter`

- <span id="intoiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="intoiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Iter<'a>`

```rust
struct Iter<'a> {
    messages: slice::Iter<'a, ErrorMessage>,
}
```

*Defined in [`syn-2.0.111/src/error.rs:448-450`](../../../.source_1765521767/syn-2.0.111/src/error.rs#L448-L450)*

#### Trait Implementations

##### `impl Any for Iter<'a>`

- <span id="iter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Iter<'a>`

- <span id="iter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Iter<'a>`

- <span id="iter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Iter<'a>`

- <span id="iter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Iter<'a>`

- <span id="iter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Iter<'a>`

- <span id="iter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Iter<'a>`

- <span id="iter-iterator-type-item"></span>`type Item = Error`

- <span id="iter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for Iter<'a>`

- <span id="iter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Iter<'a>`

- <span id="iter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

