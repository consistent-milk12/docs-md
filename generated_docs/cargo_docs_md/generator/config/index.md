*[cargo_docs_md](../../index.md) / [generator](../index.md) / [config](index.md)*

---

# Module `config`

Configuration for markdown rendering.

This module provides [`RenderConfig`](../../index.md) for controlling how documentation
is rendered, and [`SourceConfig`](../../index.md) for source code integration options.

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
    pub include_source: SourceConfig,
}
```

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

- **`include_source`**: `SourceConfig`

  Source code integration options.

#### Trait Implementations

##### `impl Clone for RenderConfig`

- <span id="renderconfig-clone"></span>`fn clone(&self) -> RenderConfig` — [`RenderConfig`](../../index.md)

##### `impl Debug for RenderConfig`

- <span id="renderconfig-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RenderConfig`

- <span id="renderconfig-default"></span>`fn default() -> Self`

##### `impl<T> Instrument for RenderConfig`

##### `impl<T> IntoEither for RenderConfig`

##### `impl<D> OwoColorize for RenderConfig`

##### `impl<T> Pointable for RenderConfig`

- <span id="renderconfig-align"></span>`const ALIGN: usize`

- <span id="renderconfig-init"></span>`type Init = T`

- <span id="renderconfig-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="renderconfig-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="renderconfig-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="renderconfig-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for RenderConfig`

### `SourceConfig`

```rust
struct SourceConfig {
    pub function_bodies: bool,
    pub const_values: bool,
    pub private_items: bool,
    pub source_locations: bool,
}
```

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

  Add file:line references to items.

#### Trait Implementations

##### `impl Clone for SourceConfig`

- <span id="sourceconfig-clone"></span>`fn clone(&self) -> SourceConfig` — [`SourceConfig`](../../index.md)

##### `impl Debug for SourceConfig`

- <span id="sourceconfig-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SourceConfig`

- <span id="sourceconfig-default"></span>`fn default() -> Self`

##### `impl<T> Instrument for SourceConfig`

##### `impl<T> IntoEither for SourceConfig`

##### `impl<D> OwoColorize for SourceConfig`

##### `impl<T> Pointable for SourceConfig`

- <span id="sourceconfig-align"></span>`const ALIGN: usize`

- <span id="sourceconfig-init"></span>`type Init = T`

- <span id="sourceconfig-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="sourceconfig-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="sourceconfig-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="sourceconfig-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for SourceConfig`

