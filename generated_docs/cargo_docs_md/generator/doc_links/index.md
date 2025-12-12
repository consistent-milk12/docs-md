*[cargo_docs_md](../../index.md) / [generator](../index.md) / [doc_links](index.md)*

---

# Module `doc_links`

Intra-doc link processing for documentation generation.

This module provides [`DocLinkProcessor`](#doclinkprocessor) which transforms rustdoc
intra-doc link syntax into proper markdown links.

# Processing Pipeline
The processor applies transformations in this order:
1. Strip markdown reference definitions
2. Unhide rustdoc hidden lines in code blocks
3. Process reference-style links `[text]`ref``
4. Process path reference links ``text``
5. Process method links `[Type::method]`
6. Process backtick links `[`Name`](#name)`
7. Process plain links `[name]`
8. Convert HTML-style rustdoc links
9. Clean up blank lines

Links inside code blocks are protected from transformation.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CodeBlockTracker`](#codeblocktracker) | struct | Tracks code block state while processing documentation line by line. |
| [`DocLinkProcessor`](#doclinkprocessor) | struct | Processes doc comments to resolve intra-doc links to markdown links. |
| [`DocLinkUtils`](#doclinkutils) | struct | Utility functions for document links |
| [`LineKind`](#linekind) | enum | Classification of a line during code block processing. |

## Structs

### `CodeBlockTracker`

```rust
struct CodeBlockTracker {
    fence: Option<&'static str>,
}
```

*Defined in `src/generator/doc_links.rs:271-275`*

Tracks code block state while processing documentation line by line.

This provides a clean state machine for fence tracking that both
`unhide_code_lines` and `process_links_protected` can use, avoiding
duplicated inline fence detection logic.

# Example

```text
let mut tracker = CodeBlockTracker::new();

for line in docs.lines() {
    match tracker.classify(line) {
        LineKind::OpeningFence { bare } => { /* handle opening */ }
        LineKind::CodeContent => { /* process hidden lines, etc. */ }
        LineKind::ClosingFence => { /* output as-is */ }
        LineKind::Text => { /* process links */ }
    }
}
```

# Fence Matching

The tracker correctly handles mismatched fences:
- `~~~` inside a ```` ``` ```` block is treated as content, not a closing fence
- Only the same fence style closes a block

#### Fields

- **`fence`**: `Option<&'static str>`

  Current fence string if inside a code block (`Some("```")` or `Some("~~~")`).
  `None` when outside any code block.

#### Implementations

- <span id="codeblocktracker-new"></span>`const fn new() -> Self`

- <span id="codeblocktracker-classify"></span>`fn classify(&mut self, line: &str) -> LineKind` — [`LineKind`](#linekind)

#### Trait Implementations

##### `impl Instrument for CodeBlockTracker`

##### `impl IntoEither for CodeBlockTracker`

##### `impl OwoColorize for CodeBlockTracker`

##### `impl Pointable for CodeBlockTracker`

- <span id="codeblocktracker-pointable-const-align"></span>`const ALIGN: usize`

- <span id="codeblocktracker-pointable-type-init"></span>`type Init = T`

- <span id="codeblocktracker-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="codeblocktracker-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="codeblocktracker-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="codeblocktracker-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for CodeBlockTracker`

### `DocLinkProcessor<'a>`

```rust
struct DocLinkProcessor<'a> {
    krate: &'a rustdoc_types::Crate,
    link_registry: &'a crate::linker::LinkRegistry,
    current_file: &'a str,
    path_name_index: std::collections::HashMap<&'a str, Vec<rustdoc_types::Id>>,
}
```

*Defined in `src/generator/doc_links.rs:365-378`*

Processes doc comments to resolve intra-doc links to markdown links.

Rustdoc JSON includes a `links` field on each Item that maps intra-doc
link text to item IDs. This processor uses that map along with the
`LinkRegistry` to convert these to relative markdown links.

# Supported Patterns

- `` [`Name`](#name) `` - Backtick code links (most common)
- `` `path::to::Item` `` - Qualified path links
- `` `Type::method` `` - Method/associated item links
- `[name]` - Plain identifier links
- `[text]`ref`` - Reference-style links
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

- <span id="doclinkprocessor-with-index"></span>`fn with_index(krate: &'a Crate, link_registry: &'a LinkRegistry, current_file: &'a str, path_name_index: &HashMap<&'a str, Vec<Id>>) -> Self` — [`LinkRegistry`](../../linker/index.md#linkregistry)

- <span id="doclinkprocessor-new"></span>`fn new(krate: &'a Crate, link_registry: &'a LinkRegistry, current_file: &'a str) -> Self` — [`LinkRegistry`](../../linker/index.md#linkregistry)

- <span id="doclinkprocessor-process"></span>`fn process(&self, docs: &str, item_links: &HashMap<String, Id>) -> String`

- <span id="doclinkprocessor-process-links-protected"></span>`fn process_links_protected(&self, docs: &str, item_links: &HashMap<String, Id>) -> String`

- <span id="doclinkprocessor-process-line"></span>`fn process_line(&self, line: &str, item_links: &HashMap<String, Id>) -> String`

- <span id="doclinkprocessor-process-reference-links"></span>`fn process_reference_links(&self, text: &str, item_links: &HashMap<String, Id>) -> String`

- <span id="doclinkprocessor-process-path-reference-links"></span>`fn process_path_reference_links(&self, text: &str, item_links: &HashMap<String, Id>) -> String`

- <span id="doclinkprocessor-process-method-links"></span>`fn process_method_links(&self, text: &str, item_links: &HashMap<String, Id>) -> String`

- <span id="doclinkprocessor-process-backtick-links"></span>`fn process_backtick_links(&self, text: &str, item_links: &HashMap<String, Id>) -> String`

- <span id="doclinkprocessor-process-plain-links"></span>`fn process_plain_links(&self, text: &str, item_links: &HashMap<String, Id>) -> String`

- <span id="doclinkprocessor-process-html-links-with-context"></span>`fn process_html_links_with_context(&self, text: &str, item_links: &HashMap<String, Id>) -> String`

- <span id="doclinkprocessor-resolve-html-link-to-url"></span>`fn resolve_html_link_to_url(&self, item_name: &str, item_kind: &str, item_links: &HashMap<String, Id>) -> Option<String>`

- <span id="doclinkprocessor-kind-matches"></span>`fn kind_matches(html_kind: &str, item_kind: ItemKind) -> bool`

- <span id="doclinkprocessor-clean-blank-lines"></span>`fn clean_blank_lines(docs: &str) -> String`

- <span id="doclinkprocessor-resolve-with-strategies"></span>`fn resolve_with_strategies<T, F>(&self, link_text: &str, item_links: &HashMap<String, Id>, resolver: F) -> Option<T>`

- <span id="doclinkprocessor-resolve-to-url"></span>`fn resolve_to_url(&self, link_text: &str, item_links: &HashMap<String, Id>) -> Option<String>`

- <span id="doclinkprocessor-get-url-for-id"></span>`fn get_url_for_id(&self, id: Id) -> Option<String>`

- <span id="doclinkprocessor-get-docs-rs-url"></span>`fn get_docs_rs_url(path_info: &rustdoc_types::ItemSummary) -> Option<String>`

- <span id="doclinkprocessor-resolve-method-link"></span>`fn resolve_method_link(&self, type_name: &str, method_name: &str, item_links: &HashMap<String, Id>) -> Option<String>`

- <span id="doclinkprocessor-resolve-link"></span>`fn resolve_link(&self, link_text: &str, item_links: &HashMap<String, Id>) -> String`

- <span id="doclinkprocessor-create-link-for-id"></span>`fn create_link_for_id(&self, id: Id, display_name: &str) -> Option<String>`

- <span id="doclinkprocessor-create-docs-rs-link"></span>`fn create_docs_rs_link(path_info: &rustdoc_types::ItemSummary, display_name: &str) -> Option<String>`

#### Trait Implementations

##### `impl Instrument for DocLinkProcessor<'a>`

##### `impl IntoEither for DocLinkProcessor<'a>`

##### `impl OwoColorize for DocLinkProcessor<'a>`

##### `impl Pointable for DocLinkProcessor<'a>`

- <span id="doclinkprocessor-pointable-const-align"></span>`const ALIGN: usize`

- <span id="doclinkprocessor-pointable-type-init"></span>`type Init = T`

- <span id="doclinkprocessor-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="doclinkprocessor-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="doclinkprocessor-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="doclinkprocessor-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for DocLinkProcessor<'a>`

### `DocLinkUtils`

```rust
struct DocLinkUtils;
```

*Defined in `src/generator/doc_links.rs:1051`*

Utility functions for document links

#### Implementations

- <span id="doclinkutils-convert-html-links"></span>`fn convert_html_links(docs: &str) -> String`

- <span id="doclinkutils-strip-duplicate-title"></span>`fn strip_duplicate_title<'a>(docs: &'a str, item_name: &str) -> &'a str`

- <span id="doclinkutils-strip-reference-definitions"></span>`fn strip_reference_definitions(docs: &str) -> String`

- <span id="doclinkutils-unhide-code-lines"></span>`fn unhide_code_lines(docs: &str) -> String`

- <span id="doclinkutils-convert-path-reference-links"></span>`fn convert_path_reference_links(docs: &str) -> String`

- <span id="doclinkutils-replace-with-regex"></span>`fn replace_with_regex<F>(text: &str, re: &Regex, replacer: F) -> String`

- <span id="doclinkutils-replace-with-regex-checked"></span>`fn replace_with_regex_checked<F>(text: &str, re: &Regex, replacer: F) -> String`

#### Trait Implementations

##### `impl Instrument for DocLinkUtils`

##### `impl IntoEither for DocLinkUtils`

##### `impl OwoColorize for DocLinkUtils`

##### `impl Pointable for DocLinkUtils`

- <span id="doclinkutils-pointable-const-align"></span>`const ALIGN: usize`

- <span id="doclinkutils-pointable-type-init"></span>`type Init = T`

- <span id="doclinkutils-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="doclinkutils-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="doclinkutils-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="doclinkutils-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for DocLinkUtils`

## Enums

### `LineKind`

```rust
enum LineKind {
    OpeningFence {
        bare: bool,
    },
    ClosingFence,
    CodeContent,
    Text,
}
```

*Defined in `src/generator/doc_links.rs:230-243`*

Classification of a line during code block processing.

Used by [`CodeBlockTracker`](#codeblocktracker) to provide rich information about each line,
enabling callers to handle fences and content appropriately.

# Processing Flow

```text
Input Line          │ State Before │ Returns             │ State After
────────────────────┼──────────────┼─────────────────────┼─────────────
"```"               │ Outside      │ OpeningFence(bare)  │ Inside(```)
"```rust"           │ Outside      │ OpeningFence(!bare) │ Inside(```)
"let x = 1;"        │ Inside(```)  │ CodeContent         │ Inside(```)
"```"               │ Inside(```)  │ ClosingFence        │ Outside
"regular text"      │ Outside      │ Text                │ Outside
"~~~"               │ Inside(```)  │ CodeContent         │ Inside(```) ← mismatched!
```

#### Variants

- **`OpeningFence`**

  Opening code fence (``` or ~~~).
  `bare` is true if the fence has no language specifier (exactly "```" or "~~~").

- **`ClosingFence`**

  Closing code fence matching the opening fence.

- **`CodeContent`**

  Content inside a code block (not a fence line).

- **`Text`**

  Regular text outside any code block.

#### Trait Implementations

##### `impl Clone for LineKind`

- <span id="linekind-clone"></span>`fn clone(&self) -> LineKind` — [`LineKind`](#linekind)

##### `impl Copy for LineKind`

##### `impl Debug for LineKind`

- <span id="linekind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LineKind`

##### `impl<K> Equivalent for LineKind`

- <span id="linekind-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Instrument for LineKind`

##### `impl IntoEither for LineKind`

##### `impl OwoColorize for LineKind`

##### `impl PartialEq for LineKind`

- <span id="linekind-eq"></span>`fn eq(&self, other: &LineKind) -> bool` — [`LineKind`](#linekind)

##### `impl Pointable for LineKind`

- <span id="linekind-pointable-const-align"></span>`const ALIGN: usize`

- <span id="linekind-pointable-type-init"></span>`type Init = T`

- <span id="linekind-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="linekind-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="linekind-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="linekind-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl StructuralPartialEq for LineKind`

##### `impl WithSubscriber for LineKind`

