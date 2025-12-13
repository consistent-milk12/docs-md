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

##### `impl Any for Crate`

- <span id="crate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Crate`

- <span id="crate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Crate`

- <span id="crate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Crate`

- <span id="crate-clone"></span>`fn clone(&self) -> Crate` — [`Crate`](#crate)

##### `impl CloneToUninit for Crate`

- <span id="crate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Crate`

- <span id="crate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Crate`

- <span id="crate-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Crate`

##### `impl Eq for Crate`

##### `impl<T> From for Crate`

- <span id="crate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Crate`

- <span id="crate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Crate`

- <span id="crate-partialeq-eq"></span>`fn eq(&self, other: &Crate) -> bool` — [`Crate`](#crate)

##### `impl Serialize for Crate`

- <span id="crate-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Crate`

##### `impl ToOwned for Crate`

- <span id="crate-toowned-type-owned"></span>`type Owned = T`

- <span id="crate-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="crate-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Crate`

- <span id="crate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="crate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Crate`

- <span id="crate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="crate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Target`

- <span id="target-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Target`

- <span id="target-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Target`

- <span id="target-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Target`

- <span id="target-clone"></span>`fn clone(&self) -> Target` — [`Target`](#target)

##### `impl CloneToUninit for Target`

- <span id="target-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Target`

- <span id="target-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Target`

- <span id="target-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Target`

##### `impl Eq for Target`

##### `impl<T> From for Target`

- <span id="target-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Target`

- <span id="target-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Target`

- <span id="target-partialeq-eq"></span>`fn eq(&self, other: &Target) -> bool` — [`Target`](#target)

##### `impl Serialize for Target`

- <span id="target-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Target`

##### `impl ToOwned for Target`

- <span id="target-toowned-type-owned"></span>`type Owned = T`

- <span id="target-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="target-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Target`

- <span id="target-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="target-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Target`

- <span id="target-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="target-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for TargetFeature`

- <span id="targetfeature-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TargetFeature`

- <span id="targetfeature-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TargetFeature`

- <span id="targetfeature-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TargetFeature`

- <span id="targetfeature-clone"></span>`fn clone(&self) -> TargetFeature` — [`TargetFeature`](#targetfeature)

##### `impl CloneToUninit for TargetFeature`

- <span id="targetfeature-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for TargetFeature`

- <span id="targetfeature-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for TargetFeature`

- <span id="targetfeature-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for TargetFeature`

##### `impl Eq for TargetFeature`

##### `impl<T> From for TargetFeature`

- <span id="targetfeature-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TargetFeature`

- <span id="targetfeature-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for TargetFeature`

- <span id="targetfeature-partialeq-eq"></span>`fn eq(&self, other: &TargetFeature) -> bool` — [`TargetFeature`](#targetfeature)

##### `impl Serialize for TargetFeature`

- <span id="targetfeature-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for TargetFeature`

##### `impl ToOwned for TargetFeature`

- <span id="targetfeature-toowned-type-owned"></span>`type Owned = T`

- <span id="targetfeature-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="targetfeature-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TargetFeature`

- <span id="targetfeature-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="targetfeature-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TargetFeature`

- <span id="targetfeature-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="targetfeature-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for ExternalCrate`

- <span id="externalcrate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExternalCrate`

- <span id="externalcrate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExternalCrate`

- <span id="externalcrate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ExternalCrate`

- <span id="externalcrate-clone"></span>`fn clone(&self) -> ExternalCrate` — [`ExternalCrate`](#externalcrate)

##### `impl CloneToUninit for ExternalCrate`

- <span id="externalcrate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ExternalCrate`

- <span id="externalcrate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for ExternalCrate`

- <span id="externalcrate-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for ExternalCrate`

##### `impl Eq for ExternalCrate`

##### `impl<T> From for ExternalCrate`

- <span id="externalcrate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for ExternalCrate`

- <span id="externalcrate-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for ExternalCrate`

- <span id="externalcrate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ExternalCrate`

- <span id="externalcrate-partialeq-eq"></span>`fn eq(&self, other: &ExternalCrate) -> bool` — [`ExternalCrate`](#externalcrate)

##### `impl Serialize for ExternalCrate`

- <span id="externalcrate-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for ExternalCrate`

##### `impl ToOwned for ExternalCrate`

- <span id="externalcrate-toowned-type-owned"></span>`type Owned = T`

- <span id="externalcrate-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="externalcrate-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ExternalCrate`

- <span id="externalcrate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="externalcrate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExternalCrate`

- <span id="externalcrate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="externalcrate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for ItemSummary`

- <span id="itemsummary-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemSummary`

- <span id="itemsummary-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemSummary`

- <span id="itemsummary-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ItemSummary`

- <span id="itemsummary-clone"></span>`fn clone(&self) -> ItemSummary` — [`ItemSummary`](#itemsummary)

##### `impl CloneToUninit for ItemSummary`

- <span id="itemsummary-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ItemSummary`

- <span id="itemsummary-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for ItemSummary`

- <span id="itemsummary-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for ItemSummary`

##### `impl Eq for ItemSummary`

##### `impl<T> From for ItemSummary`

- <span id="itemsummary-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for ItemSummary`

- <span id="itemsummary-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for ItemSummary`

- <span id="itemsummary-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ItemSummary`

- <span id="itemsummary-partialeq-eq"></span>`fn eq(&self, other: &ItemSummary) -> bool` — [`ItemSummary`](#itemsummary)

##### `impl Serialize for ItemSummary`

- <span id="itemsummary-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for ItemSummary`

##### `impl ToOwned for ItemSummary`

- <span id="itemsummary-toowned-type-owned"></span>`type Owned = T`

- <span id="itemsummary-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemsummary-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ItemSummary`

- <span id="itemsummary-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemsummary-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemSummary`

- <span id="itemsummary-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemsummary-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Item`

- <span id="item-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Item`

- <span id="item-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Item`

- <span id="item-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Item`

- <span id="item-clone"></span>`fn clone(&self) -> Item` — [`Item`](#item)

##### `impl CloneToUninit for Item`

- <span id="item-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Item`

- <span id="item-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Item`

- <span id="item-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Item`

##### `impl Eq for Item`

##### `impl<T> From for Item`

- <span id="item-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Item`

- <span id="item-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Item`

- <span id="item-partialeq-eq"></span>`fn eq(&self, other: &Item) -> bool` — [`Item`](#item)

##### `impl Serialize for Item`

- <span id="item-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Item`

##### `impl ToOwned for Item`

- <span id="item-toowned-type-owned"></span>`type Owned = T`

- <span id="item-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="item-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Item`

- <span id="item-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="item-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Item`

- <span id="item-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="item-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for AttributeRepr`

- <span id="attributerepr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AttributeRepr`

- <span id="attributerepr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AttributeRepr`

- <span id="attributerepr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AttributeRepr`

- <span id="attributerepr-clone"></span>`fn clone(&self) -> AttributeRepr` — [`AttributeRepr`](#attributerepr)

##### `impl CloneToUninit for AttributeRepr`

- <span id="attributerepr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for AttributeRepr`

- <span id="attributerepr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for AttributeRepr`

- <span id="attributerepr-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for AttributeRepr`

##### `impl Eq for AttributeRepr`

##### `impl<T> From for AttributeRepr`

- <span id="attributerepr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AttributeRepr`

- <span id="attributerepr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for AttributeRepr`

- <span id="attributerepr-partialeq-eq"></span>`fn eq(&self, other: &AttributeRepr) -> bool` — [`AttributeRepr`](#attributerepr)

##### `impl Serialize for AttributeRepr`

- <span id="attributerepr-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for AttributeRepr`

##### `impl ToOwned for AttributeRepr`

- <span id="attributerepr-toowned-type-owned"></span>`type Owned = T`

- <span id="attributerepr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="attributerepr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AttributeRepr`

- <span id="attributerepr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="attributerepr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AttributeRepr`

- <span id="attributerepr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="attributerepr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Span`

- <span id="span-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Span`

- <span id="span-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Span`

- <span id="span-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Span`

- <span id="span-clone"></span>`fn clone(&self) -> Span` — [`Span`](#span)

##### `impl CloneToUninit for Span`

- <span id="span-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Span`

- <span id="span-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Span`

- <span id="span-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Span`

##### `impl Eq for Span`

##### `impl<T> From for Span`

- <span id="span-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Span`

- <span id="span-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Span`

- <span id="span-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Span`

- <span id="span-partialeq-eq"></span>`fn eq(&self, other: &Span) -> bool` — [`Span`](#span)

##### `impl Serialize for Span`

- <span id="span-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Span`

##### `impl ToOwned for Span`

- <span id="span-toowned-type-owned"></span>`type Owned = T`

- <span id="span-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="span-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Span`

- <span id="span-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="span-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Span`

- <span id="span-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="span-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Deprecation`

- <span id="deprecation-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Deprecation`

- <span id="deprecation-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Deprecation`

- <span id="deprecation-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Deprecation`

- <span id="deprecation-clone"></span>`fn clone(&self) -> Deprecation` — [`Deprecation`](#deprecation)

##### `impl CloneToUninit for Deprecation`

- <span id="deprecation-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Deprecation`

- <span id="deprecation-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Deprecation`

- <span id="deprecation-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Deprecation`

##### `impl Eq for Deprecation`

##### `impl<T> From for Deprecation`

- <span id="deprecation-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Deprecation`

- <span id="deprecation-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Deprecation`

- <span id="deprecation-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Deprecation`

- <span id="deprecation-partialeq-eq"></span>`fn eq(&self, other: &Deprecation) -> bool` — [`Deprecation`](#deprecation)

##### `impl Serialize for Deprecation`

- <span id="deprecation-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Deprecation`

##### `impl ToOwned for Deprecation`

- <span id="deprecation-toowned-type-owned"></span>`type Owned = T`

- <span id="deprecation-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="deprecation-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Deprecation`

- <span id="deprecation-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="deprecation-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Deprecation`

- <span id="deprecation-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="deprecation-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for DynTrait`

- <span id="dyntrait-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DynTrait`

- <span id="dyntrait-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DynTrait`

- <span id="dyntrait-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DynTrait`

- <span id="dyntrait-clone"></span>`fn clone(&self) -> DynTrait` — [`DynTrait`](#dyntrait)

##### `impl CloneToUninit for DynTrait`

- <span id="dyntrait-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for DynTrait`

- <span id="dyntrait-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for DynTrait`

- <span id="dyntrait-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for DynTrait`

##### `impl Eq for DynTrait`

##### `impl<T> From for DynTrait`

- <span id="dyntrait-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for DynTrait`

- <span id="dyntrait-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for DynTrait`

- <span id="dyntrait-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for DynTrait`

- <span id="dyntrait-partialeq-eq"></span>`fn eq(&self, other: &DynTrait) -> bool` — [`DynTrait`](#dyntrait)

##### `impl Serialize for DynTrait`

- <span id="dyntrait-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for DynTrait`

##### `impl ToOwned for DynTrait`

- <span id="dyntrait-toowned-type-owned"></span>`type Owned = T`

- <span id="dyntrait-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dyntrait-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DynTrait`

- <span id="dyntrait-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyntrait-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DynTrait`

- <span id="dyntrait-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyntrait-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for PolyTrait`

- <span id="polytrait-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PolyTrait`

- <span id="polytrait-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PolyTrait`

- <span id="polytrait-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for PolyTrait`

- <span id="polytrait-clone"></span>`fn clone(&self) -> PolyTrait` — [`PolyTrait`](#polytrait)

##### `impl CloneToUninit for PolyTrait`

- <span id="polytrait-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for PolyTrait`

- <span id="polytrait-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for PolyTrait`

- <span id="polytrait-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for PolyTrait`

##### `impl Eq for PolyTrait`

##### `impl<T> From for PolyTrait`

- <span id="polytrait-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for PolyTrait`

- <span id="polytrait-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for PolyTrait`

- <span id="polytrait-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for PolyTrait`

- <span id="polytrait-partialeq-eq"></span>`fn eq(&self, other: &PolyTrait) -> bool` — [`PolyTrait`](#polytrait)

##### `impl Serialize for PolyTrait`

- <span id="polytrait-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for PolyTrait`

##### `impl ToOwned for PolyTrait`

- <span id="polytrait-toowned-type-owned"></span>`type Owned = T`

- <span id="polytrait-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="polytrait-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PolyTrait`

- <span id="polytrait-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="polytrait-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PolyTrait`

- <span id="polytrait-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="polytrait-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Constant`

- <span id="constant-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Constant`

- <span id="constant-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Constant`

- <span id="constant-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Constant`

- <span id="constant-clone"></span>`fn clone(&self) -> Constant` — [`Constant`](#constant)

##### `impl CloneToUninit for Constant`

- <span id="constant-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Constant`

- <span id="constant-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Constant`

- <span id="constant-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Constant`

##### `impl Eq for Constant`

##### `impl<T> From for Constant`

- <span id="constant-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Constant`

- <span id="constant-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Constant`

- <span id="constant-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Constant`

- <span id="constant-partialeq-eq"></span>`fn eq(&self, other: &Constant) -> bool` — [`Constant`](#constant)

##### `impl Serialize for Constant`

- <span id="constant-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Constant`

##### `impl ToOwned for Constant`

- <span id="constant-toowned-type-owned"></span>`type Owned = T`

- <span id="constant-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="constant-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Constant`

- <span id="constant-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="constant-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Constant`

- <span id="constant-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="constant-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for AssocItemConstraint`

- <span id="associtemconstraint-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AssocItemConstraint`

- <span id="associtemconstraint-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AssocItemConstraint`

- <span id="associtemconstraint-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AssocItemConstraint`

- <span id="associtemconstraint-clone"></span>`fn clone(&self) -> AssocItemConstraint` — [`AssocItemConstraint`](#associtemconstraint)

##### `impl CloneToUninit for AssocItemConstraint`

- <span id="associtemconstraint-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for AssocItemConstraint`

- <span id="associtemconstraint-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for AssocItemConstraint`

- <span id="associtemconstraint-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for AssocItemConstraint`

##### `impl Eq for AssocItemConstraint`

##### `impl<T> From for AssocItemConstraint`

- <span id="associtemconstraint-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for AssocItemConstraint`

- <span id="associtemconstraint-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for AssocItemConstraint`

- <span id="associtemconstraint-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for AssocItemConstraint`

- <span id="associtemconstraint-partialeq-eq"></span>`fn eq(&self, other: &AssocItemConstraint) -> bool` — [`AssocItemConstraint`](#associtemconstraint)

##### `impl Serialize for AssocItemConstraint`

- <span id="associtemconstraint-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for AssocItemConstraint`

##### `impl ToOwned for AssocItemConstraint`

- <span id="associtemconstraint-toowned-type-owned"></span>`type Owned = T`

- <span id="associtemconstraint-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="associtemconstraint-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AssocItemConstraint`

- <span id="associtemconstraint-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="associtemconstraint-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AssocItemConstraint`

- <span id="associtemconstraint-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="associtemconstraint-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Id`

- <span id="id-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Id`

- <span id="id-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Id`

- <span id="id-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Id`

- <span id="id-clone"></span>`fn clone(&self) -> Id` — [`Id`](#id)

##### `impl CloneToUninit for Id`

- <span id="id-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Id`

##### `impl Debug for Id`

- <span id="id-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Id`

- <span id="id-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Id`

##### `impl Eq for Id`

##### `impl<T> From for Id`

- <span id="id-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Id`

- <span id="id-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Id`

- <span id="id-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Id`

- <span id="id-ord-cmp"></span>`fn cmp(&self, other: &Id) -> cmp::Ordering` — [`Id`](#id)

##### `impl PartialEq for Id`

- <span id="id-partialeq-eq"></span>`fn eq(&self, other: &Id) -> bool` — [`Id`](#id)

##### `impl PartialOrd for Id`

- <span id="id-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Id) -> option::Option<cmp::Ordering>` — [`Id`](#id)

##### `impl Serialize for Id`

- <span id="id-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Id`

##### `impl ToOwned for Id`

- <span id="id-toowned-type-owned"></span>`type Owned = T`

- <span id="id-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="id-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Id`

- <span id="id-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="id-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Id`

- <span id="id-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="id-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Module`

- <span id="module-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Module`

- <span id="module-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Module`

- <span id="module-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Module`

- <span id="module-clone"></span>`fn clone(&self) -> Module` — [`Module`](#module)

##### `impl CloneToUninit for Module`

- <span id="module-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Module`

- <span id="module-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Module`

- <span id="module-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Module`

##### `impl Eq for Module`

##### `impl<T> From for Module`

- <span id="module-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Module`

- <span id="module-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Module`

- <span id="module-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Module`

- <span id="module-partialeq-eq"></span>`fn eq(&self, other: &Module) -> bool` — [`Module`](#module)

##### `impl Serialize for Module`

- <span id="module-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Module`

##### `impl ToOwned for Module`

- <span id="module-toowned-type-owned"></span>`type Owned = T`

- <span id="module-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="module-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Module`

- <span id="module-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="module-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Module`

- <span id="module-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="module-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Union`

- <span id="union-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Union`

- <span id="union-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Union`

- <span id="union-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Union`

- <span id="union-clone"></span>`fn clone(&self) -> Union` — [`Union`](#union)

##### `impl CloneToUninit for Union`

- <span id="union-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Union`

- <span id="union-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Union`

- <span id="union-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Union`

##### `impl Eq for Union`

##### `impl<T> From for Union`

- <span id="union-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Union`

- <span id="union-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Union`

- <span id="union-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Union`

- <span id="union-partialeq-eq"></span>`fn eq(&self, other: &Union) -> bool` — [`Union`](#union)

##### `impl Serialize for Union`

- <span id="union-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Union`

##### `impl ToOwned for Union`

- <span id="union-toowned-type-owned"></span>`type Owned = T`

- <span id="union-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="union-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Union`

- <span id="union-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="union-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Union`

- <span id="union-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="union-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Struct`

- <span id="struct-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Struct`

- <span id="struct-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Struct`

- <span id="struct-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Struct`

- <span id="struct-clone"></span>`fn clone(&self) -> Struct` — [`Struct`](#struct)

##### `impl CloneToUninit for Struct`

- <span id="struct-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Struct`

- <span id="struct-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Struct`

- <span id="struct-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Struct`

##### `impl Eq for Struct`

##### `impl<T> From for Struct`

- <span id="struct-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Struct`

- <span id="struct-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Struct`

- <span id="struct-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Struct`

- <span id="struct-partialeq-eq"></span>`fn eq(&self, other: &Struct) -> bool` — [`Struct`](#struct)

##### `impl Serialize for Struct`

- <span id="struct-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Struct`

##### `impl ToOwned for Struct`

- <span id="struct-toowned-type-owned"></span>`type Owned = T`

- <span id="struct-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="struct-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Struct`

- <span id="struct-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="struct-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Struct`

- <span id="struct-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="struct-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Enum`

- <span id="enum-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Enum`

- <span id="enum-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Enum`

- <span id="enum-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Enum`

- <span id="enum-clone"></span>`fn clone(&self) -> Enum` — [`Enum`](#enum)

##### `impl CloneToUninit for Enum`

- <span id="enum-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Enum`

- <span id="enum-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Enum`

- <span id="enum-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Enum`

##### `impl Eq for Enum`

##### `impl<T> From for Enum`

- <span id="enum-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Enum`

- <span id="enum-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Enum`

- <span id="enum-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Enum`

- <span id="enum-partialeq-eq"></span>`fn eq(&self, other: &Enum) -> bool` — [`Enum`](#enum)

##### `impl Serialize for Enum`

- <span id="enum-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Enum`

##### `impl ToOwned for Enum`

- <span id="enum-toowned-type-owned"></span>`type Owned = T`

- <span id="enum-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="enum-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Enum`

- <span id="enum-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="enum-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Enum`

- <span id="enum-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="enum-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Variant`

- <span id="variant-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Variant`

- <span id="variant-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Variant`

- <span id="variant-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Variant`

- <span id="variant-clone"></span>`fn clone(&self) -> Variant` — [`Variant`](#variant)

##### `impl CloneToUninit for Variant`

- <span id="variant-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Variant`

- <span id="variant-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Variant`

- <span id="variant-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Variant`

##### `impl Eq for Variant`

##### `impl<T> From for Variant`

- <span id="variant-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Variant`

- <span id="variant-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Variant`

- <span id="variant-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Variant`

- <span id="variant-partialeq-eq"></span>`fn eq(&self, other: &Variant) -> bool` — [`Variant`](#variant)

##### `impl Serialize for Variant`

- <span id="variant-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Variant`

##### `impl ToOwned for Variant`

- <span id="variant-toowned-type-owned"></span>`type Owned = T`

- <span id="variant-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="variant-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Variant`

- <span id="variant-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="variant-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Variant`

- <span id="variant-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="variant-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Discriminant`

- <span id="discriminant-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Discriminant`

- <span id="discriminant-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Discriminant`

- <span id="discriminant-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Discriminant`

- <span id="discriminant-clone"></span>`fn clone(&self) -> Discriminant` — [`Discriminant`](#discriminant)

##### `impl CloneToUninit for Discriminant`

- <span id="discriminant-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Discriminant`

- <span id="discriminant-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Discriminant`

- <span id="discriminant-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Discriminant`

##### `impl Eq for Discriminant`

##### `impl<T> From for Discriminant`

- <span id="discriminant-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Discriminant`

- <span id="discriminant-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Discriminant`

- <span id="discriminant-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Discriminant`

- <span id="discriminant-partialeq-eq"></span>`fn eq(&self, other: &Discriminant) -> bool` — [`Discriminant`](#discriminant)

##### `impl Serialize for Discriminant`

- <span id="discriminant-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Discriminant`

##### `impl ToOwned for Discriminant`

- <span id="discriminant-toowned-type-owned"></span>`type Owned = T`

- <span id="discriminant-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="discriminant-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Discriminant`

- <span id="discriminant-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="discriminant-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Discriminant`

- <span id="discriminant-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="discriminant-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for FunctionHeader`

- <span id="functionheader-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FunctionHeader`

- <span id="functionheader-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FunctionHeader`

- <span id="functionheader-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FunctionHeader`

- <span id="functionheader-clone"></span>`fn clone(&self) -> FunctionHeader` — [`FunctionHeader`](#functionheader)

##### `impl CloneToUninit for FunctionHeader`

- <span id="functionheader-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for FunctionHeader`

- <span id="functionheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for FunctionHeader`

- <span id="functionheader-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for FunctionHeader`

##### `impl Eq for FunctionHeader`

##### `impl<T> From for FunctionHeader`

- <span id="functionheader-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for FunctionHeader`

- <span id="functionheader-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for FunctionHeader`

- <span id="functionheader-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for FunctionHeader`

- <span id="functionheader-partialeq-eq"></span>`fn eq(&self, other: &FunctionHeader) -> bool` — [`FunctionHeader`](#functionheader)

##### `impl Serialize for FunctionHeader`

- <span id="functionheader-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for FunctionHeader`

##### `impl ToOwned for FunctionHeader`

- <span id="functionheader-toowned-type-owned"></span>`type Owned = T`

- <span id="functionheader-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="functionheader-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FunctionHeader`

- <span id="functionheader-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="functionheader-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FunctionHeader`

- <span id="functionheader-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="functionheader-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Function`

- <span id="function-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Function`

- <span id="function-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Function`

- <span id="function-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Function`

- <span id="function-clone"></span>`fn clone(&self) -> Function` — [`Function`](#function)

##### `impl CloneToUninit for Function`

- <span id="function-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Function`

- <span id="function-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Function`

- <span id="function-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Function`

##### `impl Eq for Function`

##### `impl<T> From for Function`

- <span id="function-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Function`

- <span id="function-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Function`

- <span id="function-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Function`

- <span id="function-partialeq-eq"></span>`fn eq(&self, other: &Function) -> bool` — [`Function`](#function)

##### `impl Serialize for Function`

- <span id="function-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Function`

##### `impl ToOwned for Function`

- <span id="function-toowned-type-owned"></span>`type Owned = T`

- <span id="function-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="function-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Function`

- <span id="function-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="function-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Function`

- <span id="function-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="function-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Generics`

- <span id="generics-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Generics`

- <span id="generics-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Generics`

- <span id="generics-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Generics`

- <span id="generics-clone"></span>`fn clone(&self) -> Generics` — [`Generics`](#generics)

##### `impl CloneToUninit for Generics`

- <span id="generics-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Generics`

- <span id="generics-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Generics`

- <span id="generics-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Generics`

##### `impl Eq for Generics`

##### `impl<T> From for Generics`

- <span id="generics-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Generics`

- <span id="generics-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Generics`

- <span id="generics-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Generics`

- <span id="generics-partialeq-eq"></span>`fn eq(&self, other: &Generics) -> bool` — [`Generics`](#generics)

##### `impl Serialize for Generics`

- <span id="generics-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Generics`

##### `impl ToOwned for Generics`

- <span id="generics-toowned-type-owned"></span>`type Owned = T`

- <span id="generics-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="generics-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Generics`

- <span id="generics-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="generics-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Generics`

- <span id="generics-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="generics-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for GenericParamDef`

- <span id="genericparamdef-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GenericParamDef`

- <span id="genericparamdef-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GenericParamDef`

- <span id="genericparamdef-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for GenericParamDef`

- <span id="genericparamdef-clone"></span>`fn clone(&self) -> GenericParamDef` — [`GenericParamDef`](#genericparamdef)

##### `impl CloneToUninit for GenericParamDef`

- <span id="genericparamdef-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for GenericParamDef`

- <span id="genericparamdef-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for GenericParamDef`

- <span id="genericparamdef-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for GenericParamDef`

##### `impl Eq for GenericParamDef`

##### `impl<T> From for GenericParamDef`

- <span id="genericparamdef-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for GenericParamDef`

- <span id="genericparamdef-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for GenericParamDef`

- <span id="genericparamdef-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for GenericParamDef`

- <span id="genericparamdef-partialeq-eq"></span>`fn eq(&self, other: &GenericParamDef) -> bool` — [`GenericParamDef`](#genericparamdef)

##### `impl Serialize for GenericParamDef`

- <span id="genericparamdef-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for GenericParamDef`

##### `impl ToOwned for GenericParamDef`

- <span id="genericparamdef-toowned-type-owned"></span>`type Owned = T`

- <span id="genericparamdef-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="genericparamdef-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for GenericParamDef`

- <span id="genericparamdef-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="genericparamdef-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GenericParamDef`

- <span id="genericparamdef-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="genericparamdef-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Path`

- <span id="path-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Path`

- <span id="path-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Path`

- <span id="path-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Path`

- <span id="path-clone"></span>`fn clone(&self) -> Path` — [`Path`](#path)

##### `impl CloneToUninit for Path`

- <span id="path-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Path`

- <span id="path-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Path`

- <span id="path-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Path`

##### `impl Eq for Path`

##### `impl<T> From for Path`

- <span id="path-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Path`

- <span id="path-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Path`

- <span id="path-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Path`

- <span id="path-partialeq-eq"></span>`fn eq(&self, other: &Path) -> bool` — [`Path`](#path)

##### `impl Serialize for Path`

- <span id="path-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Path`

##### `impl ToOwned for Path`

- <span id="path-toowned-type-owned"></span>`type Owned = T`

- <span id="path-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="path-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Path`

- <span id="path-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="path-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Path`

- <span id="path-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="path-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for FunctionPointer`

- <span id="functionpointer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FunctionPointer`

- <span id="functionpointer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FunctionPointer`

- <span id="functionpointer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FunctionPointer`

- <span id="functionpointer-clone"></span>`fn clone(&self) -> FunctionPointer` — [`FunctionPointer`](#functionpointer)

##### `impl CloneToUninit for FunctionPointer`

- <span id="functionpointer-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for FunctionPointer`

- <span id="functionpointer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for FunctionPointer`

- <span id="functionpointer-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for FunctionPointer`

##### `impl Eq for FunctionPointer`

##### `impl<T> From for FunctionPointer`

- <span id="functionpointer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for FunctionPointer`

- <span id="functionpointer-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for FunctionPointer`

- <span id="functionpointer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for FunctionPointer`

- <span id="functionpointer-partialeq-eq"></span>`fn eq(&self, other: &FunctionPointer) -> bool` — [`FunctionPointer`](#functionpointer)

##### `impl Serialize for FunctionPointer`

- <span id="functionpointer-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for FunctionPointer`

##### `impl ToOwned for FunctionPointer`

- <span id="functionpointer-toowned-type-owned"></span>`type Owned = T`

- <span id="functionpointer-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="functionpointer-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FunctionPointer`

- <span id="functionpointer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="functionpointer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FunctionPointer`

- <span id="functionpointer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="functionpointer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for FunctionSignature`

- <span id="functionsignature-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FunctionSignature`

- <span id="functionsignature-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FunctionSignature`

- <span id="functionsignature-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FunctionSignature`

- <span id="functionsignature-clone"></span>`fn clone(&self) -> FunctionSignature` — [`FunctionSignature`](#functionsignature)

##### `impl CloneToUninit for FunctionSignature`

- <span id="functionsignature-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for FunctionSignature`

- <span id="functionsignature-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for FunctionSignature`

- <span id="functionsignature-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for FunctionSignature`

##### `impl Eq for FunctionSignature`

##### `impl<T> From for FunctionSignature`

- <span id="functionsignature-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for FunctionSignature`

- <span id="functionsignature-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for FunctionSignature`

- <span id="functionsignature-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for FunctionSignature`

- <span id="functionsignature-partialeq-eq"></span>`fn eq(&self, other: &FunctionSignature) -> bool` — [`FunctionSignature`](#functionsignature)

##### `impl Serialize for FunctionSignature`

- <span id="functionsignature-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for FunctionSignature`

##### `impl ToOwned for FunctionSignature`

- <span id="functionsignature-toowned-type-owned"></span>`type Owned = T`

- <span id="functionsignature-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="functionsignature-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FunctionSignature`

- <span id="functionsignature-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="functionsignature-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FunctionSignature`

- <span id="functionsignature-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="functionsignature-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Trait`

- <span id="trait-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Trait`

- <span id="trait-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Trait`

- <span id="trait-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Trait`

- <span id="trait-clone"></span>`fn clone(&self) -> Trait` — [`Trait`](#trait)

##### `impl CloneToUninit for Trait`

- <span id="trait-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Trait`

- <span id="trait-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Trait`

- <span id="trait-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Trait`

##### `impl Eq for Trait`

##### `impl<T> From for Trait`

- <span id="trait-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Trait`

- <span id="trait-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Trait`

- <span id="trait-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Trait`

- <span id="trait-partialeq-eq"></span>`fn eq(&self, other: &Trait) -> bool` — [`Trait`](#trait)

##### `impl Serialize for Trait`

- <span id="trait-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Trait`

##### `impl ToOwned for Trait`

- <span id="trait-toowned-type-owned"></span>`type Owned = T`

- <span id="trait-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="trait-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Trait`

- <span id="trait-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="trait-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Trait`

- <span id="trait-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="trait-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for TraitAlias`

- <span id="traitalias-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TraitAlias`

- <span id="traitalias-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TraitAlias`

- <span id="traitalias-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TraitAlias`

- <span id="traitalias-clone"></span>`fn clone(&self) -> TraitAlias` — [`TraitAlias`](#traitalias)

##### `impl CloneToUninit for TraitAlias`

- <span id="traitalias-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for TraitAlias`

- <span id="traitalias-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for TraitAlias`

- <span id="traitalias-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for TraitAlias`

##### `impl Eq for TraitAlias`

##### `impl<T> From for TraitAlias`

- <span id="traitalias-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for TraitAlias`

- <span id="traitalias-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for TraitAlias`

- <span id="traitalias-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for TraitAlias`

- <span id="traitalias-partialeq-eq"></span>`fn eq(&self, other: &TraitAlias) -> bool` — [`TraitAlias`](#traitalias)

##### `impl Serialize for TraitAlias`

- <span id="traitalias-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for TraitAlias`

##### `impl ToOwned for TraitAlias`

- <span id="traitalias-toowned-type-owned"></span>`type Owned = T`

- <span id="traitalias-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="traitalias-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TraitAlias`

- <span id="traitalias-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="traitalias-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TraitAlias`

- <span id="traitalias-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="traitalias-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Impl`

- <span id="impl-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Impl`

- <span id="impl-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Impl`

- <span id="impl-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Impl`

- <span id="impl-clone"></span>`fn clone(&self) -> Impl` — [`Impl`](#impl)

##### `impl CloneToUninit for Impl`

- <span id="impl-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Impl`

- <span id="impl-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Impl`

- <span id="impl-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Impl`

##### `impl Eq for Impl`

##### `impl<T> From for Impl`

- <span id="impl-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Impl`

- <span id="impl-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Impl`

- <span id="impl-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Impl`

- <span id="impl-partialeq-eq"></span>`fn eq(&self, other: &Impl) -> bool` — [`Impl`](#impl)

##### `impl Serialize for Impl`

- <span id="impl-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Impl`

##### `impl ToOwned for Impl`

- <span id="impl-toowned-type-owned"></span>`type Owned = T`

- <span id="impl-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="impl-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Impl`

- <span id="impl-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="impl-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Impl`

- <span id="impl-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="impl-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Use`

- <span id="use-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Use`

- <span id="use-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Use`

- <span id="use-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Use`

- <span id="use-clone"></span>`fn clone(&self) -> Use` — [`Use`](#use)

##### `impl CloneToUninit for Use`

- <span id="use-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Use`

- <span id="use-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Use`

- <span id="use-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Use`

##### `impl Eq for Use`

##### `impl<T> From for Use`

- <span id="use-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Use`

- <span id="use-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Use`

- <span id="use-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Use`

- <span id="use-partialeq-eq"></span>`fn eq(&self, other: &Use) -> bool` — [`Use`](#use)

##### `impl Serialize for Use`

- <span id="use-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Use`

##### `impl ToOwned for Use`

- <span id="use-toowned-type-owned"></span>`type Owned = T`

- <span id="use-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="use-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Use`

- <span id="use-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="use-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Use`

- <span id="use-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="use-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for ProcMacro`

- <span id="procmacro-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ProcMacro`

- <span id="procmacro-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ProcMacro`

- <span id="procmacro-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ProcMacro`

- <span id="procmacro-clone"></span>`fn clone(&self) -> ProcMacro` — [`ProcMacro`](#procmacro)

##### `impl CloneToUninit for ProcMacro`

- <span id="procmacro-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ProcMacro`

- <span id="procmacro-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for ProcMacro`

- <span id="procmacro-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for ProcMacro`

##### `impl Eq for ProcMacro`

##### `impl<T> From for ProcMacro`

- <span id="procmacro-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for ProcMacro`

- <span id="procmacro-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for ProcMacro`

- <span id="procmacro-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ProcMacro`

- <span id="procmacro-partialeq-eq"></span>`fn eq(&self, other: &ProcMacro) -> bool` — [`ProcMacro`](#procmacro)

##### `impl Serialize for ProcMacro`

- <span id="procmacro-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for ProcMacro`

##### `impl ToOwned for ProcMacro`

- <span id="procmacro-toowned-type-owned"></span>`type Owned = T`

- <span id="procmacro-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="procmacro-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ProcMacro`

- <span id="procmacro-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="procmacro-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ProcMacro`

- <span id="procmacro-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="procmacro-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for TypeAlias`

- <span id="typealias-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeAlias`

- <span id="typealias-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeAlias`

- <span id="typealias-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TypeAlias`

- <span id="typealias-clone"></span>`fn clone(&self) -> TypeAlias` — [`TypeAlias`](#typealias)

##### `impl CloneToUninit for TypeAlias`

- <span id="typealias-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for TypeAlias`

- <span id="typealias-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for TypeAlias`

- <span id="typealias-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for TypeAlias`

##### `impl Eq for TypeAlias`

##### `impl<T> From for TypeAlias`

- <span id="typealias-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for TypeAlias`

- <span id="typealias-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for TypeAlias`

- <span id="typealias-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for TypeAlias`

- <span id="typealias-partialeq-eq"></span>`fn eq(&self, other: &TypeAlias) -> bool` — [`TypeAlias`](#typealias)

##### `impl Serialize for TypeAlias`

- <span id="typealias-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for TypeAlias`

##### `impl ToOwned for TypeAlias`

- <span id="typealias-toowned-type-owned"></span>`type Owned = T`

- <span id="typealias-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typealias-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TypeAlias`

- <span id="typealias-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typealias-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeAlias`

- <span id="typealias-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typealias-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Static`

- <span id="static-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Static`

- <span id="static-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Static`

- <span id="static-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Static`

- <span id="static-clone"></span>`fn clone(&self) -> Static` — [`Static`](#static)

##### `impl CloneToUninit for Static`

- <span id="static-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Static`

- <span id="static-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Static`

- <span id="static-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Static`

##### `impl Eq for Static`

##### `impl<T> From for Static`

- <span id="static-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Static`

- <span id="static-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Static`

- <span id="static-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Static`

- <span id="static-partialeq-eq"></span>`fn eq(&self, other: &Static) -> bool` — [`Static`](#static)

##### `impl Serialize for Static`

- <span id="static-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Static`

##### `impl ToOwned for Static`

- <span id="static-toowned-type-owned"></span>`type Owned = T`

- <span id="static-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="static-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Static`

- <span id="static-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="static-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Static`

- <span id="static-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="static-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Primitive`

- <span id="primitive-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Primitive`

- <span id="primitive-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Primitive`

- <span id="primitive-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Primitive`

- <span id="primitive-clone"></span>`fn clone(&self) -> Primitive` — [`Primitive`](#primitive)

##### `impl CloneToUninit for Primitive`

- <span id="primitive-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Primitive`

- <span id="primitive-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Primitive`

- <span id="primitive-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Primitive`

##### `impl Eq for Primitive`

##### `impl<T> From for Primitive`

- <span id="primitive-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Primitive`

- <span id="primitive-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Primitive`

- <span id="primitive-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Primitive`

- <span id="primitive-partialeq-eq"></span>`fn eq(&self, other: &Primitive) -> bool` — [`Primitive`](#primitive)

##### `impl Serialize for Primitive`

- <span id="primitive-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Primitive`

##### `impl ToOwned for Primitive`

- <span id="primitive-toowned-type-owned"></span>`type Owned = T`

- <span id="primitive-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="primitive-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Primitive`

- <span id="primitive-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="primitive-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Primitive`

- <span id="primitive-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="primitive-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Attribute`

- <span id="attribute-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Attribute`

- <span id="attribute-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Attribute`

- <span id="attribute-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Attribute`

- <span id="attribute-clone"></span>`fn clone(&self) -> Attribute` — [`Attribute`](#attribute)

##### `impl CloneToUninit for Attribute`

- <span id="attribute-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Attribute`

- <span id="attribute-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Attribute`

- <span id="attribute-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Attribute`

##### `impl Eq for Attribute`

##### `impl<T> From for Attribute`

- <span id="attribute-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Attribute`

- <span id="attribute-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Attribute`

- <span id="attribute-partialeq-eq"></span>`fn eq(&self, other: &Attribute) -> bool` — [`Attribute`](#attribute)

##### `impl Serialize for Attribute`

- <span id="attribute-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Attribute`

##### `impl ToOwned for Attribute`

- <span id="attribute-toowned-type-owned"></span>`type Owned = T`

- <span id="attribute-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="attribute-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Attribute`

- <span id="attribute-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="attribute-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Attribute`

- <span id="attribute-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="attribute-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for ReprKind`

- <span id="reprkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReprKind`

- <span id="reprkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReprKind`

- <span id="reprkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ReprKind`

- <span id="reprkind-clone"></span>`fn clone(&self) -> ReprKind` — [`ReprKind`](#reprkind)

##### `impl CloneToUninit for ReprKind`

- <span id="reprkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ReprKind`

- <span id="reprkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for ReprKind`

- <span id="reprkind-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for ReprKind`

##### `impl Eq for ReprKind`

##### `impl<T> From for ReprKind`

- <span id="reprkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ReprKind`

- <span id="reprkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ReprKind`

- <span id="reprkind-partialeq-eq"></span>`fn eq(&self, other: &ReprKind) -> bool` — [`ReprKind`](#reprkind)

##### `impl Serialize for ReprKind`

- <span id="reprkind-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for ReprKind`

##### `impl ToOwned for ReprKind`

- <span id="reprkind-toowned-type-owned"></span>`type Owned = T`

- <span id="reprkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="reprkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ReprKind`

- <span id="reprkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="reprkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReprKind`

- <span id="reprkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="reprkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Visibility`

- <span id="visibility-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Visibility`

- <span id="visibility-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Visibility`

- <span id="visibility-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Visibility`

- <span id="visibility-clone"></span>`fn clone(&self) -> Visibility` — [`Visibility`](#visibility)

##### `impl CloneToUninit for Visibility`

- <span id="visibility-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Visibility`

- <span id="visibility-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Visibility`

- <span id="visibility-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Visibility`

##### `impl Eq for Visibility`

##### `impl<T> From for Visibility`

- <span id="visibility-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Visibility`

- <span id="visibility-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Visibility`

- <span id="visibility-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Visibility`

- <span id="visibility-partialeq-eq"></span>`fn eq(&self, other: &Visibility) -> bool` — [`Visibility`](#visibility)

##### `impl Serialize for Visibility`

- <span id="visibility-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Visibility`

##### `impl ToOwned for Visibility`

- <span id="visibility-toowned-type-owned"></span>`type Owned = T`

- <span id="visibility-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="visibility-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Visibility`

- <span id="visibility-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="visibility-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Visibility`

- <span id="visibility-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="visibility-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for GenericArgs`

- <span id="genericargs-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GenericArgs`

- <span id="genericargs-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GenericArgs`

- <span id="genericargs-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for GenericArgs`

- <span id="genericargs-clone"></span>`fn clone(&self) -> GenericArgs` — [`GenericArgs`](#genericargs)

##### `impl CloneToUninit for GenericArgs`

- <span id="genericargs-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for GenericArgs`

- <span id="genericargs-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for GenericArgs`

- <span id="genericargs-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for GenericArgs`

##### `impl Eq for GenericArgs`

##### `impl<T> From for GenericArgs`

- <span id="genericargs-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for GenericArgs`

- <span id="genericargs-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for GenericArgs`

- <span id="genericargs-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for GenericArgs`

- <span id="genericargs-partialeq-eq"></span>`fn eq(&self, other: &GenericArgs) -> bool` — [`GenericArgs`](#genericargs)

##### `impl Serialize for GenericArgs`

- <span id="genericargs-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for GenericArgs`

##### `impl ToOwned for GenericArgs`

- <span id="genericargs-toowned-type-owned"></span>`type Owned = T`

- <span id="genericargs-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="genericargs-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for GenericArgs`

- <span id="genericargs-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="genericargs-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GenericArgs`

- <span id="genericargs-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="genericargs-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for GenericArg`

- <span id="genericarg-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GenericArg`

- <span id="genericarg-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GenericArg`

- <span id="genericarg-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for GenericArg`

- <span id="genericarg-clone"></span>`fn clone(&self) -> GenericArg` — [`GenericArg`](#genericarg)

##### `impl CloneToUninit for GenericArg`

- <span id="genericarg-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for GenericArg`

- <span id="genericarg-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for GenericArg`

- <span id="genericarg-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for GenericArg`

##### `impl Eq for GenericArg`

##### `impl<T> From for GenericArg`

- <span id="genericarg-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for GenericArg`

- <span id="genericarg-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for GenericArg`

- <span id="genericarg-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for GenericArg`

- <span id="genericarg-partialeq-eq"></span>`fn eq(&self, other: &GenericArg) -> bool` — [`GenericArg`](#genericarg)

##### `impl Serialize for GenericArg`

- <span id="genericarg-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for GenericArg`

##### `impl ToOwned for GenericArg`

- <span id="genericarg-toowned-type-owned"></span>`type Owned = T`

- <span id="genericarg-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="genericarg-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for GenericArg`

- <span id="genericarg-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="genericarg-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GenericArg`

- <span id="genericarg-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="genericarg-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for AssocItemConstraintKind`

- <span id="associtemconstraintkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AssocItemConstraintKind`

- <span id="associtemconstraintkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AssocItemConstraintKind`

- <span id="associtemconstraintkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AssocItemConstraintKind`

- <span id="associtemconstraintkind-clone"></span>`fn clone(&self) -> AssocItemConstraintKind` — [`AssocItemConstraintKind`](#associtemconstraintkind)

##### `impl CloneToUninit for AssocItemConstraintKind`

- <span id="associtemconstraintkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for AssocItemConstraintKind`

- <span id="associtemconstraintkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for AssocItemConstraintKind`

- <span id="associtemconstraintkind-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for AssocItemConstraintKind`

##### `impl Eq for AssocItemConstraintKind`

##### `impl<T> From for AssocItemConstraintKind`

- <span id="associtemconstraintkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for AssocItemConstraintKind`

- <span id="associtemconstraintkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for AssocItemConstraintKind`

- <span id="associtemconstraintkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for AssocItemConstraintKind`

- <span id="associtemconstraintkind-partialeq-eq"></span>`fn eq(&self, other: &AssocItemConstraintKind) -> bool` — [`AssocItemConstraintKind`](#associtemconstraintkind)

##### `impl Serialize for AssocItemConstraintKind`

- <span id="associtemconstraintkind-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for AssocItemConstraintKind`

##### `impl ToOwned for AssocItemConstraintKind`

- <span id="associtemconstraintkind-toowned-type-owned"></span>`type Owned = T`

- <span id="associtemconstraintkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="associtemconstraintkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AssocItemConstraintKind`

- <span id="associtemconstraintkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="associtemconstraintkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AssocItemConstraintKind`

- <span id="associtemconstraintkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="associtemconstraintkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for ItemKind`

- <span id="itemkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemKind`

- <span id="itemkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemKind`

- <span id="itemkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ItemKind`

- <span id="itemkind-clone"></span>`fn clone(&self) -> ItemKind` — [`ItemKind`](#itemkind)

##### `impl CloneToUninit for ItemKind`

- <span id="itemkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ItemKind`

##### `impl Debug for ItemKind`

- <span id="itemkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for ItemKind`

- <span id="itemkind-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for ItemKind`

##### `impl Eq for ItemKind`

##### `impl<T> From for ItemKind`

- <span id="itemkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for ItemKind`

- <span id="itemkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for ItemKind`

- <span id="itemkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ItemKind`

- <span id="itemkind-partialeq-eq"></span>`fn eq(&self, other: &ItemKind) -> bool` — [`ItemKind`](#itemkind)

##### `impl Serialize for ItemKind`

- <span id="itemkind-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for ItemKind`

##### `impl ToOwned for ItemKind`

- <span id="itemkind-toowned-type-owned"></span>`type Owned = T`

- <span id="itemkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ItemKind`

- <span id="itemkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemKind`

- <span id="itemkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for ItemEnum`

- <span id="itemenum-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemEnum`

- <span id="itemenum-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemEnum`

- <span id="itemenum-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ItemEnum`

- <span id="itemenum-clone"></span>`fn clone(&self) -> ItemEnum` — [`ItemEnum`](#itemenum)

##### `impl CloneToUninit for ItemEnum`

- <span id="itemenum-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ItemEnum`

- <span id="itemenum-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for ItemEnum`

- <span id="itemenum-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for ItemEnum`

##### `impl Eq for ItemEnum`

##### `impl<T> From for ItemEnum`

- <span id="itemenum-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for ItemEnum`

- <span id="itemenum-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for ItemEnum`

- <span id="itemenum-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ItemEnum`

- <span id="itemenum-partialeq-eq"></span>`fn eq(&self, other: &ItemEnum) -> bool` — [`ItemEnum`](#itemenum)

##### `impl Serialize for ItemEnum`

- <span id="itemenum-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for ItemEnum`

##### `impl ToOwned for ItemEnum`

- <span id="itemenum-toowned-type-owned"></span>`type Owned = T`

- <span id="itemenum-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemenum-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ItemEnum`

- <span id="itemenum-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemenum-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemEnum`

- <span id="itemenum-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemenum-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for StructKind`

- <span id="structkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StructKind`

- <span id="structkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StructKind`

- <span id="structkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StructKind`

- <span id="structkind-clone"></span>`fn clone(&self) -> StructKind` — [`StructKind`](#structkind)

##### `impl CloneToUninit for StructKind`

- <span id="structkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for StructKind`

- <span id="structkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for StructKind`

- <span id="structkind-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for StructKind`

##### `impl Eq for StructKind`

##### `impl<T> From for StructKind`

- <span id="structkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for StructKind`

- <span id="structkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for StructKind`

- <span id="structkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for StructKind`

- <span id="structkind-partialeq-eq"></span>`fn eq(&self, other: &StructKind) -> bool` — [`StructKind`](#structkind)

##### `impl Serialize for StructKind`

- <span id="structkind-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for StructKind`

##### `impl ToOwned for StructKind`

- <span id="structkind-toowned-type-owned"></span>`type Owned = T`

- <span id="structkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="structkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StructKind`

- <span id="structkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="structkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StructKind`

- <span id="structkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="structkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for VariantKind`

- <span id="variantkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VariantKind`

- <span id="variantkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VariantKind`

- <span id="variantkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for VariantKind`

- <span id="variantkind-clone"></span>`fn clone(&self) -> VariantKind` — [`VariantKind`](#variantkind)

##### `impl CloneToUninit for VariantKind`

- <span id="variantkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for VariantKind`

- <span id="variantkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for VariantKind`

- <span id="variantkind-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for VariantKind`

##### `impl Eq for VariantKind`

##### `impl<T> From for VariantKind`

- <span id="variantkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for VariantKind`

- <span id="variantkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for VariantKind`

- <span id="variantkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for VariantKind`

- <span id="variantkind-partialeq-eq"></span>`fn eq(&self, other: &VariantKind) -> bool` — [`VariantKind`](#variantkind)

##### `impl Serialize for VariantKind`

- <span id="variantkind-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for VariantKind`

##### `impl ToOwned for VariantKind`

- <span id="variantkind-toowned-type-owned"></span>`type Owned = T`

- <span id="variantkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="variantkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for VariantKind`

- <span id="variantkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="variantkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VariantKind`

- <span id="variantkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="variantkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Abi`

- <span id="abi-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Abi`

- <span id="abi-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Abi`

- <span id="abi-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Abi`

- <span id="abi-clone"></span>`fn clone(&self) -> Abi` — [`Abi`](#abi)

##### `impl CloneToUninit for Abi`

- <span id="abi-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Abi`

- <span id="abi-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Abi`

- <span id="abi-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Abi`

##### `impl Eq for Abi`

##### `impl<T> From for Abi`

- <span id="abi-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Abi`

- <span id="abi-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Abi`

- <span id="abi-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Abi`

- <span id="abi-partialeq-eq"></span>`fn eq(&self, other: &Abi) -> bool` — [`Abi`](#abi)

##### `impl Serialize for Abi`

- <span id="abi-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Abi`

##### `impl ToOwned for Abi`

- <span id="abi-toowned-type-owned"></span>`type Owned = T`

- <span id="abi-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="abi-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Abi`

- <span id="abi-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="abi-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Abi`

- <span id="abi-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="abi-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for GenericParamDefKind`

- <span id="genericparamdefkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GenericParamDefKind`

- <span id="genericparamdefkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GenericParamDefKind`

- <span id="genericparamdefkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for GenericParamDefKind`

- <span id="genericparamdefkind-clone"></span>`fn clone(&self) -> GenericParamDefKind` — [`GenericParamDefKind`](#genericparamdefkind)

##### `impl CloneToUninit for GenericParamDefKind`

- <span id="genericparamdefkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for GenericParamDefKind`

- <span id="genericparamdefkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for GenericParamDefKind`

- <span id="genericparamdefkind-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for GenericParamDefKind`

##### `impl Eq for GenericParamDefKind`

##### `impl<T> From for GenericParamDefKind`

- <span id="genericparamdefkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for GenericParamDefKind`

- <span id="genericparamdefkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for GenericParamDefKind`

- <span id="genericparamdefkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for GenericParamDefKind`

- <span id="genericparamdefkind-partialeq-eq"></span>`fn eq(&self, other: &GenericParamDefKind) -> bool` — [`GenericParamDefKind`](#genericparamdefkind)

##### `impl Serialize for GenericParamDefKind`

- <span id="genericparamdefkind-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for GenericParamDefKind`

##### `impl ToOwned for GenericParamDefKind`

- <span id="genericparamdefkind-toowned-type-owned"></span>`type Owned = T`

- <span id="genericparamdefkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="genericparamdefkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for GenericParamDefKind`

- <span id="genericparamdefkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="genericparamdefkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GenericParamDefKind`

- <span id="genericparamdefkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="genericparamdefkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for WherePredicate`

- <span id="wherepredicate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WherePredicate`

- <span id="wherepredicate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WherePredicate`

- <span id="wherepredicate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for WherePredicate`

- <span id="wherepredicate-clone"></span>`fn clone(&self) -> WherePredicate` — [`WherePredicate`](#wherepredicate)

##### `impl CloneToUninit for WherePredicate`

- <span id="wherepredicate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for WherePredicate`

- <span id="wherepredicate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for WherePredicate`

- <span id="wherepredicate-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for WherePredicate`

##### `impl Eq for WherePredicate`

##### `impl<T> From for WherePredicate`

- <span id="wherepredicate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for WherePredicate`

- <span id="wherepredicate-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for WherePredicate`

- <span id="wherepredicate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for WherePredicate`

- <span id="wherepredicate-partialeq-eq"></span>`fn eq(&self, other: &WherePredicate) -> bool` — [`WherePredicate`](#wherepredicate)

##### `impl Serialize for WherePredicate`

- <span id="wherepredicate-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for WherePredicate`

##### `impl ToOwned for WherePredicate`

- <span id="wherepredicate-toowned-type-owned"></span>`type Owned = T`

- <span id="wherepredicate-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="wherepredicate-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for WherePredicate`

- <span id="wherepredicate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="wherepredicate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WherePredicate`

- <span id="wherepredicate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="wherepredicate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for GenericBound`

- <span id="genericbound-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GenericBound`

- <span id="genericbound-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GenericBound`

- <span id="genericbound-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for GenericBound`

- <span id="genericbound-clone"></span>`fn clone(&self) -> GenericBound` — [`GenericBound`](#genericbound)

##### `impl CloneToUninit for GenericBound`

- <span id="genericbound-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for GenericBound`

- <span id="genericbound-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for GenericBound`

- <span id="genericbound-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for GenericBound`

##### `impl Eq for GenericBound`

##### `impl<T> From for GenericBound`

- <span id="genericbound-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for GenericBound`

- <span id="genericbound-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for GenericBound`

- <span id="genericbound-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for GenericBound`

- <span id="genericbound-partialeq-eq"></span>`fn eq(&self, other: &GenericBound) -> bool` — [`GenericBound`](#genericbound)

##### `impl Serialize for GenericBound`

- <span id="genericbound-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for GenericBound`

##### `impl ToOwned for GenericBound`

- <span id="genericbound-toowned-type-owned"></span>`type Owned = T`

- <span id="genericbound-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="genericbound-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for GenericBound`

- <span id="genericbound-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="genericbound-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GenericBound`

- <span id="genericbound-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="genericbound-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for TraitBoundModifier`

- <span id="traitboundmodifier-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TraitBoundModifier`

- <span id="traitboundmodifier-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TraitBoundModifier`

- <span id="traitboundmodifier-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TraitBoundModifier`

- <span id="traitboundmodifier-clone"></span>`fn clone(&self) -> TraitBoundModifier` — [`TraitBoundModifier`](#traitboundmodifier)

##### `impl CloneToUninit for TraitBoundModifier`

- <span id="traitboundmodifier-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for TraitBoundModifier`

##### `impl Debug for TraitBoundModifier`

- <span id="traitboundmodifier-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for TraitBoundModifier`

- <span id="traitboundmodifier-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for TraitBoundModifier`

##### `impl Eq for TraitBoundModifier`

##### `impl<T> From for TraitBoundModifier`

- <span id="traitboundmodifier-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for TraitBoundModifier`

- <span id="traitboundmodifier-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for TraitBoundModifier`

- <span id="traitboundmodifier-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for TraitBoundModifier`

- <span id="traitboundmodifier-partialeq-eq"></span>`fn eq(&self, other: &TraitBoundModifier) -> bool` — [`TraitBoundModifier`](#traitboundmodifier)

##### `impl Serialize for TraitBoundModifier`

- <span id="traitboundmodifier-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for TraitBoundModifier`

##### `impl ToOwned for TraitBoundModifier`

- <span id="traitboundmodifier-toowned-type-owned"></span>`type Owned = T`

- <span id="traitboundmodifier-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="traitboundmodifier-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TraitBoundModifier`

- <span id="traitboundmodifier-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="traitboundmodifier-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TraitBoundModifier`

- <span id="traitboundmodifier-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="traitboundmodifier-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for PreciseCapturingArg`

- <span id="precisecapturingarg-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PreciseCapturingArg`

- <span id="precisecapturingarg-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PreciseCapturingArg`

- <span id="precisecapturingarg-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for PreciseCapturingArg`

- <span id="precisecapturingarg-clone"></span>`fn clone(&self) -> PreciseCapturingArg` — [`PreciseCapturingArg`](#precisecapturingarg)

##### `impl CloneToUninit for PreciseCapturingArg`

- <span id="precisecapturingarg-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for PreciseCapturingArg`

- <span id="precisecapturingarg-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for PreciseCapturingArg`

- <span id="precisecapturingarg-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for PreciseCapturingArg`

##### `impl Eq for PreciseCapturingArg`

##### `impl<T> From for PreciseCapturingArg`

- <span id="precisecapturingarg-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for PreciseCapturingArg`

- <span id="precisecapturingarg-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for PreciseCapturingArg`

- <span id="precisecapturingarg-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for PreciseCapturingArg`

- <span id="precisecapturingarg-partialeq-eq"></span>`fn eq(&self, other: &PreciseCapturingArg) -> bool` — [`PreciseCapturingArg`](#precisecapturingarg)

##### `impl Serialize for PreciseCapturingArg`

- <span id="precisecapturingarg-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for PreciseCapturingArg`

##### `impl ToOwned for PreciseCapturingArg`

- <span id="precisecapturingarg-toowned-type-owned"></span>`type Owned = T`

- <span id="precisecapturingarg-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="precisecapturingarg-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PreciseCapturingArg`

- <span id="precisecapturingarg-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="precisecapturingarg-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PreciseCapturingArg`

- <span id="precisecapturingarg-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="precisecapturingarg-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Term`

- <span id="term-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Term`

- <span id="term-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Term`

- <span id="term-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Term`

- <span id="term-clone"></span>`fn clone(&self) -> Term` — [`Term`](#term)

##### `impl CloneToUninit for Term`

- <span id="term-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Term`

- <span id="term-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Term`

- <span id="term-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Term`

##### `impl Eq for Term`

##### `impl<T> From for Term`

- <span id="term-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Term`

- <span id="term-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Term`

- <span id="term-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Term`

- <span id="term-partialeq-eq"></span>`fn eq(&self, other: &Term) -> bool` — [`Term`](#term)

##### `impl Serialize for Term`

- <span id="term-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Term`

##### `impl ToOwned for Term`

- <span id="term-toowned-type-owned"></span>`type Owned = T`

- <span id="term-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="term-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Term`

- <span id="term-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="term-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Term`

- <span id="term-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="term-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Type`

- <span id="type-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Type`

- <span id="type-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Type`

- <span id="type-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Type`

- <span id="type-clone"></span>`fn clone(&self) -> Type` — [`Type`](#type)

##### `impl CloneToUninit for Type`

- <span id="type-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Type`

- <span id="type-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Type`

- <span id="type-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Type`

##### `impl Eq for Type`

##### `impl<T> From for Type`

- <span id="type-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Type`

- <span id="type-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Type`

- <span id="type-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Type`

- <span id="type-partialeq-eq"></span>`fn eq(&self, other: &Type) -> bool` — [`Type`](#type)

##### `impl Serialize for Type`

- <span id="type-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Type`

##### `impl ToOwned for Type`

- <span id="type-toowned-type-owned"></span>`type Owned = T`

- <span id="type-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="type-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Type`

- <span id="type-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="type-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Type`

- <span id="type-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="type-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for MacroKind`

- <span id="macrokind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MacroKind`

- <span id="macrokind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MacroKind`

- <span id="macrokind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MacroKind`

- <span id="macrokind-clone"></span>`fn clone(&self) -> MacroKind` — [`MacroKind`](#macrokind)

##### `impl CloneToUninit for MacroKind`

- <span id="macrokind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for MacroKind`

##### `impl Debug for MacroKind`

- <span id="macrokind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for MacroKind`

- <span id="macrokind-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for MacroKind`

##### `impl Eq for MacroKind`

##### `impl<T> From for MacroKind`

- <span id="macrokind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for MacroKind`

- <span id="macrokind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for MacroKind`

- <span id="macrokind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for MacroKind`

- <span id="macrokind-partialeq-eq"></span>`fn eq(&self, other: &MacroKind) -> bool` — [`MacroKind`](#macrokind)

##### `impl Serialize for MacroKind`

- <span id="macrokind-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for MacroKind`

##### `impl ToOwned for MacroKind`

- <span id="macrokind-toowned-type-owned"></span>`type Owned = T`

- <span id="macrokind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="macrokind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MacroKind`

- <span id="macrokind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="macrokind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MacroKind`

- <span id="macrokind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="macrokind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

