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

*Defined in [`syn-2.0.111/src/file.rs:4-84`](../../../.source_1765633015/syn-2.0.111/src/file.rs#L4-L84)*

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

- <span id="cratefilefile-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

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

