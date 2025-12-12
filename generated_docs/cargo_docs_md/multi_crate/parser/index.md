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

- <span id="multicrateparser-extract-crate-name"></span>`fn extract_crate_name(krate: &rustdoc_types::Crate, path: &Path) -> Result<String, Error>` — [`Error`](../../error/index.md#error)

#### Trait Implementations

##### `impl Instrument for MultiCrateParser`

##### `impl IntoEither for MultiCrateParser`

##### `impl OwoColorize for MultiCrateParser`

##### `impl Pointable for MultiCrateParser`

- <span id="multicrateparser-pointable-const-align"></span>`const ALIGN: usize`

- <span id="multicrateparser-pointable-type-init"></span>`type Init = T`

- <span id="multicrateparser-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="multicrateparser-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="multicrateparser-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="multicrateparser-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for MultiCrateParser`

