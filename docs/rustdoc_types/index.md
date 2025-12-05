# Crate `rustdoc_types`

Rustdoc's JSON output interface

These types are the public API exposed through the `--output-format json` flag. The [`Crate`](#crate)
struct is the root of the JSON blob and all other items are contained within.

We expose a `rustc-hash` feature that is disabled by default. This feature switches the
`std::collections::HashMap` for `rustc_hash::FxHashMap` to improve the performance of said
`HashMap` in specific situations.

`cargo-semver-checks` for example, saw a [-3% improvement][1] when benchmarking using the
`aws_sdk_ec2` JSON output (~500MB of JSON). As always, we recommend measuring the impact before
turning this feature on, as [`FxHashMap`][2] only concerns itself with hash speed, and may
increase the number of collisions.

[1]: https://rust-lang.zulipchat.com/#narrow/channel/266220-t-rustdoc/topic/rustc-hash.20and.20performance.20of.20rustdoc-types/near/474855731
[2]: https://crates.io/crates/rustc-hash

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

##### `impl Clone`

- `fn clone(self: &Self) -> Crate` — [`Crate`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Crate) -> bool` — [`Crate`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `Target`

```rust
struct Target {
    pub triple: String,
    pub target_features: Vec<TargetFeature>,
}
```

Information about a target

#### Fields

- **`triple`**: `String`

  The target triple for which this documentation was generated

- **`target_features`**: `Vec<TargetFeature>`

  A list of features valid for use in `#[target_feature]` attributes
  for the target where this rustdoc JSON was generated.

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Target` — [`Target`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Target) -> bool` — [`Target`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `TargetFeature`

```rust
struct TargetFeature {
    pub name: String,
    pub implies_features: Vec<String>,
    pub unstable_feature_gate: Option<String>,
    pub globally_enabled: bool,
}
```

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

[1]: https://doc.rust-lang.org/stable/reference/attributes/codegen.html#the-target_feature-attribute
[2]: https://doc.rust-lang.org/reference/conditional-compilation.html#target_feature

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
  
  [1]: https://doc.rust-lang.org/beta/rustc/codegen-options/index.html#target-feature
  [2]: https://doc.rust-lang.org/beta/rustc/codegen-options/index.html#target-cpu

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> TargetFeature` — [`TargetFeature`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &TargetFeature) -> bool` — [`TargetFeature`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `ExternalCrate`

```rust
struct ExternalCrate {
    pub name: String,
    pub html_root_url: Option<String>,
    pub path: std::path::PathBuf,
}
```

Metadata of a crate, either the same crate on which `rustdoc` was invoked, or its dependency.

#### Fields

- **`name`**: `String`

  The name of the crate.
  
  Note: This is the [*crate* name][crate-name], which may not be the same as the
  [*package* name][package-name]. For example, for <https://crates.io/crates/regex-syntax>,
  this field will be `regex_syntax` (which uses an `_`, not a `-`).
  
  [crate-name]: https://doc.rust-lang.org/stable/cargo/reference/cargo-targets.html#the-name-field
  [package-name]: https://doc.rust-lang.org/stable/cargo/reference/manifest.html#the-name-field

- **`html_root_url`**: `Option<String>`

  The root URL at which the crate's documentation lives.

- **`path`**: `std::path::PathBuf`

  A path from where this crate was loaded.
  
  This will typically be a `.rlib` or `.rmeta`. It can be used to determine which crate
  this was in terms of whatever build-system invoked rustc.

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> ExternalCrate` — [`ExternalCrate`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ExternalCrate) -> bool` — [`ExternalCrate`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `ItemSummary`

```rust
struct ItemSummary {
    pub crate_id: u32,
    pub path: Vec<String>,
    pub kind: ItemKind,
}
```

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
  [`String`](#string) is currently `["alloc", "string", "String"]` and [`HashMap`]`std::collections::HashMap`
  is `["std", "collections", "hash", "map", "HashMap"]`, but this is subject to change.

- **`kind`**: `ItemKind`

  Whether this item is a struct, trait, macro, etc.

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> ItemSummary` — [`ItemSummary`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ItemSummary) -> bool` — [`ItemSummary`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Item` — [`Item`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Item) -> bool` — [`Item`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `AttributeRepr`

```rust
struct AttributeRepr {
    pub kind: ReprKind,
    pub align: Option<u64>,
    pub packed: Option<u64>,
    pub int: Option<String>,
}
```

The contents of a `#[repr(...)]` attribute.

Used in `Attribute::Repr`.

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

##### `impl Clone`

- `fn clone(self: &Self) -> AttributeRepr` — [`AttributeRepr`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &AttributeRepr) -> bool` — [`AttributeRepr`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `Span`

```rust
struct Span {
    pub filename: std::path::PathBuf,
    pub begin: (usize, usize),
    pub end: (usize, usize),
}
```

A range of source code.

#### Fields

- **`filename`**: `std::path::PathBuf`

  The path to the source file for this span relative to the path `rustdoc` was invoked with.

- **`begin`**: `(usize, usize)`

  One indexed Line and Column of the first character of the `Span`.

- **`end`**: `(usize, usize)`

  One indexed Line and Column of the last character of the `Span`.

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Span` — [`Span`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Span) -> bool` — [`Span`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `Deprecation`

```rust
struct Deprecation {
    pub since: Option<String>,
    pub note: Option<String>,
}
```

Information about the deprecation of an [`Item`](#item).

#### Fields

- **`since`**: `Option<String>`

  Usually a version number when this [`Item`](#item) first became deprecated.

- **`note`**: `Option<String>`

  The reason for deprecation and/or what alternatives to use.

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Deprecation` — [`Deprecation`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Deprecation) -> bool` — [`Deprecation`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `DynTrait`

```rust
struct DynTrait {
    pub traits: Vec<PolyTrait>,
    pub lifetime: Option<String>,
}
```

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

##### `impl Clone`

- `fn clone(self: &Self) -> DynTrait` — [`DynTrait`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &DynTrait) -> bool` — [`DynTrait`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `PolyTrait`

```rust
struct PolyTrait {
    pub trait_: Path,
    pub generic_params: Vec<GenericParamDef>,
}
```

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

##### `impl Clone`

- `fn clone(self: &Self) -> PolyTrait` — [`PolyTrait`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &PolyTrait) -> bool` — [`PolyTrait`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `Constant`

```rust
struct Constant {
    pub expr: String,
    pub value: Option<String>,
    pub is_literal: bool,
}
```

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

##### `impl Clone`

- `fn clone(self: &Self) -> Constant` — [`Constant`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Constant) -> bool` — [`Constant`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `AssocItemConstraint`

```rust
struct AssocItemConstraint {
    pub name: String,
    pub args: Option<Box<GenericArgs>>,
    pub binding: AssocItemConstraintKind,
}
```

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

##### `impl Clone`

- `fn clone(self: &Self) -> AssocItemConstraint` — [`AssocItemConstraint`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &AssocItemConstraint) -> bool` — [`AssocItemConstraint`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `Id`

```rust
struct Id(u32);
```

An opaque identifier for an item.

It can be used to lookup in `Crate::index` or `Crate::paths` to resolve it
to an [`Item`](#item).

Id's are only valid within a single JSON blob. They cannot be used to
resolve references between the JSON output's for different crates.

Rustdoc makes no guarantees about the inner value of Id's. Applications
should treat them as opaque keys to lookup items, and avoid attempting
to parse them, or otherwise depend on any implementation details.

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Id` — [`Id`](../index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord`

- `fn cmp(self: &Self, other: &Id) -> $crate::cmp::Ordering` — [`Id`](../index.md)

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Id) -> bool` — [`Id`](../index.md)

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &Id) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Id`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `Module`

```rust
struct Module {
    pub is_crate: bool,
    pub items: Vec<Id>,
    pub is_stripped: bool,
}
```

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

##### `impl Clone`

- `fn clone(self: &Self) -> Module` — [`Module`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Module) -> bool` — [`Module`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `Union`

```rust
struct Union {
    pub generics: Generics,
    pub has_stripped_fields: bool,
    pub fields: Vec<Id>,
    pub impls: Vec<Id>,
}
```

A `union`.

#### Fields

- **`generics`**: `Generics`

  The generic parameters and where clauses on this union.

- **`has_stripped_fields`**: `bool`

  Whether any fields have been removed from the result, due to being private or hidden.

- **`fields`**: `Vec<Id>`

  The list of fields in the union.
  
  All of the corresponding [`Item`](#item)s are of kind `ItemEnum::StructField`.

- **`impls`**: `Vec<Id>`

  All impls (both of traits and inherent) for this union.
  
  All of the corresponding [`Item`](#item)s are of kind `ItemEnum::Impl`.

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Union` — [`Union`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Union) -> bool` — [`Union`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `Struct`

```rust
struct Struct {
    pub kind: StructKind,
    pub generics: Generics,
    pub impls: Vec<Id>,
}
```

A `struct`.

#### Fields

- **`kind`**: `StructKind`

  The kind of the struct (e.g. unit, tuple-like or struct-like) and the data specific to it,
  i.e. fields.

- **`generics`**: `Generics`

  The generic parameters and where clauses on this struct.

- **`impls`**: `Vec<Id>`

  All impls (both of traits and inherent) for this struct.
  All of the corresponding [`Item`](#item)s are of kind `ItemEnum::Impl`.

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Struct` — [`Struct`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Struct) -> bool` — [`Struct`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `Enum`

```rust
struct Enum {
    pub generics: Generics,
    pub has_stripped_variants: bool,
    pub variants: Vec<Id>,
    pub impls: Vec<Id>,
}
```

An `enum`.

#### Fields

- **`generics`**: `Generics`

  Information about the type parameters and `where` clauses of the enum.

- **`has_stripped_variants`**: `bool`

  Whether any variants have been removed from the result, due to being private or hidden.

- **`variants`**: `Vec<Id>`

  The list of variants in the enum.
  
  All of the corresponding [`Item`](#item)s are of kind `ItemEnum::Variant`

- **`impls`**: `Vec<Id>`

  `impl`s for the enum.

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Enum` — [`Enum`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Enum) -> bool` — [`Enum`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `Variant`

```rust
struct Variant {
    pub kind: VariantKind,
    pub discriminant: Option<Discriminant>,
}
```

A variant of an enum.

#### Fields

- **`kind`**: `VariantKind`

  Whether the variant is plain, a tuple-like, or struct-like. Contains the fields.

- **`discriminant`**: `Option<Discriminant>`

  The discriminant, if explicitly specified.

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Variant` — [`Variant`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Variant) -> bool` — [`Variant`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `Discriminant`

```rust
struct Discriminant {
    pub expr: String,
    pub value: String,
}
```

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

##### `impl Clone`

- `fn clone(self: &Self) -> Discriminant` — [`Discriminant`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Discriminant) -> bool` — [`Discriminant`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `FunctionHeader`

```rust
struct FunctionHeader {
    pub is_const: bool,
    pub is_unsafe: bool,
    pub is_async: bool,
    pub abi: Abi,
}
```

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

##### `impl Clone`

- `fn clone(self: &Self) -> FunctionHeader` — [`FunctionHeader`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &FunctionHeader) -> bool` — [`FunctionHeader`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `Function`

```rust
struct Function {
    pub sig: FunctionSignature,
    pub generics: Generics,
    pub header: FunctionHeader,
    pub has_body: bool,
}
```

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

##### `impl Clone`

- `fn clone(self: &Self) -> Function` — [`Function`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Function) -> bool` — [`Function`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `Generics`

```rust
struct Generics {
    pub params: Vec<GenericParamDef>,
    pub where_predicates: Vec<WherePredicate>,
}
```

Generic parameters accepted by an item and `where` clauses imposed on it and the parameters.

#### Fields

- **`params`**: `Vec<GenericParamDef>`

  A list of generic parameter definitions (e.g. `<T: Clone + Hash, U: Copy>`).

- **`where_predicates`**: `Vec<WherePredicate>`

  A list of where predicates (e.g. `where T: Iterator, T::Item: Copy`).

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Generics` — [`Generics`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Generics) -> bool` — [`Generics`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `GenericParamDef`

```rust
struct GenericParamDef {
    pub name: String,
    pub kind: GenericParamDefKind,
}
```

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

##### `impl Clone`

- `fn clone(self: &Self) -> GenericParamDef` — [`GenericParamDef`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &GenericParamDef) -> bool` — [`GenericParamDef`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `Path`

```rust
struct Path {
    pub path: String,
    pub id: Id,
    pub args: Option<Box<GenericArgs>>,
}
```

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

##### `impl Clone`

- `fn clone(self: &Self) -> Path` — [`Path`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Path) -> bool` — [`Path`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `FunctionPointer`

```rust
struct FunctionPointer {
    pub sig: FunctionSignature,
    pub generic_params: Vec<GenericParamDef>,
    pub header: FunctionHeader,
}
```

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

##### `impl Clone`

- `fn clone(self: &Self) -> FunctionPointer` — [`FunctionPointer`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &FunctionPointer) -> bool` — [`FunctionPointer`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `FunctionSignature`

```rust
struct FunctionSignature {
    pub inputs: Vec<(String, Type)>,
    pub output: Option<Type>,
    pub is_c_variadic: bool,
}
```

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

##### `impl Clone`

- `fn clone(self: &Self) -> FunctionSignature` — [`FunctionSignature`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &FunctionSignature) -> bool` — [`FunctionSignature`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Trait` — [`Trait`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Trait) -> bool` — [`Trait`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `TraitAlias`

```rust
struct TraitAlias {
    pub generics: Generics,
    pub params: Vec<GenericBound>,
}
```

A trait alias declaration, e.g. `trait Int = Add + Sub + Mul + Div;`

See [the tracking issue](https://github.com/rust-lang/rust/issues/41517)

#### Fields

- **`generics`**: `Generics`

  Information about the type parameters and `where` clauses of the alias.

- **`params`**: `Vec<GenericBound>`

  The bounds that are associated with the alias.

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> TraitAlias` — [`TraitAlias`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &TraitAlias) -> bool` — [`TraitAlias`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

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

An `impl` block.

#### Fields

- **`is_unsafe`**: `bool`

  Whether this impl is for an unsafe trait.

- **`generics`**: `Generics`

  Information about the impl’s type parameters and `where` clauses.

- **`provided_trait_methods`**: `Vec<String>`

  The list of the names of all the trait methods that weren't mentioned in this impl but
  were provided by the trait itself.
  
  For example, for this impl of the [`PartialEq`](#partialeq) trait:
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

##### `impl Clone`

- `fn clone(self: &Self) -> Impl` — [`Impl`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Impl) -> bool` — [`Impl`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `Use`

```rust
struct Use {
    pub source: String,
    pub name: String,
    pub id: Option<Id>,
    pub is_glob: bool,
}
```

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

##### `impl Clone`

- `fn clone(self: &Self) -> Use` — [`Use`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Use) -> bool` — [`Use`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `ProcMacro`

```rust
struct ProcMacro {
    pub kind: MacroKind,
    pub helpers: Vec<String>,
}
```

A procedural macro.

#### Fields

- **`kind`**: `MacroKind`

  How this macro is supposed to be called: `foo!()`, `#[foo]` or `#[derive(foo)]`

- **`helpers`**: `Vec<String>`

  Helper attributes defined by a macro to be used inside it.
  
  Defined only for derive macros.
  
  E.g. the [`Default`](#default) derive macro defines a `#[default]` helper attribute so that one can
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

##### `impl Clone`

- `fn clone(self: &Self) -> ProcMacro` — [`ProcMacro`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ProcMacro) -> bool` — [`ProcMacro`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `TypeAlias`

```rust
struct TypeAlias {
    pub type_: Type,
    pub generics: Generics,
}
```

A type alias declaration, e.g. `type Pig = std::borrow::Cow<'static, str>;`

#### Fields

- **`type_`**: `Type`

  The type referred to by this alias.

- **`generics`**: `Generics`

  Information about the type parameters and `where` clauses of the alias.

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> TypeAlias` — [`TypeAlias`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &TypeAlias) -> bool` — [`TypeAlias`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `Static`

```rust
struct Static {
    pub type_: Type,
    pub is_mutable: bool,
    pub expr: String,
    pub is_unsafe: bool,
}
```

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

##### `impl Clone`

- `fn clone(self: &Self) -> Static` — [`Static`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Static) -> bool` — [`Static`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `Primitive`

```rust
struct Primitive {
    pub name: String,
    pub impls: Vec<Id>,
}
```

A primitive type declaration. Declarations of this kind can only come from the core library.

#### Fields

- **`name`**: `String`

  The name of the type.

- **`impls`**: `Vec<Id>`

  The implementations, inherent and of traits, on the primitive type.

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Primitive` — [`Primitive`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Primitive) -> bool` — [`Primitive`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Attribute` — [`Attribute`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Attribute) -> bool` — [`Attribute`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `ReprKind`

```rust
enum ReprKind {
    Rust,
    C,
    Transparent,
    Simd,
}
```

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

##### `impl Clone`

- `fn clone(self: &Self) -> ReprKind` — [`ReprKind`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ReprKind) -> bool` — [`ReprKind`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Visibility` — [`Visibility`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Visibility) -> bool` — [`Visibility`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

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

##### `impl Clone`

- `fn clone(self: &Self) -> GenericArgs` — [`GenericArgs`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &GenericArgs) -> bool` — [`GenericArgs`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `GenericArg`

```rust
enum GenericArg {
    Lifetime(String),
    Type(Type),
    Const(Constant),
    Infer,
}
```

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

##### `impl Clone`

- `fn clone(self: &Self) -> GenericArg` — [`GenericArg`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &GenericArg) -> bool` — [`GenericArg`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `AssocItemConstraintKind`

```rust
enum AssocItemConstraintKind {
    Equality(Term),
    Constraint(Vec<GenericBound>),
}
```

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

##### `impl Clone`

- `fn clone(self: &Self) -> AssocItemConstraintKind` — [`AssocItemConstraintKind`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &AssocItemConstraintKind) -> bool` — [`AssocItemConstraintKind`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

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

##### `impl Clone`

- `fn clone(self: &Self) -> ItemKind` — [`ItemKind`](../index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ItemKind) -> bool` — [`ItemKind`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

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

##### `impl Clone`

- `fn clone(self: &Self) -> ItemEnum` — [`ItemEnum`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ItemEnum) -> bool` — [`ItemEnum`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

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

The kind of a [`Struct`](#struct) and the data specific to it, i.e. fields.

#### Variants

- **`Unit`**

  A struct with no fields and no parentheses.
  
  ```rust
  pub struct Unit;
  ```

- **`Tuple`**

  A struct with unnamed fields.
  
  All [`Id`](#id)'s will point to `ItemEnum::StructField`.
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

##### `impl Clone`

- `fn clone(self: &Self) -> StructKind` — [`StructKind`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &StructKind) -> bool` — [`StructKind`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

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
  
  All [`Id`](#id)'s will point to `ItemEnum::StructField`.
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

##### `impl Clone`

- `fn clone(self: &Self) -> VariantKind` — [`VariantKind`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &VariantKind) -> bool` — [`VariantKind`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Abi` — [`Abi`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Abi) -> bool` — [`Abi`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

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

The kind of a [`GenericParamDef`](#genericparamdef).

#### Variants

- **`Lifetime`**

  Denotes a lifetime parameter.

- **`Type`**

  Denotes a type parameter.

- **`Const`**

  Denotes a constant parameter.

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> GenericParamDefKind` — [`GenericParamDefKind`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &GenericParamDefKind) -> bool` — [`GenericParamDefKind`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

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

##### `impl Clone`

- `fn clone(self: &Self) -> WherePredicate` — [`WherePredicate`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &WherePredicate) -> bool` — [`WherePredicate`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

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

##### `impl Clone`

- `fn clone(self: &Self) -> GenericBound` — [`GenericBound`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &GenericBound) -> bool` — [`GenericBound`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `TraitBoundModifier`

```rust
enum TraitBoundModifier {
    None,
    Maybe,
    MaybeConst,
}
```

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

##### `impl Clone`

- `fn clone(self: &Self) -> TraitBoundModifier` — [`TraitBoundModifier`](../index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &TraitBoundModifier) -> bool` — [`TraitBoundModifier`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `PreciseCapturingArg`

```rust
enum PreciseCapturingArg {
    Lifetime(String),
    Param(String),
}
```

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

##### `impl Clone`

- `fn clone(self: &Self) -> PreciseCapturingArg` — [`PreciseCapturingArg`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &PreciseCapturingArg) -> bool` — [`PreciseCapturingArg`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `Term`

```rust
enum Term {
    Type(Type),
    Constant(Constant),
}
```

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

##### `impl Clone`

- `fn clone(self: &Self) -> Term` — [`Term`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Term) -> bool` — [`Term`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Type` — [`Type`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Type) -> bool` — [`Type`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

### `MacroKind`

```rust
enum MacroKind {
    Bang,
    Attr,
    Derive,
}
```

The way a [`ProcMacro`](#procmacro) is declared to be used.

#### Variants

- **`Bang`**

  A bang macro `foo!()`.

- **`Attr`**

  An attribute macro `#[foo]`.

- **`Derive`**

  A derive macro `#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> MacroKind` — [`MacroKind`](../index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &MacroKind) -> bool` — [`MacroKind`](../index.md)

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq`

## Constants

### `FORMAT_VERSION`

```rust
const FORMAT_VERSION: u32 = 57u32;
```

The version of JSON output that this crate represents.

This integer is incremented with every breaking change to the API,
and is returned along with the JSON blob as `Crate::format_version`.
Consuming code should assert that this value matches the format version(s) that it supports.

