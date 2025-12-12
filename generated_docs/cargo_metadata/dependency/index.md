*[cargo_metadata](../index.md) / [dependency](index.md)*

---

# Module `dependency`

This module contains `Dependency` and the types/functions it uses for deserialization.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Dependency`](#dependency) | struct | A dependency of the main crate |
| [`DependencyKind`](#dependencykind) | enum | Dependencies can come in three kinds |
| [`parse_dependency_kind`](#parse-dependency-kind) | fn | The `kind` can be `null`, which is interpreted as the default - `Normal`. |

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

*Defined in [`cargo_metadata-0.23.1/src/dependency.rs:52-85`](../../../.source_1765521767/cargo_metadata-0.23.1/src/dependency.rs#L52-L85)*

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
  
  Use the `Display` trait to access the contents.
  

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

##### `impl Clone for Dependency`

- <span id="dependency-clone"></span>`fn clone(&self) -> Dependency` — [`Dependency`](#dependency)

##### `impl Debug for Dependency`

- <span id="dependency-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Dependency`

- <span id="dependency-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Dependency`

##### `impl Eq for Dependency`

##### `impl Hash for Dependency`

- <span id="dependency-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Dependency`

- <span id="dependency-eq"></span>`fn eq(&self, other: &Dependency) -> bool` — [`Dependency`](#dependency)

##### `impl Serialize for Dependency`

- <span id="dependency-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Dependency`

## Enums

### `DependencyKind`

```rust
enum DependencyKind {
    Normal,
    Development,
    Build,
}
```

*Defined in [`cargo_metadata-0.23.1/src/dependency.rs:15-29`](../../../.source_1765521767/cargo_metadata-0.23.1/src/dependency.rs#L15-L29)*

Dependencies can come in three kinds

#### Variants

- **`Normal`**

  The 'normal' kind

- **`Development`**

  Those used in tests only

- **`Build`**

  Those used in build scripts only

#### Trait Implementations

##### `impl Clone for DependencyKind`

- <span id="dependencykind-clone"></span>`fn clone(&self) -> DependencyKind` — [`DependencyKind`](#dependencykind)

##### `impl Copy for DependencyKind`

##### `impl Debug for DependencyKind`

- <span id="dependencykind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DependencyKind`

- <span id="dependencykind-default"></span>`fn default() -> DependencyKind` — [`DependencyKind`](#dependencykind)

##### `impl Deserialize for DependencyKind`

- <span id="dependencykind-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for DependencyKind`

##### `impl Display for DependencyKind`

- <span id="dependencykind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DependencyKind`

##### `impl Hash for DependencyKind`

- <span id="dependencykind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for DependencyKind`

- <span id="dependencykind-eq"></span>`fn eq(&self, other: &DependencyKind) -> bool` — [`DependencyKind`](#dependencykind)

##### `impl Serialize for DependencyKind`

- <span id="dependencykind-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for DependencyKind`

##### `impl ToString for DependencyKind`

- <span id="dependencykind-to-string"></span>`fn to_string(&self) -> String`

## Functions

### `parse_dependency_kind`

```rust
fn parse_dependency_kind<'de, D>(d: D) -> Result<DependencyKind, <D as >::Error>
where
    D: Deserializer<'de>
```

*Defined in [`cargo_metadata-0.23.1/src/dependency.rs:40-45`](../../../.source_1765521767/cargo_metadata-0.23.1/src/dependency.rs#L40-L45)*

The `kind` can be `null`, which is interpreted as the default - `Normal`.

