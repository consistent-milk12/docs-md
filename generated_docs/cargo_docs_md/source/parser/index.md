*[cargo_docs_md](../../index.md) / [source](../index.md) / [parser](index.md)*

---

# Module `parser`

Source code parser using `syn`.

This module provides parsing of Rust source files to extract
information that is not available in rustdoc JSON, such as
function bodies, private items, and implementation details.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SourceParser`](#sourceparser) | struct | Parser for Rust source code using `syn`. |

## Structs

### `SourceParser`

```rust
struct SourceParser {
    crate_name: String,
    crate_version: String,
    crate_root: std::path::PathBuf,
}
```

*Defined in `src/source/parser.rs:32-41`*

Parser for Rust source code using `syn`.

#### Fields

- **`crate_name`**: `String`

  The crate name being parsed.

- **`crate_version`**: `String`

  The crate version.

- **`crate_root`**: `std::path::PathBuf`

  Root path of the crate.

#### Implementations

- <span id="sourceparser-new"></span>`const fn new(name: String, version: String, root_path: PathBuf) -> Self`

- <span id="sourceparser-parse-crate"></span>`fn parse_crate(&self) -> Result<CrateSource, Error>` — [`CrateSource`](../types/index.md#cratesource), [`Error`](../../error/index.md#error)

- <span id="sourceparser-find-entry-point"></span>`fn find_entry_point(&self) -> Result<PathBuf, Error>` — [`Error`](../../error/index.md#error)

- <span id="sourceparser-parse-module-file"></span>`fn parse_module_file(&self, path: &Path, module_path: &str, source: &mut CrateSource) -> Result<(), Error>` — [`CrateSource`](../types/index.md#cratesource), [`Error`](../../error/index.md#error)

- <span id="sourceparser-process-file"></span>`fn process_file(&self, file: &File, file_path: &Path, module_path: &str, source: &mut CrateSource) -> Result<(), Error>` — [`CrateSource`](../types/index.md#cratesource), [`Error`](../../error/index.md#error)

- <span id="sourceparser-process-item"></span>`fn process_item(&self, item: &Item, file_path: &Path, module_path: &str, source: &mut CrateSource) -> Result<(), Error>` — [`CrateSource`](../types/index.md#cratesource), [`Error`](../../error/index.md#error)

- <span id="sourceparser-process-module"></span>`fn process_module(&self, module: &ItemMod, current_file: &Path, parent_module_path: &str, source: &mut CrateSource) -> Result<(), Error>` — [`CrateSource`](../types/index.md#cratesource), [`Error`](../../error/index.md#error)

- <span id="sourceparser-find-module-file"></span>`fn find_module_file(&self, current_file: &Path, module_name: &str) -> Option<PathBuf>`

- <span id="sourceparser-extract-function"></span>`fn extract_function(&self, func: &ItemFn, file_path: &Path, module_path: &str) -> FunctionInfo` — [`FunctionInfo`](../types/index.md#functioninfo)

- <span id="sourceparser-extract-struct"></span>`fn extract_struct(&self, s: &ItemStruct, file_path: &Path, module_path: &str) -> StructInfo` — [`StructInfo`](../types/index.md#structinfo)

- <span id="sourceparser-extract-enum"></span>`fn extract_enum(&self, e: &ItemEnum, file_path: &Path, module_path: &str) -> EnumInfo` — [`EnumInfo`](../types/index.md#enuminfo)

- <span id="sourceparser-extract-trait"></span>`fn extract_trait(&self, t: &ItemTrait, file_path: &Path, module_path: &str) -> TraitInfo` — [`TraitInfo`](../types/index.md#traitinfo)

- <span id="sourceparser-extract-impl"></span>`fn extract_impl(&self, impl_block: &ItemImpl, file_path: &Path, module_path: &str) -> ImplInfo` — [`ImplInfo`](../types/index.md#implinfo)

- <span id="sourceparser-extract-const"></span>`fn extract_const(&self, c: &ItemConst, file_path: &Path, module_path: &str) -> ConstInfo` — [`ConstInfo`](../types/index.md#constinfo)

- <span id="sourceparser-extract-static"></span>`fn extract_static(&self, s: &ItemStatic, file_path: &Path, module_path: &str) -> StaticInfo` — [`StaticInfo`](../types/index.md#staticinfo)

- <span id="sourceparser-extract-type-alias"></span>`fn extract_type_alias(&self, t: &ItemType, file_path: &Path, module_path: &str) -> TypeAliasInfo` — [`TypeAliasInfo`](../types/index.md#typealiasinfo)

- <span id="sourceparser-line-of"></span>`fn line_of<T: Spanned>(item: &T) -> usize`

- <span id="sourceparser-extract-doc-comments"></span>`fn extract_doc_comments(attrs: &[Attribute]) -> Vec<String>`

- <span id="sourceparser-extract-fields"></span>`fn extract_fields(fields: &Fields) -> Vec<FieldInfo>` — [`FieldInfo`](../types/index.md#fieldinfo)

- <span id="sourceparser-parse-file"></span>`fn parse_file(path: &Path) -> Result<File, Error>` — [`Error`](../../error/index.md#error)

#### Trait Implementations

##### `impl Debug for SourceParser`

- <span id="sourceparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SourceParser`

- <span id="sourceparser-default"></span>`fn default() -> SourceParser` — [`SourceParser`](#sourceparser)

##### `impl Instrument for SourceParser`

##### `impl IntoEither for SourceParser`

##### `impl OwoColorize for SourceParser`

##### `impl Pointable for SourceParser`

- <span id="sourceparser-pointable-const-align"></span>`const ALIGN: usize`

- <span id="sourceparser-pointable-type-init"></span>`type Init = T`

- <span id="sourceparser-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="sourceparser-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="sourceparser-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="sourceparser-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for SourceParser`

