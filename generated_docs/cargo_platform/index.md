# Crate `cargo_platform`

Platform definition used by Cargo.

This defines a [`Platform`](#platform) type which is used in Cargo to specify a target platform.
There are two kinds, a named target like `x86_64-apple-darwin`, and a "cfg expression"
like `cfg(any(target_os = "macos", target_os = "ios"))`.

See `examples/matches.rs` for an example of how to match against a `Platform`.

> This crate is maintained by the Cargo team for use by the wider
> ecosystem. This crate follows semver compatibility for its APIs.


## Contents

- [Modules](#modules)
  - [`cfg`](#cfg)
  - [`error`](#error)
- [Structs](#structs)
  - [`Ident`](#ident)
  - [`ParseError`](#parseerror)
- [Enums](#enums)
  - [`Cfg`](#cfg)
  - [`CfgExpr`](#cfgexpr)
  - [`ParseErrorKind`](#parseerrorkind)
  - [`Platform`](#platform)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`cfg`](#cfg) | mod |  |
| [`error`](#error) | mod |  |
| [`Ident`](#ident) | struct |  |
| [`ParseError`](#parseerror) | struct |  |
| [`Cfg`](#cfg) | enum |  |
| [`CfgExpr`](#cfgexpr) | enum |  |
| [`ParseErrorKind`](#parseerrorkind) | enum |  |
| [`Platform`](#platform) | enum | Platform definition. |

## Modules

- [`cfg`](cfg/index.md)
- [`error`](error/index.md)

## Structs

### `Ident`

```rust
struct Ident {
    pub name: String,
    pub raw: bool,
}
```

*Defined in [`cargo-platform-0.3.1/src/cfg.rs:29-37`](../../.source_1765521767/cargo-platform-0.3.1/src/cfg.rs#L29-L37)*

A identifier

#### Fields

- **`name`**: `String`

  The identifier

- **`raw`**: `bool`

  Is this a raw ident: `r#async`
  
  It's mainly used for display and doesn't take
  part in the hash or equality (`foo` == `r#foo`).

#### Implementations

- <span id="ident-as-str"></span>`fn as_str(&self) -> &str`

#### Trait Implementations

##### `impl Clone for Ident`

- <span id="ident-clone"></span>`fn clone(&self) -> Ident` — [`Ident`](cfg/index.md#ident)

##### `impl Debug for Ident`

- <span id="ident-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Ident`

- <span id="ident-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Ident`

##### `impl Hash for Ident`

- <span id="ident-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl Ord for Ident`

- <span id="ident-cmp"></span>`fn cmp(&self, other: &Ident) -> cmp::Ordering` — [`Ident`](cfg/index.md#ident)

##### `impl PartialEq for Ident`

- <span id="ident-eq"></span>`fn eq(&self, other: &str) -> bool`

##### `impl PartialOrd for Ident`

- <span id="ident-partial-cmp"></span>`fn partial_cmp(&self, other: &Ident) -> option::Option<cmp::Ordering>` — [`Ident`](cfg/index.md#ident)

##### `impl ToString for Ident`

- <span id="ident-to-string"></span>`fn to_string(&self) -> String`

### `ParseError`

```rust
struct ParseError {
    kind: ParseErrorKind,
    orig: String,
}
```

*Defined in [`cargo-platform-0.3.1/src/error.rs:4-7`](../../.source_1765521767/cargo-platform-0.3.1/src/error.rs#L4-L7)*

#### Implementations

- <span id="parseerror-new"></span>`fn new(orig: &str, kind: ParseErrorKind) -> ParseError` — [`ParseErrorKind`](error/index.md#parseerrorkind), [`ParseError`](error/index.md#parseerror)

#### Trait Implementations

##### `impl Debug for ParseError`

- <span id="parseerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ParseError`

- <span id="parseerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for ParseError`

##### `impl ToString for ParseError`

- <span id="parseerror-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `Cfg`

```rust
enum Cfg {
    Name(Ident),
    KeyPair(Ident, String),
}
```

*Defined in [`cargo-platform-0.3.1/src/cfg.rs:20-25`](../../.source_1765521767/cargo-platform-0.3.1/src/cfg.rs#L20-L25)*

A cfg value.

#### Variants

- **`Name`**

  A named cfg value, like `unix`.

- **`KeyPair`**

  A key/value cfg pair, like `target_os = "linux"`.

#### Trait Implementations

##### `impl Clone for Cfg`

- <span id="cfg-clone"></span>`fn clone(&self) -> Cfg` — [`Cfg`](cfg/index.md#cfg)

##### `impl Debug for Cfg`

- <span id="cfg-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Cfg`

- <span id="cfg-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Cfg`

##### `impl FromStr for Cfg`

- <span id="cfg-fromstr-type-err"></span>`type Err = ParseError`

- <span id="cfg-from-str"></span>`fn from_str(s: &str) -> Result<Cfg, <Self as >::Err>` — [`Cfg`](cfg/index.md#cfg)

##### `impl Hash for Cfg`

- <span id="cfg-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Cfg`

- <span id="cfg-cmp"></span>`fn cmp(&self, other: &Cfg) -> cmp::Ordering` — [`Cfg`](cfg/index.md#cfg)

##### `impl PartialEq for Cfg`

- <span id="cfg-eq"></span>`fn eq(&self, other: &Cfg) -> bool` — [`Cfg`](cfg/index.md#cfg)

##### `impl PartialOrd for Cfg`

- <span id="cfg-partial-cmp"></span>`fn partial_cmp(&self, other: &Cfg) -> option::Option<cmp::Ordering>` — [`Cfg`](cfg/index.md#cfg)

##### `impl StructuralPartialEq for Cfg`

##### `impl ToString for Cfg`

- <span id="cfg-to-string"></span>`fn to_string(&self) -> String`

### `CfgExpr`

```rust
enum CfgExpr {
    Not(Box<CfgExpr>),
    All(Vec<CfgExpr>),
    Any(Vec<CfgExpr>),
    Value(Cfg),
    True,
    False,
}
```

*Defined in [`cargo-platform-0.3.1/src/cfg.rs:9-16`](../../.source_1765521767/cargo-platform-0.3.1/src/cfg.rs#L9-L16)*

A cfg expression.

#### Implementations

- <span id="cfgexpr-matches-key"></span>`fn matches_key(key: &str, target_cfg: &[Cfg]) -> bool` — [`Cfg`](cfg/index.md#cfg)

- <span id="cfgexpr-matches"></span>`fn matches(&self, cfg: &[Cfg]) -> bool` — [`Cfg`](cfg/index.md#cfg)

#### Trait Implementations

##### `impl Clone for CfgExpr`

- <span id="cfgexpr-clone"></span>`fn clone(&self) -> CfgExpr` — [`CfgExpr`](cfg/index.md#cfgexpr)

##### `impl Debug for CfgExpr`

- <span id="cfgexpr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for CfgExpr`

- <span id="cfgexpr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CfgExpr`

##### `impl FromStr for CfgExpr`

- <span id="cfgexpr-fromstr-type-err"></span>`type Err = ParseError`

- <span id="cfgexpr-from-str"></span>`fn from_str(s: &str) -> Result<CfgExpr, <Self as >::Err>` — [`CfgExpr`](cfg/index.md#cfgexpr)

##### `impl Hash for CfgExpr`

- <span id="cfgexpr-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for CfgExpr`

- <span id="cfgexpr-cmp"></span>`fn cmp(&self, other: &CfgExpr) -> cmp::Ordering` — [`CfgExpr`](cfg/index.md#cfgexpr)

##### `impl PartialEq for CfgExpr`

- <span id="cfgexpr-eq"></span>`fn eq(&self, other: &CfgExpr) -> bool` — [`CfgExpr`](cfg/index.md#cfgexpr)

##### `impl PartialOrd for CfgExpr`

- <span id="cfgexpr-partial-cmp"></span>`fn partial_cmp(&self, other: &CfgExpr) -> option::Option<cmp::Ordering>` — [`CfgExpr`](cfg/index.md#cfgexpr)

##### `impl StructuralPartialEq for CfgExpr`

##### `impl ToString for CfgExpr`

- <span id="cfgexpr-to-string"></span>`fn to_string(&self) -> String`

### `ParseErrorKind`

```rust
enum ParseErrorKind {
    UnterminatedString,
    UnexpectedChar(char),
    UnexpectedToken {
        expected: &'static str,
        found: &'static str,
    },
    IncompleteExpr(&'static str),
    UnterminatedExpression(String),
    InvalidTarget(String),
}
```

*Defined in [`cargo-platform-0.3.1/src/error.rs:11-21`](../../.source_1765521767/cargo-platform-0.3.1/src/error.rs#L11-L21)*

#### Trait Implementations

##### `impl Debug for ParseErrorKind`

- <span id="parseerrorkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ParseErrorKind`

- <span id="parseerrorkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for ParseErrorKind`

- <span id="parseerrorkind-to-string"></span>`fn to_string(&self) -> String`

### `Platform`

```rust
enum Platform {
    Name(String),
    Cfg(CfgExpr),
}
```

*Defined in [`cargo-platform-0.3.1/src/lib.rs:26-31`](../../.source_1765521767/cargo-platform-0.3.1/src/lib.rs#L26-L31)*

Platform definition.

#### Variants

- **`Name`**

  A named platform, like `x86_64-apple-darwin`.

- **`Cfg`**

  A cfg expression, like `cfg(windows)`.

#### Implementations

- <span id="platform-matches"></span>`fn matches(&self, name: &str, cfg: &[Cfg]) -> bool` — [`Cfg`](cfg/index.md#cfg)

- <span id="platform-validate-named-platform"></span>`fn validate_named_platform(name: &str) -> Result<(), ParseError>` — [`ParseError`](error/index.md#parseerror)

- <span id="platform-check-cfg-attributes"></span>`fn check_cfg_attributes(&self, warnings: &mut Vec<String>)`

- <span id="platform-check-cfg-keywords"></span>`fn check_cfg_keywords(&self, warnings: &mut Vec<String>, path: &Path)`

#### Trait Implementations

##### `impl Clone for Platform`

- <span id="platform-clone"></span>`fn clone(&self) -> Platform` — [`Platform`](#platform)

##### `impl Debug for Platform`

- <span id="platform-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Platform`

- <span id="platform-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl DeserializeOwned for Platform`

##### `impl Display for Platform`

- <span id="platform-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Platform`

##### `impl FromStr for Platform`

- <span id="platform-fromstr-type-err"></span>`type Err = ParseError`

- <span id="platform-from-str"></span>`fn from_str(s: &str) -> Result<Platform, ParseError>` — [`Platform`](#platform), [`ParseError`](error/index.md#parseerror)

##### `impl Hash for Platform`

- <span id="platform-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Platform`

- <span id="platform-cmp"></span>`fn cmp(&self, other: &Platform) -> cmp::Ordering` — [`Platform`](#platform)

##### `impl PartialEq for Platform`

- <span id="platform-eq"></span>`fn eq(&self, other: &Platform) -> bool` — [`Platform`](#platform)

##### `impl PartialOrd for Platform`

- <span id="platform-partial-cmp"></span>`fn partial_cmp(&self, other: &Platform) -> option::Option<cmp::Ordering>` — [`Platform`](#platform)

##### `impl Serialize for Platform`

- <span id="platform-serialize"></span>`fn serialize<S>(&self, s: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for Platform`

##### `impl ToString for Platform`

- <span id="platform-to-string"></span>`fn to_string(&self) -> String`

