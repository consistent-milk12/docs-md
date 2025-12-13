# Crate `cargo_metadata`

Structured access to the output of `cargo metadata` and `cargo --message-format=json`.
Usually used from within a `cargo-*` executable

See the [cargo book](https://doc.rust-lang.org/cargo/index.html) for
details on cargo itself.

## Examples

Get the current crate's metadata without default features but with all dependency information.

```rust
use std::path::Path;
use cargo_metadata::{MetadataCommand, CargoOpt};
let _metadata = MetadataCommand::new().exec().unwrap();
```


If you have a program that takes `--manifest-path` as an argument, you can forward that
to [MetadataCommand]:

```rust
use cargo_metadata::MetadataCommand;
use std::path::Path;
let mut args = std::env::args().skip_while(|val| !val.starts_with("--manifest-path"));
let mut cmd = MetadataCommand::new();
let manifest_path = match args.next() {
    Some(ref p) if p == "--manifest-path" => {
        cmd.manifest_path(args.next().unwrap());
    }
    Some(p) => {
        cmd.manifest_path(p.trim_start_matches("--manifest-path="));
    }
    None => {}
};

let _metadata = cmd.exec().unwrap();
```

Pass features flags, e.g. `--all-features`.

```rust
use std::path::Path;
use cargo_metadata::{MetadataCommand, CargoOpt};
let _metadata = MetadataCommand::new()
    .manifest_path("./Cargo.toml")
    .features(CargoOpt::AllFeatures)
    .exec()
    .unwrap();
```

Parse message-format output produced by other cargo commands.
It is recommended to use crates like `escargot` to produce the [Command].

```rust
use std::process::{Stdio, Command};
use cargo_metadata::Message;
let mut command = Command::new("cargo")
    .args(&["build", "--message-format=json-render-diagnostics"])
    .stdout(Stdio::piped())
    .spawn()
    .unwrap();

let reader = std::io::BufReader::new(command.stdout.take().unwrap());
for message in cargo_metadata::Message::parse_stream(reader) {
    match message.unwrap() {
        Message::CompilerMessage(msg) => {
            println!("{:?}", msg);
        },
        Message::CompilerArtifact(artifact) => {
            println!("{:?}", artifact);
        },
        Message::BuildScriptExecuted(script) => {
            println!("{:?}", script);
        },
        Message::BuildFinished(finished) => {
            println!("{:?}", finished);
        },
        _ => () // Unknown message
    }
}

let output = command.wait().expect("Couldn't get cargo's exit status");
```

## Contents

- [Modules](#modules)
  - [`dependency`](#dependency)
  - [`diagnostic`](#diagnostic)
  - [`errors`](#errors)
  - [`messages`](#messages)
- [Structs](#structs)
  - [`Dependency`](#dependency)
  - [`Artifact`](#artifact)
  - [`ArtifactProfile`](#artifactprofile)
  - [`BuildFinished`](#buildfinished)
  - [`BuildScript`](#buildscript)
  - [`CompilerMessage`](#compilermessage)
  - [`MessageIter`](#messageiter)
  - [`FeatureName`](#featurename)
  - [`PackageName`](#packagename)
  - [`PackageId`](#packageid)
  - [`Metadata`](#metadata)
  - [`WorkspaceDefaultMembers`](#workspacedefaultmembers)
  - [`Resolve`](#resolve)
  - [`Node`](#node)
  - [`NodeDep`](#nodedep)
  - [`DepKindInfo`](#depkindinfo)
  - [`Package`](#package)
  - [`Source`](#source)
  - [`Target`](#target)
  - [`MetadataCommand`](#metadatacommand)
- [Enums](#enums)
  - [`DependencyKind`](#dependencykind)
  - [`Error`](#error)
  - [`ArtifactDebuginfo`](#artifactdebuginfo)
  - [`Message`](#message)
  - [`TargetKind`](#targetkind)
  - [`CrateType`](#cratetype)
  - [`Edition`](#edition)
  - [`CargoOpt`](#cargoopt)
- [Functions](#functions)
  - [`parse_messages`](#parse-messages)
  - [`is_null`](#is-null)
  - [`default_true`](#default-true)
  - [`deserialize_rust_version`](#deserialize-rust-version)
- [Type Aliases](#type-aliases)
  - [`Result`](#result)
- [Macros](#macros)
  - [`str_newtype!`](#str-newtype)
  - [`methods_target_is_kind!`](#methods-target-is-kind)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`dependency`](#dependency) | mod | This module contains `Dependency` and the types/functions it uses for deserialization. |
| [`diagnostic`](#diagnostic) | mod | This module contains `Diagnostic` and the types/functions it uses for deserialization. |
| [`errors`](#errors) | mod |  |
| [`messages`](#messages) | mod |  |
| [`Dependency`](#dependency) | struct |  |
| [`Artifact`](#artifact) | struct |  |
| [`ArtifactProfile`](#artifactprofile) | struct |  |
| [`BuildFinished`](#buildfinished) | struct |  |
| [`BuildScript`](#buildscript) | struct |  |
| [`CompilerMessage`](#compilermessage) | struct |  |
| [`MessageIter`](#messageiter) | struct |  |
| [`FeatureName`](#featurename) | struct | Feature name newtype |
| [`PackageName`](#packagename) | struct | Package name newtype |
| [`PackageId`](#packageid) | struct | An "opaque" identifier for a package. |
| [`Metadata`](#metadata) | struct | Starting point for metadata returned by `cargo metadata` |
| [`WorkspaceDefaultMembers`](#workspacedefaultmembers) | struct | A list of default workspace members. |
| [`Resolve`](#resolve) | struct | A dependency graph |
| [`Node`](#node) | struct | A node in a dependencies graph |
| [`NodeDep`](#nodedep) | struct | A dependency in a node |
| [`DepKindInfo`](#depkindinfo) | struct | Information about a dependency kind. |
| [`Package`](#package) | struct | One or more crates described by a single `Cargo.toml` |
| [`Source`](#source) | struct | The source of a package such as crates.io. |
| [`Target`](#target) | struct | A single target (lib, bin, example, ...) provided by a crate |
| [`MetadataCommand`](#metadatacommand) | struct | A builder for configuring `cargo metadata` invocation. |
| [`DependencyKind`](#dependencykind) | enum |  |
| [`Error`](#error) | enum |  |
| [`ArtifactDebuginfo`](#artifactdebuginfo) | enum |  |
| [`Message`](#message) | enum |  |
| [`TargetKind`](#targetkind) | enum | Kind of target. |
| [`CrateType`](#cratetype) | enum | Similar to `kind`, but only reports the [Cargo crate types](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-crate-type-field) |
| [`Edition`](#edition) | enum | The Rust edition |
| [`CargoOpt`](#cargoopt) | enum | Cargo features flags |
| [`parse_messages`](#parse-messages) | fn |  |
| [`is_null`](#is-null) | fn | Helpers for default metadata fields |
| [`default_true`](#default-true) | fn |  |
| [`deserialize_rust_version`](#deserialize-rust-version) | fn | As per the Cargo Book the [`rust-version` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field) must |
| [`Result`](#result) | type |  |
| [`str_newtype!`](#str-newtype) | macro |  |
| [`methods_target_is_kind!`](#methods-target-is-kind) | macro |  |

## Modules

- [`dependency`](dependency/index.md) — This module contains `Dependency` and the types/functions it uses for deserialization.
- [`diagnostic`](diagnostic/index.md) — This module contains `Diagnostic` and the types/functions it uses for deserialization.
- [`errors`](errors/index.md)
- [`messages`](messages/index.md)

## Structs

### `Dependency`

```rust
struct Dependency {
    pub name: String,
    pub source: Option<crate::Source>,
    pub req: semver::VersionReq,
    pub kind: DependencyKind,
    pub optional: bool,
    pub uses_default_features: bool,
    pub features: Vec<String>,
    pub target: Option<Platform>,
    pub rename: Option<String>,
    pub registry: Option<String>,
    pub path: Option<camino::Utf8PathBuf>,
}
```

*Defined in [`cargo_metadata-0.23.1/src/dependency.rs:52-85`](../../.source_1765521767/cargo_metadata-0.23.1/src/dependency.rs#L52-L85)*

A dependency of the main crate

#### Fields

- **`name`**: `String`

  Name as given in the `Cargo.toml`

- **`source`**: `Option<crate::Source>`

  The source of dependency

- **`req`**: `semver::VersionReq`

  The required version

- **`kind`**: `DependencyKind`

  The kind of dependency this is

- **`optional`**: `bool`

  Whether this dependency is required or optional

- **`uses_default_features`**: `bool`

  Whether the default features in this dependency are used.

- **`features`**: `Vec<String>`

  The list of features enabled for this dependency.

- **`target`**: `Option<Platform>`

  The target this dependency is specific to.
  
  Use the [`Display`]() trait to access the contents.
  

- **`rename`**: `Option<String>`

  If the dependency is renamed, this is the new name for the dependency
  as a string.  None if it is not renamed.

- **`registry`**: `Option<String>`

  The URL of the index of the registry where this dependency is from.
  
  If None, the dependency is from crates.io.

- **`path`**: `Option<camino::Utf8PathBuf>`

  The file system path for a local path dependency.
  
  Only produced on cargo 1.51+

#### Trait Implementations

##### `impl Any for Dependency`

- <span id="dependency-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Dependency`

- <span id="dependency-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Dependency`

- <span id="dependency-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Dependency`

- <span id="dependency-clone"></span>`fn clone(&self) -> Dependency` — [`Dependency`](dependency/index.md#dependency)

##### `impl CloneToUninit for Dependency`

- <span id="dependency-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Dependency`

- <span id="dependency-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Dependency`

- <span id="dependency-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Dependency`

##### `impl Eq for Dependency`

##### `impl<T> From for Dependency`

- <span id="dependency-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Dependency`

- <span id="dependency-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Dependency`

- <span id="dependency-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Dependency`

- <span id="dependency-partialeq-eq"></span>`fn eq(&self, other: &Dependency) -> bool` — [`Dependency`](dependency/index.md#dependency)

##### `impl Serialize for Dependency`

- <span id="dependency-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Dependency`

##### `impl ToOwned for Dependency`

- <span id="dependency-toowned-type-owned"></span>`type Owned = T`

- <span id="dependency-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dependency-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Dependency`

- <span id="dependency-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dependency-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Dependency`

- <span id="dependency-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dependency-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Artifact`

```rust
struct Artifact {
    pub package_id: super::PackageId,
    pub manifest_path: camino::Utf8PathBuf,
    pub target: super::Target,
    pub profile: ArtifactProfile,
    pub features: Vec<String>,
    pub filenames: Vec<camino::Utf8PathBuf>,
    pub executable: Option<camino::Utf8PathBuf>,
    pub fresh: bool,
}
```

*Defined in [`cargo_metadata-0.23.1/src/messages.rs:156-175`](../../.source_1765521767/cargo_metadata-0.23.1/src/messages.rs#L156-L175)*

A compiler-generated file.

#### Fields

- **`package_id`**: `super::PackageId`

  The package this artifact belongs to

- **`manifest_path`**: `camino::Utf8PathBuf`

  Path to the `Cargo.toml` file

- **`target`**: `super::Target`

  The target this artifact was compiled for

- **`profile`**: `ArtifactProfile`

  The profile this artifact was compiled with

- **`features`**: `Vec<String>`

  The enabled features for this artifact

- **`filenames`**: `Vec<camino::Utf8PathBuf>`

  The full paths to the generated artifacts
  (e.g. binary file and separate debug info)

- **`executable`**: `Option<camino::Utf8PathBuf>`

  Path to the executable file

- **`fresh`**: `bool`

  If true, then the files were already generated

#### Trait Implementations

##### `impl Any for Artifact`

- <span id="artifact-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Artifact`

- <span id="artifact-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Artifact`

- <span id="artifact-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Artifact`

- <span id="artifact-clone"></span>`fn clone(&self) -> Artifact` — [`Artifact`](messages/index.md#artifact)

##### `impl CloneToUninit for Artifact`

- <span id="artifact-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Artifact`

- <span id="artifact-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Artifact`

- <span id="artifact-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Artifact`

##### `impl Eq for Artifact`

##### `impl<T> From for Artifact`

- <span id="artifact-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Artifact`

- <span id="artifact-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Artifact`

- <span id="artifact-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Artifact`

- <span id="artifact-partialeq-eq"></span>`fn eq(&self, other: &Artifact) -> bool` — [`Artifact`](messages/index.md#artifact)

##### `impl Serialize for Artifact`

- <span id="artifact-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Artifact`

##### `impl ToOwned for Artifact`

- <span id="artifact-toowned-type-owned"></span>`type Owned = T`

- <span id="artifact-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="artifact-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Artifact`

- <span id="artifact-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="artifact-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Artifact`

- <span id="artifact-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="artifact-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ArtifactProfile`

```rust
struct ArtifactProfile {
    pub opt_level: String,
    pub debuginfo: ArtifactDebuginfo,
    pub debug_assertions: bool,
    pub overflow_checks: bool,
    pub test: bool,
}
```

*Defined in [`cargo_metadata-0.23.1/src/messages.rs:15-28`](../../.source_1765521767/cargo_metadata-0.23.1/src/messages.rs#L15-L28)*

Profile settings used to determine which compiler flags to use for a
target.

#### Fields

- **`opt_level`**: `String`

  Optimization level. Possible values are 0-3, s or z.

- **`debuginfo`**: `ArtifactDebuginfo`

  The kind of debug information.

- **`debug_assertions`**: `bool`

  State of the `cfg(debug_assertions)` directive, enabling macros like
  `debug_assert!`

- **`overflow_checks`**: `bool`

  State of the overflow checks.

- **`test`**: `bool`

  Whether this profile is a test

#### Trait Implementations

##### `impl Any for ArtifactProfile`

- <span id="artifactprofile-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArtifactProfile`

- <span id="artifactprofile-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArtifactProfile`

- <span id="artifactprofile-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ArtifactProfile`

- <span id="artifactprofile-clone"></span>`fn clone(&self) -> ArtifactProfile` — [`ArtifactProfile`](messages/index.md#artifactprofile)

##### `impl CloneToUninit for ArtifactProfile`

- <span id="artifactprofile-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ArtifactProfile`

- <span id="artifactprofile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for ArtifactProfile`

- <span id="artifactprofile-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for ArtifactProfile`

##### `impl Eq for ArtifactProfile`

##### `impl<T> From for ArtifactProfile`

- <span id="artifactprofile-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for ArtifactProfile`

- <span id="artifactprofile-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for ArtifactProfile`

- <span id="artifactprofile-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ArtifactProfile`

- <span id="artifactprofile-partialeq-eq"></span>`fn eq(&self, other: &ArtifactProfile) -> bool` — [`ArtifactProfile`](messages/index.md#artifactprofile)

##### `impl Serialize for ArtifactProfile`

- <span id="artifactprofile-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for ArtifactProfile`

##### `impl ToOwned for ArtifactProfile`

- <span id="artifactprofile-toowned-type-owned"></span>`type Owned = T`

- <span id="artifactprofile-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="artifactprofile-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ArtifactProfile`

- <span id="artifactprofile-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="artifactprofile-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArtifactProfile`

- <span id="artifactprofile-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="artifactprofile-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BuildFinished`

```rust
struct BuildFinished {
    pub success: bool,
}
```

*Defined in [`cargo_metadata-0.23.1/src/messages.rs:220-223`](../../.source_1765521767/cargo_metadata-0.23.1/src/messages.rs#L220-L223)*

Final result of a build.

#### Fields

- **`success`**: `bool`

  Whether or not the build finished successfully.

#### Trait Implementations

##### `impl Any for BuildFinished`

- <span id="buildfinished-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BuildFinished`

- <span id="buildfinished-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BuildFinished`

- <span id="buildfinished-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for BuildFinished`

- <span id="buildfinished-clone"></span>`fn clone(&self) -> BuildFinished` — [`BuildFinished`](messages/index.md#buildfinished)

##### `impl CloneToUninit for BuildFinished`

- <span id="buildfinished-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for BuildFinished`

- <span id="buildfinished-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for BuildFinished`

- <span id="buildfinished-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for BuildFinished`

##### `impl Eq for BuildFinished`

##### `impl<T> From for BuildFinished`

- <span id="buildfinished-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for BuildFinished`

- <span id="buildfinished-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for BuildFinished`

- <span id="buildfinished-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for BuildFinished`

- <span id="buildfinished-partialeq-eq"></span>`fn eq(&self, other: &BuildFinished) -> bool` — [`BuildFinished`](messages/index.md#buildfinished)

##### `impl Serialize for BuildFinished`

- <span id="buildfinished-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for BuildFinished`

##### `impl ToOwned for BuildFinished`

- <span id="buildfinished-toowned-type-owned"></span>`type Owned = T`

- <span id="buildfinished-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="buildfinished-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for BuildFinished`

- <span id="buildfinished-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="buildfinished-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BuildFinished`

- <span id="buildfinished-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="buildfinished-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BuildScript`

```rust
struct BuildScript {
    pub package_id: super::PackageId,
    pub linked_libs: Vec<camino::Utf8PathBuf>,
    pub linked_paths: Vec<camino::Utf8PathBuf>,
    pub cfgs: Vec<String>,
    pub env: Vec<(String, String)>,
    pub out_dir: camino::Utf8PathBuf,
}
```

*Defined in [`cargo_metadata-0.23.1/src/messages.rs:197-213`](../../.source_1765521767/cargo_metadata-0.23.1/src/messages.rs#L197-L213)*

Output of a build script execution.

#### Fields

- **`package_id`**: `super::PackageId`

  The package this build script execution belongs to

- **`linked_libs`**: `Vec<camino::Utf8PathBuf>`

  The libs to link

- **`linked_paths`**: `Vec<camino::Utf8PathBuf>`

  The paths to search when resolving libs

- **`cfgs`**: `Vec<String>`

  Various `--cfg` flags to pass to the compiler

- **`env`**: `Vec<(String, String)>`

  The environment variables to add to the compilation

- **`out_dir`**: `camino::Utf8PathBuf`

  The `OUT_DIR` environment variable where this script places its output
  
  Added in Rust 1.41.

#### Trait Implementations

##### `impl Any for BuildScript`

- <span id="buildscript-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BuildScript`

- <span id="buildscript-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BuildScript`

- <span id="buildscript-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for BuildScript`

- <span id="buildscript-clone"></span>`fn clone(&self) -> BuildScript` — [`BuildScript`](messages/index.md#buildscript)

##### `impl CloneToUninit for BuildScript`

- <span id="buildscript-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for BuildScript`

- <span id="buildscript-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for BuildScript`

- <span id="buildscript-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for BuildScript`

##### `impl Eq for BuildScript`

##### `impl<T> From for BuildScript`

- <span id="buildscript-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for BuildScript`

- <span id="buildscript-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for BuildScript`

- <span id="buildscript-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for BuildScript`

- <span id="buildscript-partialeq-eq"></span>`fn eq(&self, other: &BuildScript) -> bool` — [`BuildScript`](messages/index.md#buildscript)

##### `impl Serialize for BuildScript`

- <span id="buildscript-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for BuildScript`

##### `impl ToOwned for BuildScript`

- <span id="buildscript-toowned-type-owned"></span>`type Owned = T`

- <span id="buildscript-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="buildscript-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for BuildScript`

- <span id="buildscript-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="buildscript-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BuildScript`

- <span id="buildscript-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="buildscript-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CompilerMessage`

```rust
struct CompilerMessage {
    pub package_id: super::PackageId,
    pub target: super::Target,
    pub message: diagnostic::Diagnostic,
}
```

*Defined in [`cargo_metadata-0.23.1/src/messages.rs:183-190`](../../.source_1765521767/cargo_metadata-0.23.1/src/messages.rs#L183-L190)*

Message left by the compiler

#### Fields

- **`package_id`**: `super::PackageId`

  The package this message belongs to

- **`target`**: `super::Target`

  The target this message is aimed at

- **`message`**: `diagnostic::Diagnostic`

  The message the compiler sent.

#### Trait Implementations

##### `impl Any for CompilerMessage`

- <span id="compilermessage-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CompilerMessage`

- <span id="compilermessage-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CompilerMessage`

- <span id="compilermessage-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CompilerMessage`

- <span id="compilermessage-clone"></span>`fn clone(&self) -> CompilerMessage` — [`CompilerMessage`](messages/index.md#compilermessage)

##### `impl CloneToUninit for CompilerMessage`

- <span id="compilermessage-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for CompilerMessage`

- <span id="compilermessage-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for CompilerMessage`

- <span id="compilermessage-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for CompilerMessage`

##### `impl Display for CompilerMessage`

- <span id="compilermessage-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CompilerMessage`

##### `impl<T> From for CompilerMessage`

- <span id="compilermessage-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for CompilerMessage`

- <span id="compilermessage-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for CompilerMessage`

- <span id="compilermessage-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for CompilerMessage`

- <span id="compilermessage-partialeq-eq"></span>`fn eq(&self, other: &CompilerMessage) -> bool` — [`CompilerMessage`](messages/index.md#compilermessage)

##### `impl Serialize for CompilerMessage`

- <span id="compilermessage-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for CompilerMessage`

##### `impl ToOwned for CompilerMessage`

- <span id="compilermessage-toowned-type-owned"></span>`type Owned = T`

- <span id="compilermessage-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="compilermessage-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for CompilerMessage`

- <span id="compilermessage-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for CompilerMessage`

- <span id="compilermessage-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="compilermessage-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CompilerMessage`

- <span id="compilermessage-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="compilermessage-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MessageIter<R>`

```rust
struct MessageIter<R> {
    input: R,
}
```

*Defined in [`cargo_metadata-0.23.1/src/messages.rs:262-264`](../../.source_1765521767/cargo_metadata-0.23.1/src/messages.rs#L262-L264)*

An iterator of Messages.

#### Trait Implementations

##### `impl Any for MessageIter<R>`

- <span id="messageiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MessageIter<R>`

- <span id="messageiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MessageIter<R>`

- <span id="messageiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for MessageIter<R>`

- <span id="messageiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MessageIter<R>`

- <span id="messageiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for MessageIter<R>`

- <span id="messageiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="messageiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="messageiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: BufRead> Iterator for MessageIter<R>`

- <span id="messageiter-iterator-type-item"></span>`type Item = Result<Message, Error>`

- <span id="messageiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for MessageIter<R>`

- <span id="messageiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="messageiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MessageIter<R>`

- <span id="messageiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="messageiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FeatureName<T: AsRef<str>>`

```rust
struct FeatureName<T: AsRef<str>>(T);
```

*Defined in [`cargo_metadata-0.23.1/src/lib.rs:209-219`](../../.source_1765521767/cargo_metadata-0.23.1/src/lib.rs#L209-L219)*

Feature name newtype

Based on [cargo-util-schema's string newtype] but with two crucial differences:

- This newtype does not verify the wrapped string.
- This newtype allows comparison with arbitrary types that implement `AsRef<str>`.


#### Implementations

- <span id="featurename-into-inner"></span>`fn into_inner(self) -> T`

  Convert the wrapped string into its inner type `T`

#### Trait Implementations

##### `impl<T> Any for FeatureName<T>`

- <span id="featurename-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T: AsRef<str>> AsRef for FeatureName<T>`

- <span id="featurename-asref-as-ref"></span>`fn as_ref(&self) -> &str`

##### `impl<T> Borrow for FeatureName<T>`

- <span id="featurename-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FeatureName<T>`

- <span id="featurename-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone + AsRef<str>> Clone for FeatureName<T>`

- <span id="featurename-clone"></span>`fn clone(&self) -> FeatureName<T>` — [`FeatureName`](#featurename)

##### `impl<T> CloneToUninit for FeatureName<T>`

- <span id="featurename-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug + AsRef<str>> Debug for FeatureName<T>`

- <span id="featurename-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: AsRef<str>> Deref for FeatureName<T>`

- <span id="featurename-deref-type-target"></span>`type Target = T`

- <span id="featurename-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T: AsRef<str> + serde::Deserialize<'de>> Deserialize for FeatureName<T>`

- <span id="featurename-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>` — [`Result`](errors/index.md#result)

##### `impl<T> DeserializeOwned for FeatureName<T>`

##### `impl<T: AsRef<str>> Display for FeatureName<T>`

- <span id="featurename-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq + AsRef<str>> Eq for FeatureName<T>`

##### `impl<T> From for FeatureName<T>`

- <span id="featurename-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromStr for FeatureName<String>`

- <span id="featurename-fromstr-type-err"></span>`type Err = Infallible`

- <span id="featurename-fromstr-from-str"></span>`fn from_str(value: &str) -> Result<Self, <Self as >::Err>` — [`Result`](errors/index.md#result)

##### `impl<T: hash::Hash + AsRef<str>> Hash for FeatureName<T>`

- <span id="featurename-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T, U> Into for FeatureName<T>`

- <span id="featurename-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::Ord + AsRef<str>> Ord for FeatureName<T>`

- <span id="featurename-ord-cmp"></span>`fn cmp(&self, other: &FeatureName<T>) -> cmp::Ordering` — [`FeatureName`](#featurename)

##### `impl<T: AsRef<str>, Rhs: AsRef<str>> PartialEq for FeatureName<T>`

- <span id="featurename-partialeq-eq"></span>`fn eq(&self, other: &Rhs) -> bool`

##### `impl<T: cmp::PartialOrd + AsRef<str>> PartialOrd for FeatureName<T>`

- <span id="featurename-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &FeatureName<T>) -> option::Option<cmp::Ordering>` — [`FeatureName`](#featurename)

##### `impl<T> Receiver for FeatureName<T>`

- <span id="featurename-receiver-type-target"></span>`type Target = T`

##### `impl<T> Serialize for FeatureName<T>`

- <span id="featurename-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl<T> ToOwned for FeatureName<T>`

- <span id="featurename-toowned-type-owned"></span>`type Owned = T`

- <span id="featurename-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="featurename-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T> ToString for FeatureName<T>`

- <span id="featurename-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T, U> TryFrom for FeatureName<T>`

- <span id="featurename-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="featurename-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for FeatureName<T>`

- <span id="featurename-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="featurename-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PackageName<T: AsRef<str>>`

```rust
struct PackageName<T: AsRef<str>>(T);
```

*Defined in [`cargo_metadata-0.23.1/src/lib.rs:221-231`](../../.source_1765521767/cargo_metadata-0.23.1/src/lib.rs#L221-L231)*

Package name newtype

Based on [cargo-util-schema's string newtype] but with two crucial differences:

- This newtype does not verify the wrapped string.
- This newtype allows comparison with arbitrary types that implement `AsRef<str>`.


#### Implementations

- <span id="packagename-into-inner"></span>`fn into_inner(self) -> T`

  Convert the wrapped string into its inner type `T`

#### Trait Implementations

##### `impl<T> Any for PackageName<T>`

- <span id="packagename-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T: AsRef<str>> AsRef for PackageName<T>`

- <span id="packagename-asref-as-ref"></span>`fn as_ref(&self) -> &str`

##### `impl<T> Borrow for PackageName<T>`

- <span id="packagename-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PackageName<T>`

- <span id="packagename-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone + AsRef<str>> Clone for PackageName<T>`

- <span id="packagename-clone"></span>`fn clone(&self) -> PackageName<T>` — [`PackageName`](#packagename)

##### `impl<T> CloneToUninit for PackageName<T>`

- <span id="packagename-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug + AsRef<str>> Debug for PackageName<T>`

- <span id="packagename-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: AsRef<str>> Deref for PackageName<T>`

- <span id="packagename-deref-type-target"></span>`type Target = T`

- <span id="packagename-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T: AsRef<str> + serde::Deserialize<'de>> Deserialize for PackageName<T>`

- <span id="packagename-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>` — [`Result`](errors/index.md#result)

##### `impl<T> DeserializeOwned for PackageName<T>`

##### `impl<T: AsRef<str>> Display for PackageName<T>`

- <span id="packagename-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq + AsRef<str>> Eq for PackageName<T>`

##### `impl<T> From for PackageName<T>`

- <span id="packagename-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromStr for PackageName<String>`

- <span id="packagename-fromstr-type-err"></span>`type Err = Infallible`

- <span id="packagename-fromstr-from-str"></span>`fn from_str(value: &str) -> Result<Self, <Self as >::Err>` — [`Result`](errors/index.md#result)

##### `impl<T: hash::Hash + AsRef<str>> Hash for PackageName<T>`

- <span id="packagename-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T, U> Into for PackageName<T>`

- <span id="packagename-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::Ord + AsRef<str>> Ord for PackageName<T>`

- <span id="packagename-ord-cmp"></span>`fn cmp(&self, other: &PackageName<T>) -> cmp::Ordering` — [`PackageName`](#packagename)

##### `impl<T: AsRef<str>, Rhs: AsRef<str>> PartialEq for PackageName<T>`

- <span id="packagename-partialeq-eq"></span>`fn eq(&self, other: &Rhs) -> bool`

##### `impl<T: cmp::PartialOrd + AsRef<str>> PartialOrd for PackageName<T>`

- <span id="packagename-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &PackageName<T>) -> option::Option<cmp::Ordering>` — [`PackageName`](#packagename)

##### `impl<T> Receiver for PackageName<T>`

- <span id="packagename-receiver-type-target"></span>`type Target = T`

##### `impl<T> Serialize for PackageName<T>`

- <span id="packagename-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl<T> ToOwned for PackageName<T>`

- <span id="packagename-toowned-type-owned"></span>`type Owned = T`

- <span id="packagename-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="packagename-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T> ToString for PackageName<T>`

- <span id="packagename-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T, U> TryFrom for PackageName<T>`

- <span id="packagename-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="packagename-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for PackageName<T>`

- <span id="packagename-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="packagename-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PackageId`

```rust
struct PackageId {
    pub repr: String,
}
```

*Defined in [`cargo_metadata-0.23.1/src/lib.rs:241-244`](../../.source_1765521767/cargo_metadata-0.23.1/src/lib.rs#L241-L244)*

An "opaque" identifier for a package.

It is possible to inspect the `repr` field, if the need arises, but its
precise format is an implementation detail and is subject to change.

`Metadata` can be indexed by `PackageId`.

#### Fields

- **`repr`**: `String`

  The underlying string representation of id.

#### Trait Implementations

##### `impl Any for PackageId`

- <span id="packageid-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PackageId`

- <span id="packageid-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PackageId`

- <span id="packageid-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for PackageId`

- <span id="packageid-clone"></span>`fn clone(&self) -> PackageId` — [`PackageId`](#packageid)

##### `impl CloneToUninit for PackageId`

- <span id="packageid-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for PackageId`

- <span id="packageid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for PackageId`

- <span id="packageid-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for PackageId`

##### `impl Display for PackageId`

- <span id="packageid-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for PackageId`

##### `impl<T> From for PackageId`

- <span id="packageid-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for PackageId`

- <span id="packageid-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Index for Metadata`

- <span id="metadata-index-type-output"></span>`type Output = Package`

- <span id="metadata-index"></span>`fn index(&self, idx: &'a PackageId) -> &<Self as >::Output` — [`PackageId`](#packageid)

##### `impl<U> Into for PackageId`

- <span id="packageid-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for PackageId`

- <span id="packageid-ord-cmp"></span>`fn cmp(&self, other: &PackageId) -> cmp::Ordering` — [`PackageId`](#packageid)

##### `impl PartialEq for PackageId`

- <span id="packageid-partialeq-eq"></span>`fn eq(&self, other: &PackageId) -> bool` — [`PackageId`](#packageid)

##### `impl PartialOrd for PackageId`

- <span id="packageid-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &PackageId) -> option::Option<cmp::Ordering>` — [`PackageId`](#packageid)

##### `impl Serialize for PackageId`

- <span id="packageid-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for PackageId`

##### `impl ToOwned for PackageId`

- <span id="packageid-toowned-type-owned"></span>`type Owned = T`

- <span id="packageid-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="packageid-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for PackageId`

- <span id="packageid-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for PackageId`

- <span id="packageid-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="packageid-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PackageId`

- <span id="packageid-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="packageid-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Metadata`

```rust
struct Metadata {
    pub packages: Vec<Package>,
    pub workspace_members: Vec<PackageId>,
    pub workspace_default_members: WorkspaceDefaultMembers,
    pub resolve: Option<Resolve>,
    pub workspace_root: camino::Utf8PathBuf,
    pub target_directory: camino::Utf8PathBuf,
    pub build_directory: Option<camino::Utf8PathBuf>,
    pub workspace_metadata: serde_json::Value,
    version: usize,
}
```

*Defined in [`cargo_metadata-0.23.1/src/lib.rs:262-289`](../../.source_1765521767/cargo_metadata-0.23.1/src/lib.rs#L262-L289)*

Starting point for metadata returned by `cargo metadata`

#### Fields

- **`packages`**: `Vec<Package>`

  A list of all crates referenced by this crate (and the crate itself)

- **`workspace_members`**: `Vec<PackageId>`

  A list of all workspace members

- **`workspace_default_members`**: `WorkspaceDefaultMembers`

  The list of default workspace members
  
  This is not available if running with a version of Cargo older than 1.71.
  
  You can check whether it is available or missing using respectively
  `WorkspaceDefaultMembers::is_available` and `WorkspaceDefaultMembers::is_missing`.

- **`resolve`**: `Option<Resolve>`

  Dependencies graph

- **`workspace_root`**: `camino::Utf8PathBuf`

  Workspace root

- **`target_directory`**: `camino::Utf8PathBuf`

  Target directory

- **`build_directory`**: `Option<camino::Utf8PathBuf>`

  Build directory

- **`workspace_metadata`**: `serde_json::Value`

  The workspace-level metadata object. Null if non-existent.

- **`version`**: `usize`

  The metadata format version

#### Implementations

- <span id="metadata-root-package"></span>`fn root_package(&self) -> Option<&Package>` — [`Package`](#package)

  Get the workspace's root package of this metadata instance.

- <span id="metadata-workspace-packages"></span>`fn workspace_packages(&self) -> Vec<&Package>` — [`Package`](#package)

  Get the workspace packages.

- <span id="metadata-workspace-default-packages"></span>`fn workspace_default_packages(&self) -> Vec<&Package>` — [`Package`](#package)

  Get the workspace default packages.

  

  # Panics

  

  This will panic if running with a version of Cargo older than 1.71.

#### Trait Implementations

##### `impl Any for Metadata`

- <span id="metadata-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Metadata`

- <span id="metadata-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Metadata`

- <span id="metadata-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Metadata`

- <span id="metadata-clone"></span>`fn clone(&self) -> Metadata` — [`Metadata`](#metadata)

##### `impl CloneToUninit for Metadata`

- <span id="metadata-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Metadata`

- <span id="metadata-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Metadata`

- <span id="metadata-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Metadata`

##### `impl Eq for Metadata`

##### `impl<T> From for Metadata`

- <span id="metadata-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Metadata`

- <span id="metadata-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Index for Metadata`

- <span id="metadata-index-type-output"></span>`type Output = Package`

- <span id="metadata-index"></span>`fn index(&self, idx: &'a PackageId) -> &<Self as >::Output` — [`PackageId`](#packageid)

##### `impl<U> Into for Metadata`

- <span id="metadata-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Metadata`

- <span id="metadata-partialeq-eq"></span>`fn eq(&self, other: &Metadata) -> bool` — [`Metadata`](#metadata)

##### `impl Serialize for Metadata`

- <span id="metadata-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Metadata`

##### `impl ToOwned for Metadata`

- <span id="metadata-toowned-type-owned"></span>`type Owned = T`

- <span id="metadata-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="metadata-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Metadata`

- <span id="metadata-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="metadata-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Metadata`

- <span id="metadata-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="metadata-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WorkspaceDefaultMembers`

```rust
struct WorkspaceDefaultMembers(Option<Vec<PackageId>>);
```

*Defined in [`cargo_metadata-0.23.1/src/lib.rs:353`](../../.source_1765521767/cargo_metadata-0.23.1/src/lib.rs#L353)*

A list of default workspace members.

See `Metadata::workspace_default_members`.

It is only available if running a version of Cargo of 1.71 or newer.

# Panics

Dereferencing when running an older version of Cargo will panic.

#### Implementations

- <span id="workspacedefaultmembers-is-available"></span>`fn is_available(&self) -> bool`

  Return `true` if the list of workspace default members is supported by

  the called cargo-metadata version and `false` otherwise.

  

  In particular useful when parsing the output of `cargo-metadata` for

  versions of Cargo < 1.71, as dereferencing [`WorkspaceDefaultMembers`](#workspacedefaultmembers)

  for these versions will panic.

  

  Opposite of `WorkspaceDefaultMembers::is_missing`.

- <span id="workspacedefaultmembers-is-missing"></span>`fn is_missing(&self) -> bool`

  Return `false` if the list of workspace default members is supported by

  the called cargo-metadata version and `true` otherwise.

  

  In particular useful when parsing the output of `cargo-metadata` for

  versions of Cargo < 1.71, as dereferencing [`WorkspaceDefaultMembers`](#workspacedefaultmembers)

  for these versions will panic.

  

  Opposite of `WorkspaceDefaultMembers::is_available`.

#### Trait Implementations

##### `impl Any for WorkspaceDefaultMembers`

- <span id="workspacedefaultmembers-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WorkspaceDefaultMembers`

- <span id="workspacedefaultmembers-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WorkspaceDefaultMembers`

- <span id="workspacedefaultmembers-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for WorkspaceDefaultMembers`

- <span id="workspacedefaultmembers-clone"></span>`fn clone(&self) -> WorkspaceDefaultMembers` — [`WorkspaceDefaultMembers`](#workspacedefaultmembers)

##### `impl CloneToUninit for WorkspaceDefaultMembers`

- <span id="workspacedefaultmembers-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for WorkspaceDefaultMembers`

- <span id="workspacedefaultmembers-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for WorkspaceDefaultMembers`

- <span id="workspacedefaultmembers-default"></span>`fn default() -> WorkspaceDefaultMembers` — [`WorkspaceDefaultMembers`](#workspacedefaultmembers)

##### `impl Deref for WorkspaceDefaultMembers`

- <span id="workspacedefaultmembers-deref-type-target"></span>`type Target = [PackageId]`

- <span id="workspacedefaultmembers-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Deserialize for WorkspaceDefaultMembers`

- <span id="workspacedefaultmembers-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for WorkspaceDefaultMembers`

##### `impl Eq for WorkspaceDefaultMembers`

##### `impl<T> From for WorkspaceDefaultMembers`

- <span id="workspacedefaultmembers-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for WorkspaceDefaultMembers`

- <span id="workspacedefaultmembers-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for WorkspaceDefaultMembers`

- <span id="workspacedefaultmembers-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for WorkspaceDefaultMembers`

- <span id="workspacedefaultmembers-partialeq-eq"></span>`fn eq(&self, other: &WorkspaceDefaultMembers) -> bool` — [`WorkspaceDefaultMembers`](#workspacedefaultmembers)

##### `impl Receiver for WorkspaceDefaultMembers`

- <span id="workspacedefaultmembers-receiver-type-target"></span>`type Target = T`

##### `impl Serialize for WorkspaceDefaultMembers`

- <span id="workspacedefaultmembers-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for WorkspaceDefaultMembers`

##### `impl ToOwned for WorkspaceDefaultMembers`

- <span id="workspacedefaultmembers-toowned-type-owned"></span>`type Owned = T`

- <span id="workspacedefaultmembers-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="workspacedefaultmembers-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for WorkspaceDefaultMembers`

- <span id="workspacedefaultmembers-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="workspacedefaultmembers-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WorkspaceDefaultMembers`

- <span id="workspacedefaultmembers-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="workspacedefaultmembers-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Resolve`

```rust
struct Resolve {
    pub nodes: Vec<Node>,
    pub root: Option<PackageId>,
}
```

*Defined in [`cargo_metadata-0.23.1/src/lib.rs:396-402`](../../.source_1765521767/cargo_metadata-0.23.1/src/lib.rs#L396-L402)*

A dependency graph

#### Fields

- **`nodes`**: `Vec<Node>`

  Nodes in a dependencies graph

- **`root`**: `Option<PackageId>`

  The crate for which the metadata was read.

#### Trait Implementations

##### `impl Any for Resolve`

- <span id="resolve-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Resolve`

- <span id="resolve-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Resolve`

- <span id="resolve-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Resolve`

- <span id="resolve-clone"></span>`fn clone(&self) -> Resolve` — [`Resolve`](#resolve)

##### `impl CloneToUninit for Resolve`

- <span id="resolve-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Resolve`

- <span id="resolve-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Resolve`

- <span id="resolve-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Resolve`

##### `impl Eq for Resolve`

##### `impl<T> From for Resolve`

- <span id="resolve-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Resolve`

- <span id="resolve-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Index for Resolve`

- <span id="resolve-index-type-output"></span>`type Output = Node`

- <span id="resolve-index"></span>`fn index(&self, idx: &'a PackageId) -> &<Self as >::Output` — [`PackageId`](#packageid)

##### `impl<U> Into for Resolve`

- <span id="resolve-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Resolve`

- <span id="resolve-partialeq-eq"></span>`fn eq(&self, other: &Resolve) -> bool` — [`Resolve`](#resolve)

##### `impl Serialize for Resolve`

- <span id="resolve-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Resolve`

##### `impl ToOwned for Resolve`

- <span id="resolve-toowned-type-owned"></span>`type Owned = T`

- <span id="resolve-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="resolve-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Resolve`

- <span id="resolve-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="resolve-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Resolve`

- <span id="resolve-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="resolve-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Node`

```rust
struct Node {
    pub id: PackageId,
    pub deps: Vec<NodeDep>,
    pub dependencies: Vec<PackageId>,
    pub features: Vec<FeatureName>,
}
```

*Defined in [`cargo_metadata-0.23.1/src/lib.rs:420-436`](../../.source_1765521767/cargo_metadata-0.23.1/src/lib.rs#L420-L436)*

A node in a dependencies graph

#### Fields

- **`id`**: `PackageId`

  An opaque identifier for a package

- **`deps`**: `Vec<NodeDep>`

  Dependencies in a structured format.
  
  `deps` handles renamed dependencies whereas `dependencies` does not.

- **`dependencies`**: `Vec<PackageId>`

  List of opaque identifiers for this node's dependencies.
  It doesn't support renamed dependencies. See `deps`.

- **`features`**: `Vec<FeatureName>`

  Features enabled on the crate

#### Trait Implementations

##### `impl Any for Node`

- <span id="node-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Node`

- <span id="node-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Node`

- <span id="node-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Node`

- <span id="node-clone"></span>`fn clone(&self) -> Node` — [`Node`](#node)

##### `impl CloneToUninit for Node`

- <span id="node-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Node`

- <span id="node-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Node`

- <span id="node-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Node`

##### `impl Eq for Node`

##### `impl<T> From for Node`

- <span id="node-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Node`

- <span id="node-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Node`

- <span id="node-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Node`

- <span id="node-partialeq-eq"></span>`fn eq(&self, other: &Node) -> bool` — [`Node`](#node)

##### `impl Serialize for Node`

- <span id="node-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Node`

##### `impl ToOwned for Node`

- <span id="node-toowned-type-owned"></span>`type Owned = T`

- <span id="node-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="node-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Node`

- <span id="node-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="node-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Node`

- <span id="node-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="node-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `NodeDep`

```rust
struct NodeDep {
    pub name: String,
    pub pkg: PackageId,
    pub dep_kinds: Vec<DepKindInfo>,
}
```

*Defined in [`cargo_metadata-0.23.1/src/lib.rs:443-460`](../../.source_1765521767/cargo_metadata-0.23.1/src/lib.rs#L443-L460)*

A dependency in a node

#### Fields

- **`name`**: `String`

  The name of the dependency's library target.
  If the crate was renamed, it is the new name.
  
  If -Zbindeps is enabled local references may result in an empty
  string.
  
  After -Zbindeps gets stabilized, cargo has indicated this field
  will become deprecated.

- **`pkg`**: `PackageId`

  Package ID (opaque unique identifier)

- **`dep_kinds`**: `Vec<DepKindInfo>`

  The kinds of dependencies.
  
  This field was added in Rust 1.41.

#### Trait Implementations

##### `impl Any for NodeDep`

- <span id="nodedep-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NodeDep`

- <span id="nodedep-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NodeDep`

- <span id="nodedep-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for NodeDep`

- <span id="nodedep-clone"></span>`fn clone(&self) -> NodeDep` — [`NodeDep`](#nodedep)

##### `impl CloneToUninit for NodeDep`

- <span id="nodedep-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for NodeDep`

- <span id="nodedep-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for NodeDep`

- <span id="nodedep-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for NodeDep`

##### `impl Eq for NodeDep`

##### `impl<T> From for NodeDep`

- <span id="nodedep-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for NodeDep`

- <span id="nodedep-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for NodeDep`

- <span id="nodedep-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for NodeDep`

- <span id="nodedep-partialeq-eq"></span>`fn eq(&self, other: &NodeDep) -> bool` — [`NodeDep`](#nodedep)

##### `impl Serialize for NodeDep`

- <span id="nodedep-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for NodeDep`

##### `impl ToOwned for NodeDep`

- <span id="nodedep-toowned-type-owned"></span>`type Owned = T`

- <span id="nodedep-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="nodedep-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for NodeDep`

- <span id="nodedep-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="nodedep-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NodeDep`

- <span id="nodedep-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="nodedep-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DepKindInfo`

```rust
struct DepKindInfo {
    pub kind: DependencyKind,
    pub target: Option<dependency::Platform>,
}
```

*Defined in [`cargo_metadata-0.23.1/src/lib.rs:467-483`](../../.source_1765521767/cargo_metadata-0.23.1/src/lib.rs#L467-L483)*

Information about a dependency kind.

#### Fields

- **`kind`**: `DependencyKind`

  The kind of dependency.

- **`target`**: `Option<dependency::Platform>`

  The target platform for the dependency.
  
  This is `None` if it is not a target dependency.
  
  Use the [`Display`]() trait to access the contents.
  
  By default all platform dependencies are included in the resolve
  graph. Use Cargo's `--filter-platform` flag if you only want to
  include dependencies for a specific platform.
  

#### Trait Implementations

##### `impl Any for DepKindInfo`

- <span id="depkindinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DepKindInfo`

- <span id="depkindinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DepKindInfo`

- <span id="depkindinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DepKindInfo`

- <span id="depkindinfo-clone"></span>`fn clone(&self) -> DepKindInfo` — [`DepKindInfo`](#depkindinfo)

##### `impl CloneToUninit for DepKindInfo`

- <span id="depkindinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for DepKindInfo`

- <span id="depkindinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for DepKindInfo`

- <span id="depkindinfo-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for DepKindInfo`

##### `impl Eq for DepKindInfo`

##### `impl<T> From for DepKindInfo`

- <span id="depkindinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for DepKindInfo`

- <span id="depkindinfo-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for DepKindInfo`

- <span id="depkindinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for DepKindInfo`

- <span id="depkindinfo-partialeq-eq"></span>`fn eq(&self, other: &DepKindInfo) -> bool` — [`DepKindInfo`](#depkindinfo)

##### `impl Serialize for DepKindInfo`

- <span id="depkindinfo-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for DepKindInfo`

##### `impl ToOwned for DepKindInfo`

- <span id="depkindinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="depkindinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="depkindinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DepKindInfo`

- <span id="depkindinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="depkindinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DepKindInfo`

- <span id="depkindinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="depkindinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Package`

```rust
struct Package {
    pub name: PackageName,
    pub version: semver::Version,
    pub authors: Vec<String>,
    pub id: PackageId,
    pub source: Option<Source>,
    pub description: Option<String>,
    pub dependencies: Vec<Dependency>,
    pub license: Option<String>,
    pub license_file: Option<camino::Utf8PathBuf>,
    pub targets: Vec<Target>,
    pub features: std::collections::BTreeMap<String, Vec<String>>,
    pub manifest_path: camino::Utf8PathBuf,
    pub categories: Vec<String>,
    pub keywords: Vec<String>,
    pub readme: Option<camino::Utf8PathBuf>,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub edition: Edition,
    pub metadata: serde_json::Value,
    pub links: Option<String>,
    pub publish: Option<Vec<String>>,
    pub default_run: Option<String>,
    pub rust_version: Option<semver::Version>,
}
```

*Defined in [`cargo_metadata-0.23.1/src/lib.rs:493-614`](../../.source_1765521767/cargo_metadata-0.23.1/src/lib.rs#L493-L614)*

One or more crates described by a single `Cargo.toml`

Each `target` of a `Package` will be built as a crate.
For more information, see <https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html>.

#### Fields

- **`name`**: `PackageName`

  The [`name` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-name-field) as given in the `Cargo.toml`

- **`version`**: `semver::Version`

  The [`version` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field) as specified in the `Cargo.toml`

- **`authors`**: `Vec<String>`

  The [`authors` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-authors-field) as specified in the `Cargo.toml`

- **`id`**: `PackageId`

  An opaque identifier for a package

- **`source`**: `Option<Source>`

  The source of the package, e.g.
  crates.io or `None` for local projects.

- **`description`**: `Option<String>`

  The [`description` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-description-field) as specified in the `Cargo.toml`

- **`dependencies`**: `Vec<Dependency>`

  List of dependencies of this particular package

- **`license`**: `Option<String>`

  The [`license` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-license-and-license-file-fields) as specified in the `Cargo.toml`

- **`license_file`**: `Option<camino::Utf8PathBuf>`

  The [`license-file` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-license-and-license-file-fields) as specified in the `Cargo.toml`.
  If the package is using a nonstandard license, this key may be specified instead of
  `license`, and must point to a file relative to the manifest.

- **`targets`**: `Vec<Target>`

  Targets provided by the crate (lib, bin, example, test, ...)

- **`features`**: `std::collections::BTreeMap<String, Vec<String>>`

  Features provided by the crate, mapped to the features required by that feature.

- **`manifest_path`**: `camino::Utf8PathBuf`

  Path containing the `Cargo.toml`

- **`categories`**: `Vec<String>`

  The [`categories` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-categories-field) as specified in the `Cargo.toml`

- **`keywords`**: `Vec<String>`

  The [`keywords` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-keywords-field) as specified in the `Cargo.toml`

- **`readme`**: `Option<camino::Utf8PathBuf>`

  The [`readme` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-readme-field) as specified in the `Cargo.toml`

- **`repository`**: `Option<String>`

  The [`repository` URL](https://doc.rust-lang.org/cargo/reference/manifest.html#the-repository-field) as specified in the `Cargo.toml`

- **`homepage`**: `Option<String>`

  The [`homepage` URL](https://doc.rust-lang.org/cargo/reference/manifest.html#the-homepage-field) as specified in the `Cargo.toml`.
  
  On versions of cargo before 1.49, this will always be [`None`](#none).

- **`documentation`**: `Option<String>`

  The [`documentation` URL](https://doc.rust-lang.org/cargo/reference/manifest.html#the-documentation-field) as specified in the `Cargo.toml`.
  
  On versions of cargo before 1.49, this will always be [`None`](#none).

- **`edition`**: `Edition`

  The default Rust edition for the package (either what's specified in the [`edition` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-edition-field)
  or defaulting to [`Edition::E2015`](#editione2015)).
  
  Beware that individual targets may specify their own edition in
  `Target::edition`.

- **`metadata`**: `serde_json::Value`

  Contents of the free form [`package.metadata` section](https://doc.rust-lang.org/cargo/reference/manifest.html#the-metadata-table).
  
  This contents can be serialized to a struct using serde:
  
  ```rust
  use serde::Deserialize;
  use serde_json::json;
  
  #[derive(Debug, Deserialize)]
  struct SomePackageMetadata {
      some_value: i32,
  }
  
  let value = json!({
      "some_value": 42,
  });
  
  let package_metadata: SomePackageMetadata = serde_json::from_value(value).unwrap();
  assert_eq!(package_metadata.some_value, 42);
  
  ```

- **`links`**: `Option<String>`

  The name of a native library the package is linking to.

- **`publish`**: `Option<Vec<String>>`

  List of registries to which this package may be published (derived from the [`publish` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-publish-field)).
  
  Publishing is unrestricted if `None`, and forbidden if the `Vec` is empty.
  
  This is always `None` if running with a version of Cargo older than 1.39.

- **`default_run`**: `Option<String>`

  The [`default-run` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-default-run-field) as given in the `Cargo.toml`
  The default binary to run by `cargo run`.
  
  This is always `None` if running with a version of Cargo older than 1.55.

- **`rust_version`**: `Option<semver::Version>`

  The [`rust-version` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field) as specified in the `Cargo.toml`.
  The minimum supported Rust version of this package.
  
  This is always `None` if running with a version of Cargo older than 1.58.

#### Implementations

- <span id="package-license-file"></span>`fn license_file(&self) -> Option<Utf8PathBuf>`

  Full path to the license file if one is present in the manifest

- <span id="package-readme"></span>`fn readme(&self) -> Option<Utf8PathBuf>`

  Full path to the readme file if one is present in the manifest

#### Trait Implementations

##### `impl Any for Package`

- <span id="package-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Package`

- <span id="package-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Package`

- <span id="package-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Package`

- <span id="package-clone"></span>`fn clone(&self) -> Package` — [`Package`](#package)

##### `impl CloneToUninit for Package`

- <span id="package-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Package`

- <span id="package-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Package`

- <span id="package-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Package`

##### `impl Eq for Package`

##### `impl<T> From for Package`

- <span id="package-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Package`

- <span id="package-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Package`

- <span id="package-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Package`

- <span id="package-partialeq-eq"></span>`fn eq(&self, other: &Package) -> bool` — [`Package`](#package)

##### `impl Serialize for Package`

- <span id="package-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Package`

##### `impl ToOwned for Package`

- <span id="package-toowned-type-owned"></span>`type Owned = T`

- <span id="package-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="package-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Package`

- <span id="package-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="package-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Package`

- <span id="package-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="package-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Source`

```rust
struct Source {
    pub repr: String,
}
```

*Defined in [`cargo_metadata-0.23.1/src/lib.rs:661-664`](../../.source_1765521767/cargo_metadata-0.23.1/src/lib.rs#L661-L664)*

The source of a package such as crates.io.

It is possible to inspect the `repr` field, if the need arises, but its
precise format is an implementation detail and is subject to change.

#### Fields

- **`repr`**: `String`

  The underlying string representation of a source.

#### Implementations

- <span id="source-is-crates-io"></span>`fn is_crates_io(&self) -> bool`

  Returns true if the source is crates.io.

#### Trait Implementations

##### `impl Any for Source`

- <span id="source-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Source`

- <span id="source-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Source`

- <span id="source-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Source`

- <span id="source-clone"></span>`fn clone(&self) -> Source` — [`Source`](#source)

##### `impl CloneToUninit for Source`

- <span id="source-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Source`

- <span id="source-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Source`

- <span id="source-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Source`

##### `impl Display for Source`

- <span id="source-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Source`

##### `impl<T> From for Source`

- <span id="source-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Source`

- <span id="source-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Source`

- <span id="source-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Source`

- <span id="source-partialeq-eq"></span>`fn eq(&self, other: &Source) -> bool` — [`Source`](#source)

##### `impl Serialize for Source`

- <span id="source-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Source`

##### `impl ToOwned for Source`

- <span id="source-toowned-type-owned"></span>`type Owned = T`

- <span id="source-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="source-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Source`

- <span id="source-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Source`

- <span id="source-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="source-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Source`

- <span id="source-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="source-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Target`

```rust
struct Target {
    pub name: String,
    pub kind: Vec<TargetKind>,
    pub crate_types: Vec<CrateType>,
    pub required_features: Vec<String>,
    pub src_path: camino::Utf8PathBuf,
    pub edition: Edition,
    pub doctest: bool,
    pub test: bool,
    pub doc: bool,
}
```

*Defined in [`cargo_metadata-0.23.1/src/lib.rs:684-736`](../../.source_1765521767/cargo_metadata-0.23.1/src/lib.rs#L684-L736)*

A single target (lib, bin, example, ...) provided by a crate

#### Fields

- **`name`**: `String`

  Name as given in the `Cargo.toml` or generated from the file name

- **`kind`**: `Vec<TargetKind>`

  Kind of target.
  
  The possible values are `example`, `test`, `bench`, `custom-build` and
  [Cargo crate types](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-crate-type-field):
  `bin`, `lib`, `rlib`, `dylib`, `cdylib`, `staticlib`, `proc-macro`.
  
  Other possible values may be added in the future.

- **`crate_types`**: `Vec<CrateType>`

  Similar to `kind`, but only reports the
  [Cargo crate types](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-crate-type-field):
  `bin`, `lib`, `rlib`, `dylib`, `cdylib`, `staticlib`, `proc-macro`.
  Everything that's not a proc macro or a library of some kind is reported as "bin".
  
  Other possible values may be added in the future.

- **`required_features`**: `Vec<String>`

  This target is built only if these features are enabled.
  It doesn't apply to `lib` targets.

- **`src_path`**: `camino::Utf8PathBuf`

  Path to the main source file of the target

- **`edition`**: `Edition`

  Rust edition for this target

- **`doctest`**: `bool`

  Whether or not this target has doc tests enabled, and the target is
  compatible with doc testing.
  
  This is always `true` if running with a version of Cargo older than 1.37.

- **`test`**: `bool`

  Whether or not this target is tested by default by `cargo test`.
  
  This is always `true` if running with a version of Cargo older than 1.47.

- **`doc`**: `bool`

  Whether or not this target is documented by `cargo doc`.
  
  This is always `true` if running with a version of Cargo older than 1.50.

#### Implementations

- <span id="target-is-kind"></span>`fn is_kind(&self, name: TargetKind) -> bool` — [`TargetKind`](#targetkind)

  Return true if this target is of the given kind.

- <span id="target-is-lib"></span>`fn is_lib(&self) -> bool`

  Return true if this target is of kind `$kind`.

- <span id="target-is-bin"></span>`fn is_bin(&self) -> bool`

  Return true if this target is of kind `$kind`.

- <span id="target-is-example"></span>`fn is_example(&self) -> bool`

  Return true if this target is of kind `$kind`.

- <span id="target-is-test"></span>`fn is_test(&self) -> bool`

  Return true if this target is of kind `$kind`.

- <span id="target-is-bench"></span>`fn is_bench(&self) -> bool`

  Return true if this target is of kind `$kind`.

- <span id="target-is-custom-build"></span>`fn is_custom_build(&self) -> bool`

  Return true if this target is of kind `$kind`.

- <span id="target-is-proc-macro"></span>`fn is_proc_macro(&self) -> bool`

  Return true if this target is of kind `$kind`.

- <span id="target-is-cdylib"></span>`fn is_cdylib(&self) -> bool`

  Return true if this target is of kind `$kind`.

- <span id="target-is-dylib"></span>`fn is_dylib(&self) -> bool`

  Return true if this target is of kind `$kind`.

- <span id="target-is-rlib"></span>`fn is_rlib(&self) -> bool`

  Return true if this target is of kind `$kind`.

- <span id="target-is-staticlib"></span>`fn is_staticlib(&self) -> bool`

  Return true if this target is of kind `$kind`.

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

##### `impl Hash for Target`

- <span id="target-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

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

### `MetadataCommand`

```rust
struct MetadataCommand {
    cargo_path: Option<std::path::PathBuf>,
    manifest_path: Option<std::path::PathBuf>,
    current_dir: Option<std::path::PathBuf>,
    no_deps: bool,
    features: Vec<String>,
    all_features: bool,
    no_default_features: bool,
    other_options: Vec<String>,
    env: std::collections::BTreeMap<std::ffi::OsString, Option<std::ffi::OsString>>,
    verbose: bool,
}
```

*Defined in [`cargo_metadata-0.23.1/src/lib.rs:1005-1031`](../../.source_1765521767/cargo_metadata-0.23.1/src/lib.rs#L1005-L1031)*

A builder for configuring `cargo metadata` invocation.

#### Fields

- **`cargo_path`**: `Option<std::path::PathBuf>`

  Path to `cargo` executable.  If not set, this will use the
  the `$CARGO` environment variable, and if that is not set, will
  simply be `cargo`.

- **`manifest_path`**: `Option<std::path::PathBuf>`

  Path to `Cargo.toml`

- **`current_dir`**: `Option<std::path::PathBuf>`

  Current directory of the `cargo metadata` process.

- **`no_deps`**: `bool`

  Output information only about workspace members and don't fetch dependencies.

- **`features`**: `Vec<String>`

  Collections of `CargoOpt::SomeFeatures(..)`

- **`all_features`**: `bool`

  Latched `CargoOpt::AllFeatures`

- **`no_default_features`**: `bool`

  Latched `CargoOpt::NoDefaultFeatures`

- **`other_options`**: `Vec<String>`

  Arbitrary command line flags to pass to `cargo`. These will be added
  to the end of the command line invocation.

- **`env`**: `std::collections::BTreeMap<std::ffi::OsString, Option<std::ffi::OsString>>`

  Arbitrary environment variables to set or remove (depending on
  [`Option`](../clap_derive/index.md) value) when running `cargo`. These will be merged into the
  calling environment, overriding any which clash.

- **`verbose`**: `bool`

  Show stderr

#### Implementations

- <span id="metadatacommand-new"></span>`fn new() -> MetadataCommand` — [`MetadataCommand`](#metadatacommand)

  Creates a default `cargo metadata` command, which will look for

  `Cargo.toml` in the ancestors of the current directory.

- <span id="metadatacommand-cargo-path"></span>`fn cargo_path(&mut self, path: impl Into<PathBuf>) -> &mut MetadataCommand` — [`MetadataCommand`](#metadatacommand)

  Path to `cargo` executable.  If not set, this will use the

  the `$CARGO` environment variable, and if that is not set, will

  simply be `cargo`.

- <span id="metadatacommand-manifest-path"></span>`fn manifest_path(&mut self, path: impl Into<PathBuf>) -> &mut MetadataCommand` — [`MetadataCommand`](#metadatacommand)

  Path to `Cargo.toml`

- <span id="metadatacommand-current-dir"></span>`fn current_dir(&mut self, path: impl Into<PathBuf>) -> &mut MetadataCommand` — [`MetadataCommand`](#metadatacommand)

  Current directory of the `cargo metadata` process.

- <span id="metadatacommand-no-deps"></span>`fn no_deps(&mut self) -> &mut MetadataCommand` — [`MetadataCommand`](#metadatacommand)

  Output information only about workspace members and don't fetch dependencies.

- <span id="metadatacommand-features"></span>`fn features(&mut self, features: CargoOpt) -> &mut MetadataCommand` — [`CargoOpt`](#cargoopt), [`MetadataCommand`](#metadatacommand)

  Which features to include.

  

  Call this multiple times to specify advanced feature configurations:

  

  ```no_run

  use cargo_metadata::{CargoOpt, MetadataCommand};

  MetadataCommand::new()

      .features(CargoOpt::NoDefaultFeatures)

      .features(CargoOpt::SomeFeatures(vec!["feat1".into(), "feat2".into()]))

      .features(CargoOpt::SomeFeatures(vec!["feat3".into()]))

      // ...

      ;

  ```

  

  # Panics

  

  `cargo metadata` rejects multiple `--no-default-features` flags. Similarly, the `features()`

  method panics when specifying multiple `CargoOpt::NoDefaultFeatures`:

  

  ```should_panic

  use cargo_metadata::{CargoOpt, MetadataCommand};

  MetadataCommand::new()

      .features(CargoOpt::NoDefaultFeatures)

      .features(CargoOpt::NoDefaultFeatures) // <-- panic!

      // ...

      ;

  ```

  

  The method also panics for multiple `CargoOpt::AllFeatures` arguments:

  

  ```should_panic

  use cargo_metadata::{CargoOpt, MetadataCommand};

  MetadataCommand::new()

      .features(CargoOpt::AllFeatures)

      .features(CargoOpt::AllFeatures) // <-- panic!

      // ...

      ;

  ```

- <span id="metadatacommand-other-options"></span>`fn other_options(&mut self, options: impl Into<Vec<String>>) -> &mut MetadataCommand` — [`MetadataCommand`](#metadatacommand)

  Arbitrary command line flags to pass to `cargo`.  These will be added

  to the end of the command line invocation.

- <span id="metadatacommand-env"></span>`fn env<K: Into<OsString>, V: Into<OsString>>(&mut self, key: K, val: V) -> &mut MetadataCommand` — [`MetadataCommand`](#metadatacommand)

  Arbitrary environment variables to set when running `cargo`.  These will be merged into

  the calling environment, overriding any which clash.

  

  Some examples of when you may want to use this:

  1. Setting cargo config values without needing a .cargo/config.toml file, e.g. to set

     `CARGO_NET_GIT_FETCH_WITH_CLI=true`

  2. To specify a custom path to RUSTC if your rust toolchain components aren't laid out in

     the way cargo expects by default.

  

  ```no_run

  use cargo_metadata::{CargoOpt, MetadataCommand};

  MetadataCommand::new()

      .env("CARGO_NET_GIT_FETCH_WITH_CLI", "true")

      .env("RUSTC", "/path/to/rustc")

      // ...

      ;

  ```

- <span id="metadatacommand-env-remove"></span>`fn env_remove<K: Into<OsString>>(&mut self, key: K) -> &mut MetadataCommand` — [`MetadataCommand`](#metadatacommand)

  Arbitrary environment variables to remove when running `cargo`.  These will be merged into

  the calling environment, overriding any which clash.

  

  Some examples of when you may want to use this:

  - Removing inherited environment variables in build scripts that can cause an error

    when calling `cargo metadata` (for example, when cross-compiling).

  

  ```no_run

  use cargo_metadata::{CargoOpt, MetadataCommand};

  MetadataCommand::new()

      .env_remove("CARGO_ENCODED_RUSTFLAGS")

      // ...

      ;

  ```

- <span id="metadatacommand-verbose"></span>`fn verbose(&mut self, verbose: bool) -> &mut MetadataCommand` — [`MetadataCommand`](#metadatacommand)

  Set whether to show stderr

- <span id="metadatacommand-cargo-command"></span>`fn cargo_command(&self) -> Command`

  Builds a command for `cargo metadata`.  This is the first

  part of the work of `exec`.

- <span id="metadatacommand-parse"></span>`fn parse<T: AsRef<str>>(data: T) -> Result<Metadata>` — [`Result`](errors/index.md#result), [`Metadata`](#metadata)

  Parses `cargo metadata` output.  `data` must have been

  produced by a command built with `cargo_command`.

- <span id="metadatacommand-exec"></span>`fn exec(&self) -> Result<Metadata>` — [`Result`](errors/index.md#result), [`Metadata`](#metadata)

  Runs configured `cargo metadata` and returns parsed `Metadata`.

#### Trait Implementations

##### `impl Any for MetadataCommand`

- <span id="metadatacommand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MetadataCommand`

- <span id="metadatacommand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MetadataCommand`

- <span id="metadatacommand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MetadataCommand`

- <span id="metadatacommand-clone"></span>`fn clone(&self) -> MetadataCommand` — [`MetadataCommand`](#metadatacommand)

##### `impl CloneToUninit for MetadataCommand`

- <span id="metadatacommand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for MetadataCommand`

- <span id="metadatacommand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MetadataCommand`

- <span id="metadatacommand-default"></span>`fn default() -> MetadataCommand` — [`MetadataCommand`](#metadatacommand)

##### `impl<T> From for MetadataCommand`

- <span id="metadatacommand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MetadataCommand`

- <span id="metadatacommand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for MetadataCommand`

- <span id="metadatacommand-toowned-type-owned"></span>`type Owned = T`

- <span id="metadatacommand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="metadatacommand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MetadataCommand`

- <span id="metadatacommand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="metadatacommand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MetadataCommand`

- <span id="metadatacommand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="metadatacommand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `DependencyKind`

```rust
enum DependencyKind {
    Normal,
    Development,
    Build,
}
```

*Defined in [`cargo_metadata-0.23.1/src/dependency.rs:15-29`](../../.source_1765521767/cargo_metadata-0.23.1/src/dependency.rs#L15-L29)*

Dependencies can come in three kinds

#### Variants

- **`Normal`**

  The 'normal' kind

- **`Development`**

  Those used in tests only

- **`Build`**

  Those used in build scripts only

#### Trait Implementations

##### `impl Any for DependencyKind`

- <span id="dependencykind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DependencyKind`

- <span id="dependencykind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DependencyKind`

- <span id="dependencykind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DependencyKind`

- <span id="dependencykind-clone"></span>`fn clone(&self) -> DependencyKind` — [`DependencyKind`](dependency/index.md#dependencykind)

##### `impl CloneToUninit for DependencyKind`

- <span id="dependencykind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for DependencyKind`

##### `impl Debug for DependencyKind`

- <span id="dependencykind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DependencyKind`

- <span id="dependencykind-default"></span>`fn default() -> DependencyKind` — [`DependencyKind`](dependency/index.md#dependencykind)

##### `impl Deserialize for DependencyKind`

- <span id="dependencykind-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for DependencyKind`

##### `impl Display for DependencyKind`

- <span id="dependencykind-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DependencyKind`

##### `impl<T> From for DependencyKind`

- <span id="dependencykind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for DependencyKind`

- <span id="dependencykind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for DependencyKind`

- <span id="dependencykind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for DependencyKind`

- <span id="dependencykind-partialeq-eq"></span>`fn eq(&self, other: &DependencyKind) -> bool` — [`DependencyKind`](dependency/index.md#dependencykind)

##### `impl Serialize for DependencyKind`

- <span id="dependencykind-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for DependencyKind`

##### `impl ToOwned for DependencyKind`

- <span id="dependencykind-toowned-type-owned"></span>`type Owned = T`

- <span id="dependencykind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dependencykind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for DependencyKind`

- <span id="dependencykind-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for DependencyKind`

- <span id="dependencykind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dependencykind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DependencyKind`

- <span id="dependencykind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dependencykind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Error`

```rust
enum Error {
    CargoMetadata {
        stderr: String,
    },
    Io(io::Error),
    Utf8(std::str::Utf8Error),
    ErrUtf8(std::string::FromUtf8Error),
    Json(::serde_json::Error),
    NoJson,
}
```

*Defined in [`cargo_metadata-0.23.1/src/errors.rs:25-52`](../../.source_1765521767/cargo_metadata-0.23.1/src/errors.rs#L25-L52)*

Error returned when executing/parsing `cargo metadata` fails.

# Note about Backtraces

This error type does not contain backtraces, but each error variant
comes from _one_ specific place, so it's not really needed for the
inside of this crate. If you need a backtrace down to, but not inside
of, a failed call of `cargo_metadata` you can do one of multiple thinks:

1. Convert it to a `failure::Error` (possible using the `?` operator),
   which is similar to a `Box<::std::error::Error + 'static + Send  + Sync>`.
2. Have appropriate variants in your own error type. E.g. you could wrap
   a `failure::Context<Error>` or add a `failure::Backtrace` field (which
   is empty if `RUST_BACKTRACE` is not set, so it's simple to use).
3. You still can place a failure based error into a `error_chain` if you
   really want to. (Either through foreign_links or by making it a field
   value of a `ErrorKind` variant).


#### Variants

- **`CargoMetadata`**

  Error during execution of `cargo metadata`

- **`Io`**

  IO Error during execution of `cargo metadata`

- **`Utf8`**

  Output of `cargo metadata` was not valid utf8

- **`ErrUtf8`**

  Error output of `cargo metadata` was not valid utf8

- **`Json`**

  Deserialization error (structure of json did not match expected structure)

- **`NoJson`**

  The output did not contain any json

#### Trait Implementations

##### `impl Any for Error`

- <span id="error-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Error`

- <span id="error-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Error`

- <span id="error-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Error for Error`

- <span id="error-error-source"></span>`fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private17::Error>`

##### `impl<T> From for Error`

- <span id="error-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Error`

- <span id="error-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for Error`

- <span id="error-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Error`

- <span id="error-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="error-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Error`

- <span id="error-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="error-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ArtifactDebuginfo`

```rust
enum ArtifactDebuginfo {
    None,
    LineDirectivesOnly,
    LineTablesOnly,
    Limited,
    Full,
    UnknownInt(i64),
    UnknownString(String),
}
```

*Defined in [`cargo_metadata-0.23.1/src/messages.rs:33-57`](../../.source_1765521767/cargo_metadata-0.23.1/src/messages.rs#L33-L57)*

The kind of debug information included in the artifact.

#### Variants

- **`None`**

  No debug information.

- **`LineDirectivesOnly`**

  Line directives only.

- **`LineTablesOnly`**

  Line tables only.

- **`Limited`**

  Debug information without type or variable-level information.

- **`Full`**

  Full debug information.

- **`UnknownInt`**

  An unknown integer level.
  
  This may be produced by a version of rustc in the future that has
  additional levels represented by an integer that are not known by this
  version of `cargo_metadata`.

- **`UnknownString`**

  An unknown string level.
  
  This may be produced by a version of rustc in the future that has
  additional levels represented by a string that are not known by this
  version of `cargo_metadata`.

#### Trait Implementations

##### `impl Any for ArtifactDebuginfo`

- <span id="artifactdebuginfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArtifactDebuginfo`

- <span id="artifactdebuginfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArtifactDebuginfo`

- <span id="artifactdebuginfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ArtifactDebuginfo`

- <span id="artifactdebuginfo-clone"></span>`fn clone(&self) -> ArtifactDebuginfo` — [`ArtifactDebuginfo`](messages/index.md#artifactdebuginfo)

##### `impl CloneToUninit for ArtifactDebuginfo`

- <span id="artifactdebuginfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ArtifactDebuginfo`

- <span id="artifactdebuginfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ArtifactDebuginfo`

- <span id="artifactdebuginfo-default"></span>`fn default() -> ArtifactDebuginfo` — [`ArtifactDebuginfo`](messages/index.md#artifactdebuginfo)

##### `impl Deserialize for ArtifactDebuginfo`

- <span id="artifactdebuginfo-deserialize"></span>`fn deserialize<D>(d: D) -> Result<ArtifactDebuginfo, <D as >::Error>` — [`ArtifactDebuginfo`](messages/index.md#artifactdebuginfo)

##### `impl DeserializeOwned for ArtifactDebuginfo`

##### `impl Display for ArtifactDebuginfo`

- <span id="artifactdebuginfo-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ArtifactDebuginfo`

##### `impl<T> From for ArtifactDebuginfo`

- <span id="artifactdebuginfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for ArtifactDebuginfo`

- <span id="artifactdebuginfo-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for ArtifactDebuginfo`

- <span id="artifactdebuginfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ArtifactDebuginfo`

- <span id="artifactdebuginfo-partialeq-eq"></span>`fn eq(&self, other: &ArtifactDebuginfo) -> bool` — [`ArtifactDebuginfo`](messages/index.md#artifactdebuginfo)

##### `impl Serialize for ArtifactDebuginfo`

- <span id="artifactdebuginfo-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for ArtifactDebuginfo`

##### `impl ToOwned for ArtifactDebuginfo`

- <span id="artifactdebuginfo-toowned-type-owned"></span>`type Owned = T`

- <span id="artifactdebuginfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="artifactdebuginfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for ArtifactDebuginfo`

- <span id="artifactdebuginfo-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for ArtifactDebuginfo`

- <span id="artifactdebuginfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="artifactdebuginfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArtifactDebuginfo`

- <span id="artifactdebuginfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="artifactdebuginfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Message`

```rust
enum Message {
    CompilerArtifact(Artifact),
    CompilerMessage(CompilerMessage),
    BuildScriptExecuted(BuildScript),
    BuildFinished(BuildFinished),
    TextLine(String),
}
```

*Defined in [`cargo_metadata-0.23.1/src/messages.rs:229-245`](../../.source_1765521767/cargo_metadata-0.23.1/src/messages.rs#L229-L245)*

A cargo message

#### Variants

- **`CompilerArtifact`**

  The compiler generated an artifact

- **`CompilerMessage`**

  The compiler wants to display a message

- **`BuildScriptExecuted`**

  A build script successfully executed.

- **`BuildFinished`**

  The build has finished.
  
  This is emitted at the end of the build as the last message.
  Added in Rust 1.44.

- **`TextLine`**

  A line of text which isn't a cargo or compiler message.
  Line separator is not included

#### Implementations

- <span id="message-parse-stream"></span>`fn parse_stream<R: Read>(input: R) -> MessageIter<R>` — [`MessageIter`](messages/index.md#messageiter)

  Creates an iterator of Message from a Read outputting a stream of JSON

  messages. For usage information, look at the top-level documentation.

#### Trait Implementations

##### `impl Any for Message`

- <span id="message-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Message`

- <span id="message-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Message`

- <span id="message-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Message`

- <span id="message-clone"></span>`fn clone(&self) -> Message` — [`Message`](messages/index.md#message)

##### `impl CloneToUninit for Message`

- <span id="message-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Message`

- <span id="message-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Message`

- <span id="message-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Message`

##### `impl Eq for Message`

##### `impl<T> From for Message`

- <span id="message-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Message`

- <span id="message-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Message`

- <span id="message-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Message`

- <span id="message-partialeq-eq"></span>`fn eq(&self, other: &Message) -> bool` — [`Message`](messages/index.md#message)

##### `impl Serialize for Message`

- <span id="message-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Message`

##### `impl ToOwned for Message`

- <span id="message-toowned-type-owned"></span>`type Owned = T`

- <span id="message-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="message-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Message`

- <span id="message-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="message-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Message`

- <span id="message-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="message-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TargetKind`

```rust
enum TargetKind {
    Bench,
    Bin,
    CustomBuild,
    CDyLib,
    DyLib,
    Example,
    Lib,
    ProcMacro,
    RLib,
    StaticLib,
    Test,
    Unknown(String),
}
```

*Defined in [`cargo_metadata-0.23.1/src/lib.rs:780-817`](../../.source_1765521767/cargo_metadata-0.23.1/src/lib.rs#L780-L817)*

Kind of target.

The possible values are `example`, `test`, `bench`, `custom-build` and
[Cargo crate types](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-crate-type-field):
`bin`, `lib`, `rlib`, `dylib`, `cdylib`, `staticlib`, `proc-macro`.

Other possible values may be added in the future.

#### Variants

- **`Bench`**

  `cargo bench` target

- **`Bin`**

  Binary executable target

- **`CustomBuild`**

  Custom build target

- **`CDyLib`**

  Dynamic system library target

- **`DyLib`**

  Dynamic Rust library target

- **`Example`**

  Example target

- **`Lib`**

  Rust library

- **`ProcMacro`**

  Procedural Macro

- **`RLib`**

  Rust library for use as an intermediate artifact

- **`StaticLib`**

  Static system library

- **`Test`**

  Test target

- **`Unknown`**

  Unknown type

#### Trait Implementations

##### `impl Any for TargetKind`

- <span id="targetkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TargetKind`

- <span id="targetkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TargetKind`

- <span id="targetkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TargetKind`

- <span id="targetkind-clone"></span>`fn clone(&self) -> TargetKind` — [`TargetKind`](#targetkind)

##### `impl CloneToUninit for TargetKind`

- <span id="targetkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for TargetKind`

- <span id="targetkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for TargetKind`

- <span id="targetkind-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for TargetKind`

##### `impl Display for TargetKind`

- <span id="targetkind-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TargetKind`

##### `impl<T> From for TargetKind`

- <span id="targetkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromStr for TargetKind`

- <span id="targetkind-fromstr-type-err"></span>`type Err = Infallible`

- <span id="targetkind-fromstr-from-str"></span>`fn from_str(s: &str) -> std::result::Result<Self, <Self as >::Err>`

##### `impl Hash for TargetKind`

- <span id="targetkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for TargetKind`

- <span id="targetkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for TargetKind`

- <span id="targetkind-ord-cmp"></span>`fn cmp(&self, other: &TargetKind) -> cmp::Ordering` — [`TargetKind`](#targetkind)

##### `impl PartialEq for TargetKind`

- <span id="targetkind-partialeq-eq"></span>`fn eq(&self, other: &TargetKind) -> bool` — [`TargetKind`](#targetkind)

##### `impl PartialOrd for TargetKind`

- <span id="targetkind-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &TargetKind) -> option::Option<cmp::Ordering>` — [`TargetKind`](#targetkind)

##### `impl Serialize for TargetKind`

- <span id="targetkind-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for TargetKind`

##### `impl ToOwned for TargetKind`

- <span id="targetkind-toowned-type-owned"></span>`type Owned = T`

- <span id="targetkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="targetkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for TargetKind`

- <span id="targetkind-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for TargetKind`

- <span id="targetkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="targetkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TargetKind`

- <span id="targetkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="targetkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CrateType`

```rust
enum CrateType {
    Bin,
    CDyLib,
    DyLib,
    Lib,
    ProcMacro,
    RLib,
    StaticLib,
    Unknown(String),
}
```

*Defined in [`cargo_metadata-0.23.1/src/lib.rs:874-899`](../../.source_1765521767/cargo_metadata-0.23.1/src/lib.rs#L874-L899)*

Similar to `kind`, but only reports the
[Cargo crate types](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-crate-type-field):

`bin`, `lib`, `rlib`, `dylib`, `cdylib`, `staticlib`, `proc-macro`.
Everything that's not a proc macro or a library of some kind is reported as "bin".

Other possible values may be added in the future.

#### Variants

- **`Bin`**

  Binary executable target

- **`CDyLib`**

  Dynamic system library target

- **`DyLib`**

  Dynamic Rust library target

- **`Lib`**

  Rust library

- **`ProcMacro`**

  Procedural Macro

- **`RLib`**

  Rust library for use as an intermediate artifact

- **`StaticLib`**

  Static system library

- **`Unknown`**

  Unkown type

#### Trait Implementations

##### `impl Any for CrateType`

- <span id="cratetype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CrateType`

- <span id="cratetype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CrateType`

- <span id="cratetype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CrateType`

- <span id="cratetype-clone"></span>`fn clone(&self) -> CrateType` — [`CrateType`](#cratetype)

##### `impl CloneToUninit for CrateType`

- <span id="cratetype-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for CrateType`

- <span id="cratetype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for CrateType`

- <span id="cratetype-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for CrateType`

##### `impl Display for CrateType`

- <span id="cratetype-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CrateType`

##### `impl<T> From for CrateType`

- <span id="cratetype-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromStr for CrateType`

- <span id="cratetype-fromstr-type-err"></span>`type Err = Infallible`

- <span id="cratetype-fromstr-from-str"></span>`fn from_str(s: &str) -> std::result::Result<Self, <Self as >::Err>`

##### `impl Hash for CrateType`

- <span id="cratetype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for CrateType`

- <span id="cratetype-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for CrateType`

- <span id="cratetype-ord-cmp"></span>`fn cmp(&self, other: &CrateType) -> cmp::Ordering` — [`CrateType`](#cratetype)

##### `impl PartialEq for CrateType`

- <span id="cratetype-partialeq-eq"></span>`fn eq(&self, other: &CrateType) -> bool` — [`CrateType`](#cratetype)

##### `impl PartialOrd for CrateType`

- <span id="cratetype-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &CrateType) -> option::Option<cmp::Ordering>` — [`CrateType`](#cratetype)

##### `impl Serialize for CrateType`

- <span id="cratetype-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for CrateType`

##### `impl ToOwned for CrateType`

- <span id="cratetype-toowned-type-owned"></span>`type Owned = T`

- <span id="cratetype-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="cratetype-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for CrateType`

- <span id="cratetype-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for CrateType`

- <span id="cratetype-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cratetype-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CrateType`

- <span id="cratetype-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cratetype-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Edition`

```rust
enum Edition {
    E2015,
    E2018,
    E2021,
    E2024,
}
```

*Defined in [`cargo_metadata-0.23.1/src/lib.rs:945-965`](../../.source_1765521767/cargo_metadata-0.23.1/src/lib.rs#L945-L965)*

The Rust edition

As of writing this comment rust editions 2027 and 2030 are not actually a thing yet but are parsed nonetheless for future proofing.

#### Variants

- **`E2015`**

  Edition 2015

- **`E2018`**

  Edition 2018

- **`E2021`**

  Edition 2021

- **`E2024`**

  Edition 2024

#### Implementations

- <span id="edition-as-str"></span>`fn as_str(&self) -> &'static str`

  Return the string representation of the edition

#### Trait Implementations

##### `impl Any for Edition`

- <span id="edition-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Edition`

- <span id="edition-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Edition`

- <span id="edition-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Edition`

- <span id="edition-clone"></span>`fn clone(&self) -> Edition` — [`Edition`](#edition)

##### `impl CloneToUninit for Edition`

- <span id="edition-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Edition`

##### `impl Debug for Edition`

- <span id="edition-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Edition`

- <span id="edition-default"></span>`fn default() -> Edition` — [`Edition`](#edition)

##### `impl Deserialize for Edition`

- <span id="edition-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Edition`

##### `impl Display for Edition`

- <span id="edition-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Edition`

##### `impl<T> From for Edition`

- <span id="edition-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Edition`

- <span id="edition-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Edition`

- <span id="edition-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Edition`

- <span id="edition-ord-cmp"></span>`fn cmp(&self, other: &Edition) -> cmp::Ordering` — [`Edition`](#edition)

##### `impl PartialEq for Edition`

- <span id="edition-partialeq-eq"></span>`fn eq(&self, other: &Edition) -> bool` — [`Edition`](#edition)

##### `impl PartialOrd for Edition`

- <span id="edition-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Edition) -> option::Option<cmp::Ordering>` — [`Edition`](#edition)

##### `impl Serialize for Edition`

- <span id="edition-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Edition`

##### `impl ToOwned for Edition`

- <span id="edition-toowned-type-owned"></span>`type Owned = T`

- <span id="edition-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="edition-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Edition`

- <span id="edition-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Edition`

- <span id="edition-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="edition-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Edition`

- <span id="edition-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="edition-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CargoOpt`

```rust
enum CargoOpt {
    AllFeatures,
    NoDefaultFeatures,
    SomeFeatures(Vec<String>),
}
```

*Defined in [`cargo_metadata-0.23.1/src/lib.rs:994-1001`](../../.source_1765521767/cargo_metadata-0.23.1/src/lib.rs#L994-L1001)*

Cargo features flags

#### Variants

- **`AllFeatures`**

  Run cargo with `--features-all`

- **`NoDefaultFeatures`**

  Run cargo with `--no-default-features`

- **`SomeFeatures`**

  Run cargo with `--features <FEATURES>`

#### Trait Implementations

##### `impl Any for CargoOpt`

- <span id="cargoopt-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CargoOpt`

- <span id="cargoopt-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CargoOpt`

- <span id="cargoopt-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CargoOpt`

- <span id="cargoopt-clone"></span>`fn clone(&self) -> CargoOpt` — [`CargoOpt`](#cargoopt)

##### `impl CloneToUninit for CargoOpt`

- <span id="cargoopt-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for CargoOpt`

- <span id="cargoopt-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CargoOpt`

- <span id="cargoopt-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CargoOpt`

- <span id="cargoopt-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for CargoOpt`

- <span id="cargoopt-toowned-type-owned"></span>`type Owned = T`

- <span id="cargoopt-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="cargoopt-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CargoOpt`

- <span id="cargoopt-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cargoopt-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CargoOpt`

- <span id="cargoopt-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cargoopt-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `parse_messages`

```rust
fn parse_messages<R: Read>(input: R) -> serde_json::StreamDeserializer<'static, serde_json::de::IoRead<R>, Message>
```

*Defined in [`cargo_metadata-0.23.1/src/messages.rs:295-297`](../../.source_1765521767/cargo_metadata-0.23.1/src/messages.rs#L295-L297)*

Creates an iterator of Message from a Read outputting a stream of JSON
messages. For usage information, look at the top-level documentation.

### `is_null`

```rust
fn is_null(value: &serde_json::Value) -> bool
```

*Defined in [`cargo_metadata-0.23.1/src/lib.rs:253-255`](../../.source_1765521767/cargo_metadata-0.23.1/src/lib.rs#L253-L255)*

Helpers for default metadata fields

### `default_true`

```rust
fn default_true() -> bool
```

*Defined in [`cargo_metadata-0.23.1/src/lib.rs:988-990`](../../.source_1765521767/cargo_metadata-0.23.1/src/lib.rs#L988-L990)*

### `deserialize_rust_version`

```rust
fn deserialize_rust_version<'de, D>(deserializer: D) -> std::result::Result<Option<semver::Version>, <D as >::Error>
where
    D: Deserializer<'de>
```

*Defined in [`cargo_metadata-0.23.1/src/lib.rs:1256-1287`](../../.source_1765521767/cargo_metadata-0.23.1/src/lib.rs#L1256-L1287)*

As per the Cargo Book the [`rust-version` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field) must:

> be a bare version number with two or three components;
> it cannot include semver operators or pre-release identifiers.

[`semver::Version`](../semver/index.md) however requires three components. This function takes
care of appending `.0` if the provided version number only has two components
and ensuring that it does not contain a pre-release version or build metadata.

## Type Aliases

### `Result<T, E>`

```rust
type Result<T, E> = ::std::result::Result<T, E>;
```

*Defined in [`cargo_metadata-0.23.1/src/errors.rs:4`](../../.source_1765521767/cargo_metadata-0.23.1/src/errors.rs#L4)*

Custom result type for `cargo_metadata::Error`

## Macros

### `str_newtype!`

*Defined in [`cargo_metadata-0.23.1/src/lib.rs:131-207`](../../.source_1765521767/cargo_metadata-0.23.1/src/lib.rs#L131-L207)*

### `methods_target_is_kind!`

*Defined in [`cargo_metadata-0.23.1/src/lib.rs:738-747`](../../.source_1765521767/cargo_metadata-0.23.1/src/lib.rs#L738-L747)*

