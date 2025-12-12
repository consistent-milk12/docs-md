# Crate `semver`

[![github]](https://github.com/dtolnay/semver)&ensp;[![crates-io]](https://crates.io/crates/semver)&ensp;[![docs-rs]](https://docs.rs/semver)



<br>

A parser and evaluator for Cargo's flavor of Semantic Versioning.

Semantic Versioning (see <https://semver.org>) is a guideline for how
version numbers are assigned and incremented. It is widely followed within
the Cargo/crates.io ecosystem for Rust.

<br>

# Example

```rust
use semver::{BuildMetadata, Prerelease, Version, VersionReq};

fn main() {
    let req = VersionReq::parse(">=1.2.3, <1.8.0").unwrap();

    // Check whether this requirement matches version 1.2.3-alpha.1 (no)
    let version = Version {
        major: 1,
        minor: 2,
        patch: 3,
        pre: Prerelease::new("alpha.1").unwrap(),
        build: BuildMetadata::EMPTY,
    };
    assert!(!req.matches(&version));

    // Check whether it matches 1.3.0 (yes it does)
    let version = Version::parse("1.3.0").unwrap();
    assert!(req.matches(&version));
}
```

<br><br>

# Scope of this crate

Besides Cargo, several other package ecosystems and package managers for
other languages also use SemVer:&ensp;RubyGems/Bundler for Ruby, npm for
JavaScript, Composer for PHP, CocoaPods for Objective-C...

The `semver` crate is specifically intended to implement Cargo's
interpretation of Semantic Versioning.

Where the various tools differ in their interpretation or implementation of
the spec, this crate follows the implementation choices made by Cargo. If
you are operating on version numbers from some other package ecosystem, you
will want to use a different semver library which is appropriate to that
ecosystem.

The extent of Cargo's SemVer support is documented in the *[Specifying
Dependencies]* chapter of the Cargo reference.


## Contents

- [Modules](#modules)
  - [`display`](#display)
  - [`error`](#error)
  - [`eval`](#eval)
  - [`identifier`](#identifier)
  - [`impls`](#impls)
  - [`parse`](#parse)
  - [`serde`](#serde)
- [Structs](#structs)
  - [`Error`](#error)
  - [`Version`](#version)
  - [`VersionReq`](#versionreq)
  - [`Comparator`](#comparator)
  - [`Prerelease`](#prerelease)
  - [`BuildMetadata`](#buildmetadata)
- [Enums](#enums)
  - [`Op`](#op)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`display`](#display) | mod |  |
| [`error`](#error) | mod |  |
| [`eval`](#eval) | mod |  |
| [`identifier`](#identifier) | mod |  |
| [`impls`](#impls) | mod |  |
| [`parse`](#parse) | mod |  |
| [`serde`](#serde) | mod |  |
| [`Error`](#error) | struct |  |
| [`Version`](#version) | struct | **SemVer version** as defined by <https://semver.org>. |
| [`VersionReq`](#versionreq) | struct | **SemVer version requirement** describing the intersection of some version comparators, such as `>=1.2.3, <1.8`. |
| [`Comparator`](#comparator) | struct | A pair of comparison operator and partial version, such as `>=1.2`. |
| [`Prerelease`](#prerelease) | struct | Optional pre-release identifier on a version string. |
| [`BuildMetadata`](#buildmetadata) | struct | Optional build metadata identifier. |
| [`Op`](#op) | enum | SemVer comparison operator: `=`, `>`, `>=`, `<`, `<=`, `~`, `^`, `*`. |

## Modules

- [`display`](display/index.md)
- [`error`](error/index.md)
- [`eval`](eval/index.md)
- [`identifier`](identifier/index.md)
- [`impls`](impls/index.md)
- [`parse`](parse/index.md)
- [`serde`](serde/index.md)

## Structs

### `Error`

```rust
struct Error {
    kind: crate::error::ErrorKind,
}
```

*Defined in [`semver-1.0.27/src/parse.rs:21-23`](../../.source_1765521767/semver-1.0.27/src/parse.rs#L21-L23)*

Error parsing a SemVer version or version requirement.

# Example

```rust
use semver::Version;

fn main() {
    let err = Version::parse("1.q.r").unwrap_err();

    // "unexpected character 'q' while parsing minor version number"
    eprintln!("{}", err);
}
```

#### Implementations

- <span id="error-new"></span>`fn new(kind: ErrorKind) -> Self` — [`ErrorKind`](error/index.md#errorkind)

#### Trait Implementations

##### `impl Debug for crate::parse::Error`

- <span id="crateparseerror-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for crate::parse::Error`

- <span id="crateparseerror-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for crate::parse::Error`

##### `impl ToString for Error`

- <span id="error-to-string"></span>`fn to_string(&self) -> String`

### `Version`

```rust
struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre: Prerelease,
    pub build: BuildMetadata,
}
```

*Defined in [`semver-1.0.27/src/lib.rs:159-165`](../../.source_1765521767/semver-1.0.27/src/lib.rs#L159-L165)*

**SemVer version** as defined by <https://semver.org>.

# Syntax

- The major, minor, and patch numbers may be any integer 0 through u64::MAX.
  When representing a SemVer version as a string, each number is written as
  a base 10 integer. For example, `1.0.119`.

- Leading zeros are forbidden in those positions. For example `1.01.00` is
  invalid as a SemVer version.

- The pre-release identifier, if present, must conform to the syntax
  documented for [`Prerelease`](#prerelease).

- The build metadata, if present, must conform to the syntax documented for
  [`BuildMetadata`](#buildmetadata).

- Whitespace is not allowed anywhere in the version.

# Total ordering

Given any two SemVer versions, one is less than, greater than, or equal to
the other. Versions may be compared against one another using Rust's usual
comparison operators.

- The major, minor, and patch number are compared numerically from left to
  right, lexicographically ordered as a 3-tuple of integers. So for example
  version `1.5.0` is less than version `1.19.0`, despite the fact that
  "1.19.0" &lt; "1.5.0" as ASCIIbetically compared strings and 1.19 &lt; 1.5
  as real numbers.

- When major, minor, and patch are equal, a pre-release version is
  considered less than the ordinary release:&ensp;version `1.0.0-alpha.1` is
  less than version `1.0.0`.

- Two pre-releases of the same major, minor, patch are compared by
  lexicographic ordering of dot-separated components of the pre-release
  string.

  - Identifiers consisting of only digits are compared
    numerically:&ensp;`1.0.0-pre.8` is less than `1.0.0-pre.12`.

  - Identifiers that contain a letter or hyphen are compared in ASCII sort
    order:&ensp;`1.0.0-pre12` is less than `1.0.0-pre8`.

  - Any numeric identifier is always less than any non-numeric
    identifier:&ensp;`1.0.0-pre.1` is less than `1.0.0-pre.x`.

Example:&ensp;`1.0.0-alpha`&ensp;&lt;&ensp;`1.0.0-alpha.1`&ensp;&lt;&ensp;`1.0.0-alpha.beta`&ensp;&lt;&ensp;`1.0.0-beta`&ensp;&lt;&ensp;`1.0.0-beta.2`&ensp;&lt;&ensp;`1.0.0-beta.11`&ensp;&lt;&ensp;`1.0.0-rc.1`&ensp;&lt;&ensp;`1.0.0`

#### Implementations

- <span id="version-new"></span>`const fn new(major: u64, minor: u64, patch: u64) -> Self`

- <span id="version-parse"></span>`fn parse(text: &str) -> Result<Self, Error>` — [`Error`](parse/index.md#error)

- <span id="version-cmp-precedence"></span>`fn cmp_precedence(&self, other: &Self) -> Ordering`

#### Trait Implementations

##### `impl Clone for Version`

- <span id="version-clone"></span>`fn clone(&self) -> Version` — [`Version`](#version)

##### `impl Debug for crate::Version`

- <span id="crateversion-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for crate::Version`

- <span id="crateversion-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl DeserializeOwned for Version`

##### `impl Display for crate::Version`

- <span id="crateversion-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Version`

##### `impl FromStr for crate::Version`

- <span id="crateversion-fromstr-type-err"></span>`type Err = Error`

- <span id="crateversion-from-str"></span>`fn from_str(text: &str) -> Result<Self, <Self as >::Err>`

##### `impl Hash for Version`

- <span id="version-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Version`

- <span id="version-cmp"></span>`fn cmp(&self, other: &Version) -> cmp::Ordering` — [`Version`](#version)

##### `impl PartialEq for Version`

- <span id="version-eq"></span>`fn eq(&self, other: &Version) -> bool` — [`Version`](#version)

##### `impl PartialOrd for Version`

- <span id="version-partial-cmp"></span>`fn partial_cmp(&self, other: &Version) -> option::Option<cmp::Ordering>` — [`Version`](#version)

##### `impl Serialize for crate::Version`

- <span id="crateversion-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for Version`

##### `impl ToString for Version`

- <span id="version-to-string"></span>`fn to_string(&self) -> String`

### `VersionReq`

```rust
struct VersionReq {
    pub comparators: alloc::vec::Vec<Comparator>,
}
```

*Defined in [`semver-1.0.27/src/lib.rs:185-187`](../../.source_1765521767/semver-1.0.27/src/lib.rs#L185-L187)*

**SemVer version requirement** describing the intersection of some version
comparators, such as `>=1.2.3, <1.8`.

# Syntax

- Either `*` (meaning "any"), or one or more comma-separated comparators.

- A [`Comparator`](#comparator) is an operator ([`Op`](#op)) and a partial version, separated
  by optional whitespace. For example `>=1.0.0` or `>=1.0`.

- Build metadata is syntactically permitted on the partial versions, but is
  completely ignored, as it's never relevant to whether any comparator
  matches a particular version.

- Whitespace is permitted around commas and around operators. Whitespace is
  not permitted within a partial version, i.e. anywhere between the major
  version number and its minor, patch, pre-release, or build metadata.

#### Implementations

- <span id="versionreq-const-star"></span>`const STAR: Self`

- <span id="versionreq-parse"></span>`fn parse(text: &str) -> Result<Self, Error>` — [`Error`](parse/index.md#error)

- <span id="versionreq-matches"></span>`fn matches(&self, version: &Version) -> bool` — [`Version`](#version)

#### Trait Implementations

##### `impl Clone for VersionReq`

- <span id="versionreq-clone"></span>`fn clone(&self) -> VersionReq` — [`VersionReq`](#versionreq)

##### `impl Debug for VersionReq`

- <span id="versionreq-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for VersionReq`

- <span id="versionreq-default"></span>`fn default() -> Self`

##### `impl Deserialize for crate::VersionReq`

- <span id="crateversionreq-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl DeserializeOwned for VersionReq`

##### `impl Display for crate::VersionReq`

- <span id="crateversionreq-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for VersionReq`

##### `impl FromIterator for crate::VersionReq`

- <span id="crateversionreq-from-iter"></span>`fn from_iter<I>(iter: I) -> Self`

##### `impl FromStr for crate::VersionReq`

- <span id="crateversionreq-fromstr-type-err"></span>`type Err = Error`

- <span id="crateversionreq-from-str"></span>`fn from_str(text: &str) -> Result<Self, <Self as >::Err>`

##### `impl Hash for VersionReq`

- <span id="versionreq-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for VersionReq`

- <span id="versionreq-eq"></span>`fn eq(&self, other: &VersionReq) -> bool` — [`VersionReq`](#versionreq)

##### `impl Serialize for crate::VersionReq`

- <span id="crateversionreq-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for VersionReq`

##### `impl ToString for VersionReq`

- <span id="versionreq-to-string"></span>`fn to_string(&self) -> String`

### `Comparator`

```rust
struct Comparator {
    pub op: Op,
    pub major: u64,
    pub minor: Option<u64>,
    pub patch: Option<u64>,
    pub pre: Prerelease,
}
```

*Defined in [`semver-1.0.27/src/lib.rs:192-200`](../../.source_1765521767/semver-1.0.27/src/lib.rs#L192-L200)*

A pair of comparison operator and partial version, such as `>=1.2`. Forms
one piece of a VersionReq.

#### Fields

- **`patch`**: `Option<u64>`

  Patch is only allowed if minor is Some.

- **`pre`**: `Prerelease`

  Non-empty pre-release is only allowed if patch is Some.

#### Implementations

- <span id="comparator-parse"></span>`fn parse(text: &str) -> Result<Self, Error>` — [`Error`](parse/index.md#error)

- <span id="comparator-matches"></span>`fn matches(&self, version: &Version) -> bool` — [`Version`](#version)

#### Trait Implementations

##### `impl Clone for Comparator`

- <span id="comparator-clone"></span>`fn clone(&self) -> Comparator` — [`Comparator`](#comparator)

##### `impl Debug for Comparator`

- <span id="comparator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for crate::Comparator`

- <span id="cratecomparator-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl DeserializeOwned for Comparator`

##### `impl Display for crate::Comparator`

- <span id="cratecomparator-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Comparator`

##### `impl FromIterator for crate::VersionReq`

- <span id="crateversionreq-from-iter"></span>`fn from_iter<I>(iter: I) -> Self`

##### `impl FromStr for crate::Comparator`

- <span id="cratecomparator-fromstr-type-err"></span>`type Err = Error`

- <span id="cratecomparator-from-str"></span>`fn from_str(text: &str) -> Result<Self, <Self as >::Err>`

##### `impl Hash for Comparator`

- <span id="comparator-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Comparator`

- <span id="comparator-eq"></span>`fn eq(&self, other: &Comparator) -> bool` — [`Comparator`](#comparator)

##### `impl Serialize for crate::Comparator`

- <span id="cratecomparator-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for Comparator`

##### `impl ToString for Comparator`

- <span id="comparator-to-string"></span>`fn to_string(&self) -> String`

### `Prerelease`

```rust
struct Prerelease {
    identifier: crate::identifier::Identifier,
}
```

*Defined in [`semver-1.0.27/src/lib.rs:310-312`](../../.source_1765521767/semver-1.0.27/src/lib.rs#L310-L312)*

Optional pre-release identifier on a version string. This comes after `-` in
a SemVer version, like `1.0.0-alpha.1`

# Examples

Some real world pre-release idioms drawn from crates.io:

- **[mio]** <code>0.7.0-<b>alpha.1</b></code> &mdash; the most common style
  for numbering pre-releases.

- **[pest]** <code>1.0.0-<b>beta.8</b></code>,&ensp;<code>1.0.0-<b>rc.0</b></code>
  &mdash; this crate makes a distinction between betas and release
  candidates.

- **[sassers]** <code>0.11.0-<b>shitshow</b></code> &mdash; ???.

- **[atomic-utils]** <code>0.0.0-<b>reserved</b></code> &mdash; a squatted
  crate name.




*Tip:* Be aware that if you are planning to number your own pre-releases,
you should prefer to separate the numeric part from any non-numeric
identifiers by using a dot in between. That is, prefer pre-releases
`alpha.1`, `alpha.2`, etc rather than `alpha1`, `alpha2` etc. The SemVer
spec's rule for pre-release precedence has special treatment of numeric
components in the pre-release string, but only if there are no non-digit
characters in the same dot-separated component. So you'd have `alpha.2` &lt;
`alpha.11` as intended, but `alpha11` &lt; `alpha2`.

# Syntax

Pre-release strings are a series of dot separated identifiers immediately
following the patch version. Identifiers must comprise only ASCII
alphanumerics and hyphens: `0-9`, `A-Z`, `a-z`, `-`. Identifiers must not be
empty. Numeric identifiers must not include leading zeros.

# Total ordering

Pre-releases have a total order defined by the SemVer spec. It uses
lexicographic ordering of dot-separated components. Identifiers consisting
of only digits are compared numerically. Otherwise, identifiers are compared
in ASCII sort order. Any numeric identifier is always less than any
non-numeric identifier.

Example:&ensp;`alpha`&ensp;&lt;&ensp;`alpha.85`&ensp;&lt;&ensp;`alpha.90`&ensp;&lt;&ensp;`alpha.200`&ensp;&lt;&ensp;`alpha.0a`&ensp;&lt;&ensp;`alpha.1a0`&ensp;&lt;&ensp;`alpha.a`&ensp;&lt;&ensp;`beta`

#### Implementations

- <span id="prerelease-const-empty"></span>`const EMPTY: Self`

- <span id="prerelease-new"></span>`fn new(text: &str) -> Result<Self, Error>` — [`Error`](parse/index.md#error)

- <span id="prerelease-as-str"></span>`fn as_str(&self) -> &str`

- <span id="prerelease-is-empty"></span>`fn is_empty(&self) -> bool`

#### Trait Implementations

##### `impl Clone for Prerelease`

- <span id="prerelease-clone"></span>`fn clone(&self) -> Prerelease` — [`Prerelease`](#prerelease)

##### `impl Debug for crate::Prerelease`

- <span id="crateprerelease-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Prerelease`

- <span id="prerelease-default"></span>`fn default() -> Prerelease` — [`Prerelease`](#prerelease)

##### `impl Deref for crate::Prerelease`

- <span id="crateprerelease-deref-type-target"></span>`type Target = str`

- <span id="crateprerelease-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Display for crate::Prerelease`

- <span id="crateprerelease-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Prerelease`

##### `impl FromStr for crate::Prerelease`

- <span id="crateprerelease-fromstr-type-err"></span>`type Err = Error`

- <span id="crateprerelease-from-str"></span>`fn from_str(text: &str) -> Result<Self, <Self as >::Err>`

##### `impl Hash for Prerelease`

- <span id="prerelease-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for crate::Prerelease`

- <span id="crateprerelease-cmp"></span>`fn cmp(&self, rhs: &Self) -> Ordering`

##### `impl PartialEq for Prerelease`

- <span id="prerelease-eq"></span>`fn eq(&self, other: &Prerelease) -> bool` — [`Prerelease`](#prerelease)

##### `impl PartialOrd for crate::Prerelease`

- <span id="crateprerelease-partial-cmp"></span>`fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>`

##### `impl Receiver for Prerelease`

- <span id="prerelease-receiver-type-target"></span>`type Target = T`

##### `impl StructuralPartialEq for Prerelease`

##### `impl ToString for Prerelease`

- <span id="prerelease-to-string"></span>`fn to_string(&self) -> String`

### `BuildMetadata`

```rust
struct BuildMetadata {
    identifier: crate::identifier::Identifier,
}
```

*Defined in [`semver-1.0.27/src/lib.rs:368-370`](../../.source_1765521767/semver-1.0.27/src/lib.rs#L368-L370)*

Optional build metadata identifier. This comes after `+` in a SemVer
version, as in `0.8.1+zstd.1.5.0`.

# Examples

Some real world build metadata idioms drawn from crates.io:

- **[libgit2-sys]** <code>0.12.20+<b>1.1.0</b></code> &mdash; for this
  crate, the build metadata indicates the version of the C libgit2 library
  that the Rust crate is built against.

- **[mashup]** <code>0.1.13+<b>deprecated</b></code> &mdash; just the word
  "deprecated" for a crate that has been superseded by another. Eventually
  people will take notice of this in Cargo's build output where it lists the
  crates being compiled.

- **[google-bigquery2]** <code>2.0.4+<b>20210327</b></code> &mdash; this
  library is automatically generated from an official API schema, and the
  build metadata indicates the date on which that schema was last captured.

- **[fbthrift-git]** <code>0.0.6+<b>c7fcc0e</b></code> &mdash; this crate is
  published from snapshots of a big company monorepo. In monorepo
  development, there is no concept of versions, and all downstream code is
  just updated atomically in the same commit that breaking changes to a
  library are landed. Therefore for crates.io purposes, every published
  version must be assumed to be incompatible with the previous. The build
  metadata provides the source control hash of the snapshotted code.




# Syntax

Build metadata is a series of dot separated identifiers immediately
following the patch or pre-release version. Identifiers must comprise only
ASCII alphanumerics and hyphens: `0-9`, `A-Z`, `a-z`, `-`. Identifiers must
not be empty. Leading zeros *are* allowed, unlike any other place in the
SemVer grammar.

# Total ordering

Build metadata is ignored in evaluating `VersionReq`; it plays no role in
whether a `Version` matches any one of the comparison operators.

However for comparing build metadatas among one another, they do have a
total order which is determined by lexicographic ordering of dot-separated
components. Identifiers consisting of only digits are compared numerically.
Otherwise, identifiers are compared in ASCII sort order. Any numeric
identifier is always less than any non-numeric identifier.

Example:&ensp;`demo`&ensp;&lt;&ensp;`demo.85`&ensp;&lt;&ensp;`demo.90`&ensp;&lt;&ensp;`demo.090`&ensp;&lt;&ensp;`demo.200`&ensp;&lt;&ensp;`demo.1a0`&ensp;&lt;&ensp;`demo.a`&ensp;&lt;&ensp;`memo`

#### Implementations

- <span id="buildmetadata-const-empty"></span>`const EMPTY: Self`

- <span id="buildmetadata-new"></span>`fn new(text: &str) -> Result<Self, Error>` — [`Error`](parse/index.md#error)

- <span id="buildmetadata-as-str"></span>`fn as_str(&self) -> &str`

- <span id="buildmetadata-is-empty"></span>`fn is_empty(&self) -> bool`

#### Trait Implementations

##### `impl Clone for BuildMetadata`

- <span id="buildmetadata-clone"></span>`fn clone(&self) -> BuildMetadata` — [`BuildMetadata`](#buildmetadata)

##### `impl Debug for crate::BuildMetadata`

- <span id="cratebuildmetadata-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BuildMetadata`

- <span id="buildmetadata-default"></span>`fn default() -> BuildMetadata` — [`BuildMetadata`](#buildmetadata)

##### `impl Deref for crate::BuildMetadata`

- <span id="cratebuildmetadata-deref-type-target"></span>`type Target = str`

- <span id="cratebuildmetadata-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Display for crate::BuildMetadata`

- <span id="cratebuildmetadata-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for BuildMetadata`

##### `impl FromStr for crate::BuildMetadata`

- <span id="cratebuildmetadata-fromstr-type-err"></span>`type Err = Error`

- <span id="cratebuildmetadata-from-str"></span>`fn from_str(text: &str) -> Result<Self, <Self as >::Err>`

##### `impl Hash for BuildMetadata`

- <span id="buildmetadata-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for crate::BuildMetadata`

- <span id="cratebuildmetadata-cmp"></span>`fn cmp(&self, rhs: &Self) -> Ordering`

##### `impl PartialEq for BuildMetadata`

- <span id="buildmetadata-eq"></span>`fn eq(&self, other: &BuildMetadata) -> bool` — [`BuildMetadata`](#buildmetadata)

##### `impl PartialOrd for crate::BuildMetadata`

- <span id="cratebuildmetadata-partial-cmp"></span>`fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>`

##### `impl Receiver for BuildMetadata`

- <span id="buildmetadata-receiver-type-target"></span>`type Target = T`

##### `impl StructuralPartialEq for BuildMetadata`

##### `impl ToString for BuildMetadata`

- <span id="buildmetadata-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `Op`

```rust
enum Op {
    Exact,
    Greater,
    GreaterEq,
    Less,
    LessEq,
    Tilde,
    Caret,
    Wildcard,
}
```

*Defined in [`semver-1.0.27/src/lib.rs:249-258`](../../.source_1765521767/semver-1.0.27/src/lib.rs#L249-L258)*

SemVer comparison operator: `=`, `>`, `>=`, `<`, `<=`, `~`, `^`, `*`.

# Op::Exact
- &ensp;**`=I.J.K`**&emsp;&mdash;&emsp;exactly the version I.J.K
- &ensp;**`=I.J`**&emsp;&mdash;&emsp;equivalent to `>=I.J.0, <I.(J+1).0`
- &ensp;**`=I`**&emsp;&mdash;&emsp;equivalent to `>=I.0.0, <(I+1).0.0`

# Op::Greater
- &ensp;**`>I.J.K`**
- &ensp;**`>I.J`**&emsp;&mdash;&emsp;equivalent to `>=I.(J+1).0`
- &ensp;**`>I`**&emsp;&mdash;&emsp;equivalent to `>=(I+1).0.0`

# Op::GreaterEq
- &ensp;**`>=I.J.K`**
- &ensp;**`>=I.J`**&emsp;&mdash;&emsp;equivalent to `>=I.J.0`
- &ensp;**`>=I`**&emsp;&mdash;&emsp;equivalent to `>=I.0.0`

# Op::Less
- &ensp;**`<I.J.K`**
- &ensp;**`<I.J`**&emsp;&mdash;&emsp;equivalent to `<I.J.0`
- &ensp;**`<I`**&emsp;&mdash;&emsp;equivalent to `<I.0.0`

# Op::LessEq
- &ensp;**`<=I.J.K`**
- &ensp;**`<=I.J`**&emsp;&mdash;&emsp;equivalent to `<I.(J+1).0`
- &ensp;**`<=I`**&emsp;&mdash;&emsp;equivalent to `<(I+1).0.0`

# Op::Tilde&emsp;("patch" updates)
*Tilde requirements allow the **patch** part of the semver version (the third number) to increase.*
- &ensp;**`~I.J.K`**&emsp;&mdash;&emsp;equivalent to `>=I.J.K, <I.(J+1).0`
- &ensp;**`~I.J`**&emsp;&mdash;&emsp;equivalent to `=I.J`
- &ensp;**`~I`**&emsp;&mdash;&emsp;equivalent to `=I`

# Op::Caret&emsp;("compatible" updates)
*Caret requirements allow parts that are **right of the first nonzero** part of the semver version to increase.*
- &ensp;**`^I.J.K`**&ensp;(for I\>0)&emsp;&mdash;&emsp;equivalent to `>=I.J.K, <(I+1).0.0`
- &ensp;**`^0.J.K`**&ensp;(for J\>0)&emsp;&mdash;&emsp;equivalent to `>=0.J.K, <0.(J+1).0`
- &ensp;**`^0.0.K`**&emsp;&mdash;&emsp;equivalent to `=0.0.K`
- &ensp;**`^I.J`**&ensp;(for I\>0 or J\>0)&emsp;&mdash;&emsp;equivalent to `^I.J.0`
- &ensp;**`^0.0`**&emsp;&mdash;&emsp;equivalent to `=0.0`
- &ensp;**`^I`**&emsp;&mdash;&emsp;equivalent to `=I`

# Op::Wildcard
- &ensp;**`I.J.*`**&emsp;&mdash;&emsp;equivalent to `=I.J`
- &ensp;**`I.*`**&ensp;or&ensp;**`I.*.*`**&emsp;&mdash;&emsp;equivalent to `=I`

#### Implementations

- <span id="crateop-const-default"></span>`const DEFAULT: Self`

#### Trait Implementations

##### `impl Clone for Op`

- <span id="op-clone"></span>`fn clone(&self) -> Op` — [`Op`](#op)

##### `impl Copy for Op`

##### `impl Debug for Op`

- <span id="op-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Op`

##### `impl Hash for Op`

- <span id="op-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Op`

- <span id="op-eq"></span>`fn eq(&self, other: &Op) -> bool` — [`Op`](#op)

##### `impl StructuralPartialEq for Op`

