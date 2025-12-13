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

##### `impl Any for FunctionInfo`

- <span id="functioninfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FunctionInfo`

- <span id="functioninfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FunctionInfo`

- <span id="functioninfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FunctionInfo`

- <span id="functioninfo-clone"></span>`fn clone(&self) -> FunctionInfo` — [`FunctionInfo`](#functioninfo)

##### `impl CloneToUninit for FunctionInfo`

- <span id="functioninfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for FunctionInfo`

- <span id="functioninfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FunctionInfo`

- <span id="functioninfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for FunctionInfo`

##### `impl<U> Into for FunctionInfo`

- <span id="functioninfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FunctionInfo`

##### `impl OwoColorize for FunctionInfo`

##### `impl Pointable for FunctionInfo`

- <span id="functioninfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="functioninfo-pointable-type-init"></span>`type Init = T`

- <span id="functioninfo-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="functioninfo-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="functioninfo-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="functioninfo-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for FunctionInfo`

- <span id="functioninfo-toowned-type-owned"></span>`type Owned = T`

- <span id="functioninfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="functioninfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FunctionInfo`

- <span id="functioninfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="functioninfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FunctionInfo`

- <span id="functioninfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="functioninfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for StructInfo`

- <span id="structinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StructInfo`

- <span id="structinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StructInfo`

- <span id="structinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StructInfo`

- <span id="structinfo-clone"></span>`fn clone(&self) -> StructInfo` — [`StructInfo`](#structinfo)

##### `impl CloneToUninit for StructInfo`

- <span id="structinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for StructInfo`

- <span id="structinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for StructInfo`

- <span id="structinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for StructInfo`

##### `impl<U> Into for StructInfo`

- <span id="structinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for StructInfo`

##### `impl OwoColorize for StructInfo`

##### `impl Pointable for StructInfo`

- <span id="structinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="structinfo-pointable-type-init"></span>`type Init = T`

- <span id="structinfo-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="structinfo-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="structinfo-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="structinfo-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for StructInfo`

- <span id="structinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="structinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="structinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StructInfo`

- <span id="structinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="structinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StructInfo`

- <span id="structinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="structinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for FieldInfo`

- <span id="fieldinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FieldInfo`

- <span id="fieldinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FieldInfo`

- <span id="fieldinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FieldInfo`

- <span id="fieldinfo-clone"></span>`fn clone(&self) -> FieldInfo` — [`FieldInfo`](#fieldinfo)

##### `impl CloneToUninit for FieldInfo`

- <span id="fieldinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for FieldInfo`

- <span id="fieldinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FieldInfo`

- <span id="fieldinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for FieldInfo`

##### `impl<U> Into for FieldInfo`

- <span id="fieldinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FieldInfo`

##### `impl OwoColorize for FieldInfo`

##### `impl Pointable for FieldInfo`

- <span id="fieldinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="fieldinfo-pointable-type-init"></span>`type Init = T`

- <span id="fieldinfo-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="fieldinfo-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="fieldinfo-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="fieldinfo-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for FieldInfo`

- <span id="fieldinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="fieldinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fieldinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FieldInfo`

- <span id="fieldinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fieldinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FieldInfo`

- <span id="fieldinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fieldinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for EnumInfo`

- <span id="enuminfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EnumInfo`

- <span id="enuminfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EnumInfo`

- <span id="enuminfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for EnumInfo`

- <span id="enuminfo-clone"></span>`fn clone(&self) -> EnumInfo` — [`EnumInfo`](#enuminfo)

##### `impl CloneToUninit for EnumInfo`

- <span id="enuminfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for EnumInfo`

- <span id="enuminfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for EnumInfo`

- <span id="enuminfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for EnumInfo`

##### `impl<U> Into for EnumInfo`

- <span id="enuminfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for EnumInfo`

##### `impl OwoColorize for EnumInfo`

##### `impl Pointable for EnumInfo`

- <span id="enuminfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="enuminfo-pointable-type-init"></span>`type Init = T`

- <span id="enuminfo-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="enuminfo-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="enuminfo-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="enuminfo-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for EnumInfo`

- <span id="enuminfo-toowned-type-owned"></span>`type Owned = T`

- <span id="enuminfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="enuminfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for EnumInfo`

- <span id="enuminfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="enuminfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EnumInfo`

- <span id="enuminfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="enuminfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for VariantInfo`

- <span id="variantinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VariantInfo`

- <span id="variantinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VariantInfo`

- <span id="variantinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for VariantInfo`

- <span id="variantinfo-clone"></span>`fn clone(&self) -> VariantInfo` — [`VariantInfo`](#variantinfo)

##### `impl CloneToUninit for VariantInfo`

- <span id="variantinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for VariantInfo`

- <span id="variantinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for VariantInfo`

- <span id="variantinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for VariantInfo`

##### `impl<U> Into for VariantInfo`

- <span id="variantinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for VariantInfo`

##### `impl OwoColorize for VariantInfo`

##### `impl Pointable for VariantInfo`

- <span id="variantinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="variantinfo-pointable-type-init"></span>`type Init = T`

- <span id="variantinfo-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="variantinfo-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="variantinfo-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="variantinfo-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for VariantInfo`

- <span id="variantinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="variantinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="variantinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for VariantInfo`

- <span id="variantinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="variantinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VariantInfo`

- <span id="variantinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="variantinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for TraitInfo`

- <span id="traitinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TraitInfo`

- <span id="traitinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TraitInfo`

- <span id="traitinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TraitInfo`

- <span id="traitinfo-clone"></span>`fn clone(&self) -> TraitInfo` — [`TraitInfo`](#traitinfo)

##### `impl CloneToUninit for TraitInfo`

- <span id="traitinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for TraitInfo`

- <span id="traitinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TraitInfo`

- <span id="traitinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for TraitInfo`

##### `impl<U> Into for TraitInfo`

- <span id="traitinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TraitInfo`

##### `impl OwoColorize for TraitInfo`

##### `impl Pointable for TraitInfo`

- <span id="traitinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="traitinfo-pointable-type-init"></span>`type Init = T`

- <span id="traitinfo-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="traitinfo-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="traitinfo-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="traitinfo-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for TraitInfo`

- <span id="traitinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="traitinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="traitinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TraitInfo`

- <span id="traitinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="traitinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TraitInfo`

- <span id="traitinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="traitinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for ImplInfo`

- <span id="implinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImplInfo`

- <span id="implinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImplInfo`

- <span id="implinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ImplInfo`

- <span id="implinfo-clone"></span>`fn clone(&self) -> ImplInfo` — [`ImplInfo`](#implinfo)

##### `impl CloneToUninit for ImplInfo`

- <span id="implinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ImplInfo`

- <span id="implinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ImplInfo`

- <span id="implinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for ImplInfo`

##### `impl<U> Into for ImplInfo`

- <span id="implinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ImplInfo`

##### `impl OwoColorize for ImplInfo`

##### `impl Pointable for ImplInfo`

- <span id="implinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="implinfo-pointable-type-init"></span>`type Init = T`

- <span id="implinfo-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="implinfo-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="implinfo-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="implinfo-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for ImplInfo`

- <span id="implinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="implinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="implinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ImplInfo`

- <span id="implinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="implinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImplInfo`

- <span id="implinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="implinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for ConstInfo`

- <span id="constinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ConstInfo`

- <span id="constinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ConstInfo`

- <span id="constinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ConstInfo`

- <span id="constinfo-clone"></span>`fn clone(&self) -> ConstInfo` — [`ConstInfo`](#constinfo)

##### `impl CloneToUninit for ConstInfo`

- <span id="constinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ConstInfo`

- <span id="constinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ConstInfo`

- <span id="constinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for ConstInfo`

##### `impl<U> Into for ConstInfo`

- <span id="constinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ConstInfo`

##### `impl OwoColorize for ConstInfo`

##### `impl Pointable for ConstInfo`

- <span id="constinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="constinfo-pointable-type-init"></span>`type Init = T`

- <span id="constinfo-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="constinfo-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="constinfo-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="constinfo-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for ConstInfo`

- <span id="constinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="constinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="constinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ConstInfo`

- <span id="constinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="constinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ConstInfo`

- <span id="constinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="constinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for StaticInfo`

- <span id="staticinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StaticInfo`

- <span id="staticinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StaticInfo`

- <span id="staticinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StaticInfo`

- <span id="staticinfo-clone"></span>`fn clone(&self) -> StaticInfo` — [`StaticInfo`](#staticinfo)

##### `impl CloneToUninit for StaticInfo`

- <span id="staticinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for StaticInfo`

- <span id="staticinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for StaticInfo`

- <span id="staticinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for StaticInfo`

##### `impl<U> Into for StaticInfo`

- <span id="staticinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for StaticInfo`

##### `impl OwoColorize for StaticInfo`

##### `impl Pointable for StaticInfo`

- <span id="staticinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="staticinfo-pointable-type-init"></span>`type Init = T`

- <span id="staticinfo-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="staticinfo-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="staticinfo-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="staticinfo-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for StaticInfo`

- <span id="staticinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="staticinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="staticinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StaticInfo`

- <span id="staticinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="staticinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StaticInfo`

- <span id="staticinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="staticinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for TypeAliasInfo`

- <span id="typealiasinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeAliasInfo`

- <span id="typealiasinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeAliasInfo`

- <span id="typealiasinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TypeAliasInfo`

- <span id="typealiasinfo-clone"></span>`fn clone(&self) -> TypeAliasInfo` — [`TypeAliasInfo`](#typealiasinfo)

##### `impl CloneToUninit for TypeAliasInfo`

- <span id="typealiasinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for TypeAliasInfo`

- <span id="typealiasinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TypeAliasInfo`

- <span id="typealiasinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for TypeAliasInfo`

##### `impl<U> Into for TypeAliasInfo`

- <span id="typealiasinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TypeAliasInfo`

##### `impl OwoColorize for TypeAliasInfo`

##### `impl Pointable for TypeAliasInfo`

- <span id="typealiasinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="typealiasinfo-pointable-type-init"></span>`type Init = T`

- <span id="typealiasinfo-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="typealiasinfo-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="typealiasinfo-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="typealiasinfo-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for TypeAliasInfo`

- <span id="typealiasinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="typealiasinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typealiasinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TypeAliasInfo`

- <span id="typealiasinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typealiasinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeAliasInfo`

- <span id="typealiasinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typealiasinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for MacroInfo`

- <span id="macroinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MacroInfo`

- <span id="macroinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MacroInfo`

- <span id="macroinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MacroInfo`

- <span id="macroinfo-clone"></span>`fn clone(&self) -> MacroInfo` — [`MacroInfo`](#macroinfo)

##### `impl CloneToUninit for MacroInfo`

- <span id="macroinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for MacroInfo`

- <span id="macroinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MacroInfo`

- <span id="macroinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for MacroInfo`

##### `impl<U> Into for MacroInfo`

- <span id="macroinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MacroInfo`

##### `impl OwoColorize for MacroInfo`

##### `impl Pointable for MacroInfo`

- <span id="macroinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="macroinfo-pointable-type-init"></span>`type Init = T`

- <span id="macroinfo-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="macroinfo-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="macroinfo-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="macroinfo-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for MacroInfo`

- <span id="macroinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="macroinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="macroinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MacroInfo`

- <span id="macroinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="macroinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MacroInfo`

- <span id="macroinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="macroinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Create a new empty `CrateSource`.

- <span id="cratesource-find-function"></span>`fn find_function(&self, path: &str) -> Option<&FunctionInfo>` — [`FunctionInfo`](#functioninfo)

  Look up a function by its full path.

- <span id="cratesource-find-struct"></span>`fn find_struct(&self, path: &str) -> Option<&StructInfo>` — [`StructInfo`](#structinfo)

  Look up a struct by its full path.

- <span id="cratesource-find-enum"></span>`fn find_enum(&self, path: &str) -> Option<&EnumInfo>` — [`EnumInfo`](#enuminfo)

  Look up an enum by its full path.

- <span id="cratesource-find-trait"></span>`fn find_trait(&self, path: &str) -> Option<&TraitInfo>` — [`TraitInfo`](#traitinfo)

  Look up a trait by its full path.

- <span id="cratesource-find-const"></span>`fn find_const(&self, path: &str) -> Option<&ConstInfo>` — [`ConstInfo`](#constinfo)

  Look up a constant by its full path.

- <span id="cratesource-find-static"></span>`fn find_static(&self, path: &str) -> Option<&StaticInfo>` — [`StaticInfo`](#staticinfo)

  Look up a static by its full path.

- <span id="cratesource-private-items-in-module"></span>`fn private_items_in_module(&self, module_path: &str) -> Vec<PrivateItem<'_>>` — [`PrivateItem`](#privateitem)

  Get all private items (functions, structs, etc.) in a module.

#### Trait Implementations

##### `impl Any for CrateSource`

- <span id="cratesource-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CrateSource`

- <span id="cratesource-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CrateSource`

- <span id="cratesource-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for CrateSource`

- <span id="cratesource-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for CrateSource`

- <span id="cratesource-default"></span>`fn default() -> CrateSource` — [`CrateSource`](#cratesource)

##### `impl<T> From for CrateSource`

- <span id="cratesource-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for CrateSource`

##### `impl<U> Into for CrateSource`

- <span id="cratesource-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for CrateSource`

##### `impl OwoColorize for CrateSource`

##### `impl Pointable for CrateSource`

- <span id="cratesource-pointable-const-align"></span>`const ALIGN: usize`

- <span id="cratesource-pointable-type-init"></span>`type Init = T`

- <span id="cratesource-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="cratesource-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="cratesource-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="cratesource-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for CrateSource`

- <span id="cratesource-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cratesource-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CrateSource`

- <span id="cratesource-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cratesource-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for PrivateItem<'a>`

- <span id="privateitem-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PrivateItem<'a>`

- <span id="privateitem-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PrivateItem<'a>`

- <span id="privateitem-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for PrivateItem<'a>`

- <span id="privateitem-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PrivateItem<'a>`

- <span id="privateitem-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for PrivateItem<'a>`

##### `impl<U> Into for PrivateItem<'a>`

- <span id="privateitem-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for PrivateItem<'a>`

##### `impl OwoColorize for PrivateItem<'a>`

##### `impl Pointable for PrivateItem<'a>`

- <span id="privateitem-pointable-const-align"></span>`const ALIGN: usize`

- <span id="privateitem-pointable-type-init"></span>`type Init = T`

- <span id="privateitem-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="privateitem-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="privateitem-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="privateitem-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for PrivateItem<'a>`

- <span id="privateitem-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="privateitem-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PrivateItem<'a>`

- <span id="privateitem-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="privateitem-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for PrivateItem<'a>`

