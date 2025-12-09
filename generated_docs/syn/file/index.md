*[syn](../index.md) / [file](index.md)*

---

# Module `file`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`File`](#file) | struct | A complete file of Rust source code. |

## Modules

- [`parsing`](parsing/index.md)
- [`printing`](printing/index.md)

## Structs

### `File`

```rust
struct File {
    pub shebang: Option<String>,
    pub attrs: Vec<crate::attr::Attribute>,
    pub items: Vec<crate::item::Item>,
}
```

*Defined in [`syn-2.0.111/src/file.rs:4-84`](../../../.source_1765210505/syn-2.0.111/src/file.rs#L4-L84)*

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

- <span id="cratefile-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::File`

- <span id="cratefile-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::File`

##### `impl Hash for crate::File`

- <span id="cratefile-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::file::File`

- <span id="cratefilefile-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::File`

- <span id="cratefile-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for File`

##### `impl Spanned for File`

- <span id="file-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::file::File`

- <span id="cratefilefile-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

