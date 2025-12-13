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
6. Process backtick links ``Name``
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

  Create a new tracker starting outside any code block.

- <span id="codeblocktracker-classify"></span>`fn classify(&mut self, line: &str) -> LineKind` — [`LineKind`](#linekind)

  Classify a line and update the tracker's state.

  

  This method both returns the line's classification AND updates

  the tracker's state. Call once per line in order.

  

  # State Transitions

  

  ```text

  ┌─────────┐  "```" or "~~~"  ┌──────────┐

  │ Outside │ ───────────────→ │  Inside  │

  │         │ ←─────────────── │          │

  └─────────┘  matching fence  └──────────┘

  ```

#### Trait Implementations

##### `impl Any for CodeBlockTracker`

- <span id="codeblocktracker-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CodeBlockTracker`

- <span id="codeblocktracker-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CodeBlockTracker`

- <span id="codeblocktracker-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for CodeBlockTracker`

- <span id="codeblocktracker-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for CodeBlockTracker`

##### `impl<U> Into for CodeBlockTracker`

- <span id="codeblocktracker-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for CodeBlockTracker`

##### `impl OwoColorize for CodeBlockTracker`

##### `impl Pointable for CodeBlockTracker`

- <span id="codeblocktracker-pointable-const-align"></span>`const ALIGN: usize`

- <span id="codeblocktracker-pointable-type-init"></span>`type Init = T`

- <span id="codeblocktracker-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="codeblocktracker-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="codeblocktracker-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="codeblocktracker-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for CodeBlockTracker`

- <span id="codeblocktracker-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="codeblocktracker-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CodeBlockTracker`

- <span id="codeblocktracker-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="codeblocktracker-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

- <span id="doclinkprocessor-with-index"></span>`fn with_index(krate: &'a Crate, link_registry: &'a LinkRegistry, current_file: &'a str, path_name_index: &HashMap<&'a str, Vec<Id>>) -> Self` — [`LinkRegistry`](../../linker/index.md#linkregistry)

  Create a new processor with a pre-built path name index.

  

  This is the preferred constructor when the index has already been built

  (e.g., in `GeneratorContext`), avoiding redundant index construction.

- <span id="doclinkprocessor-new"></span>`fn new(krate: &'a Crate, link_registry: &'a LinkRegistry, current_file: &'a str) -> Self` — [`LinkRegistry`](../../linker/index.md#linkregistry)

  Create a new processor for the given context.

  

  Builds the path name index internally. Prefer `Self::with_index` when

  the index has already been built to avoid redundant computation.

- <span id="doclinkprocessor-process"></span>`fn process(&self, docs: &str, item_links: &HashMap<String, Id>) -> String`

  Process a doc string and resolve all intra-doc links.

  

  Uses the item's `links` map to resolve link text to IDs,

  then uses `LinkRegistry` to convert IDs to relative paths.

- <span id="doclinkprocessor-process-links-protected"></span>`fn process_links_protected(&self, docs: &str, item_links: &HashMap<String, Id>) -> String`

  Process links while protecting code block contents.

  

  Uses [`CodeBlockTracker`](#codeblocktracker) to identify which lines are inside code blocks

  (and should be left unchanged) vs regular text (which needs link processing).

- <span id="doclinkprocessor-process-line"></span>`fn process_line(&self, line: &str, item_links: &HashMap<String, Id>) -> String`

  Process a single line for all link types.

- <span id="doclinkprocessor-process-reference-links"></span>`fn process_reference_links(&self, text: &str, item_links: &HashMap<String, Id>) -> String`

  Process reference-style links `[display text]`Span``.

- <span id="doclinkprocessor-process-path-reference-links"></span>`fn process_path_reference_links(&self, text: &str, item_links: &HashMap<String, Id>) -> String`

  Process path reference links ``text``.

- <span id="doclinkprocessor-process-method-links"></span>`fn process_method_links(&self, text: &str, item_links: &HashMap<String, Id>) -> String`

  Process method links `[``Type::method``]`.

- <span id="doclinkprocessor-process-backtick-links"></span>`fn process_backtick_links(&self, text: &str, item_links: &HashMap<String, Id>) -> String`

  Process backtick links ``Name``.

- <span id="doclinkprocessor-process-plain-links"></span>`fn process_plain_links(&self, text: &str, item_links: &HashMap<String, Id>) -> String`

  Process plain links `[name]`.

- <span id="doclinkprocessor-process-html-links-with-context"></span>`fn process_html_links_with_context(&self, text: &str, item_links: &HashMap<String, Id>) -> String`

  Process HTML-style rustdoc links with context awareness.

  

  Instead of blindly converting all HTML links to local anchors,

  this method checks if the item actually exists on the current page.

  If not, it tries to resolve to docs.rs or removes the broken link.

  

  For method links (e.g., `struct.Foo.html#method.bar`), creates a

  method anchor like `#foo-bar` for deep linking.

- <span id="doclinkprocessor-resolve-html-link-to-url"></span>`fn resolve_html_link_to_url(&self, item_name: &str, item_kind: &str, item_links: &HashMap<String, Id>) -> Option<String>`

  Try to resolve an HTML-style link to a proper URL.

  

  Returns a URL if the item can be resolved (either locally or to docs.rs),

  or None if the item cannot be found.

- <span id="doclinkprocessor-kind-matches"></span>`fn kind_matches(html_kind: &str, item_kind: ItemKind) -> bool`

  Check if the HTML link kind matches the rustdoc item kind.

- <span id="doclinkprocessor-clean-blank-lines"></span>`fn clean_blank_lines(docs: &str) -> String`

  Clean up multiple consecutive blank lines.

- <span id="doclinkprocessor-resolve-with-strategies"></span>`fn resolve_with_strategies<T, F>(&self, link_text: &str, item_links: &HashMap<String, Id>, resolver: F) -> Option<T>`

  Generic 3-strategy resolution with per-strategy display names.

  

  Unifies the resolution logic used by `resolve_to_url` and `resolve_link`.

  The resolver closure receives both the `Id` and the appropriate display name

  for that strategy:

  - Strategy 1 (exact match): uses original `link_text` (preserves qualified paths)

  - Strategy 2 & 3 (fuzzy matches): uses `short_name`

  

  # Type Parameters

  

  * `T` - The result type (e.g., `String` for URLs or markdown links)

  

  # Arguments

  

  * `link_text` - Original link text from documentation

  * `item_links` - Pre-resolved links from rustdoc

  * `resolver` - Closure that takes `(Id, display_name)` and returns `Option<T>`

- <span id="doclinkprocessor-resolve-to-url"></span>`fn resolve_to_url(&self, link_text: &str, item_links: &HashMap<String, Id>) -> Option<String>`

  Resolve a link reference to a URL.

  

  Uses the generic 3-strategy resolver. Display name is ignored since

  we only need the URL.

- <span id="doclinkprocessor-get-url-for-id"></span>`fn get_url_for_id(&self, id: Id) -> Option<String>`

  Get the URL for an ID (local or docs.rs).

- <span id="doclinkprocessor-get-docs-rs-url"></span>`fn get_docs_rs_url(path_info: &rustdoc_types::ItemSummary) -> Option<String>`

  Get docs.rs URL for an external crate item.

- <span id="doclinkprocessor-resolve-method-link"></span>`fn resolve_method_link(&self, type_name: &str, method_name: &str, item_links: &HashMap<String, Id>) -> Option<String>`

  Resolve a method link to a markdown link with method anchor.

  

  Links to the type's page with a method anchor for deep linking

  (e.g., `#hashmap-new` for `HashMap::new`).

- <span id="doclinkprocessor-resolve-link"></span>`fn resolve_link(&self, link_text: &str, item_links: &HashMap<String, Id>) -> String`

  Try to resolve link text to a markdown link.

  

  Uses the generic 3-strategy resolver. Falls back to unresolved link format

  (backtick-wrapped text in brackets) if resolution fails.

- <span id="doclinkprocessor-create-link-for-id"></span>`fn create_link_for_id(&self, id: Id, display_name: &str) -> Option<String>`

  Create a markdown link for an ID.

- <span id="doclinkprocessor-create-docs-rs-link"></span>`fn create_docs_rs_link(path_info: &rustdoc_types::ItemSummary, display_name: &str) -> Option<String>`

  Create a docs.rs link for an external crate item.

#### Trait Implementations

##### `impl Any for DocLinkProcessor<'a>`

- <span id="doclinkprocessor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DocLinkProcessor<'a>`

- <span id="doclinkprocessor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DocLinkProcessor<'a>`

- <span id="doclinkprocessor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for DocLinkProcessor<'a>`

- <span id="doclinkprocessor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for DocLinkProcessor<'a>`

##### `impl<U> Into for DocLinkProcessor<'a>`

- <span id="doclinkprocessor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for DocLinkProcessor<'a>`

##### `impl OwoColorize for DocLinkProcessor<'a>`

##### `impl Pointable for DocLinkProcessor<'a>`

- <span id="doclinkprocessor-pointable-const-align"></span>`const ALIGN: usize`

- <span id="doclinkprocessor-pointable-type-init"></span>`type Init = T`

- <span id="doclinkprocessor-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="doclinkprocessor-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="doclinkprocessor-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="doclinkprocessor-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for DocLinkProcessor<'a>`

- <span id="doclinkprocessor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="doclinkprocessor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DocLinkProcessor<'a>`

- <span id="doclinkprocessor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="doclinkprocessor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for DocLinkProcessor<'a>`

### `DocLinkUtils`

```rust
struct DocLinkUtils;
```

*Defined in `src/generator/doc_links.rs:1051`*

Utility functions for document links

#### Implementations

- <span id="doclinkutils-convert-html-links"></span>`fn convert_html_links(docs: &str) -> String`

  Convert HTML-style rustdoc links to markdown anchors.

  

  Transforms links like:

  - `(#numberprefix)` -> `(#numberprefix)`

  - `(#foo-bar)` -> `(#foo-bar)` (type-method anchor)

  

  This is useful for multi-crate documentation where the full processor

  context may not be available.

- <span id="doclinkutils-strip-duplicate-title"></span>`fn strip_duplicate_title<'a>(docs: &'a str, item_name: &str) -> &'a str`

  Strip duplicate title from documentation.

  

  Some crate/module docs start with `# title` which duplicates the generated

  `# Crate 'name'` or `# Module 'name'` heading.

  

  # Arguments

  

  * `docs` - The documentation string to process

  * `item_name` - The name of the crate or module being documented

  

  # Returns

  

  The docs with the leading title removed if it matches the item name,

  otherwise the original docs unchanged.

- <span id="doclinkutils-strip-reference-definitions"></span>`fn strip_reference_definitions(docs: &str) -> String`

  Strip markdown reference definition lines.

  

  Removes lines like ``Name`: path::to::item` which are no longer needed

  after intra-doc links are processed.

- <span id="doclinkutils-unhide-code-lines"></span>`fn unhide_code_lines(docs: &str) -> String`

  Unhide rustdoc hidden lines in code blocks and add language identifiers.

  

  This function performs two transformations on code blocks:

  1. Lines starting with `# ` inside code blocks are hidden in rustdoc

     but compiled. We remove the prefix to show the full example.

  2. Bare code fences (` ``` `) are converted to ` ```rust ` since doc

     examples are Rust code.

  

  Uses [`CodeBlockTracker`](#codeblocktracker) to manage fence state.

- <span id="doclinkutils-convert-path-reference-links"></span>`fn convert_path_reference_links(docs: &str) -> String`

  Convert path-style reference links to inline code.

  

  Transforms: ```ProgressTracker```

  Into: `` `ProgressTracker` ``

  

  Without full link resolution context, we can't create valid anchors,

  so we preserve the display text as inline code.

- <span id="doclinkutils-replace-with-regex"></span>`fn replace_with_regex<F>(text: &str, re: &Regex, replacer: F) -> String`

  Replace regex matches using a closure.

- <span id="doclinkutils-replace-with-regex-checked"></span>`fn replace_with_regex_checked<F>(text: &str, re: &Regex, replacer: F) -> String`

  Replace regex matches with access to the text after the match.

#### Trait Implementations

##### `impl Any for DocLinkUtils`

- <span id="doclinkutils-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DocLinkUtils`

- <span id="doclinkutils-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DocLinkUtils`

- <span id="doclinkutils-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for DocLinkUtils`

- <span id="doclinkutils-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for DocLinkUtils`

##### `impl<U> Into for DocLinkUtils`

- <span id="doclinkutils-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for DocLinkUtils`

##### `impl OwoColorize for DocLinkUtils`

##### `impl Pointable for DocLinkUtils`

- <span id="doclinkutils-pointable-const-align"></span>`const ALIGN: usize`

- <span id="doclinkutils-pointable-type-init"></span>`type Init = T`

- <span id="doclinkutils-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="doclinkutils-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="doclinkutils-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="doclinkutils-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for DocLinkUtils`

- <span id="doclinkutils-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="doclinkutils-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DocLinkUtils`

- <span id="doclinkutils-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="doclinkutils-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for LineKind`

- <span id="linekind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LineKind`

- <span id="linekind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LineKind`

- <span id="linekind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LineKind`

- <span id="linekind-clone"></span>`fn clone(&self) -> LineKind` — [`LineKind`](#linekind)

##### `impl CloneToUninit for LineKind`

- <span id="linekind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for LineKind`

##### `impl Debug for LineKind`

- <span id="linekind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LineKind`

##### `impl<K> Equivalent for LineKind`

- <span id="linekind-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl<T> From for LineKind`

- <span id="linekind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for LineKind`

##### `impl<U> Into for LineKind`

- <span id="linekind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for LineKind`

##### `impl OwoColorize for LineKind`

##### `impl PartialEq for LineKind`

- <span id="linekind-partialeq-eq"></span>`fn eq(&self, other: &LineKind) -> bool` — [`LineKind`](#linekind)

##### `impl Pointable for LineKind`

- <span id="linekind-pointable-const-align"></span>`const ALIGN: usize`

- <span id="linekind-pointable-type-init"></span>`type Init = T`

- <span id="linekind-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="linekind-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="linekind-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="linekind-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl StructuralPartialEq for LineKind`

##### `impl ToOwned for LineKind`

- <span id="linekind-toowned-type-owned"></span>`type Owned = T`

- <span id="linekind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="linekind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LineKind`

- <span id="linekind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="linekind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LineKind`

- <span id="linekind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="linekind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for LineKind`

