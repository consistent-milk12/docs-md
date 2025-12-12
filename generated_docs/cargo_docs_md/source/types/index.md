*[cargo_docs_md](../../index.md) / [source](../index.md) / [types](index.md)*

---

# Module `types`

Types for representing parsed source code information.

These types complement rustdoc JSON by providing information
that is only available from parsing source code directly.

## Contents

- [Structs](#structs)
  - [`FunctionInfo`](#functioninfo)
  - [`StructInfo`](#structinfo)
  - [`FieldInfo`](#fieldinfo)
  - [`EnumInfo`](#enuminfo)
  - [`VariantInfo`](#variantinfo)
  - [`TraitInfo`](#traitinfo)
  - [`ImplInfo`](#implinfo)
  - [`ConstInfo`](#constinfo)
  - [`StaticInfo`](#staticinfo)
  - [`TypeAliasInfo`](#typealiasinfo)
  - [`MacroInfo`](#macroinfo)
  - [`CrateSource`](#cratesource)
- [Enums](#enums)
  - [`PrivateItem`](#privateitem)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FunctionInfo`](#functioninfo) | struct | Information about a parsed function, including its body. |
| [`StructInfo`](#structinfo) | struct | Information about a parsed struct. |
| [`FieldInfo`](#fieldinfo) | struct | Information about a struct or enum field. |
| [`EnumInfo`](#enuminfo) | struct | Information about a parsed enum. |
| [`VariantInfo`](#variantinfo) | struct | Information about an enum variant. |
| [`TraitInfo`](#traitinfo) | struct | Information about a parsed trait. |
| [`ImplInfo`](#implinfo) | struct | Information about an impl block. |
| [`ConstInfo`](#constinfo) | struct | Information about a constant. |
| [`StaticInfo`](#staticinfo) | struct | Information about a static variable. |
| [`TypeAliasInfo`](#typealiasinfo) | struct | Information about a type alias. |
| [`MacroInfo`](#macroinfo) | struct | Information about a macro definition. |
| [`CrateSource`](#cratesource) | struct | Aggregated source information for an entire crate. |
| [`PrivateItem`](#privateitem) | enum | A reference to a private item for rendering. |

## Structs

### `FunctionInfo`

```rust
struct FunctionInfo {
    pub name: String,
    pub module_path: String,
    pub signature: String,
    pub body: String,
    pub is_public: bool,
    pub doc_comments: Vec<String>,
    pub source_file: std::path::PathBuf,
    pub line_number: usize,
}
```

*Defined in `src/source/types.rs:10-34`*

Information about a parsed function, including its body.

#### Fields

- **`name`**: `String`

  The function name.

- **`module_path`**: `String`

  Full module path (e.g., `crate::module::submodule`).

- **`signature`**: `String`

  The function signature as a string.

- **`body`**: `String`

  The function body as source code.

- **`is_public`**: `bool`

  Whether this function is public.

- **`doc_comments`**: `Vec<String>`

  Doc comments extracted from `///` or `//!` attributes.

- **`source_file`**: `std::path::PathBuf`

  Source file where this function is defined.

- **`line_number`**: `usize`

  Line number where the function starts.

#### Trait Implementations

##### `impl Clone for FunctionInfo`

- <span id="functioninfo-clone"></span>`fn clone(&self) -> FunctionInfo` — [`FunctionInfo`](#functioninfo)

##### `impl Debug for FunctionInfo`

- <span id="functioninfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for FunctionInfo`

##### `impl IntoEither for FunctionInfo`

##### `impl OwoColorize for FunctionInfo`

##### `impl Pointable for FunctionInfo`

- <span id="functioninfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="functioninfo-pointable-type-init"></span>`type Init = T`

- <span id="functioninfo-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="functioninfo-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="functioninfo-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="functioninfo-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for FunctionInfo`

### `StructInfo`

```rust
struct StructInfo {
    pub name: String,
    pub module_path: String,
    pub definition: String,
    pub is_public: bool,
    pub doc_comments: Vec<String>,
    pub source_file: std::path::PathBuf,
    pub line_number: usize,
    pub fields: Vec<FieldInfo>,
}
```

*Defined in `src/source/types.rs:38-62`*

Information about a parsed struct.

#### Fields

- **`name`**: `String`

  The struct name.

- **`module_path`**: `String`

  Full module path.

- **`definition`**: `String`

  The full struct definition as source code.

- **`is_public`**: `bool`

  Whether this struct is public.

- **`doc_comments`**: `Vec<String>`

  Doc comments.

- **`source_file`**: `std::path::PathBuf`

  Source file location.

- **`line_number`**: `usize`

  Line number.

- **`fields`**: `Vec<FieldInfo>`

  Field information (for struct fields).

#### Trait Implementations

##### `impl Clone for StructInfo`

- <span id="structinfo-clone"></span>`fn clone(&self) -> StructInfo` — [`StructInfo`](#structinfo)

##### `impl Debug for StructInfo`

- <span id="structinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for StructInfo`

##### `impl IntoEither for StructInfo`

##### `impl OwoColorize for StructInfo`

##### `impl Pointable for StructInfo`

- <span id="structinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="structinfo-pointable-type-init"></span>`type Init = T`

- <span id="structinfo-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="structinfo-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="structinfo-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="structinfo-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for StructInfo`

### `FieldInfo`

```rust
struct FieldInfo {
    pub name: Option<String>,
    pub ty: String,
    pub is_public: bool,
    pub doc_comments: Vec<String>,
}
```

*Defined in `src/source/types.rs:66-78`*

Information about a struct or enum field.

#### Fields

- **`name`**: `Option<String>`

  Field name (None for tuple struct fields).

- **`ty`**: `String`

  Field type as a string.

- **`is_public`**: `bool`

  Whether this field is public.

- **`doc_comments`**: `Vec<String>`

  Doc comments for this field.

#### Trait Implementations

##### `impl Clone for FieldInfo`

- <span id="fieldinfo-clone"></span>`fn clone(&self) -> FieldInfo` — [`FieldInfo`](#fieldinfo)

##### `impl Debug for FieldInfo`

- <span id="fieldinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for FieldInfo`

##### `impl IntoEither for FieldInfo`

##### `impl OwoColorize for FieldInfo`

##### `impl Pointable for FieldInfo`

- <span id="fieldinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="fieldinfo-pointable-type-init"></span>`type Init = T`

- <span id="fieldinfo-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="fieldinfo-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="fieldinfo-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="fieldinfo-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for FieldInfo`

### `EnumInfo`

```rust
struct EnumInfo {
    pub name: String,
    pub module_path: String,
    pub definition: String,
    pub is_public: bool,
    pub doc_comments: Vec<String>,
    pub source_file: std::path::PathBuf,
    pub line_number: usize,
    pub variants: Vec<VariantInfo>,
}
```

*Defined in `src/source/types.rs:82-106`*

Information about a parsed enum.

#### Fields

- **`name`**: `String`

  The enum name.

- **`module_path`**: `String`

  Full module path.

- **`definition`**: `String`

  The full enum definition as source code.

- **`is_public`**: `bool`

  Whether this enum is public.

- **`doc_comments`**: `Vec<String>`

  Doc comments.

- **`source_file`**: `std::path::PathBuf`

  Source file location.

- **`line_number`**: `usize`

  Line number.

- **`variants`**: `Vec<VariantInfo>`

  Variant information.

#### Trait Implementations

##### `impl Clone for EnumInfo`

- <span id="enuminfo-clone"></span>`fn clone(&self) -> EnumInfo` — [`EnumInfo`](#enuminfo)

##### `impl Debug for EnumInfo`

- <span id="enuminfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for EnumInfo`

##### `impl IntoEither for EnumInfo`

##### `impl OwoColorize for EnumInfo`

##### `impl Pointable for EnumInfo`

- <span id="enuminfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="enuminfo-pointable-type-init"></span>`type Init = T`

- <span id="enuminfo-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="enuminfo-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="enuminfo-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="enuminfo-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for EnumInfo`

### `VariantInfo`

```rust
struct VariantInfo {
    pub name: String,
    pub doc_comments: Vec<String>,
    pub fields: Vec<FieldInfo>,
}
```

*Defined in `src/source/types.rs:110-119`*

Information about an enum variant.

#### Fields

- **`name`**: `String`

  Variant name.

- **`doc_comments`**: `Vec<String>`

  Doc comments for this variant.

- **`fields`**: `Vec<FieldInfo>`

  Fields (for tuple or struct variants).

#### Trait Implementations

##### `impl Clone for VariantInfo`

- <span id="variantinfo-clone"></span>`fn clone(&self) -> VariantInfo` — [`VariantInfo`](#variantinfo)

##### `impl Debug for VariantInfo`

- <span id="variantinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for VariantInfo`

##### `impl IntoEither for VariantInfo`

##### `impl OwoColorize for VariantInfo`

##### `impl Pointable for VariantInfo`

- <span id="variantinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="variantinfo-pointable-type-init"></span>`type Init = T`

- <span id="variantinfo-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="variantinfo-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="variantinfo-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="variantinfo-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for VariantInfo`

### `TraitInfo`

```rust
struct TraitInfo {
    pub name: String,
    pub module_path: String,
    pub definition: String,
    pub is_public: bool,
    pub doc_comments: Vec<String>,
    pub source_file: std::path::PathBuf,
    pub line_number: usize,
}
```

*Defined in `src/source/types.rs:123-144`*

Information about a parsed trait.

#### Fields

- **`name`**: `String`

  The trait name.

- **`module_path`**: `String`

  Full module path.

- **`definition`**: `String`

  The full trait definition as source code.

- **`is_public`**: `bool`

  Whether this trait is public.

- **`doc_comments`**: `Vec<String>`

  Doc comments.

- **`source_file`**: `std::path::PathBuf`

  Source file location.

- **`line_number`**: `usize`

  Line number.

#### Trait Implementations

##### `impl Clone for TraitInfo`

- <span id="traitinfo-clone"></span>`fn clone(&self) -> TraitInfo` — [`TraitInfo`](#traitinfo)

##### `impl Debug for TraitInfo`

- <span id="traitinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for TraitInfo`

##### `impl IntoEither for TraitInfo`

##### `impl OwoColorize for TraitInfo`

##### `impl Pointable for TraitInfo`

- <span id="traitinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="traitinfo-pointable-type-init"></span>`type Init = T`

- <span id="traitinfo-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="traitinfo-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="traitinfo-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="traitinfo-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for TraitInfo`

### `ImplInfo`

```rust
struct ImplInfo {
    pub self_ty: String,
    pub trait_name: Option<String>,
    pub module_path: String,
    pub methods: Vec<FunctionInfo>,
    pub source_file: std::path::PathBuf,
    pub line_number: usize,
}
```

*Defined in `src/source/types.rs:148-166`*

Information about an impl block.

#### Fields

- **`self_ty`**: `String`

  The type being implemented for (e.g., `MyStruct`).

- **`trait_name`**: `Option<String>`

  The trait being implemented (if any).

- **`module_path`**: `String`

  Full module path where this impl is defined.

- **`methods`**: `Vec<FunctionInfo>`

  Methods in this impl block.

- **`source_file`**: `std::path::PathBuf`

  Source file location.

- **`line_number`**: `usize`

  Line number.

#### Trait Implementations

##### `impl Clone for ImplInfo`

- <span id="implinfo-clone"></span>`fn clone(&self) -> ImplInfo` — [`ImplInfo`](#implinfo)

##### `impl Debug for ImplInfo`

- <span id="implinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for ImplInfo`

##### `impl IntoEither for ImplInfo`

##### `impl OwoColorize for ImplInfo`

##### `impl Pointable for ImplInfo`

- <span id="implinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="implinfo-pointable-type-init"></span>`type Init = T`

- <span id="implinfo-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="implinfo-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="implinfo-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="implinfo-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for ImplInfo`

### `ConstInfo`

```rust
struct ConstInfo {
    pub name: String,
    pub module_path: String,
    pub ty: String,
    pub value: String,
    pub is_public: bool,
    pub doc_comments: Vec<String>,
    pub source_file: std::path::PathBuf,
    pub line_number: usize,
}
```

*Defined in `src/source/types.rs:170-194`*

Information about a constant.

#### Fields

- **`name`**: `String`

  The constant name.

- **`module_path`**: `String`

  Full module path.

- **`ty`**: `String`

  The type of the constant.

- **`value`**: `String`

  The value expression as source code.

- **`is_public`**: `bool`

  Whether this constant is public.

- **`doc_comments`**: `Vec<String>`

  Doc comments.

- **`source_file`**: `std::path::PathBuf`

  Source file location.

- **`line_number`**: `usize`

  Line number.

#### Trait Implementations

##### `impl Clone for ConstInfo`

- <span id="constinfo-clone"></span>`fn clone(&self) -> ConstInfo` — [`ConstInfo`](#constinfo)

##### `impl Debug for ConstInfo`

- <span id="constinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for ConstInfo`

##### `impl IntoEither for ConstInfo`

##### `impl OwoColorize for ConstInfo`

##### `impl Pointable for ConstInfo`

- <span id="constinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="constinfo-pointable-type-init"></span>`type Init = T`

- <span id="constinfo-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="constinfo-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="constinfo-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="constinfo-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for ConstInfo`

### `StaticInfo`

```rust
struct StaticInfo {
    pub name: String,
    pub module_path: String,
    pub ty: String,
    pub value: String,
    pub is_mutable: bool,
    pub is_public: bool,
    pub doc_comments: Vec<String>,
    pub source_file: std::path::PathBuf,
    pub line_number: usize,
}
```

*Defined in `src/source/types.rs:198-225`*

Information about a static variable.

#### Fields

- **`name`**: `String`

  The static name.

- **`module_path`**: `String`

  Full module path.

- **`ty`**: `String`

  The type of the static.

- **`value`**: `String`

  The value expression as source code.

- **`is_mutable`**: `bool`

  Whether this static is mutable.

- **`is_public`**: `bool`

  Whether this static is public.

- **`doc_comments`**: `Vec<String>`

  Doc comments.

- **`source_file`**: `std::path::PathBuf`

  Source file location.

- **`line_number`**: `usize`

  Line number.

#### Trait Implementations

##### `impl Clone for StaticInfo`

- <span id="staticinfo-clone"></span>`fn clone(&self) -> StaticInfo` — [`StaticInfo`](#staticinfo)

##### `impl Debug for StaticInfo`

- <span id="staticinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for StaticInfo`

##### `impl IntoEither for StaticInfo`

##### `impl OwoColorize for StaticInfo`

##### `impl Pointable for StaticInfo`

- <span id="staticinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="staticinfo-pointable-type-init"></span>`type Init = T`

- <span id="staticinfo-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="staticinfo-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="staticinfo-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="staticinfo-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for StaticInfo`

### `TypeAliasInfo`

```rust
struct TypeAliasInfo {
    pub name: String,
    pub module_path: String,
    pub aliased_type: String,
    pub is_public: bool,
    pub doc_comments: Vec<String>,
    pub source_file: std::path::PathBuf,
    pub line_number: usize,
}
```

*Defined in `src/source/types.rs:229-250`*

Information about a type alias.

#### Fields

- **`name`**: `String`

  The type alias name.

- **`module_path`**: `String`

  Full module path.

- **`aliased_type`**: `String`

  The aliased type as a string.

- **`is_public`**: `bool`

  Whether this type alias is public.

- **`doc_comments`**: `Vec<String>`

  Doc comments.

- **`source_file`**: `std::path::PathBuf`

  Source file location.

- **`line_number`**: `usize`

  Line number.

#### Trait Implementations

##### `impl Clone for TypeAliasInfo`

- <span id="typealiasinfo-clone"></span>`fn clone(&self) -> TypeAliasInfo` — [`TypeAliasInfo`](#typealiasinfo)

##### `impl Debug for TypeAliasInfo`

- <span id="typealiasinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for TypeAliasInfo`

##### `impl IntoEither for TypeAliasInfo`

##### `impl OwoColorize for TypeAliasInfo`

##### `impl Pointable for TypeAliasInfo`

- <span id="typealiasinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="typealiasinfo-pointable-type-init"></span>`type Init = T`

- <span id="typealiasinfo-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="typealiasinfo-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="typealiasinfo-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="typealiasinfo-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for TypeAliasInfo`

### `MacroInfo`

```rust
struct MacroInfo {
    pub name: String,
    pub module_path: String,
    pub definition: String,
    pub doc_comments: Vec<String>,
    pub source_file: std::path::PathBuf,
    pub line_number: usize,
}
```

*Defined in `src/source/types.rs:254-272`*

Information about a macro definition.

#### Fields

- **`name`**: `String`

  The macro name.

- **`module_path`**: `String`

  Full module path.

- **`definition`**: `String`

  The full macro definition as source code.

- **`doc_comments`**: `Vec<String>`

  Doc comments.

- **`source_file`**: `std::path::PathBuf`

  Source file location.

- **`line_number`**: `usize`

  Line number.

#### Trait Implementations

##### `impl Clone for MacroInfo`

- <span id="macroinfo-clone"></span>`fn clone(&self) -> MacroInfo` — [`MacroInfo`](#macroinfo)

##### `impl Debug for MacroInfo`

- <span id="macroinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for MacroInfo`

##### `impl IntoEither for MacroInfo`

##### `impl OwoColorize for MacroInfo`

##### `impl Pointable for MacroInfo`

- <span id="macroinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="macroinfo-pointable-type-init"></span>`type Init = T`

- <span id="macroinfo-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="macroinfo-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="macroinfo-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="macroinfo-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for MacroInfo`

### `CrateSource`

```rust
struct CrateSource {
    pub name: String,
    pub version: String,
    pub root_path: std::path::PathBuf,
    pub functions: Vec<FunctionInfo>,
    pub structs: Vec<StructInfo>,
    pub enums: Vec<EnumInfo>,
    pub traits: Vec<TraitInfo>,
    pub impls: Vec<ImplInfo>,
    pub constants: Vec<ConstInfo>,
    pub statics: Vec<StaticInfo>,
    pub type_aliases: Vec<TypeAliasInfo>,
    pub macros: Vec<MacroInfo>,
}
```

*Defined in `src/source/types.rs:276-312`*

Aggregated source information for an entire crate.

#### Fields

- **`name`**: `String`

  Crate name.

- **`version`**: `String`

  Crate version (from Cargo.toml).

- **`root_path`**: `std::path::PathBuf`

  Root path of the crate source.

- **`functions`**: `Vec<FunctionInfo>`

  All parsed functions (including methods).

- **`structs`**: `Vec<StructInfo>`

  All parsed structs.

- **`enums`**: `Vec<EnumInfo>`

  All parsed enums.

- **`traits`**: `Vec<TraitInfo>`

  All parsed traits.

- **`impls`**: `Vec<ImplInfo>`

  All parsed impl blocks.

- **`constants`**: `Vec<ConstInfo>`

  All parsed constants.

- **`statics`**: `Vec<StaticInfo>`

  All parsed statics.

- **`type_aliases`**: `Vec<TypeAliasInfo>`

  All parsed type aliases.

- **`macros`**: `Vec<MacroInfo>`

  All parsed macro definitions.

#### Implementations

- <span id="cratesource-new"></span>`fn new(name: String, version: String, root_path: PathBuf) -> Self`

- <span id="cratesource-find-function"></span>`fn find_function(&self, path: &str) -> Option<&FunctionInfo>` — [`FunctionInfo`](#functioninfo)

- <span id="cratesource-find-struct"></span>`fn find_struct(&self, path: &str) -> Option<&StructInfo>` — [`StructInfo`](#structinfo)

- <span id="cratesource-find-enum"></span>`fn find_enum(&self, path: &str) -> Option<&EnumInfo>` — [`EnumInfo`](#enuminfo)

- <span id="cratesource-find-trait"></span>`fn find_trait(&self, path: &str) -> Option<&TraitInfo>` — [`TraitInfo`](#traitinfo)

- <span id="cratesource-find-const"></span>`fn find_const(&self, path: &str) -> Option<&ConstInfo>` — [`ConstInfo`](#constinfo)

- <span id="cratesource-find-static"></span>`fn find_static(&self, path: &str) -> Option<&StaticInfo>` — [`StaticInfo`](#staticinfo)

- <span id="cratesource-private-items-in-module"></span>`fn private_items_in_module(&self, module_path: &str) -> Vec<PrivateItem<'_>>` — [`PrivateItem`](#privateitem)

#### Trait Implementations

##### `impl Debug for CrateSource`

- <span id="cratesource-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for CrateSource`

- <span id="cratesource-default"></span>`fn default() -> CrateSource` — [`CrateSource`](#cratesource)

##### `impl Instrument for CrateSource`

##### `impl IntoEither for CrateSource`

##### `impl OwoColorize for CrateSource`

##### `impl Pointable for CrateSource`

- <span id="cratesource-pointable-const-align"></span>`const ALIGN: usize`

- <span id="cratesource-pointable-type-init"></span>`type Init = T`

- <span id="cratesource-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="cratesource-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="cratesource-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="cratesource-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for CrateSource`

## Enums

### `PrivateItem<'a>`

```rust
enum PrivateItem<'a> {
    Function(&'a FunctionInfo),
    Struct(&'a StructInfo),
    Enum(&'a EnumInfo),
    Const(&'a ConstInfo),
    TypeAlias(&'a TypeAliasInfo),
}
```

*Defined in `src/source/types.rs:409-424`*

A reference to a private item for rendering.

#### Variants

- **`Function`**

  A private function.

- **`Struct`**

  A private struct.

- **`Enum`**

  A private enum.

- **`Const`**

  A private constant.

- **`TypeAlias`**

  A private type alias.

#### Trait Implementations

##### `impl Debug for PrivateItem<'a>`

- <span id="privateitem-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for PrivateItem<'a>`

##### `impl IntoEither for PrivateItem<'a>`

##### `impl OwoColorize for PrivateItem<'a>`

##### `impl Pointable for PrivateItem<'a>`

- <span id="privateitem-pointable-const-align"></span>`const ALIGN: usize`

- <span id="privateitem-pointable-type-init"></span>`type Init = T`

- <span id="privateitem-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="privateitem-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="privateitem-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="privateitem-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for PrivateItem<'a>`

