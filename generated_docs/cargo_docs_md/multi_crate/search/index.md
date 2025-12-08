*[cargo_docs_md](../../index.md) / [multi_crate](../index.md) / [search](index.md)*

---

# Module `search`

Search index generation for multi-crate documentation.

This module provides [`SearchIndexGenerator`](../../index.md) which creates a JSON search index
containing all documented items across multiple crates. The index can be used
with client-side search libraries like Fuse.js, Lunr.js, or `FlexSearch`.

# Output Format

The generated `search_index.json` contains:

```json
{
  "items": [
    {
      "name": "Span",
      "path": "tracing::span::Span",
      "kind": "struct",
      "crate": "tracing",
      "file": "tracing/span/index.md",
      "summary": "A handle representing a span..."
    }
  ]
}
```

# Usage

```ignore
let generator = SearchIndexGenerator::new(&crates);
generator.write(Path::new("generated_docs/"))?;
```

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SearchEntry`](#searchentry) | struct | A single searchable item in the index. |
| [`SearchIndex`](#searchindex) | struct | The complete search index containing all searchable items. |
| [`SearchIndexGenerator`](#searchindexgenerator) | struct | Generator for multi-crate search indices. |

## Structs

### `SearchEntry`

```rust
struct SearchEntry {
    pub name: String,
    pub path: String,
    pub kind: &'static str,
    pub crate_name: String,
    pub file: String,
    pub summary: Option<String>,
}
```

A single searchable item in the index.

Contains all metadata needed for search and display in results.

#### Fields

- **`name`**: `String`

  Item name (e.g., "Span", "spawn", "Error").

- **`path`**: `String`

  Full path including crate (e.g., "`tracing::span::Span`").

- **`kind`**: `&'static str`

  Item kind for filtering and display.
  
  One of: "mod", "struct", "enum", "trait", "fn", "type", "const", "macro"

- **`crate_name`**: `String`

  Crate this item belongs to.

- **`file`**: `String`

  Relative file path to the markdown documentation.

- **`summary`**: `Option<String>`

  First line of documentation for preview in search results.
  
  `None` if the item has no documentation.

#### Trait Implementations

##### `impl Debug for SearchEntry`

- <span id="searchentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Instrument for SearchEntry`

##### `impl<T> IntoEither for SearchEntry`

##### `impl<D> OwoColorize for SearchEntry`

##### `impl<T> Pointable for SearchEntry`

- <span id="searchentry-align"></span>`const ALIGN: usize`

- <span id="searchentry-init"></span>`type Init = T`

- <span id="searchentry-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="searchentry-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="searchentry-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="searchentry-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl Serialize for SearchEntry`

- <span id="searchentry-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl<T> WithSubscriber for SearchEntry`

### `SearchIndex`

```rust
struct SearchIndex {
    pub items: Vec<SearchEntry>,
}
```

The complete search index containing all searchable items.

Serialized to `search_index.json` for client-side consumption.

#### Fields

- **`items`**: `Vec<SearchEntry>`

  All searchable items across all crates.

#### Trait Implementations

##### `impl Debug for SearchIndex`

- <span id="searchindex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Instrument for SearchIndex`

##### `impl<T> IntoEither for SearchIndex`

##### `impl<D> OwoColorize for SearchIndex`

##### `impl<T> Pointable for SearchIndex`

- <span id="searchindex-align"></span>`const ALIGN: usize`

- <span id="searchindex-init"></span>`type Init = T`

- <span id="searchindex-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="searchindex-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="searchindex-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="searchindex-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl Serialize for SearchIndex`

- <span id="searchindex-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl<T> WithSubscriber for SearchIndex`

### `SearchIndexGenerator<'a>`

```rust
struct SearchIndexGenerator<'a> {
    crates: &'a super::CrateCollection,
    include_private: bool,
    rendered_items: std::collections::HashMap<String, std::collections::HashSet<rustdoc_types::Id>>,
}
```

Generator for multi-crate search indices.

Traverses all crates in a [`CrateCollection`](../../index.md) and builds a comprehensive
search index of all public items (or all items if `include_private` is set).

# Example

```ignore
let crates = MultiCrateParser::parse_directory(Path::new("target/doc"))?;
let rendered_items = generator.generate();  // Returns HashMap<String, HashSet<Id>>
let generator = SearchIndexGenerator::new(&crates, false, rendered_items);
generator.write(Path::new("generated_docs/"))?;
```

#### Fields

- **`crates`**: `&'a super::CrateCollection`

  Collection of crates to index.

- **`include_private`**: `bool`

  Whether to include private items in the search index.
  
  When false (default), only public items are indexed.
  When true, all items regardless of visibility are indexed.

- **`rendered_items`**: `std::collections::HashMap<String, std::collections::HashSet<rustdoc_types::Id>>`

  Set of item IDs that were actually rendered per crate.
  
  Only items in this set will appear in the search index.
  This ensures the search index matches the generated documentation.

#### Implementations

- <span id="searchindexgenerator-new"></span>`const fn new(crates: &'a CrateCollection, include_private: bool, rendered_items: HashMap<String, HashSet<Id>>) -> Self` — [`CrateCollection`](../../index.md)

- <span id="searchindexgenerator-generate"></span>`fn generate(&self) -> SearchIndex` — [`SearchIndex`](../../index.md)

- <span id="searchindexgenerator-write"></span>`fn write(&self, output_dir: &Path) -> std::io::Result<()>`

- <span id="searchindexgenerator-index-crate"></span>`fn index_crate(&self, items: &mut Vec<SearchEntry>, crate_name: &str, krate: &Crate)` — [`SearchEntry`](#searchentry)

- <span id="searchindexgenerator-build-path-map"></span>`fn build_path_map(krate: &Crate) -> HashMap<Id, String>`

- <span id="searchindexgenerator-compute-file-path"></span>`fn compute_file_path(crate_name: &str, module_path: &str, kind: &str) -> String`

#### Trait Implementations

##### `impl<T> Instrument for SearchIndexGenerator<'a>`

##### `impl<T> IntoEither for SearchIndexGenerator<'a>`

##### `impl<D> OwoColorize for SearchIndexGenerator<'a>`

##### `impl<T> Pointable for SearchIndexGenerator<'a>`

- <span id="searchindexgenerator-align"></span>`const ALIGN: usize`

- <span id="searchindexgenerator-init"></span>`type Init = T`

- <span id="searchindexgenerator-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="searchindexgenerator-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="searchindexgenerator-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="searchindexgenerator-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for SearchIndexGenerator<'a>`

