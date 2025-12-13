*[cargo_docs_md](../../index.md) / [multi_crate](../index.md) / [parser](index.md)*

---

# Module `parser`

Multi-crate JSON parser.

This module provides [`MultiCrateParser`](#multicrateparser) which scans a directory for
rustdoc JSON files and parses them into a [`CrateCollection`](../collection/index.md).

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MultiCrateParser`](#multicrateparser) | struct | Parser for multiple rustdoc JSON files in a directory. |

## Structs

### `MultiCrateParser`

```rust
struct MultiCrateParser;
```

*Defined in `src/multi_crate/parser.rs:26`*

Parser for multiple rustdoc JSON files in a directory.

Discovers JSON files and parses each one, extracting the crate name
from the root module item.

# Example

```ignore
let crates = MultiCrateParser::parse_directory(Path::new("target/doc"))?;
println!("Found {} crates", crates.len());
```

#### Implementations

- <span id="multicrateparser-parse-directory"></span>`fn parse_directory(dir: &Path) -> Result<CrateCollection, Error>` — [`CrateCollection`](../collection/index.md#cratecollection), [`Error`](../../error/index.md#error)

  Parse all rustdoc JSON files in a directory.

  

  Scans the top level of the directory for `*.json` files and

  attempts to parse each one as rustdoc JSON. Files that aren't

  valid rustdoc JSON (e.g., search indices) are silently skipped.

  

  # Arguments

  

  * `dir` - Path to directory containing JSON files

  

  # Returns

  

  A `CrateCollection` containing all successfully parsed crates.

  

  # Errors

  

  - [`Error::InvalidDirectory`](../../index.md) if the path is invalid

  - [`Error::NoJsonFiles`](../../index.md) if no valid JSON files found

  - [`Error::DuplicateCrate`](../../index.md) if multiple files define the same crate

  - [`Error::NoCrateName`](../../index.md) if a JSON file has no root module

- <span id="multicrateparser-extract-crate-name"></span>`fn extract_crate_name(krate: &rustdoc_types::Crate, path: &Path) -> Result<String, Error>` — [`Error`](../../error/index.md#error)

  Extract the crate name from a parsed Crate.

  

  The crate name is stored in the root item's `name` field.

#### Trait Implementations

##### `impl Any for MultiCrateParser`

- <span id="multicrateparser-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MultiCrateParser`

- <span id="multicrateparser-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MultiCrateParser`

- <span id="multicrateparser-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for MultiCrateParser`

- <span id="multicrateparser-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for MultiCrateParser`

##### `impl<U> Into for MultiCrateParser`

- <span id="multicrateparser-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MultiCrateParser`

##### `impl OwoColorize for MultiCrateParser`

##### `impl Pointable for MultiCrateParser`

- <span id="multicrateparser-pointable-const-align"></span>`const ALIGN: usize`

- <span id="multicrateparser-pointable-type-init"></span>`type Init = T`

- <span id="multicrateparser-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="multicrateparser-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="multicrateparser-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="multicrateparser-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for MultiCrateParser`

- <span id="multicrateparser-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="multicrateparser-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MultiCrateParser`

- <span id="multicrateparser-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="multicrateparser-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for MultiCrateParser`

