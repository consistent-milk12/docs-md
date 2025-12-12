*[cargo_platform](../index.md) / [cfg](index.md)*

---

# Module `cfg`

## Contents

- [Structs](#structs)
  - [`Ident`](#ident)
  - [`Tokenizer`](#tokenizer)
  - [`Parser`](#parser)
  - [`CommaSep`](#commasep)
- [Enums](#enums)
  - [`CfgExpr`](#cfgexpr)
  - [`Cfg`](#cfg)
  - [`Token`](#token)
- [Functions](#functions)
  - [`is_ident_start`](#is-ident-start)
  - [`is_ident_rest`](#is-ident-rest)
- [Constants](#constants)
  - [`KEYWORDS`](#keywords)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Ident`](#ident) | struct | A identifier |
| [`Tokenizer`](#tokenizer) | struct |  |
| [`Parser`](#parser) | struct |  |
| [`CommaSep`](#commasep) | struct |  |
| [`CfgExpr`](#cfgexpr) | enum | A cfg expression. |
| [`Cfg`](#cfg) | enum | A cfg value. |
| [`Token`](#token) | enum |  |
| [`is_ident_start`](#is-ident-start) | fn |  |
| [`is_ident_rest`](#is-ident-rest) | fn |  |
| [`KEYWORDS`](#keywords) | const | The list of keywords. |

## Structs

### `Ident`

```rust
struct Ident {
    pub name: String,
    pub raw: bool,
}
```

*Defined in [`cargo-platform-0.3.1/src/cfg.rs:29-37`](../../../.source_1765521767/cargo-platform-0.3.1/src/cfg.rs#L29-L37)*

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

- <span id="ident-clone"></span>`fn clone(&self) -> Ident` — [`Ident`](#ident)

##### `impl Debug for Ident`

- <span id="ident-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Ident`

- <span id="ident-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Ident`

##### `impl Hash for Ident`

- <span id="ident-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl Ord for Ident`

- <span id="ident-cmp"></span>`fn cmp(&self, other: &Ident) -> cmp::Ordering` — [`Ident`](#ident)

##### `impl PartialEq for Ident`

- <span id="ident-eq"></span>`fn eq(&self, other: &str) -> bool`

##### `impl PartialOrd for Ident`

- <span id="ident-partial-cmp"></span>`fn partial_cmp(&self, other: &Ident) -> option::Option<cmp::Ordering>` — [`Ident`](#ident)

##### `impl ToString for Ident`

- <span id="ident-to-string"></span>`fn to_string(&self) -> String`

### `Tokenizer<'a>`

```rust
struct Tokenizer<'a> {
    s: iter::Peekable<str::CharIndices<'a>>,
    orig: &'a str,
}
```

*Defined in [`cargo-platform-0.3.1/src/cfg.rs:58-61`](../../../.source_1765521767/cargo-platform-0.3.1/src/cfg.rs#L58-L61)*

#### Trait Implementations

##### `impl Clone for Tokenizer<'a>`

- <span id="tokenizer-clone"></span>`fn clone(&self) -> Tokenizer<'a>` — [`Tokenizer`](#tokenizer)

##### `impl IntoIterator for Tokenizer<'a>`

- <span id="tokenizer-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tokenizer-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tokenizer-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Tokenizer<'a>`

- <span id="tokenizer-iterator-type-item"></span>`type Item = Result<Token<'a>, ParseError>`

- <span id="tokenizer-next"></span>`fn next(&mut self) -> Option<Result<Token<'a>, ParseError>>` — [`Token`](#token), [`ParseError`](../error/index.md#parseerror)

### `Parser<'a>`

```rust
struct Parser<'a> {
    t: Tokenizer<'a>,
}
```

*Defined in [`cargo-platform-0.3.1/src/cfg.rs:63-65`](../../../.source_1765521767/cargo-platform-0.3.1/src/cfg.rs#L63-L65)*

#### Implementations

- <span id="parser-new"></span>`fn new(s: &'a str) -> Parser<'a>` — [`Parser`](#parser)

- <span id="parser-expr"></span>`fn expr(&mut self) -> Result<CfgExpr, ParseError>` — [`CfgExpr`](#cfgexpr), [`ParseError`](../error/index.md#parseerror)

- <span id="parser-cfg"></span>`fn cfg(&mut self) -> Result<Cfg, ParseError>` — [`Cfg`](#cfg), [`ParseError`](../error/index.md#parseerror)

- <span id="parser-peek"></span>`fn peek(&mut self) -> Option<Result<Token<'a>, ParseError>>` — [`Token`](#token), [`ParseError`](../error/index.md#parseerror)

- <span id="parser-try"></span>`fn try(&mut self, token: &Token<'a>) -> bool` — [`Token`](#token)

- <span id="parser-eat"></span>`fn eat(&mut self, token: &Token<'a>) -> Result<(), ParseError>` — [`Token`](#token), [`ParseError`](../error/index.md#parseerror)

- <span id="parser-rest"></span>`fn rest(&self) -> Option<&str>`

### `CommaSep<'a, T>`

```rust
struct CommaSep<'a, T>(&'a [T]);
```

*Defined in [`cargo-platform-0.3.1/src/cfg.rs:187`](../../../.source_1765521767/cargo-platform-0.3.1/src/cfg.rs#L187)*

#### Trait Implementations

##### `impl<T: fmt::Display> Display for CommaSep<'a, T>`

- <span id="commasep-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for CommaSep<'a, T>`

- <span id="commasep-to-string"></span>`fn to_string(&self) -> String`

## Enums

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

*Defined in [`cargo-platform-0.3.1/src/cfg.rs:9-16`](../../../.source_1765521767/cargo-platform-0.3.1/src/cfg.rs#L9-L16)*

A cfg expression.

#### Implementations

- <span id="cfgexpr-matches-key"></span>`fn matches_key(key: &str, target_cfg: &[Cfg]) -> bool` — [`Cfg`](#cfg)

- <span id="cfgexpr-matches"></span>`fn matches(&self, cfg: &[Cfg]) -> bool` — [`Cfg`](#cfg)

#### Trait Implementations

##### `impl Clone for CfgExpr`

- <span id="cfgexpr-clone"></span>`fn clone(&self) -> CfgExpr` — [`CfgExpr`](#cfgexpr)

##### `impl Debug for CfgExpr`

- <span id="cfgexpr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for CfgExpr`

- <span id="cfgexpr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CfgExpr`

##### `impl FromStr for CfgExpr`

- <span id="cfgexpr-fromstr-type-err"></span>`type Err = ParseError`

- <span id="cfgexpr-from-str"></span>`fn from_str(s: &str) -> Result<CfgExpr, <Self as >::Err>` — [`CfgExpr`](#cfgexpr)

##### `impl Hash for CfgExpr`

- <span id="cfgexpr-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for CfgExpr`

- <span id="cfgexpr-cmp"></span>`fn cmp(&self, other: &CfgExpr) -> cmp::Ordering` — [`CfgExpr`](#cfgexpr)

##### `impl PartialEq for CfgExpr`

- <span id="cfgexpr-eq"></span>`fn eq(&self, other: &CfgExpr) -> bool` — [`CfgExpr`](#cfgexpr)

##### `impl PartialOrd for CfgExpr`

- <span id="cfgexpr-partial-cmp"></span>`fn partial_cmp(&self, other: &CfgExpr) -> option::Option<cmp::Ordering>` — [`CfgExpr`](#cfgexpr)

##### `impl StructuralPartialEq for CfgExpr`

##### `impl ToString for CfgExpr`

- <span id="cfgexpr-to-string"></span>`fn to_string(&self) -> String`

### `Cfg`

```rust
enum Cfg {
    Name(Ident),
    KeyPair(Ident, String),
}
```

*Defined in [`cargo-platform-0.3.1/src/cfg.rs:20-25`](../../../.source_1765521767/cargo-platform-0.3.1/src/cfg.rs#L20-L25)*

A cfg value.

#### Variants

- **`Name`**

  A named cfg value, like `unix`.

- **`KeyPair`**

  A key/value cfg pair, like `target_os = "linux"`.

#### Trait Implementations

##### `impl Clone for Cfg`

- <span id="cfg-clone"></span>`fn clone(&self) -> Cfg` — [`Cfg`](#cfg)

##### `impl Debug for Cfg`

- <span id="cfg-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Cfg`

- <span id="cfg-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Cfg`

##### `impl FromStr for Cfg`

- <span id="cfg-fromstr-type-err"></span>`type Err = ParseError`

- <span id="cfg-from-str"></span>`fn from_str(s: &str) -> Result<Cfg, <Self as >::Err>` — [`Cfg`](#cfg)

##### `impl Hash for Cfg`

- <span id="cfg-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Cfg`

- <span id="cfg-cmp"></span>`fn cmp(&self, other: &Cfg) -> cmp::Ordering` — [`Cfg`](#cfg)

##### `impl PartialEq for Cfg`

- <span id="cfg-eq"></span>`fn eq(&self, other: &Cfg) -> bool` — [`Cfg`](#cfg)

##### `impl PartialOrd for Cfg`

- <span id="cfg-partial-cmp"></span>`fn partial_cmp(&self, other: &Cfg) -> option::Option<cmp::Ordering>` — [`Cfg`](#cfg)

##### `impl StructuralPartialEq for Cfg`

##### `impl ToString for Cfg`

- <span id="cfg-to-string"></span>`fn to_string(&self) -> String`

### `Token<'a>`

```rust
enum Token<'a> {
    LeftParen,
    RightParen,
    Ident(bool, &'a str),
    Comma,
    Equals,
    String(&'a str),
}
```

*Defined in [`cargo-platform-0.3.1/src/cfg.rs:40-47`](../../../.source_1765521767/cargo-platform-0.3.1/src/cfg.rs#L40-L47)*

#### Implementations

- <span id="token-classify"></span>`fn classify(&self) -> &'static str`

#### Trait Implementations

##### `impl PartialEq for Token<'a>`

- <span id="token-eq"></span>`fn eq(&self, other: &Token<'a>) -> bool` — [`Token`](#token)

##### `impl StructuralPartialEq for Token<'a>`

## Functions

### `is_ident_start`

```rust
fn is_ident_start(ch: char) -> bool
```

*Defined in [`cargo-platform-0.3.1/src/cfg.rs:410-412`](../../../.source_1765521767/cargo-platform-0.3.1/src/cfg.rs#L410-L412)*

### `is_ident_rest`

```rust
fn is_ident_rest(ch: char) -> bool
```

*Defined in [`cargo-platform-0.3.1/src/cfg.rs:414-416`](../../../.source_1765521767/cargo-platform-0.3.1/src/cfg.rs#L414-L416)*

## Constants

### `KEYWORDS`
```rust
const KEYWORDS: &[&str; 2];
```

*Defined in [`cargo-platform-0.3.1/src/cfg.rs:55`](../../../.source_1765521767/cargo-platform-0.3.1/src/cfg.rs#L55)*

The list of keywords.

We should consider all the keywords, but some are conditional on
the edition so for now we just consider true/false.

<https://doc.rust-lang.org/reference/keywords.html>

