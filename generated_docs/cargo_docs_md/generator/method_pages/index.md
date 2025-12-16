*[cargo_docs_md](../../index.md) / [generator](../index.md) / [method_pages](index.md)*

---

# Module `method_pages`

Separate method page generation for large traits.

This module provides [`MethodPageGenerator`](#methodpagegenerator) for generating individual markdown
pages for each method when a trait exceeds the configured method threshold.
This improves navigation and readability for traits with many methods.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MethodPage`](#methodpage) | struct | A generated method page ready to be written to disk. |
| [`MethodPageGenerator`](#methodpagegenerator) | struct | Generator for individual method pages. |

## Structs

### `MethodPage`

```rust
struct MethodPage {
    pub path: String,
    pub content: String,
}
```

*Defined in `src/generator/method_pages.rs:16-22`*

A generated method page ready to be written to disk.

#### Fields

- **`path`**: `String`

  Relative path for the method page (e.g., "itertools/methods/coalesce.md")

- **`content`**: `String`

  The markdown content for the method page.

#### Trait Implementations

##### `impl Any for MethodPage`

- <span id="methodpage-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MethodPage`

- <span id="methodpage-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MethodPage`

- <span id="methodpage-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MethodPage`

- <span id="methodpage-clone"></span>`fn clone(&self) -> MethodPage` — [`MethodPage`](#methodpage)

##### `impl CloneToUninit for MethodPage`

- <span id="methodpage-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for MethodPage`

- <span id="methodpage-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MethodPage`

- <span id="methodpage-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for MethodPage`

##### `impl<U> Into for MethodPage`

- <span id="methodpage-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MethodPage`

##### `impl OwoColorize for MethodPage`

##### `impl Pointable for MethodPage`

- <span id="methodpage-pointable-const-align"></span>`const ALIGN: usize`

- <span id="methodpage-pointable-type-init"></span>`type Init = T`

- <span id="methodpage-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="methodpage-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="methodpage-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="methodpage-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for MethodPage`

- <span id="methodpage-toowned-type-owned"></span>`type Owned = T`

- <span id="methodpage-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="methodpage-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MethodPage`

- <span id="methodpage-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="methodpage-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MethodPage`

- <span id="methodpage-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="methodpage-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for MethodPage`

### `MethodPageGenerator<'a>`

```rust
struct MethodPageGenerator<'a> {
    type_renderer: &'a crate::types::TypeRenderer<'a>,
    trait_name: &'a str,
    base_dir: String,
    module_path: &'a [String],
}
```

*Defined in `src/generator/method_pages.rs:29-41`*

Generator for individual method pages.

When a trait has more methods than the configured threshold, this generator
creates separate markdown pages for each method, improving navigation in
large documentation sets.

#### Fields

- **`type_renderer`**: `&'a crate::types::TypeRenderer<'a>`

  Type renderer for signatures.

- **`trait_name`**: `&'a str`

  Trait name for breadcrumbs and titles.

- **`base_dir`**: `String`

  Base directory for method pages (e.g., "itertools/methods/").

- **`module_path`**: `&'a [String]`

  Module path for breadcrumbs.

#### Implementations

- <span id="methodpagegenerator-new"></span>`const fn new(type_renderer: &'a TypeRenderer<'a>, trait_name: &'a str, base_dir: String, module_path: &'a [String]) -> Self` — [`TypeRenderer`](../../types/index.md#typerenderer)

  Create a new method page generator.
  
  ##### Arguments
  
  * `type_renderer` - Type renderer for method signatures
  * `trait_name` - Name of the trait containing the methods
  * `base_dir` - Base directory for method pages
  * `module_path` - Module path for breadcrumb generation

- <span id="methodpagegenerator-generate-method-page"></span>`fn generate_method_page(&self, method: &Item, docs: Option<String>) -> Option<MethodPage>` — [`MethodPage`](#methodpage)

  Generate a method page for a single method item.
  
  ##### Arguments
  
  * `method` - The method item from rustdoc
  * `docs` - Processed documentation string (with resolved links)
  
  ##### Returns
  
  A `MethodPage` containing the path and content, or `None` if not a function.

- <span id="methodpagegenerator-generate-method-link"></span>`fn generate_method_link(&self, method: &Item, first_line_doc: Option<&str>) -> Option<String>`

  Generate a summary link for a method (used in the main trait page).
  
  ##### Arguments
  
  * `method` - The method item
  * `first_line_doc` - First line of documentation for summary
  
  ##### Returns
  
  Markdown link to the method page with summary.

- <span id="methodpagegenerator-write-breadcrumbs"></span>`fn write_breadcrumbs(&self, content: &mut String, method_name: &str)`

  Write breadcrumb navigation to the content.

#### Trait Implementations

##### `impl Any for MethodPageGenerator<'a>`

- <span id="methodpagegenerator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MethodPageGenerator<'a>`

- <span id="methodpagegenerator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MethodPageGenerator<'a>`

- <span id="methodpagegenerator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for MethodPageGenerator<'a>`

- <span id="methodpagegenerator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for MethodPageGenerator<'a>`

##### `impl<U> Into for MethodPageGenerator<'a>`

- <span id="methodpagegenerator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MethodPageGenerator<'a>`

##### `impl OwoColorize for MethodPageGenerator<'a>`

##### `impl Pointable for MethodPageGenerator<'a>`

- <span id="methodpagegenerator-pointable-const-align"></span>`const ALIGN: usize`

- <span id="methodpagegenerator-pointable-type-init"></span>`type Init = T`

- <span id="methodpagegenerator-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="methodpagegenerator-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="methodpagegenerator-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="methodpagegenerator-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for MethodPageGenerator<'a>`

- <span id="methodpagegenerator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="methodpagegenerator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MethodPageGenerator<'a>`

- <span id="methodpagegenerator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="methodpagegenerator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for MethodPageGenerator<'a>`

