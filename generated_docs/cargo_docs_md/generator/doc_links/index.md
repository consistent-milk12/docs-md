*[cargo_docs_md](../../index.md) / [generator](../index.md) / [doc_links](index.md)*

---

# Module `doc_links`

Intra-doc link processing for documentation generation.

This module provides [`DocLinkProcessor`](../index.md) which transforms rustdoc
intra-doc link syntax into proper markdown links.

# Processing Pipeline
The processor applies transformations in this order:
1. Strip markdown reference definitions
2. Unhide rustdoc hidden lines in code blocks
3. Process reference-style links `[text]`ref``
4. Process path reference links ``text``
5. Process method links `[Type::method]`
6. Process backtick links ``Name``
7. Process plain links `[name]`
8. Convert HTML-style rustdoc links
9. Clean up blank lines

Links inside code blocks are protected from transformation.

## Contents

- [Structs](#structs)
  - [`DocLinkProcessor`](#doclinkprocessor)
- [Functions](#functions)
  - [`convert_html_links`](#convert_html_links)
  - [`strip_duplicate_title`](#strip_duplicate_title)
  - [`strip_reference_definitions`](#strip_reference_definitions)
  - [`unhide_code_lines`](#unhide_code_lines)
  - [`detect_fence`](#detect_fence)
  - [`convert_path_reference_links`](#convert_path_reference_links)
  - [`replace_with_regex`](#replace_with_regex)
  - [`replace_with_regex_checked`](#replace_with_regex_checked)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DocLinkProcessor`](#doclinkprocessor) | struct | Processes doc comments to resolve intra-doc links to markdown links. |
| [`convert_html_links`](#convert_html_links) | fn | Convert HTML-style rustdoc links to markdown anchors. |
| [`strip_duplicate_title`](#strip_duplicate_title) | fn | Strip duplicate title from documentation. |
| [`strip_reference_definitions`](#strip_reference_definitions) | fn | Strip markdown reference definition lines. |
| [`unhide_code_lines`](#unhide_code_lines) | fn | Unhide rustdoc hidden lines in code blocks and add language identifiers. |
| [`detect_fence`](#detect_fence) | fn | Detect a code fence and return the fence string. |
| [`convert_path_reference_links`](#convert_path_reference_links) | fn | Convert path-style reference links to inline code. |
| [`replace_with_regex`](#replace_with_regex) | fn | Replace regex matches using a closure. |
| [`replace_with_regex_checked`](#replace_with_regex_checked) | fn | Replace regex matches with access to the text after the match. |

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

- `` `Name` `` - Backtick code links (most common)
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

- <span id="doclinkprocessor-new"></span>`fn new(krate: &'a Crate, link_registry: &'a LinkRegistry, current_file: &'a str) -> Self` — [`LinkRegistry`](../../index.md)

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

- <span id="doclinkprocessor-resolve-to-url"></span>`fn resolve_to_url(&self, link_text: &str, item_links: &HashMap<String, Id>) -> Option<String>`

- <span id="doclinkprocessor-get-url-for-id"></span>`fn get_url_for_id(&self, id: Id) -> Option<String>`

- <span id="doclinkprocessor-get-docs-rs-url"></span>`fn get_docs_rs_url(path_info: &rustdoc_types::ItemSummary) -> Option<String>`

- <span id="doclinkprocessor-resolve-method-link"></span>`fn resolve_method_link(&self, type_name: &str, method_name: &str, item_links: &HashMap<String, Id>) -> Option<String>`

- <span id="doclinkprocessor-resolve-link"></span>`fn resolve_link(&self, link_text: &str, item_links: &HashMap<String, Id>) -> String`

- <span id="doclinkprocessor-create-link-for-id"></span>`fn create_link_for_id(&self, id: Id, display_name: &str) -> Option<String>`

- <span id="doclinkprocessor-create-docs-rs-link"></span>`fn create_docs_rs_link(path_info: &rustdoc_types::ItemSummary, display_name: &str) -> Option<String>`

#### Trait Implementations

##### `impl<T> Instrument for DocLinkProcessor<'a>`

##### `impl<T> IntoEither for DocLinkProcessor<'a>`

##### `impl<D> OwoColorize for DocLinkProcessor<'a>`

##### `impl<T> Pointable for DocLinkProcessor<'a>`

- <span id="doclinkprocessor-align"></span>`const ALIGN: usize`

- <span id="doclinkprocessor-init"></span>`type Init = T`

- <span id="doclinkprocessor-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="doclinkprocessor-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="doclinkprocessor-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="doclinkprocessor-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for DocLinkProcessor<'a>`

## Functions

### `convert_html_links`

```rust
fn convert_html_links(docs: &str) -> String
```

Convert HTML-style rustdoc links to markdown anchors.

Transforms links like:
- `(#numberprefix)` -> `(#numberprefix)`
- `(#foo-bar)` -> `(#foo-bar)` (type-method anchor)

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

Removes lines like ``Name`: path::to::item` which are no longer needed
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

### `detect_fence`

```rust
fn detect_fence(trimmed: &str) -> Option<&'static str>
```

Detect a code fence and return the fence string.

### `convert_path_reference_links`

```rust
fn convert_path_reference_links(docs: &str) -> String
```

Convert path-style reference links to inline code.

Transforms: ```ProgressTracker```
Into: `` `ProgressTracker` ``

Without full link resolution context, we can't create valid anchors,
so we preserve the display text as inline code.

### `replace_with_regex`

```rust
fn replace_with_regex<F>(text: &str, re: &regex::Regex, replacer: F) -> String
where
    F: Fn(&regex::Captures<'_>) -> String
```

Replace regex matches using a closure.

### `replace_with_regex_checked`

```rust
fn replace_with_regex_checked<F>(text: &str, re: &regex::Regex, replacer: F) -> String
where
    F: Fn(&regex::Captures<'_>, &str) -> String
```

Replace regex matches with access to the text after the match.

