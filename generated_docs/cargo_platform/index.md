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

*Defined in [`cargo-platform-0.3.1/src/cfg.rs:29-37`](../../.source_1765633015/cargo-platform-0.3.1/src/cfg.rs#L29-L37)*

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

##### `impl Any for Ident`

- <span id="ident-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Ident`

- <span id="ident-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Ident`

- <span id="ident-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Ident`

- <span id="ident-clone"></span>`fn clone(&self) -> Ident` — [`Ident`](cfg/index.md#ident)

##### `impl CloneToUninit for Ident`

- <span id="ident-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Ident`

- <span id="ident-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Ident`

- <span id="ident-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Ident`

##### `impl<T> From for Ident`

- <span id="ident-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Ident`

- <span id="ident-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<U> Into for Ident`

- <span id="ident-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Ident`

- <span id="ident-ord-cmp"></span>`fn cmp(&self, other: &Ident) -> cmp::Ordering` — [`Ident`](cfg/index.md#ident)

##### `impl PartialEq for Ident`

- <span id="ident-partialeq-eq"></span>`fn eq(&self, other: &str) -> bool`

##### `impl PartialOrd for Ident`

- <span id="ident-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Ident) -> option::Option<cmp::Ordering>` — [`Ident`](cfg/index.md#ident)

##### `impl ToOwned for Ident`

- <span id="ident-toowned-type-owned"></span>`type Owned = T`

- <span id="ident-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="ident-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Ident`

- <span id="ident-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Ident`

- <span id="ident-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ident-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Ident`

- <span id="ident-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ident-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ParseError`

```rust
struct ParseError {
    kind: ParseErrorKind,
    orig: String,
}
```

*Defined in [`cargo-platform-0.3.1/src/error.rs:4-7`](../../.source_1765633015/cargo-platform-0.3.1/src/error.rs#L4-L7)*

#### Implementations

- <span id="parseerror-new"></span>`fn new(orig: &str, kind: ParseErrorKind) -> ParseError` — [`ParseErrorKind`](error/index.md#parseerrorkind), [`ParseError`](error/index.md#parseerror)

#### Trait Implementations

##### `impl Any for ParseError`

- <span id="parseerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ParseError`

- <span id="parseerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ParseError`

- <span id="parseerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ParseError`

- <span id="parseerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ParseError`

- <span id="parseerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for ParseError`

##### `impl<T> From for ParseError`

- <span id="parseerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ParseError`

- <span id="parseerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for ParseError`

- <span id="parseerror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for ParseError`

- <span id="parseerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parseerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ParseError`

- <span id="parseerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parseerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Cfg`

```rust
enum Cfg {
    Name(Ident),
    KeyPair(Ident, String),
}
```

*Defined in [`cargo-platform-0.3.1/src/cfg.rs:20-25`](../../.source_1765633015/cargo-platform-0.3.1/src/cfg.rs#L20-L25)*

A cfg value.

#### Variants

- **`Name`**

  A named cfg value, like `unix`.

- **`KeyPair`**

  A key/value cfg pair, like `target_os = "linux"`.

#### Trait Implementations

##### `impl Any for Cfg`

- <span id="cfg-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Cfg`

- <span id="cfg-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Cfg`

- <span id="cfg-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Cfg`

- <span id="cfg-clone"></span>`fn clone(&self) -> Cfg` — [`Cfg`](cfg/index.md#cfg)

##### `impl CloneToUninit for Cfg`

- <span id="cfg-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Cfg`

- <span id="cfg-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Cfg`

- <span id="cfg-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Cfg`

##### `impl<T> From for Cfg`

- <span id="cfg-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromStr for Cfg`

- <span id="cfg-fromstr-type-err"></span>`type Err = ParseError`

- <span id="cfg-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Cfg, <Self as >::Err>` — [`Cfg`](cfg/index.md#cfg)

##### `impl Hash for Cfg`

- <span id="cfg-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Cfg`

- <span id="cfg-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Cfg`

- <span id="cfg-ord-cmp"></span>`fn cmp(&self, other: &Cfg) -> cmp::Ordering` — [`Cfg`](cfg/index.md#cfg)

##### `impl PartialEq for Cfg`

- <span id="cfg-partialeq-eq"></span>`fn eq(&self, other: &Cfg) -> bool` — [`Cfg`](cfg/index.md#cfg)

##### `impl PartialOrd for Cfg`

- <span id="cfg-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Cfg) -> option::Option<cmp::Ordering>` — [`Cfg`](cfg/index.md#cfg)

##### `impl StructuralPartialEq for Cfg`

##### `impl ToOwned for Cfg`

- <span id="cfg-toowned-type-owned"></span>`type Owned = T`

- <span id="cfg-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="cfg-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Cfg`

- <span id="cfg-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Cfg`

- <span id="cfg-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cfg-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Cfg`

- <span id="cfg-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cfg-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`cargo-platform-0.3.1/src/cfg.rs:9-16`](../../.source_1765633015/cargo-platform-0.3.1/src/cfg.rs#L9-L16)*

A cfg expression.

#### Implementations

- <span id="cfgexpr-matches-key"></span>`fn matches_key(key: &str, target_cfg: &[Cfg]) -> bool` — [`Cfg`](cfg/index.md#cfg)

  Utility function to check if the key, "cfg(..)" matches the `target_cfg`

- <span id="cfgexpr-matches"></span>`fn matches(&self, cfg: &[Cfg]) -> bool` — [`Cfg`](cfg/index.md#cfg)

#### Trait Implementations

##### `impl Any for CfgExpr`

- <span id="cfgexpr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CfgExpr`

- <span id="cfgexpr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CfgExpr`

- <span id="cfgexpr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CfgExpr`

- <span id="cfgexpr-clone"></span>`fn clone(&self) -> CfgExpr` — [`CfgExpr`](cfg/index.md#cfgexpr)

##### `impl CloneToUninit for CfgExpr`

- <span id="cfgexpr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for CfgExpr`

- <span id="cfgexpr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for CfgExpr`

- <span id="cfgexpr-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CfgExpr`

##### `impl<T> From for CfgExpr`

- <span id="cfgexpr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromStr for CfgExpr`

- <span id="cfgexpr-fromstr-type-err"></span>`type Err = ParseError`

- <span id="cfgexpr-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<CfgExpr, <Self as >::Err>` — [`CfgExpr`](cfg/index.md#cfgexpr)

##### `impl Hash for CfgExpr`

- <span id="cfgexpr-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for CfgExpr`

- <span id="cfgexpr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for CfgExpr`

- <span id="cfgexpr-ord-cmp"></span>`fn cmp(&self, other: &CfgExpr) -> cmp::Ordering` — [`CfgExpr`](cfg/index.md#cfgexpr)

##### `impl PartialEq for CfgExpr`

- <span id="cfgexpr-partialeq-eq"></span>`fn eq(&self, other: &CfgExpr) -> bool` — [`CfgExpr`](cfg/index.md#cfgexpr)

##### `impl PartialOrd for CfgExpr`

- <span id="cfgexpr-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &CfgExpr) -> option::Option<cmp::Ordering>` — [`CfgExpr`](cfg/index.md#cfgexpr)

##### `impl StructuralPartialEq for CfgExpr`

##### `impl ToOwned for CfgExpr`

- <span id="cfgexpr-toowned-type-owned"></span>`type Owned = T`

- <span id="cfgexpr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="cfgexpr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for CfgExpr`

- <span id="cfgexpr-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for CfgExpr`

- <span id="cfgexpr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cfgexpr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CfgExpr`

- <span id="cfgexpr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cfgexpr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`cargo-platform-0.3.1/src/error.rs:11-21`](../../.source_1765633015/cargo-platform-0.3.1/src/error.rs#L11-L21)*

#### Trait Implementations

##### `impl Any for ParseErrorKind`

- <span id="parseerrorkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ParseErrorKind`

- <span id="parseerrorkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ParseErrorKind`

- <span id="parseerrorkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ParseErrorKind`

- <span id="parseerrorkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ParseErrorKind`

- <span id="parseerrorkind-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ParseErrorKind`

- <span id="parseerrorkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ParseErrorKind`

- <span id="parseerrorkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for ParseErrorKind`

- <span id="parseerrorkind-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for ParseErrorKind`

- <span id="parseerrorkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parseerrorkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ParseErrorKind`

- <span id="parseerrorkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parseerrorkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Platform`

```rust
enum Platform {
    Name(String),
    Cfg(CfgExpr),
}
```

*Defined in [`cargo-platform-0.3.1/src/lib.rs:26-31`](../../.source_1765633015/cargo-platform-0.3.1/src/lib.rs#L26-L31)*

Platform definition.

#### Variants

- **`Name`**

  A named platform, like `x86_64-apple-darwin`.

- **`Cfg`**

  A cfg expression, like `cfg(windows)`.

#### Implementations

- <span id="platform-matches"></span>`fn matches(&self, name: &str, cfg: &[Cfg]) -> bool` — [`Cfg`](cfg/index.md#cfg)

  Returns whether the Platform matches the given target and cfg.

  

  The named target and cfg values should be obtained from `rustc`.

- <span id="platform-validate-named-platform"></span>`fn validate_named_platform(name: &str) -> Result<(), ParseError>` — [`ParseError`](error/index.md#parseerror)

- <span id="platform-check-cfg-attributes"></span>`fn check_cfg_attributes(&self, warnings: &mut Vec<String>)`

- <span id="platform-check-cfg-keywords"></span>`fn check_cfg_keywords(&self, warnings: &mut Vec<String>, path: &Path)`

#### Trait Implementations

##### `impl Any for Platform`

- <span id="platform-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Platform`

- <span id="platform-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Platform`

- <span id="platform-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Platform`

- <span id="platform-clone"></span>`fn clone(&self) -> Platform` — [`Platform`](#platform)

##### `impl CloneToUninit for Platform`

- <span id="platform-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Platform`

- <span id="platform-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Platform`

- <span id="platform-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl DeserializeOwned for Platform`

##### `impl Display for Platform`

- <span id="platform-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Platform`

##### `impl<T> From for Platform`

- <span id="platform-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromStr for Platform`

- <span id="platform-fromstr-type-err"></span>`type Err = ParseError`

- <span id="platform-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Platform, ParseError>` — [`Platform`](#platform), [`ParseError`](error/index.md#parseerror)

##### `impl Hash for Platform`

- <span id="platform-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Platform`

- <span id="platform-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Platform`

- <span id="platform-ord-cmp"></span>`fn cmp(&self, other: &Platform) -> cmp::Ordering` — [`Platform`](#platform)

##### `impl PartialEq for Platform`

- <span id="platform-partialeq-eq"></span>`fn eq(&self, other: &Platform) -> bool` — [`Platform`](#platform)

##### `impl PartialOrd for Platform`

- <span id="platform-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Platform) -> option::Option<cmp::Ordering>` — [`Platform`](#platform)

##### `impl Serialize for Platform`

- <span id="platform-serialize"></span>`fn serialize<S>(&self, s: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for Platform`

##### `impl ToOwned for Platform`

- <span id="platform-toowned-type-owned"></span>`type Owned = T`

- <span id="platform-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="platform-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Platform`

- <span id="platform-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Platform`

- <span id="platform-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="platform-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Platform`

- <span id="platform-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="platform-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

