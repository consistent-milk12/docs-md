*[cargo_docs_md](../../index.md) / [multi_crate](../index.md) / [search](index.md)*

---

# Module `search`

Search index generation for multi-crate documentation.

This module provides [`SearchIndexGenerator`](#searchindexgenerator) which creates a JSON search index
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

*Defined in `src/multi_crate/search.rs:45-69`*

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

##### `impl Instrument for SearchEntry`

##### `impl IntoEither for SearchEntry`

##### `impl OwoColorize for SearchEntry`

##### `impl Pointable for SearchEntry`

- <span id="searchentry-pointable-const-align"></span>`const ALIGN: usize`

- <span id="searchentry-pointable-type-init"></span>`type Init = T`

- <span id="searchentry-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="searchentry-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="searchentry-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="searchentry-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl Serialize for SearchEntry`

- <span id="searchentry-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl WithSubscriber for SearchEntry`

### `SearchIndex`

```rust
struct SearchIndex {
    pub items: Vec<SearchEntry>,
}
```

*Defined in `src/multi_crate/search.rs:75-78`*

The complete search index containing all searchable items.

Serialized to `search_index.json` for client-side consumption.

#### Fields

- **`items`**: `Vec<SearchEntry>`

  All searchable items across all crates.

#### Trait Implementations

##### `impl Debug for SearchIndex`

- <span id="searchindex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for SearchIndex`

##### `impl IntoEither for SearchIndex`

##### `impl OwoColorize for SearchIndex`

##### `impl Pointable for SearchIndex`

- <span id="searchindex-pointable-const-align"></span>`const ALIGN: usize`

- <span id="searchindex-pointable-type-init"></span>`type Init = T`

- <span id="searchindex-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="searchindex-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="searchindex-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="searchindex-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl Serialize for SearchIndex`

- <span id="searchindex-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl WithSubscriber for SearchIndex`

### `SearchIndexGenerator<'a>`

```rust
struct SearchIndexGenerator<'a> {
    crates: &'a super::CrateCollection,
    include_private: bool,
    rendered_items: std::collections::HashMap<String, std::collections::HashSet<rustdoc_types::Id>>,
}
```

*Defined in `src/multi_crate/search.rs:93-108`*

Generator for multi-crate search indices.

Traverses all crates in a [`CrateCollection`](../collection/index.md) and builds a comprehensive
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

- <span id="searchindexgenerator-new"></span>`const fn new(crates: &'a CrateCollection, include_private: bool, rendered_items: HashMap<String, HashSet<Id>>) -> Self` — [`CrateCollection`](../collection/index.md#cratecollection)

- <span id="searchindexgenerator-generate"></span>`fn generate(&self) -> SearchIndex` — [`SearchIndex`](#searchindex)

- <span id="searchindexgenerator-write"></span>`fn write(&self, output_dir: &Path) -> std::io::Result<()>`

- <span id="searchindexgenerator-index-crate"></span>`fn index_crate(&self, items: &mut Vec<SearchEntry>, crate_name: &str, krate: &Crate)` — [`SearchEntry`](#searchentry)

- <span id="searchindexgenerator-build-path-map"></span>`fn build_path_map(krate: &Crate) -> HashMap<Id, String>`

- <span id="searchindexgenerator-compute-file-path"></span>`fn compute_file_path(crate_name: &str, module_path: &str, kind: &str) -> String`

#### Trait Implementations

##### `impl Instrument for SearchIndexGenerator<'a>`

##### `impl IntoEither for SearchIndexGenerator<'a>`

##### `impl OwoColorize for SearchIndexGenerator<'a>`

##### `impl Pointable for SearchIndexGenerator<'a>`

- <span id="searchindexgenerator-pointable-const-align"></span>`const ALIGN: usize`

- <span id="searchindexgenerator-pointable-type-init"></span>`type Init = T`

- <span id="searchindexgenerator-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="searchindexgenerator-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="searchindexgenerator-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="searchindexgenerator-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for SearchIndexGenerator<'a>`

