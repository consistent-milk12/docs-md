*[docs_md](../../index.md) / [generator](../index.md) / [doc_links](index.md)*

---

# Module `doc_links`

Intra-doc link processing for documentation generation.

This module provides [`DocLinkProcessor`](#doclinkprocessor) which transforms rustdoc
intra-doc link syntax into proper markdown links.

# Processing Pipeline
The processor applies transformations in this order:
1. Strip markdown reference definitions
2. Unhide rustdoc hidden lines in code blocks
3. Process reference-style links `[text][`ref`](#ref)`
4. Process path reference links ``text``
5. Process method links `[Type::method]`
6. Process backtick links `[`Name`](#name)`
7. Process plain links `[name]`
8. Convert HTML-style rustdoc links
9. Clean up blank lines

Links inside code blocks are protected from transformation.

## Structs

### `DocLinkProcessor<'a>`

```rust
struct DocLinkProcessor<'a> {
    krate: &'a rustdoc_types::Crate,
    link_registry: &'a crate::linker::LinkRegistry,
    current_file: &'a str,
    path_name_index: std::collections::HashMap<&'a str, Vec<rustdoc_types::Id>>,
}
```

Processes doc comments to resolve intra-doc links to markdown links.

Rustdoc JSON includes a `links` field on each Item that maps intra-doc
link text to item IDs. This processor uses that map along with the
`LinkRegistry` to convert these to relative markdown links.

# Supported Patterns

- `` [`Name`](#name) `` - Backtick code links (most common)
- `` `path::to::Item` `` - Qualified path links
- `` `Type::method` `` - Method/associated item links
- `[name]` - Plain identifier links
- `[text][`ref`](#ref)` - Reference-style links
- ``text`` - Path reference links

# External Crate Links

Items from external crates are linked to docs.rs when possible.

# Code Block Protection

Links inside fenced code blocks are not processed.

#### Fields

- **`krate`**: `&'a rustdoc_types::Crate`

  The crate being documented (for looking up items).

- **`link_registry`**: `&'a crate::linker::LinkRegistry`

  Registry mapping IDs to file paths.

- **`current_file`**: `&'a str`

  The current file path (for relative link calculation).

- **`path_name_index`**: `std::collections::HashMap<&'a str, Vec<rustdoc_types::Id>>`

  Index mapping item names to their IDs for fast lookup.
  Built from `krate.paths` at construction time.

#### Implementations

- `fn new(krate: &'a Crate, link_registry: &'a LinkRegistry, current_file: &'a str) -> Self` — [`LinkRegistry`](../../../linker/index.md)

- `fn process(self: &Self, docs: &str, item_links: &HashMap<String, Id>) -> String`

- `fn process_links_protected(self: &Self, docs: &str, item_links: &HashMap<String, Id>) -> String`

- `fn process_line(self: &Self, line: &str, item_links: &HashMap<String, Id>) -> String`

- `fn process_reference_links(self: &Self, text: &str, item_links: &HashMap<String, Id>) -> String`

- `fn process_path_reference_links(self: &Self, text: &str, item_links: &HashMap<String, Id>) -> String`

- `fn process_method_links(self: &Self, text: &str, item_links: &HashMap<String, Id>) -> String`

- `fn process_backtick_links(self: &Self, text: &str, item_links: &HashMap<String, Id>) -> String`

- `fn process_plain_links(self: &Self, text: &str, item_links: &HashMap<String, Id>) -> String`

- `fn process_html_links_with_context(self: &Self, text: &str, item_links: &HashMap<String, Id>) -> String`

- `fn resolve_html_link_to_url(self: &Self, item_name: &str, item_kind: &str, item_links: &HashMap<String, Id>) -> Option<String>`

- `fn kind_matches(html_kind: &str, item_kind: ItemKind) -> bool`

- `fn clean_blank_lines(docs: &str) -> String`

- `fn resolve_to_url(self: &Self, link_text: &str, item_links: &HashMap<String, Id>) -> Option<String>`

- `fn get_url_for_id(self: &Self, id: Id) -> Option<String>`

- `fn get_docs_rs_url(path_info: &rustdoc_types::ItemSummary) -> Option<String>`

- `fn resolve_method_link(self: &Self, type_name: &str, method_name: &str, item_links: &HashMap<String, Id>) -> Option<String>`

- `fn resolve_link(self: &Self, link_text: &str, item_links: &HashMap<String, Id>) -> String`

- `fn create_link_for_id(self: &Self, id: Id, display_name: &str) -> Option<String>`

- `fn create_docs_rs_link(path_info: &rustdoc_types::ItemSummary, display_name: &str) -> Option<String>`

#### Trait Implementations

##### `impl IntoEither<T>`

##### `impl OwoColorize<D>`

##### `impl Pointable<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Functions

### `convert_html_links`

```rust
fn convert_html_links(docs: &str) -> String
```

Convert HTML-style rustdoc links to markdown anchors.

Transforms links like:
- `(#numberprefix)` -> `(#numberprefix)`
- `` -> removes the link (methods don't have anchors)

This is useful for multi-crate documentation where the full processor
context may not be available.

### `strip_duplicate_title`

```rust
fn strip_duplicate_title<'a>(docs: &'a str, item_name: &str) -> &'a str
```

Strip duplicate title from documentation.

Some crate/module docs start with `# title` which duplicates the generated
`# Crate 'name'` or `# Module 'name'` heading.

# Arguments

* `docs` - The documentation string to process
* `item_name` - The name of the crate or module being documented

# Returns

The docs with the leading title removed if it matches the item name,
otherwise the original docs unchanged.

### `strip_reference_definitions`

```rust
fn strip_reference_definitions(docs: &str) -> String
```

Strip markdown reference definition lines.

Removes lines like `[`Name`](#name): path::to::item` which are no longer needed
after intra-doc links are processed.

### `unhide_code_lines`

```rust
fn unhide_code_lines(docs: &str) -> String
```

Unhide rustdoc hidden lines in code blocks and add language identifiers.

This function performs two transformations on code blocks:
1. Lines starting with `# ` inside code blocks are hidden in rustdoc
   but compiled. We remove the prefix to show the full example.
2. Bare code fences (` ``` `) are converted to ` ```rust ` since doc
   examples are Rust code.

### `convert_path_reference_links`

```rust
fn convert_path_reference_links(docs: &str) -> String
```

Convert path-style reference links to inline code.

Transforms: ```ProgressTracker```
Into: `` `ProgressTracker` ``

Without full link resolution context, we can't create valid anchors,
so we preserve the display text as inline code.

