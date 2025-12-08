*[syn](../index.md) / [file](index.md)*

---

# Module `file`

## Modules

- [`parsing`](parsing/index.md) - 
- [`printing`](printing/index.md) - 

## Structs

### `File`

```rust
struct File {
    pub shebang: Option<String>,
    pub attrs: Vec<crate::attr::Attribute>,
    pub items: Vec<crate::item::Item>,
}
```

A complete file of Rust source code.

Typically `File` objects are created with [`parse_file`](../index.md).

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

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::File`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::File`

##### `impl Hash for crate::File`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::file::File`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::File`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for File`

##### `impl<T> Spanned for File`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::file::File`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

