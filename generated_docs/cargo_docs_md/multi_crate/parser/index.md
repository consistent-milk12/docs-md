*[cargo_docs_md](../../index.md) / [multi_crate](../index.md) / [parser](index.md)*

---

# Module `parser`

Multi-crate JSON parser.

This module provides [`MultiCrateParser`](../../index.md) which scans a directory for
rustdoc JSON files and parses them into a [`CrateCollection`](../../index.md).

## Structs

### `MultiCrateParser`

```rust
struct MultiCrateParser;
```

Parser for multiple rustdoc JSON files in a directory.

Discovers JSON files and parses each one, extracting the crate name
from the root module item.

# Example

```ignore
let crates = MultiCrateParser::parse_directory(Path::new("target/doc"))?;
println!("Found {} crates", crates.len());
```

#### Implementations

- `fn parse_directory(dir: &Path) -> Result<CrateCollection, Error>` — [`CrateCollection`](../../index.md), [`Error`](../../error/index.md)

- `fn extract_crate_name(krate: &rustdoc_types::Crate, path: &Path) -> Result<String, Error>` — [`Error`](../../error/index.md)

#### Trait Implementations

##### `impl<T> Instrument for MultiCrateParser`

##### `impl<T> IntoEither for MultiCrateParser`

##### `impl<D> OwoColorize for MultiCrateParser`

##### `impl<T> Pointable for MultiCrateParser`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for MultiCrateParser`

