*[cargo_metadata](../index.md) / [messages](index.md)*

---

# Module `messages`

## Contents

- [Structs](#structs)
  - [`ArtifactProfile`](#artifactprofile)
  - [`Artifact`](#artifact)
  - [`CompilerMessage`](#compilermessage)
  - [`BuildScript`](#buildscript)
  - [`BuildFinished`](#buildfinished)
  - [`MessageIter`](#messageiter)
- [Enums](#enums)
  - [`ArtifactDebuginfo`](#artifactdebuginfo)
  - [`Message`](#message)
- [Functions](#functions)
  - [`parse_messages`](#parse-messages)
- [Type Aliases](#type-aliases)
  - [`MessageIterator`](#messageiterator)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ArtifactProfile`](#artifactprofile) | struct | Profile settings used to determine which compiler flags to use for a target. |
| [`Artifact`](#artifact) | struct | A compiler-generated file. |
| [`CompilerMessage`](#compilermessage) | struct | Message left by the compiler |
| [`BuildScript`](#buildscript) | struct | Output of a build script execution. |
| [`BuildFinished`](#buildfinished) | struct | Final result of a build. |
| [`MessageIter`](#messageiter) | struct | An iterator of Messages. |
| [`ArtifactDebuginfo`](#artifactdebuginfo) | enum | The kind of debug information included in the artifact. |
| [`Message`](#message) | enum | A cargo message |
| [`parse_messages`](#parse-messages) | fn | Creates an iterator of Message from a Read outputting a stream of JSON messages. |
| [`MessageIterator`](#messageiterator) | type | An iterator of Message. |

## Structs

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

*Defined in [`cargo_metadata-0.23.1/src/messages.rs:15-28`](../../../.source_1765521767/cargo_metadata-0.23.1/src/messages.rs#L15-L28)*

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

##### `impl Clone for ArtifactProfile`

- <span id="artifactprofile-clone"></span>`fn clone(&self) -> ArtifactProfile` — [`ArtifactProfile`](#artifactprofile)

##### `impl Debug for ArtifactProfile`

- <span id="artifactprofile-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for ArtifactProfile`

- <span id="artifactprofile-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for ArtifactProfile`

##### `impl Eq for ArtifactProfile`

##### `impl Hash for ArtifactProfile`

- <span id="artifactprofile-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ArtifactProfile`

- <span id="artifactprofile-eq"></span>`fn eq(&self, other: &ArtifactProfile) -> bool` — [`ArtifactProfile`](#artifactprofile)

##### `impl Serialize for ArtifactProfile`

- <span id="artifactprofile-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for ArtifactProfile`

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

*Defined in [`cargo_metadata-0.23.1/src/messages.rs:156-175`](../../../.source_1765521767/cargo_metadata-0.23.1/src/messages.rs#L156-L175)*

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

##### `impl Clone for Artifact`

- <span id="artifact-clone"></span>`fn clone(&self) -> Artifact` — [`Artifact`](#artifact)

##### `impl Debug for Artifact`

- <span id="artifact-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Artifact`

- <span id="artifact-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Artifact`

##### `impl Eq for Artifact`

##### `impl Hash for Artifact`

- <span id="artifact-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Artifact`

- <span id="artifact-eq"></span>`fn eq(&self, other: &Artifact) -> bool` — [`Artifact`](#artifact)

##### `impl Serialize for Artifact`

- <span id="artifact-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Artifact`

### `CompilerMessage`

```rust
struct CompilerMessage {
    pub package_id: super::PackageId,
    pub target: super::Target,
    pub message: diagnostic::Diagnostic,
}
```

*Defined in [`cargo_metadata-0.23.1/src/messages.rs:183-190`](../../../.source_1765521767/cargo_metadata-0.23.1/src/messages.rs#L183-L190)*

Message left by the compiler

#### Fields

- **`package_id`**: `super::PackageId`

  The package this message belongs to

- **`target`**: `super::Target`

  The target this message is aimed at

- **`message`**: `diagnostic::Diagnostic`

  The message the compiler sent.

#### Trait Implementations

##### `impl Clone for CompilerMessage`

- <span id="compilermessage-clone"></span>`fn clone(&self) -> CompilerMessage` — [`CompilerMessage`](#compilermessage)

##### `impl Debug for CompilerMessage`

- <span id="compilermessage-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for CompilerMessage`

- <span id="compilermessage-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for CompilerMessage`

##### `impl Display for CompilerMessage`

- <span id="compilermessage-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CompilerMessage`

##### `impl Hash for CompilerMessage`

- <span id="compilermessage-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for CompilerMessage`

- <span id="compilermessage-eq"></span>`fn eq(&self, other: &CompilerMessage) -> bool` — [`CompilerMessage`](#compilermessage)

##### `impl Serialize for CompilerMessage`

- <span id="compilermessage-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for CompilerMessage`

##### `impl ToString for CompilerMessage`

- <span id="compilermessage-to-string"></span>`fn to_string(&self) -> String`

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

*Defined in [`cargo_metadata-0.23.1/src/messages.rs:197-213`](../../../.source_1765521767/cargo_metadata-0.23.1/src/messages.rs#L197-L213)*

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

##### `impl Clone for BuildScript`

- <span id="buildscript-clone"></span>`fn clone(&self) -> BuildScript` — [`BuildScript`](#buildscript)

##### `impl Debug for BuildScript`

- <span id="buildscript-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for BuildScript`

- <span id="buildscript-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for BuildScript`

##### `impl Eq for BuildScript`

##### `impl Hash for BuildScript`

- <span id="buildscript-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for BuildScript`

- <span id="buildscript-eq"></span>`fn eq(&self, other: &BuildScript) -> bool` — [`BuildScript`](#buildscript)

##### `impl Serialize for BuildScript`

- <span id="buildscript-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for BuildScript`

### `BuildFinished`

```rust
struct BuildFinished {
    pub success: bool,
}
```

*Defined in [`cargo_metadata-0.23.1/src/messages.rs:220-223`](../../../.source_1765521767/cargo_metadata-0.23.1/src/messages.rs#L220-L223)*

Final result of a build.

#### Fields

- **`success`**: `bool`

  Whether or not the build finished successfully.

#### Trait Implementations

##### `impl Clone for BuildFinished`

- <span id="buildfinished-clone"></span>`fn clone(&self) -> BuildFinished` — [`BuildFinished`](#buildfinished)

##### `impl Debug for BuildFinished`

- <span id="buildfinished-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for BuildFinished`

- <span id="buildfinished-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for BuildFinished`

##### `impl Eq for BuildFinished`

##### `impl Hash for BuildFinished`

- <span id="buildfinished-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for BuildFinished`

- <span id="buildfinished-eq"></span>`fn eq(&self, other: &BuildFinished) -> bool` — [`BuildFinished`](#buildfinished)

##### `impl Serialize for BuildFinished`

- <span id="buildfinished-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for BuildFinished`

### `MessageIter<R>`

```rust
struct MessageIter<R> {
    input: R,
}
```

*Defined in [`cargo_metadata-0.23.1/src/messages.rs:262-264`](../../../.source_1765521767/cargo_metadata-0.23.1/src/messages.rs#L262-L264)*

An iterator of Messages.

#### Trait Implementations

##### `impl IntoIterator for MessageIter<R>`

- <span id="messageiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="messageiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="messageiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: BufRead> Iterator for MessageIter<R>`

- <span id="messageiter-iterator-type-item"></span>`type Item = Result<Message, Error>`

- <span id="messageiter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Enums

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

*Defined in [`cargo_metadata-0.23.1/src/messages.rs:33-57`](../../../.source_1765521767/cargo_metadata-0.23.1/src/messages.rs#L33-L57)*

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

##### `impl Clone for ArtifactDebuginfo`

- <span id="artifactdebuginfo-clone"></span>`fn clone(&self) -> ArtifactDebuginfo` — [`ArtifactDebuginfo`](#artifactdebuginfo)

##### `impl Debug for ArtifactDebuginfo`

- <span id="artifactdebuginfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ArtifactDebuginfo`

- <span id="artifactdebuginfo-default"></span>`fn default() -> ArtifactDebuginfo` — [`ArtifactDebuginfo`](#artifactdebuginfo)

##### `impl Deserialize for ArtifactDebuginfo`

- <span id="artifactdebuginfo-deserialize"></span>`fn deserialize<D>(d: D) -> Result<ArtifactDebuginfo, <D as >::Error>` — [`ArtifactDebuginfo`](#artifactdebuginfo)

##### `impl DeserializeOwned for ArtifactDebuginfo`

##### `impl Display for ArtifactDebuginfo`

- <span id="artifactdebuginfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ArtifactDebuginfo`

##### `impl Hash for ArtifactDebuginfo`

- <span id="artifactdebuginfo-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ArtifactDebuginfo`

- <span id="artifactdebuginfo-eq"></span>`fn eq(&self, other: &ArtifactDebuginfo) -> bool` — [`ArtifactDebuginfo`](#artifactdebuginfo)

##### `impl Serialize for ArtifactDebuginfo`

- <span id="artifactdebuginfo-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for ArtifactDebuginfo`

##### `impl ToString for ArtifactDebuginfo`

- <span id="artifactdebuginfo-to-string"></span>`fn to_string(&self) -> String`

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

*Defined in [`cargo_metadata-0.23.1/src/messages.rs:229-245`](../../../.source_1765521767/cargo_metadata-0.23.1/src/messages.rs#L229-L245)*

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

- <span id="message-parse-stream"></span>`fn parse_stream<R: Read>(input: R) -> MessageIter<R>` — [`MessageIter`](#messageiter)

#### Trait Implementations

##### `impl Clone for Message`

- <span id="message-clone"></span>`fn clone(&self) -> Message` — [`Message`](#message)

##### `impl Debug for Message`

- <span id="message-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Message`

- <span id="message-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Message`

##### `impl Eq for Message`

##### `impl Hash for Message`

- <span id="message-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Message`

- <span id="message-eq"></span>`fn eq(&self, other: &Message) -> bool` — [`Message`](#message)

##### `impl Serialize for Message`

- <span id="message-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Message`

## Functions

### `parse_messages`

```rust
fn parse_messages<R: Read>(input: R) -> serde_json::StreamDeserializer<'static, serde_json::de::IoRead<R>, Message>
```

*Defined in [`cargo_metadata-0.23.1/src/messages.rs:295-297`](../../../.source_1765521767/cargo_metadata-0.23.1/src/messages.rs#L295-L297)*

Creates an iterator of Message from a Read outputting a stream of JSON
messages. For usage information, look at the top-level documentation.

## Type Aliases

### `MessageIterator<R>`

```rust
type MessageIterator<R> = serde_json::StreamDeserializer<'static, serde_json::de::IoRead<R>, Message>;
```

*Defined in [`cargo_metadata-0.23.1/src/messages.rs:289-290`](../../../.source_1765521767/cargo_metadata-0.23.1/src/messages.rs#L289-L290)*

An iterator of Message.

