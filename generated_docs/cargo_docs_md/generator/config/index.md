*[cargo_docs_md](../../index.md) / [generator](../index.md) / [config](index.md)*

---

# Module `config`

Configuration for markdown rendering.

This module provides [`RenderConfig`](#renderconfig) for controlling how documentation
is rendered, and [`SourceConfig`](#sourceconfig) for source code integration options.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RenderConfig`](#renderconfig) | struct | Configuration options for markdown rendering. |
| [`SourceConfig`](#sourceconfig) | struct | Configuration for source code integration. |

## Structs

### `RenderConfig`

```rust
struct RenderConfig {
    pub toc_threshold: usize,
    pub quick_reference: bool,
    pub group_impls: bool,
    pub hide_trivial_derives: bool,
    pub method_anchors: bool,
    pub full_method_docs: bool,
    pub include_source: SourceConfig,
}
```

*Defined in `src/generator/config.rs:14-39`*

Configuration options for markdown rendering.

#### Fields

- **`toc_threshold`**: `usize`

  Generate table of contents for modules with more than this many items.

- **`quick_reference`**: `bool`

  Generate quick reference tables at the top of modules.

- **`group_impls`**: `bool`

  Group impl blocks by category (Derive, Conversion, Iterator, etc.).

- **`hide_trivial_derives`**: `bool`

  Hide trivial derive implementations (Clone, Copy, Debug, etc.).

- **`method_anchors`**: `bool`

  Generate method-level anchors for deep linking.

- **`full_method_docs`**: `bool`

  Include full method documentation instead of first-paragraph summaries.
  
  When `false` (default), method docs in impl blocks show only the first
  paragraph (up to the first blank line). When `true`, the complete
  documentation is included.

- **`include_source`**: `SourceConfig`

  Source code integration options.

#### Trait Implementations

##### `impl Any for RenderConfig`

- <span id="renderconfig-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RenderConfig`

- <span id="renderconfig-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RenderConfig`

- <span id="renderconfig-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RenderConfig`

- <span id="renderconfig-clone"></span>`fn clone(&self) -> RenderConfig` — [`RenderConfig`](#renderconfig)

##### `impl CloneToUninit for RenderConfig`

- <span id="renderconfig-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for RenderConfig`

- <span id="renderconfig-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RenderConfig`

- <span id="renderconfig-default"></span>`fn default() -> Self`

##### `impl<T> From for RenderConfig`

- <span id="renderconfig-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for RenderConfig`

##### `impl<U> Into for RenderConfig`

- <span id="renderconfig-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for RenderConfig`

##### `impl OwoColorize for RenderConfig`

##### `impl Pointable for RenderConfig`

- <span id="renderconfig-pointable-const-align"></span>`const ALIGN: usize`

- <span id="renderconfig-pointable-type-init"></span>`type Init = T`

- <span id="renderconfig-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="renderconfig-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="renderconfig-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="renderconfig-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for RenderConfig`

- <span id="renderconfig-toowned-type-owned"></span>`type Owned = T`

- <span id="renderconfig-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="renderconfig-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RenderConfig`

- <span id="renderconfig-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="renderconfig-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RenderConfig`

- <span id="renderconfig-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="renderconfig-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for RenderConfig`

### `SourceConfig`

```rust
struct SourceConfig {
    pub function_bodies: bool,
    pub const_values: bool,
    pub private_items: bool,
    pub source_locations: bool,
    pub source_dir: Option<std::path::PathBuf>,
}
```

*Defined in `src/generator/config.rs:49-68`*

Configuration for source code integration.

Requires the `source-parsing` feature to have any effect.

#### Fields

- **`function_bodies`**: `bool`

  Include function bodies in collapsible sections.

- **`const_values`**: `bool`

  Show actual values for constants and statics.

- **`private_items`**: `bool`

  Include private items in a separate section.

- **`source_locations`**: `bool`

  Add <file:line> references to items.

- **`source_dir`**: `Option<std::path::PathBuf>`

  Path to the `.source_*` directory containing collected dependency sources.
  
  When set, source location references will use paths relative to this directory
  and generate clickable links. When `None`, absolute paths from rustdoc JSON
  are displayed without links.

#### Trait Implementations

##### `impl Any for SourceConfig`

- <span id="sourceconfig-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SourceConfig`

- <span id="sourceconfig-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SourceConfig`

- <span id="sourceconfig-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SourceConfig`

- <span id="sourceconfig-clone"></span>`fn clone(&self) -> SourceConfig` — [`SourceConfig`](#sourceconfig)

##### `impl CloneToUninit for SourceConfig`

- <span id="sourceconfig-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SourceConfig`

- <span id="sourceconfig-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SourceConfig`

- <span id="sourceconfig-default"></span>`fn default() -> SourceConfig` — [`SourceConfig`](#sourceconfig)

##### `impl<T> From for SourceConfig`

- <span id="sourceconfig-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for SourceConfig`

##### `impl<U> Into for SourceConfig`

- <span id="sourceconfig-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SourceConfig`

##### `impl OwoColorize for SourceConfig`

##### `impl Pointable for SourceConfig`

- <span id="sourceconfig-pointable-const-align"></span>`const ALIGN: usize`

- <span id="sourceconfig-pointable-type-init"></span>`type Init = T`

- <span id="sourceconfig-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="sourceconfig-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="sourceconfig-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="sourceconfig-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for SourceConfig`

- <span id="sourceconfig-toowned-type-owned"></span>`type Owned = T`

- <span id="sourceconfig-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="sourceconfig-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SourceConfig`

- <span id="sourceconfig-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sourceconfig-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SourceConfig`

- <span id="sourceconfig-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sourceconfig-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for SourceConfig`

