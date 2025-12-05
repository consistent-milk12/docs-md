*[syn](../index.md) / [meta](index.md)*

---

# Module `meta`

Facility for interpreting structured content inside of an `Attribute`.

## Structs

### `ParseNestedMeta<'a>`

```rust
struct ParseNestedMeta<'a> {
    pub path: crate::path::Path,
    pub input: crate::parse::ParseStream<'a>,
}
```

Context for parsing a single property in the conventional syntax for
structured attributes.

# Examples

Refer to usage examples on the following two entry-points:

- `Attribute::parse_nested_meta` if you have an entire `Attribute` to
  parse. Always use this if possible. Generally this is able to produce
  better error messages because `Attribute` holds span information for all
  of the delimiters therein.

- `syn::meta::parser` if you are implementing a `proc_macro_attribute`
  macro and parsing the arguments to the attribute macro, i.e. the ones
  written in the same attribute that dispatched the macro invocation. Rustc
  does not pass span information for the surrounding delimiters into the
  attribute macro invocation in this situation, so error messages might be
  less precise.



#### Implementations

- `fn value(self: &Self) -> Result<ParseStream<'a>>`
  Used when parsing `key = "value"` syntax.

- `fn parse_nested_meta(self: &Self, logic: impl FnMut(ParseNestedMeta<'_>) -> Result<()>) -> Result<()>`
  Used when parsing `list(...)` syntax **if** the content inside the

- `fn error(self: &Self, msg: impl Display) -> Error`
  Report that the attribute's content did not conform to expectations.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `parser`

```rust
fn parser(logic: impl FnMut(ParseNestedMeta<'_>) -> crate::error::Result<()>) -> impl Parser<Output = ()>
```

Make a parser that is usable with `parse_macro_input!` in a
`#[proc_macro_attribute]` macro.

*Warning:* When parsing attribute args **other than** the
`proc_macro::TokenStream` input of a `proc_macro_attribute`, you do **not**
need this function. In several cases your callers will get worse error
messages if you use this function, because the surrounding delimiter's span
is concealed from attribute macros by rustc. Use
`Attribute::parse_nested_meta` instead.

# Example

This example implements an attribute macro whose invocations look like this:

```rust
const IGNORE: &str = stringify! {
#[tea(kind = "EarlGrey", hot)]
struct Picard {...}
};
```

The "parameters" supported by the attribute are:

- `kind = "..."`
- `hot`
- `with(sugar, milk, ...)`, a comma-separated list of ingredients

```rust
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, LitStr, Path};

const IGNORE: &str = stringify! {
#[proc_macro_attribute]
};
pub fn tea(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut kind: Option<LitStr> = None;
    let mut hot: bool = false;
    let mut with: Vec<Path> = Vec::new();
    let tea_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("kind") {
            kind = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("hot") {
            hot = true;
            Ok(())
        } else if meta.path.is_ident("with") {
            meta.parse_nested_meta(|meta| {
                with.push(meta.path);
                Ok(())
            })
        } else {
            Err(meta.error("unsupported tea property"))
        }
    });

    parse_macro_input!(args with tea_parser);
    eprintln!("kind={kind:?} hot={hot} with={with:?}");

    /* ... */
  TokenStream::new()
}
```

The `syn::meta` library will take care of dealing with the commas including
trailing commas, and producing sensible error messages on unexpected input.

```console
error: expected `,`
 --> src/main.rs:3:37
  |
3 | #[tea(kind = "EarlGrey", with(sugar = "lol", milk))]
  |                                     ^
```

# Example

Same as above but we factor out most of the logic into a separate function.

```rust
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::meta::ParseNestedMeta;
use syn::parse::{Parser, Result};
use syn::{parse_macro_input, LitStr, Path};

const IGNORE: &str = stringify! {
#[proc_macro_attribute]
};
pub fn tea(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut attrs = TeaAttributes::default();
    let tea_parser = syn::meta::parser(|meta| attrs.parse(meta));
    parse_macro_input!(args with tea_parser);

    /* ... */
  TokenStream::new()
}

#[derive(Default)]
struct TeaAttributes {
    kind: Option<LitStr>,
    hot: bool,
    with: Vec<Path>,
}

impl TeaAttributes {
    fn parse(&mut self, meta: ParseNestedMeta) -> Result<()> {
        if meta.path.is_ident("kind") {
            self.kind = Some(meta.value()?.parse()?);
            Ok(())
        } else /* just like in last example */
          { unimplemented!() }

    }
}
```

