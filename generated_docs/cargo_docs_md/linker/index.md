*[cargo_docs_md](../index.md) / [linker](index.md)*

---

# Module `linker`

Cross-reference linking for markdown documentation.

This module provides the `LinkRegistry` which maps rustdoc item IDs to their
corresponding markdown file paths. This enables creating clickable links
between items in the generated documentation.

# How It Works

1. During initialization, `LinkRegistry::build()` traverses the entire crate
   and records where each item's documentation will be written.

2. During markdown generation, `create_link()` is called to generate
   relative links from one file to another.

# Path Formats

The registry supports two output formats:

- **Flat**: `module.md`, `parent__child.md` (double-underscore separators)
- **Nested**: `module/index.md`, `parent/child/index.md` (directory structure)

# Example

```ignore
let registry = LinkRegistry::build(&krate, true); // flat format
let link = registry.create_link(&some_id, "index.md");
// Returns: Some("[`ItemName`](module.md)")
```

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AnchorUtils`](#anchorutils) | struct | Utilify functions to handle anchors |
| [`LinkRegistry`](#linkregistry) | struct | Registry mapping item IDs to their documentation file paths. |
| [`AssocItemKind`](#associtemkind) | enum | Kind of associated item for anchor generation. |
| [`ImplContext`](#implcontext) | enum | Context for generating impl item anchors, distinguishing inherent vs trait impls. |

## Structs

### `AnchorUtils`

```rust
struct AnchorUtils;
```

*Defined in `src/linker.rs:68`*

Utilify functions to handle anchors

#### Implementations

- <span id="anchorutils-assoc-item-anchor"></span>`fn assoc_item_anchor(type_name: &str, item_name: &str, kind: AssocItemKind) -> String` — [`AssocItemKind`](#associtemkind)

  Generate a compound anchor for an associated item on a type.

  

  This creates a unique anchor that combines the type name, item kind, and item name,

  enabling deep linking to specific items. The format is `typename-itemname` for methods

  (backward compatible), and `typename-kind-itemname` for constants and types to avoid

  collisions.

  

  # Arguments

  

  * `type_name` - The name of the type (struct, enum, trait, etc.)

  * `item_name` - The name of the method or associated item

  * `kind` - The kind of associated item (method, const, or type)

  

  # Examples

  

  ```ignore

  assert_eq!(assoc_item_anchor("Parser", "parse", AssocItemKind::Method), "parser-parse");

  assert_eq!(assoc_item_anchor("HashMap", "new", AssocItemKind::Method), "hashmap-new");

  assert_eq!(assoc_item_anchor("Vec", "Item", AssocItemKind::Type), "vec-type-item");

  assert_eq!(assoc_item_anchor("Vec", "ALIGN", AssocItemKind::Const), "vec-const-align");

  ```

- <span id="anchorutils-method-anchor"></span>`fn method_anchor(type_name: &str, method_name: &str) -> String`

  Generate a compound anchor for a method on a type.

  

  This creates a unique anchor that combines the type name and method name,

  enabling deep linking to specific methods. The format is `typename-methodname`,

  where both parts are slugified.

  

  # Arguments

  

  * `type_name` - The name of the type (struct, enum, trait, etc.)

  * `method_name` - The name of the method or associated item

  

  # Examples

  

  ```ignore

  assert_eq!(method_anchor("Parser", "parse"), "parser-parse");

  assert_eq!(method_anchor("HashMap", "new"), "hashmap-new");

  assert_eq!(method_anchor("Vec<T>", "push"), "vec-push");

  ```

- <span id="anchorutils-impl-item-anchor"></span>`fn impl_item_anchor(type_name: &str, item_name: &str, kind: AssocItemKind, impl_ctx: ImplContext<'_>) -> String` — [`AssocItemKind`](#associtemkind), [`ImplContext`](#implcontext)

  Generate an anchor for an associated item in an impl block, with trait disambiguation.

  

  This extends `assoc_item_anchor` to handle trait impls, where multiple traits

  may define the same associated type (e.g., `Output` in both `Add` and `Sub`).

  

  # Disambiguation Strategy

  

  - **Associated types/consts**: Always include trait name (high collision risk)

  - **Methods**: Only include trait name when it differs from the method name

    - Avoids redundant `Clone::clone` → `type-clone-clone`

    - Keeps `Debug::fmt` → `type-debug-fmt` for disambiguation from `Display::fmt`

  

  # Arguments

  

  * `type_name` - The name of the implementing type

  * `item_name` - The name of the associated item

  * `kind` - The kind of associated item

  * `impl_ctx` - Whether this is an inherent or trait impl

  

  # Anchor Formats

  

  | Context | Kind | Format | Example |

  |---------|------|--------|---------|

  | Inherent | Method | `type-method` | `vec-push` |

  | Inherent | Type | `type-type-item` | `vec-type-item` |

  | Inherent | Const | `type-const-item` | `vec-const-align` |

  | Trait(Clone) | Method | `type-method` | `vec-clone` (trait=method) |

  | Trait(Debug) | Method | `type-trait-method` | `vec-debug-fmt` (trait≠method) |

  | Trait(Add) | Type | `type-trait-type-item` | `vec-add-type-output` |

  | Trait(Add) | Const | `type-trait-const-item` | `vec-add-const-max` |

- <span id="anchorutils-slugify-anchor"></span>`fn slugify_anchor(name: &str) -> String`

  Convert a name to a GitHub-style markdown anchor slug.

  

  This normalizes item names to match the anchor IDs generated by markdown

  renderers (GitHub, mdBook, etc.) when they process headings.

  

  # Rules Applied

  

  1. Apply Unicode NFC normalization (canonical composition)

  2. Convert to lowercase (full Unicode, not just ASCII)

  3. Remove backticks (markdown code formatting)

  4. Remove generics (`<T>`, `<K, V>`) by stripping `<...>` content

  5. Replace spaces and underscores with hyphens

  6. Remove non-alphanumeric characters (except hyphens)

  7. Collapse consecutive hyphens

  8. Trim leading/trailing hyphens

  

  # Examples

  

  ```ignore

  assert_eq!(slugify_anchor("HashMap"), "hashmap");

  assert_eq!(slugify_anchor("HashMap<K, V>"), "hashmap");

  assert_eq!(slugify_anchor("my_function"), "my-function");

  assert_eq!(slugify_anchor("Into<T>"), "into");

  assert_eq!(slugify_anchor("Größe"), "größe");

  ```

- <span id="anchorutils-slugify-anchor-ascii"></span>`fn slugify_anchor_ascii(name: &str) -> String`

  Fast ASCII-only slugification (no allocation for normalization).

- <span id="anchorutils-slugify-anchor-impl"></span>`fn slugify_anchor_impl(name: &str) -> String`

  Unicode-aware slugification with full lowercase support.

- <span id="anchorutils-item-has-anchor"></span>`const fn item_has_anchor(kind: ItemKind) -> bool`

  Check if an item kind generates a heading anchor in markdown.

  

  Only certain item types get `### \`Name\` headings in the generated output.

  Other items (methods, fields, variants) are rendered as bullet points

  without heading anchors.

  

  # Items with anchors

  

  - Struct, Enum, Trait, Function, Constant, `TypeAlias`, Macro, Module

  

  # Items without anchors

  

  - Methods (in impl blocks)

  - Struct fields

  - Enum variants

  - Associated types/constants

  - Trait methods

#### Trait Implementations

##### `impl Any for AnchorUtils`

- <span id="anchorutils-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AnchorUtils`

- <span id="anchorutils-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AnchorUtils`

- <span id="anchorutils-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for AnchorUtils`

- <span id="anchorutils-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for AnchorUtils`

##### `impl<U> Into for AnchorUtils`

- <span id="anchorutils-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for AnchorUtils`

##### `impl OwoColorize for AnchorUtils`

##### `impl Pointable for AnchorUtils`

- <span id="anchorutils-pointable-const-align"></span>`const ALIGN: usize`

- <span id="anchorutils-pointable-type-init"></span>`type Init = T`

- <span id="anchorutils-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="anchorutils-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="anchorutils-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="anchorutils-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for AnchorUtils`

- <span id="anchorutils-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="anchorutils-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AnchorUtils`

- <span id="anchorutils-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="anchorutils-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for AnchorUtils`

### `LinkRegistry`

```rust
struct LinkRegistry {
    item_paths: std::collections::HashMap<rustdoc_types::Id, String>,
    item_names: std::collections::HashMap<rustdoc_types::Id, String>,
}
```

*Defined in `src/linker.rs:366-378`*

Registry mapping item IDs to their documentation file paths.

This is the central data structure for cross-reference resolution.
It's built once during generation and queried whenever we need to
create links between items.

#### Fields

- **`item_paths`**: `std::collections::HashMap<rustdoc_types::Id, String>`

  Maps each item's ID to the markdown file path where it's documented.
  
  Paths are relative to the output directory root.
  Examples: `"index.md"`, `"span.md"`, `"span/index.md"`

- **`item_names`**: `std::collections::HashMap<rustdoc_types::Id, String>`

  Maps each item's ID to its display name.
  
  Used to generate the link text (e.g., `[`name`](path)`).
  This is typically the item's identifier without the full path.

#### Implementations

- <span id="linkregistry-build"></span>`fn build(krate: &Crate, flat_format: bool, include_private: bool) -> Self`

  Build a link registry by traversing all items in the crate.

  

  This function walks the module tree starting from the root and records

  the file path where each item will be documented. The paths depend on

  the output format (flat vs nested).

  

  # Arguments

  

  * `krate` - The parsed rustdoc crate containing all items

  * `flat_format` - If true, use flat paths (`mod.md`); if false, use nested (`mod/index.md`)

  * `include_private` - If true, include non-public items; if false, only public items

  

  # Returns

  

  A populated `LinkRegistry` ready for link creation.

  

  # Algorithm

  

  1. Start at the crate root module

  2. For each top-level module: register it and recursively process children

  3. For structs/enums/traits at root level: register them to `index.md`

  4. Other items (functions, constants) are registered within their module's file

  5. Items are filtered by visibility unless `include_private` is true

- <span id="linkregistry-register-module-items"></span>`fn register_module_items(&mut self, krate: &Crate, module_id: Id, module_item: &rustdoc_types::Item, path: &str, module_prefix: &str, flat_format: bool, include_private: bool)`

  Recursively register all items within a module.

  

  This is called for each module in the crate to populate the registry

  with all items that can be linked to.

  

  # Arguments

  

  * `krate` - The full crate for looking up item details

  * `module_id` - ID of the module being registered

  * `module_item` - The module's Item data

  * `path` - File path where this module's docs will be written

  * `module_prefix` - Prefix for building child paths (e.g., "parent" or "`parent__child`")

  * `flat_format` - Whether to use flat naming convention

  * `include_private` - Whether to include non-public items

- <span id="linkregistry-register-glob-items"></span>`fn register_glob_items(&mut self, krate: &Crate, use_item: &rustdoc_types::Use, path: &str, include_private: bool)`

  Register items from a glob re-export target module.

- <span id="linkregistry-get-path"></span>`fn get_path(&self, id: Id) -> Option<&String>`

  Get the file path where an item is documented.

  

  # Arguments

  

  * `id` - The item's unique ID from rustdoc JSON

  

  # Returns

  

  The relative file path (e.g., `"span.md"` or `"span/index.md"`),

  or `None` if the item isn't registered.

- <span id="linkregistry-get-name"></span>`fn get_name(&self, id: Id) -> Option<&String>`

  Get the display name for an item.

  

  # Arguments

  

  * `id` - The item's unique ID from rustdoc JSON

  

  # Returns

  

  The item's name for display in links (e.g., `"Span"`),

  or `None` if the item isn't registered.

- <span id="linkregistry-create-link"></span>`fn create_link(&self, id: Id, from_path: &str) -> Option<String>`

  Create a markdown link to an item from a given source file.

  

  This is the main method used during markdown generation to create

  clickable links between documented items.

  

  # Arguments

  

  * `id` - The target item's ID

  * `from_path` - The source file creating the link (e.g., `"index.md"`)

  

  # Returns

  

  A formatted markdown link like `[``ItemName``](path/to/file.md)`,

  or `None` if the target item isn't registered.

  

  # Link Types

  

  - **Same file**: Returns an anchor link (`#itemname`)

  - **Different file**: Returns a relative path (`../other/file.md`)

  

  # Example

  

  ```ignore

  // From index.md linking to span.md

  registry.create_link(&span_id, "index.md")

  // Returns: Some("[`Span`](span.md)")

  

  // From span/index.md linking to index.md

  registry.create_link(&root_id, "span/index.md")

  // Returns: Some("[`crate`](../index.md)")

  ```

- <span id="linkregistry-compute-relative-path"></span>`fn compute_relative_path(from: &str, to: &str) -> String`

  Compute the relative path from one file to another.

  

  This function calculates the relative path needed to navigate from

  one markdown file to another within the generated documentation.

  Uses `pathdiff` for robust cross-platform path calculation.

  

  # Arguments

  

  * `from` - The source file path (e.g., `"span/index.md"`)

  * `to` - The target file path (e.g., `"field/index.md"`)

  

  # Returns

  

  A relative path string (e.g., `"../field/index.md"`)

  

  # Examples

  

  - Same directory: `"index.md"` → `"span.md"` = `"span.md"`

  - Into subdirectory: `"index.md"` → `"span/index.md"` = `"span/index.md"`

  - Up to parent: `"span/index.md"` → `"index.md"` = `"../index.md"`

  - Sibling directory: `"span/index.md"` → `"field/index.md"` = `"../field/index.md"`

#### Trait Implementations

##### `impl Any for LinkRegistry`

- <span id="linkregistry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LinkRegistry`

- <span id="linkregistry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LinkRegistry`

- <span id="linkregistry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for LinkRegistry`

- <span id="linkregistry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for LinkRegistry`

- <span id="linkregistry-default"></span>`fn default() -> LinkRegistry` — [`LinkRegistry`](#linkregistry)

##### `impl<T> From for LinkRegistry`

- <span id="linkregistry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for LinkRegistry`

##### `impl<U> Into for LinkRegistry`

- <span id="linkregistry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for LinkRegistry`

##### `impl OwoColorize for LinkRegistry`

##### `impl Pointable for LinkRegistry`

- <span id="linkregistry-pointable-const-align"></span>`const ALIGN: usize`

- <span id="linkregistry-pointable-type-init"></span>`type Init = T`

- <span id="linkregistry-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="linkregistry-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="linkregistry-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="linkregistry-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for LinkRegistry`

- <span id="linkregistry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="linkregistry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LinkRegistry`

- <span id="linkregistry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="linkregistry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for LinkRegistry`

## Enums

### `AssocItemKind`

```rust
enum AssocItemKind {
    Method,
    Const,
    Type,
}
```

*Defined in `src/linker.rs:41-50`*

Kind of associated item for anchor generation.

Used to disambiguate anchors when multiple items share the same name
(e.g., `type Init` and `fn init` in the same impl block).

#### Variants

- **`Method`**

  A method or function (`fn`)

- **`Const`**

  An associated constant (`const`)

- **`Type`**

  An associated type (`type`)

#### Trait Implementations

##### `impl Any for AssocItemKind`

- <span id="associtemkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AssocItemKind`

- <span id="associtemkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AssocItemKind`

- <span id="associtemkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AssocItemKind`

- <span id="associtemkind-clone"></span>`fn clone(&self) -> AssocItemKind` — [`AssocItemKind`](#associtemkind)

##### `impl CloneToUninit for AssocItemKind`

- <span id="associtemkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for AssocItemKind`

##### `impl Debug for AssocItemKind`

- <span id="associtemkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AssocItemKind`

##### `impl<K> Equivalent for AssocItemKind`

- <span id="associtemkind-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl<T> From for AssocItemKind`

- <span id="associtemkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for AssocItemKind`

##### `impl<U> Into for AssocItemKind`

- <span id="associtemkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for AssocItemKind`

##### `impl OwoColorize for AssocItemKind`

##### `impl PartialEq for AssocItemKind`

- <span id="associtemkind-partialeq-eq"></span>`fn eq(&self, other: &AssocItemKind) -> bool` — [`AssocItemKind`](#associtemkind)

##### `impl Pointable for AssocItemKind`

- <span id="associtemkind-pointable-const-align"></span>`const ALIGN: usize`

- <span id="associtemkind-pointable-type-init"></span>`type Init = T`

- <span id="associtemkind-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="associtemkind-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="associtemkind-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="associtemkind-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl StructuralPartialEq for AssocItemKind`

##### `impl ToOwned for AssocItemKind`

- <span id="associtemkind-toowned-type-owned"></span>`type Owned = T`

- <span id="associtemkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="associtemkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AssocItemKind`

- <span id="associtemkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="associtemkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AssocItemKind`

- <span id="associtemkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="associtemkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for AssocItemKind`

### `ImplContext<'a>`

```rust
enum ImplContext<'a> {
    Inherent,
    Trait(&'a str),
}
```

*Defined in `src/linker.rs:59-65`*

Context for generating impl item anchors, distinguishing inherent vs trait impls.

For trait impls, we need to include the trait name in the anchor to avoid
duplicate anchors when multiple traits define the same associated type/const.
For example, both `impl Add for Foo` and `impl Sub for Foo` might have
`type Output`, which would create duplicate anchors without the trait name.

#### Variants

- **`Inherent`**

  Inherent impl (no trait) - anchors use format `typename-itemname`

- **`Trait`**

  Trait impl - anchors include trait name: `typename-traitname-itemname`

#### Trait Implementations

##### `impl Any for ImplContext<'a>`

- <span id="implcontext-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImplContext<'a>`

- <span id="implcontext-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImplContext<'a>`

- <span id="implcontext-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ImplContext<'a>`

- <span id="implcontext-clone"></span>`fn clone(&self) -> ImplContext<'a>` — [`ImplContext`](#implcontext)

##### `impl CloneToUninit for ImplContext<'a>`

- <span id="implcontext-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ImplContext<'a>`

##### `impl Debug for ImplContext<'a>`

- <span id="implcontext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ImplContext<'a>`

- <span id="implcontext-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for ImplContext<'a>`

##### `impl<U> Into for ImplContext<'a>`

- <span id="implcontext-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ImplContext<'a>`

##### `impl OwoColorize for ImplContext<'a>`

##### `impl Pointable for ImplContext<'a>`

- <span id="implcontext-pointable-const-align"></span>`const ALIGN: usize`

- <span id="implcontext-pointable-type-init"></span>`type Init = T`

- <span id="implcontext-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="implcontext-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="implcontext-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="implcontext-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for ImplContext<'a>`

- <span id="implcontext-toowned-type-owned"></span>`type Owned = T`

- <span id="implcontext-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="implcontext-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ImplContext<'a>`

- <span id="implcontext-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="implcontext-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImplContext<'a>`

- <span id="implcontext-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="implcontext-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for ImplContext<'a>`

