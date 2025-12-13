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

*Defined in [`cargo-platform-0.3.1/src/cfg.rs:29-37`](../../../.source_1765633015/cargo-platform-0.3.1/src/cfg.rs#L29-L37)*

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

- <span id="ident-clone"></span>`fn clone(&self) -> Ident` — [`Ident`](#ident)

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

- <span id="ident-ord-cmp"></span>`fn cmp(&self, other: &Ident) -> cmp::Ordering` — [`Ident`](#ident)

##### `impl PartialEq for Ident`

- <span id="ident-partialeq-eq"></span>`fn eq(&self, other: &str) -> bool`

##### `impl PartialOrd for Ident`

- <span id="ident-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Ident) -> option::Option<cmp::Ordering>` — [`Ident`](#ident)

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

### `Tokenizer<'a>`

```rust
struct Tokenizer<'a> {
    s: iter::Peekable<str::CharIndices<'a>>,
    orig: &'a str,
}
```

*Defined in [`cargo-platform-0.3.1/src/cfg.rs:58-61`](../../../.source_1765633015/cargo-platform-0.3.1/src/cfg.rs#L58-L61)*

#### Trait Implementations

##### `impl Any for Tokenizer<'a>`

- <span id="tokenizer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Tokenizer<'a>`

- <span id="tokenizer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Tokenizer<'a>`

- <span id="tokenizer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Tokenizer<'a>`

- <span id="tokenizer-clone"></span>`fn clone(&self) -> Tokenizer<'a>` — [`Tokenizer`](#tokenizer)

##### `impl CloneToUninit for Tokenizer<'a>`

- <span id="tokenizer-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for Tokenizer<'a>`

- <span id="tokenizer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Tokenizer<'a>`

- <span id="tokenizer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Tokenizer<'a>`

- <span id="tokenizer-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tokenizer-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tokenizer-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Tokenizer<'a>`

- <span id="tokenizer-iterator-type-item"></span>`type Item = Result<Token<'a>, ParseError>`

- <span id="tokenizer-iterator-next"></span>`fn next(&mut self) -> Option<Result<Token<'a>, ParseError>>` — [`Token`](#token), [`ParseError`](../error/index.md#parseerror)

##### `impl ToOwned for Tokenizer<'a>`

- <span id="tokenizer-toowned-type-owned"></span>`type Owned = T`

- <span id="tokenizer-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tokenizer-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Tokenizer<'a>`

- <span id="tokenizer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tokenizer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Tokenizer<'a>`

- <span id="tokenizer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tokenizer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Parser<'a>`

```rust
struct Parser<'a> {
    t: Tokenizer<'a>,
}
```

*Defined in [`cargo-platform-0.3.1/src/cfg.rs:63-65`](../../../.source_1765633015/cargo-platform-0.3.1/src/cfg.rs#L63-L65)*

#### Implementations

- <span id="parser-new"></span>`fn new(s: &'a str) -> Parser<'a>` — [`Parser`](#parser)

- <span id="parser-expr"></span>`fn expr(&mut self) -> Result<CfgExpr, ParseError>` — [`CfgExpr`](#cfgexpr), [`ParseError`](../error/index.md#parseerror)

- <span id="parser-cfg"></span>`fn cfg(&mut self) -> Result<Cfg, ParseError>` — [`Cfg`](#cfg), [`ParseError`](../error/index.md#parseerror)

- <span id="parser-peek"></span>`fn peek(&mut self) -> Option<Result<Token<'a>, ParseError>>` — [`Token`](#token), [`ParseError`](../error/index.md#parseerror)

- <span id="parser-try"></span>`fn try(&mut self, token: &Token<'a>) -> bool` — [`Token`](#token)

- <span id="parser-eat"></span>`fn eat(&mut self, token: &Token<'a>) -> Result<(), ParseError>` — [`Token`](#token), [`ParseError`](../error/index.md#parseerror)

- <span id="parser-rest"></span>`fn rest(&self) -> Option<&str>`

  Returns the rest of the input from the current location.

#### Trait Implementations

##### `impl Any for Parser<'a>`

- <span id="parser-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Parser<'a>`

- <span id="parser-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Parser<'a>`

- <span id="parser-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Parser<'a>`

- <span id="parser-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Parser<'a>`

- <span id="parser-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Parser<'a>`

- <span id="parser-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parser-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Parser<'a>`

- <span id="parser-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parser-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CommaSep<'a, T>`

```rust
struct CommaSep<'a, T>(&'a [T]);
```

*Defined in [`cargo-platform-0.3.1/src/cfg.rs:187`](../../../.source_1765633015/cargo-platform-0.3.1/src/cfg.rs#L187)*

#### Trait Implementations

##### `impl<T> Any for CommaSep<'a, T>`

- <span id="commasep-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CommaSep<'a, T>`

- <span id="commasep-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CommaSep<'a, T>`

- <span id="commasep-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Display> Display for CommaSep<'a, T>`

- <span id="commasep-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CommaSep<'a, T>`

- <span id="commasep-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for CommaSep<'a, T>`

- <span id="commasep-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> ToString for CommaSep<'a, T>`

- <span id="commasep-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T, U> TryFrom for CommaSep<'a, T>`

- <span id="commasep-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="commasep-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for CommaSep<'a, T>`

- <span id="commasep-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="commasep-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`cargo-platform-0.3.1/src/cfg.rs:9-16`](../../../.source_1765633015/cargo-platform-0.3.1/src/cfg.rs#L9-L16)*

A cfg expression.

#### Implementations

- <span id="cfgexpr-matches-key"></span>`fn matches_key(key: &str, target_cfg: &[Cfg]) -> bool` — [`Cfg`](#cfg)

  Utility function to check if the key, "cfg(..)" matches the `target_cfg`

- <span id="cfgexpr-matches"></span>`fn matches(&self, cfg: &[Cfg]) -> bool` — [`Cfg`](#cfg)

#### Trait Implementations

##### `impl Any for CfgExpr`

- <span id="cfgexpr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CfgExpr`

- <span id="cfgexpr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CfgExpr`

- <span id="cfgexpr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CfgExpr`

- <span id="cfgexpr-clone"></span>`fn clone(&self) -> CfgExpr` — [`CfgExpr`](#cfgexpr)

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

- <span id="cfgexpr-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<CfgExpr, <Self as >::Err>` — [`CfgExpr`](#cfgexpr)

##### `impl Hash for CfgExpr`

- <span id="cfgexpr-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for CfgExpr`

- <span id="cfgexpr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for CfgExpr`

- <span id="cfgexpr-ord-cmp"></span>`fn cmp(&self, other: &CfgExpr) -> cmp::Ordering` — [`CfgExpr`](#cfgexpr)

##### `impl PartialEq for CfgExpr`

- <span id="cfgexpr-partialeq-eq"></span>`fn eq(&self, other: &CfgExpr) -> bool` — [`CfgExpr`](#cfgexpr)

##### `impl PartialOrd for CfgExpr`

- <span id="cfgexpr-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &CfgExpr) -> option::Option<cmp::Ordering>` — [`CfgExpr`](#cfgexpr)

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

### `Cfg`

```rust
enum Cfg {
    Name(Ident),
    KeyPair(Ident, String),
}
```

*Defined in [`cargo-platform-0.3.1/src/cfg.rs:20-25`](../../../.source_1765633015/cargo-platform-0.3.1/src/cfg.rs#L20-L25)*

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

- <span id="cfg-clone"></span>`fn clone(&self) -> Cfg` — [`Cfg`](#cfg)

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

- <span id="cfg-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Cfg, <Self as >::Err>` — [`Cfg`](#cfg)

##### `impl Hash for Cfg`

- <span id="cfg-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Cfg`

- <span id="cfg-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Cfg`

- <span id="cfg-ord-cmp"></span>`fn cmp(&self, other: &Cfg) -> cmp::Ordering` — [`Cfg`](#cfg)

##### `impl PartialEq for Cfg`

- <span id="cfg-partialeq-eq"></span>`fn eq(&self, other: &Cfg) -> bool` — [`Cfg`](#cfg)

##### `impl PartialOrd for Cfg`

- <span id="cfg-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Cfg) -> option::Option<cmp::Ordering>` — [`Cfg`](#cfg)

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

*Defined in [`cargo-platform-0.3.1/src/cfg.rs:40-47`](../../../.source_1765633015/cargo-platform-0.3.1/src/cfg.rs#L40-L47)*

#### Implementations

- <span id="token-classify"></span>`fn classify(&self) -> &'static str`

#### Trait Implementations

##### `impl Any for Token<'a>`

- <span id="token-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Token<'a>`

- <span id="token-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Token<'a>`

- <span id="token-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Token<'a>`

- <span id="token-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Token<'a>`

- <span id="token-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Token<'a>`

- <span id="token-partialeq-eq"></span>`fn eq(&self, other: &Token<'a>) -> bool` — [`Token`](#token)

##### `impl StructuralPartialEq for Token<'a>`

##### `impl<U> TryFrom for Token<'a>`

- <span id="token-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="token-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Token<'a>`

- <span id="token-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="token-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `is_ident_start`

```rust
fn is_ident_start(ch: char) -> bool
```

*Defined in [`cargo-platform-0.3.1/src/cfg.rs:410-412`](../../../.source_1765633015/cargo-platform-0.3.1/src/cfg.rs#L410-L412)*

### `is_ident_rest`

```rust
fn is_ident_rest(ch: char) -> bool
```

*Defined in [`cargo-platform-0.3.1/src/cfg.rs:414-416`](../../../.source_1765633015/cargo-platform-0.3.1/src/cfg.rs#L414-L416)*

## Constants

### `KEYWORDS`
```rust
const KEYWORDS: &[&str; 2];
```

*Defined in [`cargo-platform-0.3.1/src/cfg.rs:55`](../../../.source_1765633015/cargo-platform-0.3.1/src/cfg.rs#L55)*

The list of keywords.

We should consider all the keywords, but some are conditional on
the edition so for now we just consider true/false.

<https://doc.rust-lang.org/reference/keywords.html>

