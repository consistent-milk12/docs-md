*[cargo_docs_md](../../../index.md) / [source](../../index.md) / [integration](../index.md) / [traits](index.md)*

---

# Module `traits`

Source access trait definition.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SourceAccess`](#sourceaccess) | trait | Access to parsed source code information. |

## Traits

### `SourceAccess`

```rust
trait SourceAccess { ... }
```

*Defined in `src/source/integration/traits.rs:13-53`*

Access to parsed source code information.

This trait provides access to source code details not available in
rustdoc JSON, including function bodies, private items and actual
constant values.

#### Required Methods

- `fn crate_source(&self) -> Option<&CrateSource>`

  Get parsed source for the current crate.

#### Provided Methods

- `fn find_function_source(&self, path: &str) -> Option<&FunctionInfo>`

  Find function source by full path (e.g., `"crate::module::func_name"`).

- `fn find_struct_source(&self, path: &str) -> Option<&StructInfo>`

  Find struct source by full path.

- `fn find_enum_source(&self, path: &str) -> Option<&EnumInfo>`

  Find enum source by full path.

- `fn find_trait_source(&self, path: &str) -> Option<&TraitInfo>`

  Find trait source by full path.

- `fn find_const_source(&self, path: &str) -> Option<&ConstInfo>`

  Find constant source by full path.

- `fn find_static_source(&self, path: &str) -> Option<&StaticInfo>`

  Find static source by full path.

- `fn private_items_in_module(&self, module_path: &str) -> Vec<PrivateItem<'_>>`

  Get all private items in a module.

