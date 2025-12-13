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

  Create a new source parser for a crate.

- <span id="sourceparser-parse-crate"></span>`fn parse_crate(&self) -> Result<CrateSource, Error>` — [`CrateSource`](../types/index.md#cratesource), [`Error`](../../error/index.md#error)

  Parse an entire crate starting from its root.

  

  # Errors

  

  Returns an error if any source file cannot be parsed.

- <span id="sourceparser-find-entry-point"></span>`fn find_entry_point(&self) -> Result<PathBuf, Error>` — [`Error`](../../error/index.md#error)

  Find the crate entry point (lib.rs or main.rs).

- <span id="sourceparser-parse-module-file"></span>`fn parse_module_file(&self, path: &Path, module_path: &str, source: &mut CrateSource) -> Result<(), Error>` — [`CrateSource`](../types/index.md#cratesource), [`Error`](../../error/index.md#error)

  Parse a single module file and its submodules.

- <span id="sourceparser-process-file"></span>`fn process_file(&self, file: &File, file_path: &Path, module_path: &str, source: &mut CrateSource) -> Result<(), Error>` — [`CrateSource`](../types/index.md#cratesource), [`Error`](../../error/index.md#error)

  Process a parsed file, extracting items and following submodules.

- <span id="sourceparser-process-item"></span>`fn process_item(&self, item: &Item, file_path: &Path, module_path: &str, source: &mut CrateSource) -> Result<(), Error>` — [`CrateSource`](../types/index.md#cratesource), [`Error`](../../error/index.md#error)

  Process a single item from a file.

- <span id="sourceparser-process-module"></span>`fn process_module(&self, module: &ItemMod, current_file: &Path, parent_module_path: &str, source: &mut CrateSource) -> Result<(), Error>` — [`CrateSource`](../types/index.md#cratesource), [`Error`](../../error/index.md#error)

  Process a module declaration, potentially following to an external file.

- <span id="sourceparser-find-module-file"></span>`fn find_module_file(&self, current_file: &Path, module_name: &str) -> Option<PathBuf>`

  Find the file for an external module declaration.

- <span id="sourceparser-extract-function"></span>`fn extract_function(&self, func: &ItemFn, file_path: &Path, module_path: &str) -> FunctionInfo` — [`FunctionInfo`](../types/index.md#functioninfo)

  Extract function information.

- <span id="sourceparser-extract-struct"></span>`fn extract_struct(&self, s: &ItemStruct, file_path: &Path, module_path: &str) -> StructInfo` — [`StructInfo`](../types/index.md#structinfo)

  Extract struct information.

- <span id="sourceparser-extract-enum"></span>`fn extract_enum(&self, e: &ItemEnum, file_path: &Path, module_path: &str) -> EnumInfo` — [`EnumInfo`](../types/index.md#enuminfo)

  Extract enum information.

- <span id="sourceparser-extract-trait"></span>`fn extract_trait(&self, t: &ItemTrait, file_path: &Path, module_path: &str) -> TraitInfo` — [`TraitInfo`](../types/index.md#traitinfo)

  Extract trait information.

- <span id="sourceparser-extract-impl"></span>`fn extract_impl(&self, impl_block: &ItemImpl, file_path: &Path, module_path: &str) -> ImplInfo` — [`ImplInfo`](../types/index.md#implinfo)

  Extract impl block information.

- <span id="sourceparser-extract-const"></span>`fn extract_const(&self, c: &ItemConst, file_path: &Path, module_path: &str) -> ConstInfo` — [`ConstInfo`](../types/index.md#constinfo)

  Extract constant information.

- <span id="sourceparser-extract-static"></span>`fn extract_static(&self, s: &ItemStatic, file_path: &Path, module_path: &str) -> StaticInfo` — [`StaticInfo`](../types/index.md#staticinfo)

  Extract static information.

- <span id="sourceparser-extract-type-alias"></span>`fn extract_type_alias(&self, t: &ItemType, file_path: &Path, module_path: &str) -> TypeAliasInfo` — [`TypeAliasInfo`](../types/index.md#typealiasinfo)

  Extract type alias information.

- <span id="sourceparser-line-of"></span>`fn line_of<T: Spanned>(item: &T) -> usize`

  Extract the starting line number from a spanned item.

  

  Uses `proc-macro2`'s span-locations feature to get accurate line numbers.

- <span id="sourceparser-extract-doc-comments"></span>`fn extract_doc_comments(attrs: &[Attribute]) -> Vec<String>`

  Extract doc comments from attributes.

  

  Doc comments in Rust are represented as `#[doc = "..."]` attributes.

- <span id="sourceparser-extract-fields"></span>`fn extract_fields(fields: &Fields) -> Vec<FieldInfo>` — [`FieldInfo`](../types/index.md#fieldinfo)

  Extract field information from struct/enum fields.

- <span id="sourceparser-parse-file"></span>`fn parse_file(path: &Path) -> Result<File, Error>` — [`Error`](../../error/index.md#error)

  Parse a single file without traversing modules.

  

  Useful for quick parsing of individual files.

  

  # Errors

  

  Returns an error if the file cannot be read or parsed.

#### Trait Implementations

##### `impl Any for SourceParser`

- <span id="sourceparser-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SourceParser`

- <span id="sourceparser-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SourceParser`

- <span id="sourceparser-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for SourceParser`

- <span id="sourceparser-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SourceParser`

- <span id="sourceparser-default"></span>`fn default() -> SourceParser` — [`SourceParser`](#sourceparser)

##### `impl<T> From for SourceParser`

- <span id="sourceparser-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for SourceParser`

##### `impl<U> Into for SourceParser`

- <span id="sourceparser-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SourceParser`

##### `impl OwoColorize for SourceParser`

##### `impl Pointable for SourceParser`

- <span id="sourceparser-pointable-const-align"></span>`const ALIGN: usize`

- <span id="sourceparser-pointable-type-init"></span>`type Init = T`

- <span id="sourceparser-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="sourceparser-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="sourceparser-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="sourceparser-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for SourceParser`

- <span id="sourceparser-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sourceparser-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SourceParser`

- <span id="sourceparser-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sourceparser-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for SourceParser`

