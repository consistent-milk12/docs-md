# Crate `rustdoc_types`

Rustdoc's JSON output interface

These types are the public API exposed through the `--output-format json` flag. The [`Crate`](#crate)
struct is the root of the JSON blob and all other items are contained within.

We expose a `rustc-hash` feature that is disabled by default. This feature switches the
[`std::collections::HashMap`](../hashbrown/hash_map/index.md) for `rustc_hash::FxHashMap` to improve the performance of said
`HashMap` in specific situations.

`cargo-semver-checks` for example, saw a [-3% improvement][1] when benchmarking using the
`aws_sdk_ec2` JSON output (~500MB of JSON). As always, we recommend measuring the impact before
turning this feature on, as [`FxHashMap`][2] only concerns itself with hash speed, and may
increase the number of collisions.



## Contents

- [Structs](#structs)
  - [`Crate`](#crate)
  - [`Target`](#target)
  - [`TargetFeature`](#targetfeature)
  - [`ExternalCrate`](#externalcrate)
  - [`ItemSummary`](#itemsummary)
  - [`Item`](#item)
  - [`AttributeRepr`](#attributerepr)
  - [`Span`](#span)
  - [`Deprecation`](#deprecation)
  - [`DynTrait`](#dyntrait)
  - [`PolyTrait`](#polytrait)
  - [`Constant`](#constant)
  - [`AssocItemConstraint`](#associtemconstraint)
  - [`Id`](#id)
  - [`Module`](#module)
  - [`Union`](#union)
  - [`Struct`](#struct)
  - [`Enum`](#enum)
  - [`Variant`](#variant)
  - [`Discriminant`](#discriminant)
  - [`FunctionHeader`](#functionheader)
  - [`Function`](#function)
  - [`Generics`](#generics)
  - [`GenericParamDef`](#genericparamdef)
  - [`Path`](#path)
  - [`FunctionPointer`](#functionpointer)
  - [`FunctionSignature`](#functionsignature)
  - [`Trait`](#trait)
  - [`TraitAlias`](#traitalias)
  - [`Impl`](#impl)
  - [`Use`](#use)
  - [`ProcMacro`](#procmacro)
  - [`TypeAlias`](#typealias)
  - [`Static`](#static)
  - [`Primitive`](#primitive)
- [Enums](#enums)
  - [`Attribute`](#attribute)
  - [`ReprKind`](#reprkind)
  - [`Visibility`](#visibility)
  - [`GenericArgs`](#genericargs)
  - [`GenericArg`](#genericarg)
  - [`AssocItemConstraintKind`](#associtemconstraintkind)
  - [`ItemKind`](#itemkind)
  - [`ItemEnum`](#itemenum)
  - [`StructKind`](#structkind)
  - [`VariantKind`](#variantkind)
  - [`Abi`](#abi)
  - [`GenericParamDefKind`](#genericparamdefkind)
  - [`WherePredicate`](#wherepredicate)
  - [`GenericBound`](#genericbound)
  - [`TraitBoundModifier`](#traitboundmodifier)
  - [`PreciseCapturingArg`](#precisecapturingarg)
  - [`Term`](#term)
  - [`Type`](#type)
  - [`MacroKind`](#macrokind)
- [Constants](#constants)
  - [`FORMAT_VERSION`](#format-version)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Crate`](#crate) | struct | The root of the emitted JSON blob. |
| [`Target`](#target) | struct | Information about a target |
| [`TargetFeature`](#targetfeature) | struct | Information about a target feature. |
| [`ExternalCrate`](#externalcrate) | struct | Metadata of a crate, either the same crate on which `rustdoc` was invoked, or its dependency. |
| [`ItemSummary`](#itemsummary) | struct | Information about an external (not defined in the local crate) [`Item`]. |
| [`Item`](#item) | struct | Anything that can hold documentation - modules, structs, enums, functions, traits, etc. |
| [`AttributeRepr`](#attributerepr) | struct | The contents of a `#[repr(...)]` attribute. |
| [`Span`](#span) | struct | A range of source code. |
| [`Deprecation`](#deprecation) | struct | Information about the deprecation of an [`Item`]. |
| [`DynTrait`](#dyntrait) | struct | Dynamic trait object type (`dyn Trait`). |
| [`PolyTrait`](#polytrait) | struct | A trait and potential HRTBs |
| [`Constant`](#constant) | struct | A constant. |
| [`AssocItemConstraint`](#associtemconstraint) | struct | Describes a bound applied to an associated type/constant. |
| [`Id`](#id) | struct | An opaque identifier for an item. |
| [`Module`](#module) | struct | A module declaration, e.g. `mod foo;` or `mod foo {}`. |
| [`Union`](#union) | struct | A `union`. |
| [`Struct`](#struct) | struct | A `struct`. |
| [`Enum`](#enum) | struct | An `enum`. |
| [`Variant`](#variant) | struct | A variant of an enum. |
| [`Discriminant`](#discriminant) | struct | The value that distinguishes a variant in an [`Enum`] from other variants. |
| [`FunctionHeader`](#functionheader) | struct | A set of fundamental properties of a function. |
| [`Function`](#function) | struct | A function declaration (including methods and other associated functions). |
| [`Generics`](#generics) | struct | Generic parameters accepted by an item and `where` clauses imposed on it and the parameters. |
| [`GenericParamDef`](#genericparamdef) | struct | One generic parameter accepted by an item. |
| [`Path`](#path) | struct | A type that has a simple path to it. |
| [`FunctionPointer`](#functionpointer) | struct | A type that is a function pointer. |
| [`FunctionSignature`](#functionsignature) | struct | The signature of a function. |
| [`Trait`](#trait) | struct | A `trait` declaration. |
| [`TraitAlias`](#traitalias) | struct | A trait alias declaration, e.g. `trait Int = Add + Sub + Mul + Div;` |
| [`Impl`](#impl) | struct | An `impl` block. |
| [`Use`](#use) | struct | A `use` statement. |
| [`ProcMacro`](#procmacro) | struct | A procedural macro. |
| [`TypeAlias`](#typealias) | struct | A type alias declaration, e.g. `type Pig = std::borrow::Cow<'static, str>;` |
| [`Static`](#static) | struct | A `static` declaration. |
| [`Primitive`](#primitive) | struct | A primitive type declaration. |
| [`Attribute`](#attribute) | enum | An attribute, e.g. `#[repr(C)]` |
| [`ReprKind`](#reprkind) | enum | The kind of `#[repr]`. |
| [`Visibility`](#visibility) | enum | Visibility of an [`Item`]. |
| [`GenericArgs`](#genericargs) | enum | A set of generic arguments provided to a path segment, e.g. |
| [`GenericArg`](#genericarg) | enum | One argument in a list of generic arguments to a path segment. |
| [`AssocItemConstraintKind`](#associtemconstraintkind) | enum | The way in which an associate type/constant is bound. |
| [`ItemKind`](#itemkind) | enum | The fundamental kind of an item. |
| [`ItemEnum`](#itemenum) | enum | Specific fields of an item. |
| [`StructKind`](#structkind) | enum | The kind of a [`Struct`] and the data specific to it, i.e. fields. |
| [`VariantKind`](#variantkind) | enum | The kind of an [`Enum`] [`Variant`] and the data specific to it, i.e. fields. |
| [`Abi`](#abi) | enum | The ABI (Application Binary Interface) used by a function. |
| [`GenericParamDefKind`](#genericparamdefkind) | enum | The kind of a [`GenericParamDef`]. |
| [`WherePredicate`](#wherepredicate) | enum | One `where` clause. |
| [`GenericBound`](#genericbound) | enum | Either a trait bound or a lifetime bound. |
| [`TraitBoundModifier`](#traitboundmodifier) | enum | A set of modifiers applied to a trait. |
| [`PreciseCapturingArg`](#precisecapturingarg) | enum | One precise capturing argument. |
| [`Term`](#term) | enum | Either a type or a constant, usually stored as the right-hand side of an equation in places like [`AssocItemConstraint`] |
| [`Type`](#type) | enum | A type. |
| [`MacroKind`](#macrokind) | enum | The way a [`ProcMacro`] is declared to be used. |
| [`FORMAT_VERSION`](#format-version) | const | The version of JSON output that this crate represents. |

## Structs

### `Crate`

```rust
struct Crate {
    pub root: Id,
    pub crate_version: Option<String>,
    pub includes_private: bool,
    pub index: std::collections::HashMap<Id, Item>,
    pub paths: std::collections::HashMap<Id, ItemSummary>,
    pub external_crates: std::collections::HashMap<u32, ExternalCrate>,
    pub target: Target,
    pub format_version: u32,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:48-67`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L48-L67)*

The root of the emitted JSON blob.

It contains all type/documentation information
about the language items in the local crate, as well as info about external items to allow
tools to find or link to them.

#### Fields

- **`root`**: `Id`

  The id of the root [`Module`](#module) item of the local crate.

- **`crate_version`**: `Option<String>`

  The version string given to `--crate-version`, if any.

- **`includes_private`**: `bool`

  Whether or not the output includes private items.

- **`index`**: `std::collections::HashMap<Id, Item>`

  A collection of all items in the local crate as well as some external traits and their
  items that are referenced locally.

- **`paths`**: `std::collections::HashMap<Id, ItemSummary>`

  Maps IDs to fully qualified paths and other info helpful for generating links.

- **`external_crates`**: `std::collections::HashMap<u32, ExternalCrate>`

  Maps `crate_id` of items to a crate name and html_root_url if it exists.

- **`target`**: `Target`

  Information about the target for which this documentation was generated

- **`format_version`**: `u32`

  A single version number to be used in the future when making backwards incompatible changes
  to the JSON output.

#### Trait Implementations

##### `impl Clone for Crate`

- <span id="crate-clone"></span>`fn clone(&self) -> Crate` — [`Crate`](#crate)

##### `impl Debug for Crate`

- <span id="crate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Crate`

- <span id="crate-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Crate`

##### `impl Eq for Crate`

##### `impl PartialEq for Crate`

- <span id="crate-eq"></span>`fn eq(&self, other: &Crate) -> bool` — [`Crate`](#crate)

##### `impl Serialize for Crate`

- <span id="crate-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Crate`

### `Target`

```rust
struct Target {
    pub triple: String,
    pub target_features: Vec<TargetFeature>,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:71-77`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L71-L77)*

Information about a target

#### Fields

- **`triple`**: `String`

  The target triple for which this documentation was generated

- **`target_features`**: `Vec<TargetFeature>`

  A list of features valid for use in `#[target_feature]` attributes
  for the target where this rustdoc JSON was generated.

#### Trait Implementations

##### `impl Clone for Target`

- <span id="target-clone"></span>`fn clone(&self) -> Target` — [`Target`](#target)

##### `impl Debug for Target`

- <span id="target-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Target`

- <span id="target-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Target`

##### `impl Eq for Target`

##### `impl PartialEq for Target`

- <span id="target-eq"></span>`fn eq(&self, other: &Target) -> bool` — [`Target`](#target)

##### `impl Serialize for Target`

- <span id="target-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Target`

### `TargetFeature`

```rust
struct TargetFeature {
    pub name: String,
    pub implies_features: Vec<String>,
    pub unstable_feature_gate: Option<String>,
    pub globally_enabled: bool,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:101-121`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L101-L121)*

Information about a target feature.

Rust target features are used to influence code generation, especially around selecting
instructions which are not universally supported by the target architecture.

Target features are commonly enabled by the [`#[target_feature]` attribute][1] to influence code
generation for a particular function, and less commonly enabled by compiler options like
`-Ctarget-feature` or `-Ctarget-cpu`. Targets themselves automatically enable certain target
features by default, for example because the target's ABI specification requires saving specific
registers which only exist in an architectural extension.

Target features can imply other target features: for example, x86-64 `avx2` implies `avx`, and
aarch64 `sve2` implies `sve`, since both of these architectural extensions depend on their
predecessors.

Target features can be probed at compile time by [`#[cfg(target_feature)]`][2] or `cfg!(…)`
conditional compilation to determine whether a target feature is enabled in a particular
context.



#### Fields

- **`name`**: `String`

  The name of this target feature.

- **`implies_features`**: `Vec<String>`

  Other target features which are implied by this target feature, if any.

- **`unstable_feature_gate`**: `Option<String>`

  If this target feature is unstable, the name of the associated language feature gate.

- **`globally_enabled`**: `bool`

  Whether this feature is globally enabled for this compilation session.
  
  Target features can be globally enabled implicitly as a result of the target's definition.
  For example, x86-64 hardware floating point ABIs require saving x87 and SSE2 registers,
  which in turn requires globally enabling the `x87` and `sse2` target features so that the
  generated machine code conforms to the target's ABI.
  
  Target features can also be globally enabled explicitly as a result of compiler flags like
  [`-Ctarget-feature`][1] or [`-Ctarget-cpu`][2].
  
  

#### Trait Implementations

##### `impl Clone for TargetFeature`

- <span id="targetfeature-clone"></span>`fn clone(&self) -> TargetFeature` — [`TargetFeature`](#targetfeature)

##### `impl Debug for TargetFeature`

- <span id="targetfeature-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for TargetFeature`

- <span id="targetfeature-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for TargetFeature`

##### `impl Eq for TargetFeature`

##### `impl PartialEq for TargetFeature`

- <span id="targetfeature-eq"></span>`fn eq(&self, other: &TargetFeature) -> bool` — [`TargetFeature`](#targetfeature)

##### `impl Serialize for TargetFeature`

- <span id="targetfeature-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for TargetFeature`

### `ExternalCrate`

```rust
struct ExternalCrate {
    pub name: String,
    pub html_root_url: Option<String>,
    pub path: std::path::PathBuf,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:125-143`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L125-L143)*

Metadata of a crate, either the same crate on which `rustdoc` was invoked, or its dependency.

#### Fields

- **`name`**: `String`

  The name of the crate.
  
  Note: This is the [*crate* name][crate-name], which may not be the same as the
  [*package* name][package-name]. For example, for <https://crates.io/crates/regex-syntax>,
  this field will be `regex_syntax` (which uses an `_`, not a `-`).
  
  

- **`html_root_url`**: `Option<String>`

  The root URL at which the crate's documentation lives.

- **`path`**: `std::path::PathBuf`

  A path from where this crate was loaded.
  
  This will typically be a `.rlib` or `.rmeta`. It can be used to determine which crate
  this was in terms of whatever build-system invoked rustc.

#### Trait Implementations

##### `impl Clone for ExternalCrate`

- <span id="externalcrate-clone"></span>`fn clone(&self) -> ExternalCrate` — [`ExternalCrate`](#externalcrate)

##### `impl Debug for ExternalCrate`

- <span id="externalcrate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for ExternalCrate`

- <span id="externalcrate-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for ExternalCrate`

##### `impl Eq for ExternalCrate`

##### `impl Hash for ExternalCrate`

- <span id="externalcrate-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ExternalCrate`

- <span id="externalcrate-eq"></span>`fn eq(&self, other: &ExternalCrate) -> bool` — [`ExternalCrate`](#externalcrate)

##### `impl Serialize for ExternalCrate`

- <span id="externalcrate-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for ExternalCrate`

### `ItemSummary`

```rust
struct ItemSummary {
    pub crate_id: u32,
    pub path: Vec<String>,
    pub kind: ItemKind,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:152-166`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L152-L166)*

Information about an external (not defined in the local crate) [`Item`](#item).

For external items, you don't get the same level of
information. This struct should contain enough to generate a link/reference to the item in
question, or can be used by a tool that takes the json output of multiple crates to find
the actual item definition with all the relevant info.

#### Fields

- **`crate_id`**: `u32`

  Can be used to look up the name and html_root_url of the crate this item came from in the
  `external_crates` map.

- **`path`**: `Vec<String>`

  The list of path components for the fully qualified path of this item (e.g.
  `["std", "io", "lazy", "Lazy"]` for `std::io::lazy::Lazy`).
  
  Note that items can appear in multiple paths, and the one chosen is implementation
  defined. Currently, this is the full path to where the item was defined. Eg
  [`String`](../cargo_platform/index.md) is currently `["alloc", "string", "String"]` and [`HashMap`][`std::collections::HashMap`](../hashbrown/hash_map/index.md)
  is `["std", "collections", "hash", "map", "HashMap"]`, but this is subject to change.

- **`kind`**: `ItemKind`

  Whether this item is a struct, trait, macro, etc.

#### Trait Implementations

##### `impl Clone for ItemSummary`

- <span id="itemsummary-clone"></span>`fn clone(&self) -> ItemSummary` — [`ItemSummary`](#itemsummary)

##### `impl Debug for ItemSummary`

- <span id="itemsummary-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for ItemSummary`

- <span id="itemsummary-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for ItemSummary`

##### `impl Eq for ItemSummary`

##### `impl Hash for ItemSummary`

- <span id="itemsummary-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ItemSummary`

- <span id="itemsummary-eq"></span>`fn eq(&self, other: &ItemSummary) -> bool` — [`ItemSummary`](#itemsummary)

##### `impl Serialize for ItemSummary`

- <span id="itemsummary-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for ItemSummary`

### `Item`

```rust
struct Item {
    pub id: Id,
    pub crate_id: u32,
    pub name: Option<String>,
    pub span: Option<Span>,
    pub visibility: Visibility,
    pub docs: Option<String>,
    pub links: std::collections::HashMap<String, Id>,
    pub attrs: Vec<Attribute>,
    pub deprecation: Option<Deprecation>,
    pub inner: ItemEnum,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:173-208`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L173-L208)*

Anything that can hold documentation - modules, structs, enums, functions, traits, etc.

The `Item` data type holds fields that can apply to any of these,
and leaves kind-specific details (like function args or enum variants) to the `inner` field.

#### Fields

- **`id`**: `Id`

  The unique identifier of this item. Can be used to find this item in various mappings.

- **`crate_id`**: `u32`

  This can be used as a key to the `external_crates` map of [`Crate`](#crate) to see which crate
  this item came from.

- **`name`**: `Option<String>`

  Some items such as impls don't have names.

- **`span`**: `Option<Span>`

  The source location of this item (absent if it came from a macro expansion or inline
  assembly).

- **`visibility`**: `Visibility`

  By default all documented items are public, but you can tell rustdoc to output private items
  so this field is needed to differentiate.

- **`docs`**: `Option<String>`

  The full markdown docstring of this item. Absent if there is no documentation at all,
  Some("") if there is some documentation but it is empty (EG `#[doc = ""]`).

- **`links`**: `std::collections::HashMap<String, Id>`

  This mapping resolves [intra-doc links](https://github.com/rust-lang/rfcs/blob/master/text/1946-intra-rustdoc-links.md) from the docstring to their IDs

- **`attrs`**: `Vec<Attribute>`

  Attributes on this item.
  
  Does not include `#[deprecated]` attributes: see the `Self::deprecation` field instead.
  
  Attributes appear in pretty-printed Rust form, regardless of their formatting
  in the original source code. For example:
  - `#[non_exhaustive]` and `#[must_use]` are represented as themselves.
  - `#[no_mangle]` and `#[export_name]` are also represented as themselves.
  - `#[repr(C)]` and other reprs also appear as themselves,
    though potentially with a different order: e.g. `repr(i8, C)` may become `repr(C, i8)`.
    Multiple repr attributes on the same item may be combined into an equivalent single attr.

- **`deprecation`**: `Option<Deprecation>`

  Information about the item’s deprecation, if present.

- **`inner`**: `ItemEnum`

  The type-specific fields describing this item.

#### Trait Implementations

##### `impl Clone for Item`

- <span id="item-clone"></span>`fn clone(&self) -> Item` — [`Item`](#item)

##### `impl Debug for Item`

- <span id="item-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Item`

- <span id="item-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Item`

##### `impl Eq for Item`

##### `impl PartialEq for Item`

- <span id="item-eq"></span>`fn eq(&self, other: &Item) -> bool` — [`Item`](#item)

##### `impl Serialize for Item`

- <span id="item-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Item`

### `AttributeRepr`

```rust
struct AttributeRepr {
    pub kind: ReprKind,
    pub align: Option<u64>,
    pub packed: Option<u64>,
    pub int: Option<String>,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:261-274`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L261-L274)*

The contents of a `#[repr(...)]` attribute.

Used in [`Attribute::Repr`](#attributerepr).

#### Fields

- **`kind`**: `ReprKind`

  The representation, e.g. `#[repr(C)]`, `#[repr(transparent)]`

- **`align`**: `Option<u64>`

  Alignment in bytes, if explicitly specified by `#[repr(align(...)]`.

- **`packed`**: `Option<u64>`

  Alignment in bytes, if explicitly specified by `#[repr(packed(...)]]`.

- **`int`**: `Option<String>`

  The integer type for an enum descriminant, if explicitly specified.
  
  e.g. `"i32"`, for `#[repr(C, i32)]`

#### Trait Implementations

##### `impl Clone for AttributeRepr`

- <span id="attributerepr-clone"></span>`fn clone(&self) -> AttributeRepr` — [`AttributeRepr`](#attributerepr)

##### `impl Debug for AttributeRepr`

- <span id="attributerepr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for AttributeRepr`

- <span id="attributerepr-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for AttributeRepr`

##### `impl Eq for AttributeRepr`

##### `impl PartialEq for AttributeRepr`

- <span id="attributerepr-eq"></span>`fn eq(&self, other: &AttributeRepr) -> bool` — [`AttributeRepr`](#attributerepr)

##### `impl Serialize for AttributeRepr`

- <span id="attributerepr-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for AttributeRepr`

### `Span`

```rust
struct Span {
    pub filename: std::path::PathBuf,
    pub begin: (usize, usize),
    pub end: (usize, usize),
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:296-303`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L296-L303)*

A range of source code.

#### Fields

- **`filename`**: `std::path::PathBuf`

  The path to the source file for this span relative to the path `rustdoc` was invoked with.

- **`begin`**: `(usize, usize)`

  One indexed Line and Column of the first character of the `Span`.

- **`end`**: `(usize, usize)`

  One indexed Line and Column of the last character of the `Span`.

#### Trait Implementations

##### `impl Clone for Span`

- <span id="span-clone"></span>`fn clone(&self) -> Span` — [`Span`](#span)

##### `impl Debug for Span`

- <span id="span-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Span`

- <span id="span-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Span`

##### `impl Eq for Span`

##### `impl Hash for Span`

- <span id="span-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Span`

- <span id="span-eq"></span>`fn eq(&self, other: &Span) -> bool` — [`Span`](#span)

##### `impl Serialize for Span`

- <span id="span-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Span`

### `Deprecation`

```rust
struct Deprecation {
    pub since: Option<String>,
    pub note: Option<String>,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:307-312`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L307-L312)*

Information about the deprecation of an [`Item`](#item).

#### Fields

- **`since`**: `Option<String>`

  Usually a version number when this [`Item`](#item) first became deprecated.

- **`note`**: `Option<String>`

  The reason for deprecation and/or what alternatives to use.

#### Trait Implementations

##### `impl Clone for Deprecation`

- <span id="deprecation-clone"></span>`fn clone(&self) -> Deprecation` — [`Deprecation`](#deprecation)

##### `impl Debug for Deprecation`

- <span id="deprecation-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Deprecation`

- <span id="deprecation-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Deprecation`

##### `impl Eq for Deprecation`

##### `impl Hash for Deprecation`

- <span id="deprecation-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Deprecation`

- <span id="deprecation-eq"></span>`fn eq(&self, other: &Deprecation) -> bool` — [`Deprecation`](#deprecation)

##### `impl Serialize for Deprecation`

- <span id="deprecation-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Deprecation`

### `DynTrait`

```rust
struct DynTrait {
    pub traits: Vec<PolyTrait>,
    pub lifetime: Option<String>,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:339-350`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L339-L350)*

Dynamic trait object type (`dyn Trait`).

#### Fields

- **`traits`**: `Vec<PolyTrait>`

  All the traits implemented. One of them is the vtable, and the rest must be auto traits.

- **`lifetime`**: `Option<String>`

  The lifetime of the whole dyn object
  ```text
  dyn Debug + 'static
              ^^^^^^^
              |
              this part
  ```

#### Trait Implementations

##### `impl Clone for DynTrait`

- <span id="dyntrait-clone"></span>`fn clone(&self) -> DynTrait` — [`DynTrait`](#dyntrait)

##### `impl Debug for DynTrait`

- <span id="dyntrait-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for DynTrait`

- <span id="dyntrait-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for DynTrait`

##### `impl Eq for DynTrait`

##### `impl Hash for DynTrait`

- <span id="dyntrait-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for DynTrait`

- <span id="dyntrait-eq"></span>`fn eq(&self, other: &DynTrait) -> bool` — [`DynTrait`](#dyntrait)

##### `impl Serialize for DynTrait`

- <span id="dyntrait-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for DynTrait`

### `PolyTrait`

```rust
struct PolyTrait {
    pub trait_: Path,
    pub generic_params: Vec<GenericParamDef>,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:354-364`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L354-L364)*

A trait and potential HRTBs

#### Fields

- **`trait_`**: `Path`

  The path to the trait.

- **`generic_params`**: `Vec<GenericParamDef>`

  Used for Higher-Rank Trait Bounds (HRTBs)
  ```text
  dyn for<'a> Fn() -> &'a i32"
      ^^^^^^^
  ```

#### Trait Implementations

##### `impl Clone for PolyTrait`

- <span id="polytrait-clone"></span>`fn clone(&self) -> PolyTrait` — [`PolyTrait`](#polytrait)

##### `impl Debug for PolyTrait`

- <span id="polytrait-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for PolyTrait`

- <span id="polytrait-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for PolyTrait`

##### `impl Eq for PolyTrait`

##### `impl Hash for PolyTrait`

- <span id="polytrait-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for PolyTrait`

- <span id="polytrait-eq"></span>`fn eq(&self, other: &PolyTrait) -> bool` — [`PolyTrait`](#polytrait)

##### `impl Serialize for PolyTrait`

- <span id="polytrait-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for PolyTrait`

### `Constant`

```rust
struct Constant {
    pub expr: String,
    pub value: Option<String>,
    pub is_literal: bool,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:431-440`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L431-L440)*

A constant.

#### Fields

- **`expr`**: `String`

  The stringified expression of this constant. Note that its mapping to the original
  source code is unstable and it's not guaranteed that it'll match the source code.

- **`value`**: `Option<String>`

  The value of the evaluated expression for this constant, which is only computed for numeric
  types.

- **`is_literal`**: `bool`

  Whether this constant is a bool, numeric, string, or char literal.

#### Trait Implementations

##### `impl Clone for Constant`

- <span id="constant-clone"></span>`fn clone(&self) -> Constant` — [`Constant`](#constant)

##### `impl Debug for Constant`

- <span id="constant-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Constant`

- <span id="constant-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Constant`

##### `impl Eq for Constant`

##### `impl Hash for Constant`

- <span id="constant-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Constant`

- <span id="constant-eq"></span>`fn eq(&self, other: &Constant) -> bool` — [`Constant`](#constant)

##### `impl Serialize for Constant`

- <span id="constant-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Constant`

### `AssocItemConstraint`

```rust
struct AssocItemConstraint {
    pub name: String,
    pub args: Option<Box<GenericArgs>>,
    pub binding: AssocItemConstraintKind,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:450-457`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L450-L457)*

Describes a bound applied to an associated type/constant.

Example:
```text
IntoIterator<Item = u32, IntoIter: Clone>
             ^^^^^^^^^^  ^^^^^^^^^^^^^^^
```

#### Fields

- **`name`**: `String`

  The name of the associated type/constant.

- **`args`**: `Option<Box<GenericArgs>>`

  Arguments provided to the associated type/constant.

- **`binding`**: `AssocItemConstraintKind`

  The kind of bound applied to the associated type/constant.

#### Trait Implementations

##### `impl Clone for AssocItemConstraint`

- <span id="associtemconstraint-clone"></span>`fn clone(&self) -> AssocItemConstraint` — [`AssocItemConstraint`](#associtemconstraint)

##### `impl Debug for AssocItemConstraint`

- <span id="associtemconstraint-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for AssocItemConstraint`

- <span id="associtemconstraint-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for AssocItemConstraint`

##### `impl Eq for AssocItemConstraint`

##### `impl Hash for AssocItemConstraint`

- <span id="associtemconstraint-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for AssocItemConstraint`

- <span id="associtemconstraint-eq"></span>`fn eq(&self, other: &AssocItemConstraint) -> bool` — [`AssocItemConstraint`](#associtemconstraint)

##### `impl Serialize for AssocItemConstraint`

- <span id="associtemconstraint-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for AssocItemConstraint`

### `Id`

```rust
struct Id(u32);
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:490`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L490)*

An opaque identifier for an item.

It can be used to lookup in `Crate::index` or `Crate::paths` to resolve it
to an [`Item`](#item).

Id's are only valid within a single JSON blob. They cannot be used to
resolve references between the JSON output's for different crates.

Rustdoc makes no guarantees about the inner value of Id's. Applications
should treat them as opaque keys to lookup items, and avoid attempting
to parse them, or otherwise depend on any implementation details.

#### Trait Implementations

##### `impl Clone for Id`

- <span id="id-clone"></span>`fn clone(&self) -> Id` — [`Id`](#id)

##### `impl Copy for Id`

##### `impl Debug for Id`

- <span id="id-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Id`

- <span id="id-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Id`

##### `impl Eq for Id`

##### `impl Hash for Id`

- <span id="id-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Id`

- <span id="id-cmp"></span>`fn cmp(&self, other: &Id) -> cmp::Ordering` — [`Id`](#id)

##### `impl PartialEq for Id`

- <span id="id-eq"></span>`fn eq(&self, other: &Id) -> bool` — [`Id`](#id)

##### `impl PartialOrd for Id`

- <span id="id-partial-cmp"></span>`fn partial_cmp(&self, other: &Id) -> option::Option<cmp::Ordering>` — [`Id`](#id)

##### `impl Serialize for Id`

- <span id="id-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Id`

### `Module`

```rust
struct Module {
    pub is_crate: bool,
    pub items: Vec<Id>,
    pub is_stripped: bool,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:686-697`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L686-L697)*

A module declaration, e.g. `mod foo;` or `mod foo {}`.

#### Fields

- **`is_crate`**: `bool`

  Whether this is the root item of a crate.
  
  This item doesn't correspond to any construction in the source code and is generated by the
  compiler.

- **`items`**: `Vec<Id>`

  [`Item`](#item)s declared inside this module.

- **`is_stripped`**: `bool`

  If `true`, this module is not part of the public API, but it contains
  items that are re-exported as public API.

#### Trait Implementations

##### `impl Clone for Module`

- <span id="module-clone"></span>`fn clone(&self) -> Module` — [`Module`](#module)

##### `impl Debug for Module`

- <span id="module-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Module`

- <span id="module-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Module`

##### `impl Eq for Module`

##### `impl Hash for Module`

- <span id="module-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Module`

- <span id="module-eq"></span>`fn eq(&self, other: &Module) -> bool` — [`Module`](#module)

##### `impl Serialize for Module`

- <span id="module-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Module`

### `Union`

```rust
struct Union {
    pub generics: Generics,
    pub has_stripped_fields: bool,
    pub fields: Vec<Id>,
    pub impls: Vec<Id>,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:701-714`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L701-L714)*

A `union`.

#### Fields

- **`generics`**: `Generics`

  The generic parameters and where clauses on this union.

- **`has_stripped_fields`**: `bool`

  Whether any fields have been removed from the result, due to being private or hidden.

- **`fields`**: `Vec<Id>`

  The list of fields in the union.
  
  All of the corresponding [`Item`](#item)s are of kind [`ItemEnum::StructField`](#itemenumstructfield).

- **`impls`**: `Vec<Id>`

  All impls (both of traits and inherent) for this union.
  
  All of the corresponding [`Item`](#item)s are of kind [`ItemEnum::Impl`](#itemenumimpl).

#### Trait Implementations

##### `impl Clone for Union`

- <span id="union-clone"></span>`fn clone(&self) -> Union` — [`Union`](#union)

##### `impl Debug for Union`

- <span id="union-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Union`

- <span id="union-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Union`

##### `impl Eq for Union`

##### `impl Hash for Union`

- <span id="union-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Union`

- <span id="union-eq"></span>`fn eq(&self, other: &Union) -> bool` — [`Union`](#union)

##### `impl Serialize for Union`

- <span id="union-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Union`

### `Struct`

```rust
struct Struct {
    pub kind: StructKind,
    pub generics: Generics,
    pub impls: Vec<Id>,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:718-727`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L718-L727)*

A `struct`.

#### Fields

- **`kind`**: `StructKind`

  The kind of the struct (e.g. unit, tuple-like or struct-like) and the data specific to it,
  i.e. fields.

- **`generics`**: `Generics`

  The generic parameters and where clauses on this struct.

- **`impls`**: `Vec<Id>`

  All impls (both of traits and inherent) for this struct.
  All of the corresponding [`Item`](#item)s are of kind [`ItemEnum::Impl`](#itemenumimpl).

#### Trait Implementations

##### `impl Clone for Struct`

- <span id="struct-clone"></span>`fn clone(&self) -> Struct` — [`Struct`](#struct)

##### `impl Debug for Struct`

- <span id="struct-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Struct`

- <span id="struct-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Struct`

##### `impl Eq for Struct`

##### `impl Hash for Struct`

- <span id="struct-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Struct`

- <span id="struct-eq"></span>`fn eq(&self, other: &Struct) -> bool` — [`Struct`](#struct)

##### `impl Serialize for Struct`

- <span id="struct-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Struct`

### `Enum`

```rust
struct Enum {
    pub generics: Generics,
    pub has_stripped_variants: bool,
    pub variants: Vec<Id>,
    pub impls: Vec<Id>,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:768-779`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L768-L779)*

An `enum`.

#### Fields

- **`generics`**: `Generics`

  Information about the type parameters and `where` clauses of the enum.

- **`has_stripped_variants`**: `bool`

  Whether any variants have been removed from the result, due to being private or hidden.

- **`variants`**: `Vec<Id>`

  The list of variants in the enum.
  
  All of the corresponding [`Item`](#item)s are of kind [`ItemEnum::Variant`](#itemenumvariant)

- **`impls`**: `Vec<Id>`

  `impl`s for the enum.

#### Trait Implementations

##### `impl Clone for Enum`

- <span id="enum-clone"></span>`fn clone(&self) -> Enum` — [`Enum`](#enum)

##### `impl Debug for Enum`

- <span id="enum-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Enum`

- <span id="enum-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Enum`

##### `impl Eq for Enum`

##### `impl Hash for Enum`

- <span id="enum-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Enum`

- <span id="enum-eq"></span>`fn eq(&self, other: &Enum) -> bool` — [`Enum`](#enum)

##### `impl Serialize for Enum`

- <span id="enum-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Enum`

### `Variant`

```rust
struct Variant {
    pub kind: VariantKind,
    pub discriminant: Option<Discriminant>,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:783-788`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L783-L788)*

A variant of an enum.

#### Fields

- **`kind`**: `VariantKind`

  Whether the variant is plain, a tuple-like, or struct-like. Contains the fields.

- **`discriminant`**: `Option<Discriminant>`

  The discriminant, if explicitly specified.

#### Trait Implementations

##### `impl Clone for Variant`

- <span id="variant-clone"></span>`fn clone(&self) -> Variant` — [`Variant`](#variant)

##### `impl Debug for Variant`

- <span id="variant-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Variant`

- <span id="variant-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Variant`

##### `impl Eq for Variant`

##### `impl Hash for Variant`

- <span id="variant-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Variant`

- <span id="variant-eq"></span>`fn eq(&self, other: &Variant) -> bool` — [`Variant`](#variant)

##### `impl Serialize for Variant`

- <span id="variant-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Variant`

### `Discriminant`

```rust
struct Discriminant {
    pub expr: String,
    pub value: String,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:835-849`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L835-L849)*

The value that distinguishes a variant in an [`Enum`](#enum) from other variants.

#### Fields

- **`expr`**: `String`

  The expression that produced the discriminant.
  
  Unlike `value`, this preserves the original formatting (eg suffixes,
  hexadecimal, and underscores), making it unsuitable to be machine
  interpreted.
  
  In some cases, when the value is too complex, this may be `"{ _ }"`.
  When this occurs is unstable, and may change without notice.

- **`value`**: `String`

  The numerical value of the discriminant. Stored as a string due to
  JSON's poor support for large integers, and the fact that it would need
  to store from `i128::MIN` to `u128::MAX`.

#### Trait Implementations

##### `impl Clone for Discriminant`

- <span id="discriminant-clone"></span>`fn clone(&self) -> Discriminant` — [`Discriminant`](#discriminant)

##### `impl Debug for Discriminant`

- <span id="discriminant-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Discriminant`

- <span id="discriminant-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Discriminant`

##### `impl Eq for Discriminant`

##### `impl Hash for Discriminant`

- <span id="discriminant-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Discriminant`

- <span id="discriminant-eq"></span>`fn eq(&self, other: &Discriminant) -> bool` — [`Discriminant`](#discriminant)

##### `impl Serialize for Discriminant`

- <span id="discriminant-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Discriminant`

### `FunctionHeader`

```rust
struct FunctionHeader {
    pub is_const: bool,
    pub is_unsafe: bool,
    pub is_async: bool,
    pub abi: Abi,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:853-862`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L853-L862)*

A set of fundamental properties of a function.

#### Fields

- **`is_const`**: `bool`

  Is this function marked as `const`?

- **`is_unsafe`**: `bool`

  Is this function unsafe?

- **`is_async`**: `bool`

  Is this function async?

- **`abi`**: `Abi`

  The ABI used by the function.

#### Trait Implementations

##### `impl Clone for FunctionHeader`

- <span id="functionheader-clone"></span>`fn clone(&self) -> FunctionHeader` — [`FunctionHeader`](#functionheader)

##### `impl Debug for FunctionHeader`

- <span id="functionheader-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for FunctionHeader`

- <span id="functionheader-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for FunctionHeader`

##### `impl Eq for FunctionHeader`

##### `impl Hash for FunctionHeader`

- <span id="functionheader-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for FunctionHeader`

- <span id="functionheader-eq"></span>`fn eq(&self, other: &FunctionHeader) -> bool` — [`FunctionHeader`](#functionheader)

##### `impl Serialize for FunctionHeader`

- <span id="functionheader-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for FunctionHeader`

### `Function`

```rust
struct Function {
    pub sig: FunctionSignature,
    pub generics: Generics,
    pub header: FunctionHeader,
    pub has_body: bool,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:900-909`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L900-L909)*

A function declaration (including methods and other associated functions).

#### Fields

- **`sig`**: `FunctionSignature`

  Information about the function signature, or declaration.

- **`generics`**: `Generics`

  Information about the function’s type parameters and `where` clauses.

- **`header`**: `FunctionHeader`

  Information about core properties of the function, e.g. whether it's `const`, its ABI, etc.

- **`has_body`**: `bool`

  Whether the function has a body, i.e. an implementation.

#### Trait Implementations

##### `impl Clone for Function`

- <span id="function-clone"></span>`fn clone(&self) -> Function` — [`Function`](#function)

##### `impl Debug for Function`

- <span id="function-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Function`

- <span id="function-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Function`

##### `impl Eq for Function`

##### `impl Hash for Function`

- <span id="function-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Function`

- <span id="function-eq"></span>`fn eq(&self, other: &Function) -> bool` — [`Function`](#function)

##### `impl Serialize for Function`

- <span id="function-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Function`

### `Generics`

```rust
struct Generics {
    pub params: Vec<GenericParamDef>,
    pub where_predicates: Vec<WherePredicate>,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:913-918`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L913-L918)*

Generic parameters accepted by an item and `where` clauses imposed on it and the parameters.

#### Fields

- **`params`**: `Vec<GenericParamDef>`

  A list of generic parameter definitions (e.g. `<T: Clone + Hash, U: Copy>`).

- **`where_predicates`**: `Vec<WherePredicate>`

  A list of where predicates (e.g. `where T: Iterator, T::Item: Copy`).

#### Trait Implementations

##### `impl Clone for Generics`

- <span id="generics-clone"></span>`fn clone(&self) -> Generics` — [`Generics`](#generics)

##### `impl Debug for Generics`

- <span id="generics-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Generics`

- <span id="generics-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Generics`

##### `impl Eq for Generics`

##### `impl Hash for Generics`

- <span id="generics-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Generics`

- <span id="generics-eq"></span>`fn eq(&self, other: &Generics) -> bool` — [`Generics`](#generics)

##### `impl Serialize for Generics`

- <span id="generics-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Generics`

### `GenericParamDef`

```rust
struct GenericParamDef {
    pub name: String,
    pub kind: GenericParamDefKind,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:922-932`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L922-L932)*

One generic parameter accepted by an item.

#### Fields

- **`name`**: `String`

  Name of the parameter.
  ```rust
  fn f<'resource, Resource>(x: &'resource Resource) {}
  //    ^^^^^^^^  ^^^^^^^^
  ```

- **`kind`**: `GenericParamDefKind`

  The kind of the parameter and data specific to a particular parameter kind, e.g. type
  bounds.

#### Trait Implementations

##### `impl Clone for GenericParamDef`

- <span id="genericparamdef-clone"></span>`fn clone(&self) -> GenericParamDef` — [`GenericParamDef`](#genericparamdef)

##### `impl Debug for GenericParamDef`

- <span id="genericparamdef-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for GenericParamDef`

- <span id="genericparamdef-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for GenericParamDef`

##### `impl Eq for GenericParamDef`

##### `impl Hash for GenericParamDef`

- <span id="genericparamdef-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for GenericParamDef`

- <span id="genericparamdef-eq"></span>`fn eq(&self, other: &GenericParamDef) -> bool` — [`GenericParamDef`](#genericparamdef)

##### `impl Serialize for GenericParamDef`

- <span id="genericparamdef-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for GenericParamDef`

### `Path`

```rust
struct Path {
    pub path: String,
    pub id: Id,
    pub args: Option<Box<GenericArgs>>,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:1231-1255`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L1231-L1255)*

A type that has a simple path to it. This is the kind of type of structs, unions, enums, etc.

#### Fields

- **`path`**: `String`

  The path of the type.
  
  This will be the path that is *used* (not where it is defined), so
  multiple `Path`s may have different values for this field even if
  they all refer to the same item. e.g.
  
  ```rust
  pub type Vec1 = std::vec::Vec<i32>; // path: "std::vec::Vec"
  pub type Vec2 = Vec<i32>; // path: "Vec"
  pub type Vec3 = std::prelude::v1::Vec<i32>; // path: "std::prelude::v1::Vec"
  ```

- **`id`**: `Id`

  The ID of the type.

- **`args`**: `Option<Box<GenericArgs>>`

  Generic arguments to the type.
  
  ```ignore (incomplete expression)
  std::borrow::Cow<'static, str>
  //              ^^^^^^^^^^^^^^
  ```

#### Trait Implementations

##### `impl Clone for Path`

- <span id="path-clone"></span>`fn clone(&self) -> Path` — [`Path`](#path)

##### `impl Debug for Path`

- <span id="path-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Path`

- <span id="path-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Path`

##### `impl Eq for Path`

##### `impl Hash for Path`

- <span id="path-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Path`

- <span id="path-eq"></span>`fn eq(&self, other: &Path) -> bool` — [`Path`](#path)

##### `impl Serialize for Path`

- <span id="path-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Path`

### `FunctionPointer`

```rust
struct FunctionPointer {
    pub sig: FunctionSignature,
    pub generic_params: Vec<GenericParamDef>,
    pub header: FunctionHeader,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:1259-1271`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L1259-L1271)*

A type that is a function pointer.

#### Fields

- **`sig`**: `FunctionSignature`

  The signature of the function.

- **`generic_params`**: `Vec<GenericParamDef>`

  Used for Higher-Rank Trait Bounds (HRTBs)
  
  ```ignore (incomplete expression)
     for<'c> fn(val: &'c i32) -> i32
  // ^^^^^^^
  ```

- **`header`**: `FunctionHeader`

  The core properties of the function, such as the ABI it conforms to, whether it's unsafe, etc.

#### Trait Implementations

##### `impl Clone for FunctionPointer`

- <span id="functionpointer-clone"></span>`fn clone(&self) -> FunctionPointer` — [`FunctionPointer`](#functionpointer)

##### `impl Debug for FunctionPointer`

- <span id="functionpointer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for FunctionPointer`

- <span id="functionpointer-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for FunctionPointer`

##### `impl Eq for FunctionPointer`

##### `impl Hash for FunctionPointer`

- <span id="functionpointer-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for FunctionPointer`

- <span id="functionpointer-eq"></span>`fn eq(&self, other: &FunctionPointer) -> bool` — [`FunctionPointer`](#functionpointer)

##### `impl Serialize for FunctionPointer`

- <span id="functionpointer-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for FunctionPointer`

### `FunctionSignature`

```rust
struct FunctionSignature {
    pub inputs: Vec<(String, Type)>,
    pub output: Option<Type>,
    pub is_c_variadic: bool,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:1275-1289`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L1275-L1289)*

The signature of a function.

#### Fields

- **`inputs`**: `Vec<(String, Type)>`

  List of argument names and their type.
  
  Note that not all names will be valid identifiers, as some of
  them may be patterns.

- **`output`**: `Option<Type>`

  The output type, if specified.

- **`is_c_variadic`**: `bool`

  Whether the function accepts an arbitrary amount of trailing arguments the C way.
  
  ```ignore (incomplete code)
  fn printf(fmt: &str, ...);
  ```

#### Trait Implementations

##### `impl Clone for FunctionSignature`

- <span id="functionsignature-clone"></span>`fn clone(&self) -> FunctionSignature` — [`FunctionSignature`](#functionsignature)

##### `impl Debug for FunctionSignature`

- <span id="functionsignature-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for FunctionSignature`

- <span id="functionsignature-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for FunctionSignature`

##### `impl Eq for FunctionSignature`

##### `impl Hash for FunctionSignature`

- <span id="functionsignature-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for FunctionSignature`

- <span id="functionsignature-eq"></span>`fn eq(&self, other: &FunctionSignature) -> bool` — [`FunctionSignature`](#functionsignature)

##### `impl Serialize for FunctionSignature`

- <span id="functionsignature-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for FunctionSignature`

### `Trait`

```rust
struct Trait {
    pub is_auto: bool,
    pub is_unsafe: bool,
    pub is_dyn_compatible: bool,
    pub items: Vec<Id>,
    pub generics: Generics,
    pub bounds: Vec<GenericBound>,
    pub implementations: Vec<Id>,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:1293-1311`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L1293-L1311)*

A `trait` declaration.

#### Fields

- **`is_auto`**: `bool`

  Whether the trait is marked `auto` and is thus implemented automatically
  for all applicable types.

- **`is_unsafe`**: `bool`

  Whether the trait is marked as `unsafe`.

- **`is_dyn_compatible`**: `bool`

  Whether the trait is [dyn compatible](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility)[^1].
  
  [^1]: Formerly known as "object safe".

- **`items`**: `Vec<Id>`

  Associated [`Item`](#item)s that can/must be implemented by the `impl` blocks.

- **`generics`**: `Generics`

  Information about the type parameters and `where` clauses of the trait.

- **`bounds`**: `Vec<GenericBound>`

  Constraints that must be met by the implementor of the trait.

- **`implementations`**: `Vec<Id>`

  The implementations of the trait.

#### Trait Implementations

##### `impl Clone for Trait`

- <span id="trait-clone"></span>`fn clone(&self) -> Trait` — [`Trait`](#trait)

##### `impl Debug for Trait`

- <span id="trait-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Trait`

- <span id="trait-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Trait`

##### `impl Eq for Trait`

##### `impl Hash for Trait`

- <span id="trait-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Trait`

- <span id="trait-eq"></span>`fn eq(&self, other: &Trait) -> bool` — [`Trait`](#trait)

##### `impl Serialize for Trait`

- <span id="trait-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Trait`

### `TraitAlias`

```rust
struct TraitAlias {
    pub generics: Generics,
    pub params: Vec<GenericBound>,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:1317-1322`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L1317-L1322)*

A trait alias declaration, e.g. `trait Int = Add + Sub + Mul + Div;`

See [the tracking issue](https://github.com/rust-lang/rust/issues/41517)

#### Fields

- **`generics`**: `Generics`

  Information about the type parameters and `where` clauses of the alias.

- **`params`**: `Vec<GenericBound>`

  The bounds that are associated with the alias.

#### Trait Implementations

##### `impl Clone for TraitAlias`

- <span id="traitalias-clone"></span>`fn clone(&self) -> TraitAlias` — [`TraitAlias`](#traitalias)

##### `impl Debug for TraitAlias`

- <span id="traitalias-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for TraitAlias`

- <span id="traitalias-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for TraitAlias`

##### `impl Eq for TraitAlias`

##### `impl Hash for TraitAlias`

- <span id="traitalias-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for TraitAlias`

- <span id="traitalias-eq"></span>`fn eq(&self, other: &TraitAlias) -> bool` — [`TraitAlias`](#traitalias)

##### `impl Serialize for TraitAlias`

- <span id="traitalias-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for TraitAlias`

### `Impl`

```rust
struct Impl {
    pub is_unsafe: bool,
    pub generics: Generics,
    pub provided_trait_methods: Vec<String>,
    pub trait_: Option<Path>,
    pub for_: Type,
    pub items: Vec<Id>,
    pub is_negative: bool,
    pub is_synthetic: bool,
    pub blanket_impl: Option<Type>,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:1326-1360`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L1326-L1360)*

An `impl` block.

#### Fields

- **`is_unsafe`**: `bool`

  Whether this impl is for an unsafe trait.

- **`generics`**: `Generics`

  Information about the impl’s type parameters and `where` clauses.

- **`provided_trait_methods`**: `Vec<String>`

  The list of the names of all the trait methods that weren't mentioned in this impl but
  were provided by the trait itself.
  
  For example, for this impl of the `PartialEq` trait:
  ```rust
  struct Foo;
  
  impl PartialEq for Foo {
      fn eq(&self, other: &Self) -> bool { todo!() }
  }
  ```
  This field will be `["ne"]`, as it has a default implementation defined for it.

- **`trait_`**: `Option<Path>`

  The trait being implemented or `None` if the impl is inherent, which means
  `impl Struct {}` as opposed to `impl Trait for Struct {}`.

- **`for_`**: `Type`

  The type that the impl block is for.

- **`items`**: `Vec<Id>`

  The list of associated items contained in this impl block.

- **`is_negative`**: `bool`

  Whether this is a negative impl (e.g. `!Sized` or `!Send`).

- **`is_synthetic`**: `bool`

  Whether this is an impl that’s implied by the compiler
  (for autotraits, e.g. `Send` or `Sync`).

#### Trait Implementations

##### `impl Clone for Impl`

- <span id="impl-clone"></span>`fn clone(&self) -> Impl` — [`Impl`](#impl)

##### `impl Debug for Impl`

- <span id="impl-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Impl`

- <span id="impl-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Impl`

##### `impl Eq for Impl`

##### `impl Hash for Impl`

- <span id="impl-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Impl`

- <span id="impl-eq"></span>`fn eq(&self, other: &Impl) -> bool` — [`Impl`](#impl)

##### `impl Serialize for Impl`

- <span id="impl-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Impl`

### `Use`

```rust
struct Use {
    pub source: String,
    pub name: String,
    pub id: Option<Id>,
    pub is_glob: bool,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:1365-1378`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L1365-L1378)*

A `use` statement.

#### Fields

- **`source`**: `String`

  The full path being imported.

- **`name`**: `String`

  May be different from the last segment of `source` when renaming imports:
  `use source as name;`

- **`id`**: `Option<Id>`

  The ID of the item being imported. Will be `None` in case of re-exports of primitives:
  ```rust
  pub use i32 as my_i32;
  ```

- **`is_glob`**: `bool`

  Whether this statement is a wildcard `use`, e.g. `use source::*;`

#### Trait Implementations

##### `impl Clone for Use`

- <span id="use-clone"></span>`fn clone(&self) -> Use` — [`Use`](#use)

##### `impl Debug for Use`

- <span id="use-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Use`

- <span id="use-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Use`

##### `impl Eq for Use`

##### `impl Hash for Use`

- <span id="use-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Use`

- <span id="use-eq"></span>`fn eq(&self, other: &Use) -> bool` — [`Use`](#use)

##### `impl Serialize for Use`

- <span id="use-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Use`

### `ProcMacro`

```rust
struct ProcMacro {
    pub kind: MacroKind,
    pub helpers: Vec<String>,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:1382-1401`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L1382-L1401)*

A procedural macro.

#### Fields

- **`kind`**: `MacroKind`

  How this macro is supposed to be called: `foo!()`, `#[foo]` or `#[derive(foo)]`

- **`helpers`**: `Vec<String>`

  Helper attributes defined by a macro to be used inside it.
  
  Defined only for derive macros.
  
  E.g. the [`Default`](../gimli/index.md) derive macro defines a `#[default]` helper attribute so that one can
  do:
  
  ```rust
  #[derive(Default)]
  enum Option<T> {
      #[default]
      None,
      Some(T),
  }
  ```

#### Trait Implementations

##### `impl Clone for ProcMacro`

- <span id="procmacro-clone"></span>`fn clone(&self) -> ProcMacro` — [`ProcMacro`](#procmacro)

##### `impl Debug for ProcMacro`

- <span id="procmacro-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for ProcMacro`

- <span id="procmacro-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for ProcMacro`

##### `impl Eq for ProcMacro`

##### `impl Hash for ProcMacro`

- <span id="procmacro-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ProcMacro`

- <span id="procmacro-eq"></span>`fn eq(&self, other: &ProcMacro) -> bool` — [`ProcMacro`](#procmacro)

##### `impl Serialize for ProcMacro`

- <span id="procmacro-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for ProcMacro`

### `TypeAlias`

```rust
struct TypeAlias {
    pub type_: Type,
    pub generics: Generics,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:1417-1423`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L1417-L1423)*

A type alias declaration, e.g. `type Pig = std::borrow::Cow<'static, str>;`

#### Fields

- **`type_`**: `Type`

  The type referred to by this alias.

- **`generics`**: `Generics`

  Information about the type parameters and `where` clauses of the alias.

#### Trait Implementations

##### `impl Clone for TypeAlias`

- <span id="typealias-clone"></span>`fn clone(&self) -> TypeAlias` — [`TypeAlias`](#typealias)

##### `impl Debug for TypeAlias`

- <span id="typealias-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for TypeAlias`

- <span id="typealias-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for TypeAlias`

##### `impl Eq for TypeAlias`

##### `impl Hash for TypeAlias`

- <span id="typealias-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for TypeAlias`

- <span id="typealias-eq"></span>`fn eq(&self, other: &TypeAlias) -> bool` — [`TypeAlias`](#typealias)

##### `impl Serialize for TypeAlias`

- <span id="typealias-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for TypeAlias`

### `Static`

```rust
struct Static {
    pub type_: Type,
    pub is_mutable: bool,
    pub expr: String,
    pub is_unsafe: bool,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:1427-1453`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L1427-L1453)*

A `static` declaration.

#### Fields

- **`type_`**: `Type`

  The type of the static.

- **`is_mutable`**: `bool`

  This is `true` for mutable statics, declared as `static mut X: T = f();`

- **`expr`**: `String`

  The stringified expression for the initial value.
  
  It's not guaranteed that it'll match the actual source code for the initial value.

- **`is_unsafe`**: `bool`

  Is the static `unsafe`?
  
  This is only true if it's in an `extern` block, and not explicitly marked
  as `safe`.
  
  ```rust
  unsafe extern {
      static A: i32;      // unsafe
      safe static B: i32; // safe
  }
  
  static C: i32 = 0;     // safe
  static mut D: i32 = 0; // safe
  ```

#### Trait Implementations

##### `impl Clone for Static`

- <span id="static-clone"></span>`fn clone(&self) -> Static` — [`Static`](#static)

##### `impl Debug for Static`

- <span id="static-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Static`

- <span id="static-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Static`

##### `impl Eq for Static`

##### `impl Hash for Static`

- <span id="static-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Static`

- <span id="static-eq"></span>`fn eq(&self, other: &Static) -> bool` — [`Static`](#static)

##### `impl Serialize for Static`

- <span id="static-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Static`

### `Primitive`

```rust
struct Primitive {
    pub name: String,
    pub impls: Vec<Id>,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:1457-1462`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L1457-L1462)*

A primitive type declaration. Declarations of this kind can only come from the core library.

#### Fields

- **`name`**: `String`

  The name of the type.

- **`impls`**: `Vec<Id>`

  The implementations, inherent and of traits, on the primitive type.

#### Trait Implementations

##### `impl Clone for Primitive`

- <span id="primitive-clone"></span>`fn clone(&self) -> Primitive` — [`Primitive`](#primitive)

##### `impl Debug for Primitive`

- <span id="primitive-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Primitive`

- <span id="primitive-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Primitive`

##### `impl Eq for Primitive`

##### `impl Hash for Primitive`

- <span id="primitive-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Primitive`

- <span id="primitive-eq"></span>`fn eq(&self, other: &Primitive) -> bool` — [`Primitive`](#primitive)

##### `impl Serialize for Primitive`

- <span id="primitive-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Primitive`

## Enums

### `Attribute`

```rust
enum Attribute {
    NonExhaustive,
    MustUse {
        reason: Option<String>,
    },
    MacroExport,
    ExportName(String),
    LinkSection(String),
    AutomaticallyDerived,
    Repr(AttributeRepr),
    NoMangle,
    TargetFeature {
        enable: Vec<String>,
    },
    Other(String),
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:217-255`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L217-L255)*

An attribute, e.g. `#[repr(C)]`

This doesn't include:
- `#[doc = "Doc Comment"]` or `/// Doc comment`. These are in `Item::docs` instead.
- `#[deprecated]`. These are in `Item::deprecation` instead.

#### Variants

- **`NonExhaustive`**

  `#[non_exhaustive]`

- **`MustUse`**

  `#[must_use]`

- **`MacroExport`**

  `#[macro_export]`

- **`ExportName`**

  `#[export_name = "name"]`

- **`LinkSection`**

  `#[link_section = "name"]`

- **`AutomaticallyDerived`**

  `#[automatically_derived]`

- **`Repr`**

  `#[repr]`

- **`NoMangle`**

  `#[no_mangle]`

- **`TargetFeature`**

  #[target_feature(enable = "feature1", enable = "feature2")]

- **`Other`**

  Something else.
  
  Things here are explicitly *not* covered by the [`FORMAT_VERSION`](#format-version)
  constant, and may change without bumping the format version.
  
  As an implementation detail, this is currently either:
  1. A HIR debug printing, like `"#[attr = Optimize(Speed)]"`
  2. The attribute as it appears in source form, like
     `"#[optimize(speed)]"`.

#### Trait Implementations

##### `impl Clone for Attribute`

- <span id="attribute-clone"></span>`fn clone(&self) -> Attribute` — [`Attribute`](#attribute)

##### `impl Debug for Attribute`

- <span id="attribute-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Attribute`

- <span id="attribute-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Attribute`

##### `impl Eq for Attribute`

##### `impl PartialEq for Attribute`

- <span id="attribute-eq"></span>`fn eq(&self, other: &Attribute) -> bool` — [`Attribute`](#attribute)

##### `impl Serialize for Attribute`

- <span id="attribute-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Attribute`

### `ReprKind`

```rust
enum ReprKind {
    Rust,
    C,
    Transparent,
    Simd,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:281-292`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L281-L292)*

The kind of `#[repr]`.

See [AttributeRepr::kind]`.

#### Variants

- **`Rust`**

  `#[repr(Rust)]`
  
  Also the default.

- **`C`**

  `#[repr(C)]`

- **`Transparent`**

  `#[repr(transparent)]

- **`Simd`**

  `#[repr(simd)]`

#### Trait Implementations

##### `impl Clone for ReprKind`

- <span id="reprkind-clone"></span>`fn clone(&self) -> ReprKind` — [`ReprKind`](#reprkind)

##### `impl Debug for ReprKind`

- <span id="reprkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for ReprKind`

- <span id="reprkind-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for ReprKind`

##### `impl Eq for ReprKind`

##### `impl PartialEq for ReprKind`

- <span id="reprkind-eq"></span>`fn eq(&self, other: &ReprKind) -> bool` — [`ReprKind`](#reprkind)

##### `impl Serialize for ReprKind`

- <span id="reprkind-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for ReprKind`

### `Visibility`

```rust
enum Visibility {
    Public,
    Default,
    Crate,
    Restricted {
        parent: Id,
        path: String,
    },
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:317-335`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L317-L335)*

Visibility of an [`Item`](#item).

#### Variants

- **`Public`**

  Explicitly public visibility set with `pub`.

- **`Default`**

  For the most part items are private by default. The exceptions are associated items of
  public traits and variants of public enums.

- **`Crate`**

  Explicitly crate-wide visibility set with `pub(crate)`

- **`Restricted`**

  For `pub(in path)` visibility.

#### Trait Implementations

##### `impl Clone for Visibility`

- <span id="visibility-clone"></span>`fn clone(&self) -> Visibility` — [`Visibility`](#visibility)

##### `impl Debug for Visibility`

- <span id="visibility-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Visibility`

- <span id="visibility-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Visibility`

##### `impl Eq for Visibility`

##### `impl Hash for Visibility`

- <span id="visibility-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Visibility`

- <span id="visibility-eq"></span>`fn eq(&self, other: &Visibility) -> bool` — [`Visibility`](#visibility)

##### `impl Serialize for Visibility`

- <span id="visibility-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Visibility`

### `GenericArgs`

```rust
enum GenericArgs {
    AngleBracketed {
        args: Vec<GenericArg>,
        constraints: Vec<AssocItemConstraint>,
    },
    Parenthesized {
        inputs: Vec<Type>,
        output: Option<Type>,
    },
    ReturnTypeNotation,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:374-395`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L374-L395)*

A set of generic arguments provided to a path segment, e.g.

```text
std::option::Option<u32>
                   ^^^^^
```

#### Variants

- **`AngleBracketed`**

  `<'a, 32, B: Copy, C = u32>`

- **`Parenthesized`**

  `Fn(A, B) -> C`

- **`ReturnTypeNotation`**

  `T::method(..)`

#### Trait Implementations

##### `impl Clone for GenericArgs`

- <span id="genericargs-clone"></span>`fn clone(&self) -> GenericArgs` — [`GenericArgs`](#genericargs)

##### `impl Debug for GenericArgs`

- <span id="genericargs-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for GenericArgs`

- <span id="genericargs-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for GenericArgs`

##### `impl Eq for GenericArgs`

##### `impl Hash for GenericArgs`

- <span id="genericargs-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for GenericArgs`

- <span id="genericargs-eq"></span>`fn eq(&self, other: &GenericArgs) -> bool` — [`GenericArgs`](#genericargs)

##### `impl Serialize for GenericArgs`

- <span id="genericargs-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for GenericArgs`

### `GenericArg`

```rust
enum GenericArg {
    Lifetime(String),
    Type(Type),
    Const(Constant),
    Infer,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:402-427`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L402-L427)*

One argument in a list of generic arguments to a path segment.

Part of [`GenericArgs`](#genericargs).

#### Variants

- **`Lifetime`**

  A lifetime argument.
  ```text
  std::borrow::Cow<'static, str>
                   ^^^^^^^
  ```

- **`Type`**

  A type argument.
  ```text
  std::borrow::Cow<'static, str>
                            ^^^
  ```

- **`Const`**

  A constant as a generic argument.
  ```text
  core::array::IntoIter<u32, { 640 * 1024 }>
                             ^^^^^^^^^^^^^^
  ```

- **`Infer`**

  A generic argument that's explicitly set to be inferred.
  ```text
  std::vec::Vec::<_>
                  ^
  ```

#### Trait Implementations

##### `impl Clone for GenericArg`

- <span id="genericarg-clone"></span>`fn clone(&self) -> GenericArg` — [`GenericArg`](#genericarg)

##### `impl Debug for GenericArg`

- <span id="genericarg-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for GenericArg`

- <span id="genericarg-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for GenericArg`

##### `impl Eq for GenericArg`

##### `impl Hash for GenericArg`

- <span id="genericarg-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for GenericArg`

- <span id="genericarg-eq"></span>`fn eq(&self, other: &GenericArg) -> bool` — [`GenericArg`](#genericarg)

##### `impl Serialize for GenericArg`

- <span id="genericarg-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for GenericArg`

### `AssocItemConstraintKind`

```rust
enum AssocItemConstraintKind {
    Equality(Term),
    Constraint(Vec<GenericBound>),
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:462-475`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L462-L475)*

The way in which an associate type/constant is bound.

#### Variants

- **`Equality`**

  The required value/type is specified exactly. e.g.
  ```text
  Iterator<Item = u32, IntoIter: DoubleEndedIterator>
           ^^^^^^^^^^
  ```

- **`Constraint`**

  The type is required to satisfy a set of bounds.
  ```text
  Iterator<Item = u32, IntoIter: DoubleEndedIterator>
                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  ```

#### Trait Implementations

##### `impl Clone for AssocItemConstraintKind`

- <span id="associtemconstraintkind-clone"></span>`fn clone(&self) -> AssocItemConstraintKind` — [`AssocItemConstraintKind`](#associtemconstraintkind)

##### `impl Debug for AssocItemConstraintKind`

- <span id="associtemconstraintkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for AssocItemConstraintKind`

- <span id="associtemconstraintkind-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for AssocItemConstraintKind`

##### `impl Eq for AssocItemConstraintKind`

##### `impl Hash for AssocItemConstraintKind`

- <span id="associtemconstraintkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for AssocItemConstraintKind`

- <span id="associtemconstraintkind-eq"></span>`fn eq(&self, other: &AssocItemConstraintKind) -> bool` — [`AssocItemConstraintKind`](#associtemconstraintkind)

##### `impl Serialize for AssocItemConstraintKind`

- <span id="associtemconstraintkind-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for AssocItemConstraintKind`

### `ItemKind`

```rust
enum ItemKind {
    Module,
    ExternCrate,
    Use,
    Struct,
    StructField,
    Union,
    Enum,
    Variant,
    Function,
    TypeAlias,
    Constant,
    Trait,
    TraitAlias,
    Impl,
    Static,
    ExternType,
    Macro,
    ProcAttribute,
    ProcDerive,
    AssocConst,
    AssocType,
    Primitive,
    Keyword,
    Attribute,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:497-565`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L497-L565)*

The fundamental kind of an item. Unlike [`ItemEnum`](#itemenum), this does not carry any additional info.

Part of [`ItemSummary`](#itemsummary).

#### Variants

- **`Module`**

  A module declaration, e.g. `mod foo;` or `mod foo {}`

- **`ExternCrate`**

  A crate imported via the `extern crate` syntax.

- **`Use`**

  An import of 1 or more items into scope, using the `use` keyword.

- **`Struct`**

  A `struct` declaration.

- **`StructField`**

  A field of a struct.

- **`Union`**

  A `union` declaration.

- **`Enum`**

  An `enum` declaration.

- **`Variant`**

  A variant of a enum.

- **`Function`**

  A function declaration, e.g. `fn f() {}`

- **`TypeAlias`**

  A type alias declaration, e.g. `type Pig = std::borrow::Cow<'static, str>;`

- **`Constant`**

  The declaration of a constant, e.g. `const GREETING: &str = "Hi :3";`

- **`Trait`**

  A `trait` declaration.

- **`TraitAlias`**

  A trait alias declaration, e.g. `trait Int = Add + Sub + Mul + Div;`
  
  See [the tracking issue](https://github.com/rust-lang/rust/issues/41517)

- **`Impl`**

  An `impl` block.

- **`Static`**

  A `static` declaration.

- **`ExternType`**

  `type`s from an `extern` block.
  
  See [the tracking issue](https://github.com/rust-lang/rust/issues/43467)

- **`Macro`**

  A macro declaration.
  
  Corresponds to either `ItemEnum::Macro(_)`
  or `ItemEnum::ProcMacro(ProcMacro { kind: MacroKind::Bang })`

- **`ProcAttribute`**

  A procedural macro attribute.
  
  Corresponds to `ItemEnum::ProcMacro(ProcMacro { kind: MacroKind::Attr })`

- **`ProcDerive`**

  A procedural macro usable in the `#[derive()]` attribute.
  
  Corresponds to `ItemEnum::ProcMacro(ProcMacro { kind: MacroKind::Derive })`

- **`AssocConst`**

  An associated constant of a trait or a type.

- **`AssocType`**

  An associated type of a trait or a type.

- **`Primitive`**

  A primitive type, e.g. `u32`.
  
  [`Item`](#item)s of this kind only come from the core library.

- **`Keyword`**

  A keyword declaration.
  
  [`Item`](#item)s of this kind only come from the come library and exist solely
  to carry documentation for the respective keywords.

- **`Attribute`**

  An attribute declaration.
  
  [`Item`](#item)s of this kind only come from the core library and exist solely
  to carry documentation for the respective builtin attributes.

#### Trait Implementations

##### `impl Clone for ItemKind`

- <span id="itemkind-clone"></span>`fn clone(&self) -> ItemKind` — [`ItemKind`](#itemkind)

##### `impl Copy for ItemKind`

##### `impl Debug for ItemKind`

- <span id="itemkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for ItemKind`

- <span id="itemkind-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for ItemKind`

##### `impl Eq for ItemKind`

##### `impl Hash for ItemKind`

- <span id="itemkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ItemKind`

- <span id="itemkind-eq"></span>`fn eq(&self, other: &ItemKind) -> bool` — [`ItemKind`](#itemkind)

##### `impl Serialize for ItemKind`

- <span id="itemkind-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for ItemKind`

### `ItemEnum`

```rust
enum ItemEnum {
    Module(Module),
    ExternCrate {
        name: String,
        rename: Option<String>,
    },
    Use(Use),
    Union(Union),
    Struct(Struct),
    StructField(Type),
    Enum(Enum),
    Variant(Variant),
    Function(Function),
    Trait(Trait),
    TraitAlias(TraitAlias),
    Impl(Impl),
    TypeAlias(TypeAlias),
    Constant {
        type_: Type,
        const_: Constant,
    },
    Static(Static),
    ExternType,
    Macro(String),
    ProcMacro(ProcMacro),
    Primitive(Primitive),
    AssocConst {
        type_: Type,
        value: Option<String>,
    },
    AssocType {
        generics: Generics,
        bounds: Vec<GenericBound>,
        type_: Option<Type>,
    },
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:572-682`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L572-L682)*

Specific fields of an item.

Part of [`Item`](#item).

#### Variants

- **`Module`**

  A module declaration, e.g. `mod foo;` or `mod foo {}`

- **`ExternCrate`**

  A crate imported via the `extern crate` syntax.

- **`Use`**

  An import of 1 or more items into scope, using the `use` keyword.

- **`Union`**

  A `union` declaration.

- **`Struct`**

  A `struct` declaration.

- **`StructField`**

  A field of a struct.

- **`Enum`**

  An `enum` declaration.

- **`Variant`**

  A variant of a enum.

- **`Function`**

  A function declaration (including methods and other associated functions)

- **`Trait`**

  A `trait` declaration.

- **`TraitAlias`**

  A trait alias declaration, e.g. `trait Int = Add + Sub + Mul + Div;`
  
  See [the tracking issue](https://github.com/rust-lang/rust/issues/41517)

- **`Impl`**

  An `impl` block.

- **`TypeAlias`**

  A type alias declaration, e.g. `type Pig = std::borrow::Cow<'static, str>;`

- **`Constant`**

  The declaration of a constant, e.g. `const GREETING: &str = "Hi :3";`

- **`Static`**

  A declaration of a `static`.

- **`ExternType`**

  `type`s from an `extern` block.
  
  See [the tracking issue](https://github.com/rust-lang/rust/issues/43467)

- **`Macro`**

  A macro_rules! declarative macro. Contains a single string with the source
  representation of the macro with the patterns stripped.

- **`ProcMacro`**

  A procedural macro.

- **`Primitive`**

  A primitive type, e.g. `u32`.
  
  [`Item`](#item)s of this kind only come from the core library.

- **`AssocConst`**

  An associated constant of a trait or a type.

- **`AssocType`**

  An associated type of a trait or a type.

#### Trait Implementations

##### `impl Clone for ItemEnum`

- <span id="itemenum-clone"></span>`fn clone(&self) -> ItemEnum` — [`ItemEnum`](#itemenum)

##### `impl Debug for ItemEnum`

- <span id="itemenum-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for ItemEnum`

- <span id="itemenum-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for ItemEnum`

##### `impl Eq for ItemEnum`

##### `impl Hash for ItemEnum`

- <span id="itemenum-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ItemEnum`

- <span id="itemenum-eq"></span>`fn eq(&self, other: &ItemEnum) -> bool` — [`ItemEnum`](#itemenum)

##### `impl Serialize for ItemEnum`

- <span id="itemenum-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for ItemEnum`

### `StructKind`

```rust
enum StructKind {
    Unit,
    Tuple(Vec<Option<Id>>),
    Plain {
        fields: Vec<Id>,
        has_stripped_fields: bool,
    },
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:732-764`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L732-L764)*

The kind of a [`Struct`](#struct) and the data specific to it, i.e. fields.

#### Variants

- **`Unit`**

  A struct with no fields and no parentheses.
  
  ```rust
  pub struct Unit;
  ```

- **`Tuple`**

  A struct with unnamed fields.
  
  All [`Id`](#id)'s will point to [`ItemEnum::StructField`](#itemenumstructfield).
  Unlike most of JSON, private and `#[doc(hidden)]` fields will be given as `None`
  instead of being omitted, because order matters.
  
  ```rust
  pub struct TupleStruct(i32);
  pub struct EmptyTupleStruct();
  ```

- **`Plain`**

  A struct with named fields.
  
  ```rust
  pub struct PlainStruct { x: i32 }
  pub struct EmptyPlainStruct {}
  ```

#### Trait Implementations

##### `impl Clone for StructKind`

- <span id="structkind-clone"></span>`fn clone(&self) -> StructKind` — [`StructKind`](#structkind)

##### `impl Debug for StructKind`

- <span id="structkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for StructKind`

- <span id="structkind-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for StructKind`

##### `impl Eq for StructKind`

##### `impl Hash for StructKind`

- <span id="structkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for StructKind`

- <span id="structkind-eq"></span>`fn eq(&self, other: &StructKind) -> bool` — [`StructKind`](#structkind)

##### `impl Serialize for StructKind`

- <span id="structkind-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for StructKind`

### `VariantKind`

```rust
enum VariantKind {
    Plain,
    Tuple(Vec<Option<Id>>),
    Struct {
        fields: Vec<Id>,
        has_stripped_fields: bool,
    },
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:793-831`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L793-L831)*

The kind of an [`Enum`](#enum) [`Variant`](#variant) and the data specific to it, i.e. fields.

#### Variants

- **`Plain`**

  A variant with no parentheses
  
  ```rust
  enum Demo {
      PlainVariant,
      PlainWithDiscriminant = 1,
  }
  ```

- **`Tuple`**

  A variant with unnamed fields.
  
  All [`Id`](#id)'s will point to [`ItemEnum::StructField`](#itemenumstructfield).
  Unlike most of JSON, `#[doc(hidden)]` fields will be given as `None`
  instead of being omitted, because order matters.
  
  ```rust
  enum Demo {
      TupleVariant(i32),
      EmptyTupleVariant(),
  }
  ```

- **`Struct`**

  A variant with named fields.
  
  ```rust
  enum Demo {
      StructVariant { x: i32 },
      EmptyStructVariant {},
  }
  ```

#### Trait Implementations

##### `impl Clone for VariantKind`

- <span id="variantkind-clone"></span>`fn clone(&self) -> VariantKind` — [`VariantKind`](#variantkind)

##### `impl Debug for VariantKind`

- <span id="variantkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for VariantKind`

- <span id="variantkind-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for VariantKind`

##### `impl Eq for VariantKind`

##### `impl Hash for VariantKind`

- <span id="variantkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for VariantKind`

- <span id="variantkind-eq"></span>`fn eq(&self, other: &VariantKind) -> bool` — [`VariantKind`](#variantkind)

##### `impl Serialize for VariantKind`

- <span id="variantkind-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for VariantKind`

### `Abi`

```rust
enum Abi {
    Rust,
    C {
        unwind: bool,
    },
    Cdecl {
        unwind: bool,
    },
    Stdcall {
        unwind: bool,
    },
    Fastcall {
        unwind: bool,
    },
    Aapcs {
        unwind: bool,
    },
    Win64 {
        unwind: bool,
    },
    SysV64 {
        unwind: bool,
    },
    System {
        unwind: bool,
    },
    Other(String),
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:873-896`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L873-L896)*

The ABI (Application Binary Interface) used by a function.

If a variant has an `unwind` field, this means the ABI that it represents can be specified in 2
ways: `extern "_"` and `extern "_-unwind"`, and a value of `true` for that field signifies the
latter variant.

See the [Rustonomicon section](https://doc.rust-lang.org/nightly/nomicon/ffi.html#ffi-and-unwinding)
on unwinding for more info.

#### Variants

- **`Rust`**

  The default ABI, but that can also be written explicitly with `extern "Rust"`.

- **`C`**

  Can be specified as `extern "C"` or, as a shorthand, just `extern`.

- **`Cdecl`**

  Can be specified as `extern "cdecl"`.

- **`Stdcall`**

  Can be specified as `extern "stdcall"`.

- **`Fastcall`**

  Can be specified as `extern "fastcall"`.

- **`Aapcs`**

  Can be specified as `extern "aapcs"`.

- **`Win64`**

  Can be specified as `extern "win64"`.

- **`SysV64`**

  Can be specified as `extern "sysv64"`.

- **`System`**

  Can be specified as `extern "system"`.

- **`Other`**

  Any other ABI, including unstable ones.

#### Trait Implementations

##### `impl Clone for Abi`

- <span id="abi-clone"></span>`fn clone(&self) -> Abi` — [`Abi`](#abi)

##### `impl Debug for Abi`

- <span id="abi-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Abi`

- <span id="abi-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Abi`

##### `impl Eq for Abi`

##### `impl Hash for Abi`

- <span id="abi-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Abi`

- <span id="abi-eq"></span>`fn eq(&self, other: &Abi) -> bool` — [`Abi`](#abi)

##### `impl Serialize for Abi`

- <span id="abi-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Abi`

### `GenericParamDefKind`

```rust
enum GenericParamDefKind {
    Lifetime {
        outlives: Vec<String>,
    },
    Type {
        bounds: Vec<GenericBound>,
        default: Option<Type>,
        is_synthetic: bool,
    },
    Const {
        type_: Type,
        default: Option<String>,
    },
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:937-1001`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L937-L1001)*

The kind of a [`GenericParamDef`](#genericparamdef).

#### Variants

- **`Lifetime`**

  Denotes a lifetime parameter.

- **`Type`**

  Denotes a type parameter.

- **`Const`**

  Denotes a constant parameter.

#### Trait Implementations

##### `impl Clone for GenericParamDefKind`

- <span id="genericparamdefkind-clone"></span>`fn clone(&self) -> GenericParamDefKind` — [`GenericParamDefKind`](#genericparamdefkind)

##### `impl Debug for GenericParamDefKind`

- <span id="genericparamdefkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for GenericParamDefKind`

- <span id="genericparamdefkind-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for GenericParamDefKind`

##### `impl Eq for GenericParamDefKind`

##### `impl Hash for GenericParamDefKind`

- <span id="genericparamdefkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for GenericParamDefKind`

- <span id="genericparamdefkind-eq"></span>`fn eq(&self, other: &GenericParamDefKind) -> bool` — [`GenericParamDefKind`](#genericparamdefkind)

##### `impl Serialize for GenericParamDefKind`

- <span id="genericparamdefkind-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for GenericParamDefKind`

### `WherePredicate`

```rust
enum WherePredicate {
    BoundPredicate {
        type_: Type,
        bounds: Vec<GenericBound>,
        generic_params: Vec<GenericParamDef>,
    },
    LifetimePredicate {
        lifetime: String,
        outlives: Vec<String>,
    },
    EqPredicate {
        lhs: Type,
        rhs: Term,
    },
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:1010-1051`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L1010-L1051)*

One `where` clause.
```rust
fn default<T>() -> T where T: Default { T::default() }
//                         ^^^^^^^^^^
```

#### Variants

- **`BoundPredicate`**

  A type is expected to comply with a set of bounds

- **`LifetimePredicate`**

  A lifetime is expected to outlive other lifetimes.

- **`EqPredicate`**

  A type must exactly equal another type.

#### Trait Implementations

##### `impl Clone for WherePredicate`

- <span id="wherepredicate-clone"></span>`fn clone(&self) -> WherePredicate` — [`WherePredicate`](#wherepredicate)

##### `impl Debug for WherePredicate`

- <span id="wherepredicate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for WherePredicate`

- <span id="wherepredicate-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for WherePredicate`

##### `impl Eq for WherePredicate`

##### `impl Hash for WherePredicate`

- <span id="wherepredicate-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for WherePredicate`

- <span id="wherepredicate-eq"></span>`fn eq(&self, other: &WherePredicate) -> bool` — [`WherePredicate`](#wherepredicate)

##### `impl Serialize for WherePredicate`

- <span id="wherepredicate-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for WherePredicate`

### `GenericBound`

```rust
enum GenericBound {
    TraitBound {
        trait_: Path,
        generic_params: Vec<GenericParamDef>,
        modifier: TraitBoundModifier,
    },
    Outlives(String),
    Use(Vec<PreciseCapturingArg>),
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:1056-1081`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L1056-L1081)*

Either a trait bound or a lifetime bound.

#### Variants

- **`TraitBound`**

  A trait bound.

- **`Outlives`**

  A lifetime bound, e.g.
  ```rust
  fn f<'a, T>(x: &'a str, y: &T) where T: 'a {}
  //                                     ^^^
  ```

- **`Use`**

  `use<'a, T>` precise-capturing bound syntax

#### Trait Implementations

##### `impl Clone for GenericBound`

- <span id="genericbound-clone"></span>`fn clone(&self) -> GenericBound` — [`GenericBound`](#genericbound)

##### `impl Debug for GenericBound`

- <span id="genericbound-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for GenericBound`

- <span id="genericbound-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for GenericBound`

##### `impl Eq for GenericBound`

##### `impl Hash for GenericBound`

- <span id="genericbound-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for GenericBound`

- <span id="genericbound-eq"></span>`fn eq(&self, other: &GenericBound) -> bool` — [`GenericBound`](#genericbound)

##### `impl Serialize for GenericBound`

- <span id="genericbound-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for GenericBound`

### `TraitBoundModifier`

```rust
enum TraitBoundModifier {
    None,
    Maybe,
    MaybeConst,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:1086-1096`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L1086-L1096)*

A set of modifiers applied to a trait.

#### Variants

- **`None`**

  Marks the absence of a modifier.

- **`Maybe`**

  Indicates that the trait bound relaxes a trait bound applied to a parameter by default,
  e.g. `T: Sized?`, the `Sized` trait is required for all generic type parameters by default
  unless specified otherwise with this modifier.

- **`MaybeConst`**

  Indicates that the trait bound must be applicable in both a run-time and a compile-time
  context.

#### Trait Implementations

##### `impl Clone for TraitBoundModifier`

- <span id="traitboundmodifier-clone"></span>`fn clone(&self) -> TraitBoundModifier` — [`TraitBoundModifier`](#traitboundmodifier)

##### `impl Copy for TraitBoundModifier`

##### `impl Debug for TraitBoundModifier`

- <span id="traitboundmodifier-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for TraitBoundModifier`

- <span id="traitboundmodifier-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for TraitBoundModifier`

##### `impl Eq for TraitBoundModifier`

##### `impl Hash for TraitBoundModifier`

- <span id="traitboundmodifier-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for TraitBoundModifier`

- <span id="traitboundmodifier-eq"></span>`fn eq(&self, other: &TraitBoundModifier) -> bool` — [`TraitBoundModifier`](#traitboundmodifier)

##### `impl Serialize for TraitBoundModifier`

- <span id="traitboundmodifier-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for TraitBoundModifier`

### `PreciseCapturingArg`

```rust
enum PreciseCapturingArg {
    Lifetime(String),
    Param(String),
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:1101-1112`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L1101-L1112)*

One precise capturing argument. See [the rust reference](https://doc.rust-lang.org/reference/types/impl-trait.html#precise-capturing).

#### Variants

- **`Lifetime`**

  A lifetime.
  ```rust
  pub fn hello<'a, T, const N: usize>() -> impl Sized + use<'a, T, N> {}
  //                                                        ^^

- **`Param`**

  A type or constant parameter.
  ```rust
  pub fn hello<'a, T, const N: usize>() -> impl Sized + use<'a, T, N> {}
  //                                                            ^  ^

#### Trait Implementations

##### `impl Clone for PreciseCapturingArg`

- <span id="precisecapturingarg-clone"></span>`fn clone(&self) -> PreciseCapturingArg` — [`PreciseCapturingArg`](#precisecapturingarg)

##### `impl Debug for PreciseCapturingArg`

- <span id="precisecapturingarg-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for PreciseCapturingArg`

- <span id="precisecapturingarg-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for PreciseCapturingArg`

##### `impl Eq for PreciseCapturingArg`

##### `impl Hash for PreciseCapturingArg`

- <span id="precisecapturingarg-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for PreciseCapturingArg`

- <span id="precisecapturingarg-eq"></span>`fn eq(&self, other: &PreciseCapturingArg) -> bool` — [`PreciseCapturingArg`](#precisecapturingarg)

##### `impl Serialize for PreciseCapturingArg`

- <span id="precisecapturingarg-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for PreciseCapturingArg`

### `Term`

```rust
enum Term {
    Type(Type),
    Constant(Constant),
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:1118-1137`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L1118-L1137)*

Either a type or a constant, usually stored as the right-hand side of an equation in places like
[`AssocItemConstraint`](#associtemconstraint)

#### Variants

- **`Type`**

  A type.
  
  ```rust
  fn f(x: impl IntoIterator<Item = u32>) {}
  //                               ^^^
  ```

- **`Constant`**

  A constant.
  
  ```ignore (incomplete feature in the snippet)
  trait Foo {
      const BAR: usize;
  }
  
  fn f(x: impl Foo<BAR = 42>) {}
  //                     ^^
  ```

#### Trait Implementations

##### `impl Clone for Term`

- <span id="term-clone"></span>`fn clone(&self) -> Term` — [`Term`](#term)

##### `impl Debug for Term`

- <span id="term-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Term`

- <span id="term-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Term`

##### `impl Eq for Term`

##### `impl Hash for Term`

- <span id="term-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Term`

- <span id="term-eq"></span>`fn eq(&self, other: &Term) -> bool` — [`Term`](#term)

##### `impl Serialize for Term`

- <span id="term-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Term`

### `Type`

```rust
enum Type {
    ResolvedPath(Path),
    DynTrait(DynTrait),
    Generic(String),
    Primitive(String),
    FunctionPointer(Box<FunctionPointer>),
    Tuple(Vec<Type>),
    Slice(Box<Type>),
    Array {
        type_: Box<Type>,
        len: String,
    },
    Pat {
        type_: Box<Type>,
    },
    ImplTrait(Vec<GenericBound>),
    Infer,
    RawPointer {
        is_mutable: bool,
        type_: Box<Type>,
    },
    BorrowedRef {
        lifetime: Option<String>,
        is_mutable: bool,
        type_: Box<Type>,
    },
    QualifiedPath {
        name: String,
        args: Option<Box<GenericArgs>>,
        self_type: Box<Type>,
        trait_: Option<Path>,
    },
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:1142-1227`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L1142-L1227)*

A type.

#### Variants

- **`ResolvedPath`**

  Structs, enums, unions and type aliases, e.g. `std::option::Option<u32>`

- **`DynTrait`**

  Dynamic trait object type (`dyn Trait`).

- **`Generic`**

  Parameterized types. The contained string is the name of the parameter.

- **`Primitive`**

  Built-in numeric types (e.g. `u32`, `f32`), `bool`, `char`.

- **`FunctionPointer`**

  A function pointer type, e.g. `fn(u32) -> u32`, `extern "C" fn() -> *const u8`

- **`Tuple`**

  A tuple type, e.g. `(String, u32, Box<usize>)`

- **`Slice`**

  An unsized slice type, e.g. `[u32]`.

- **`Array`**

  An array type, e.g. `[u32; 15]`

- **`Pat`**

  A pattern type, e.g. `u32 is 1..`
  
  See [the tracking issue](https://github.com/rust-lang/rust/issues/123646)

- **`ImplTrait`**

  An opaque type that satisfies a set of bounds, `impl TraitA + TraitB + ...`

- **`Infer`**

  A type that's left to be inferred, `_`

- **`RawPointer`**

  A raw pointer type, e.g. `*mut u32`, `*const u8`, etc.

- **`BorrowedRef`**

  `&'a mut String`, `&str`, etc.

- **`QualifiedPath`**

  Associated types like `<Type as Trait>::Name` and `T::Item` where
  `T: Iterator` or inherent associated types like `Struct::Name`.

#### Trait Implementations

##### `impl Clone for Type`

- <span id="type-clone"></span>`fn clone(&self) -> Type` — [`Type`](#type)

##### `impl Debug for Type`

- <span id="type-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Type`

- <span id="type-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Type`

##### `impl Eq for Type`

##### `impl Hash for Type`

- <span id="type-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Type`

- <span id="type-eq"></span>`fn eq(&self, other: &Type) -> bool` — [`Type`](#type)

##### `impl Serialize for Type`

- <span id="type-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Type`

### `MacroKind`

```rust
enum MacroKind {
    Bang,
    Attr,
    Derive,
}
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:1406-1413`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L1406-L1413)*

The way a [`ProcMacro`](#procmacro) is declared to be used.

#### Variants

- **`Bang`**

  A bang macro `foo!()`.

- **`Attr`**

  An attribute macro `#[foo]`.

- **`Derive`**

  A derive macro `#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]`

#### Trait Implementations

##### `impl Clone for MacroKind`

- <span id="macrokind-clone"></span>`fn clone(&self) -> MacroKind` — [`MacroKind`](#macrokind)

##### `impl Copy for MacroKind`

##### `impl Debug for MacroKind`

- <span id="macrokind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for MacroKind`

- <span id="macrokind-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for MacroKind`

##### `impl Eq for MacroKind`

##### `impl Hash for MacroKind`

- <span id="macrokind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for MacroKind`

- <span id="macrokind-eq"></span>`fn eq(&self, other: &MacroKind) -> bool` — [`MacroKind`](#macrokind)

##### `impl Serialize for MacroKind`

- <span id="macrokind-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for MacroKind`

## Constants

### `FORMAT_VERSION`
```rust
const FORMAT_VERSION: u32 = 57u32;
```

*Defined in [`rustdoc-types-0.57.0/src/lib.rs:40`](../../.source_1765521767/rustdoc-types-0.57.0/src/lib.rs#L40)*

The version of JSON output that this crate represents.

This integer is incremented with every breaking change to the API,
and is returned along with the JSON blob as `Crate::format_version`.
Consuming code should assert that this value matches the format version(s) that it supports.

