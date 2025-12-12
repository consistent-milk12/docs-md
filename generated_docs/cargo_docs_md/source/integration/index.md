*[cargo_docs_md](../../index.md) / [source](../index.md) / [integration](index.md)*

---

# Module `integration`

Generator integration for source-enhanced documentation.

This module provides the bridge between parsed source code and the
documentation generator, enabling features like:

- Function body rendering in `<details>` blocks
- Private items section in modules
- Actual constant/static values
- Source file and line number references

# Architecture

Instead of modifying existing generator types, this module provides:

- [`SourceAccess`](traits/index.md) - Trait for accessing parsed source data
- `SourceContext` - Wrapper that adds source access to any  `RenderContext`
- `SourceRenderer` - Helper for rendering source-enhanced content

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`context`](#context) | mod |  |
| [`render`](#render) | mod |  |
| [`traits`](#traits) | mod | Source access trait definition. |

## Modules

- [`context`](context/index.md)
- [`render`](render/index.md)
- [`traits`](traits/index.md) — Source access trait definition.

