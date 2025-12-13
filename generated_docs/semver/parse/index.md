*[semver](../index.md) / [parse](index.md)*

---

# Module `parse`

## Contents

- [Structs](#structs)
  - [`Error`](#error)
- [Functions](#functions)
  - [`numeric_identifier`](#numeric-identifier)
  - [`wildcard`](#wildcard)
  - [`dot`](#dot)
  - [`prerelease_identifier`](#prerelease-identifier)
  - [`build_identifier`](#build-identifier)
  - [`identifier`](#identifier)
  - [`op`](#op)
  - [`comparator`](#comparator)
  - [`version_req`](#version-req)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Error`](#error) | struct | Error parsing a SemVer version or version requirement. |
| [`numeric_identifier`](#numeric-identifier) | fn |  |
| [`wildcard`](#wildcard) | fn |  |
| [`dot`](#dot) | fn |  |
| [`prerelease_identifier`](#prerelease-identifier) | fn |  |
| [`build_identifier`](#build-identifier) | fn |  |
| [`identifier`](#identifier) | fn |  |
| [`op`](#op) | fn |  |
| [`comparator`](#comparator) | fn |  |
| [`version_req`](#version-req) | fn |  |

## Structs

### `Error`

```rust
struct Error {
    kind: crate::error::ErrorKind,
}
```

*Defined in [`semver-1.0.27/src/parse.rs:21-23`](../../../.source_1765633015/semver-1.0.27/src/parse.rs#L21-L23)*

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

- <span id="error-new"></span>`fn new(kind: ErrorKind) -> Self` — [`ErrorKind`](../error/index.md#errorkind)

#### Trait Implementations

##### `impl Any for Error`

- <span id="error-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Error`

- <span id="error-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Error`

- <span id="error-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for crate::parse::Error`

- <span id="crateparseerror-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for crate::parse::Error`

- <span id="crateparseerror-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for crate::parse::Error`

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

## Functions

### `numeric_identifier`

```rust
fn numeric_identifier(input: &str, pos: crate::error::Position) -> Result<(u64, &str), Error>
```

*Defined in [`semver-1.0.27/src/parse.rs:156-184`](../../../.source_1765633015/semver-1.0.27/src/parse.rs#L156-L184)*

### `wildcard`

```rust
fn wildcard(input: &str) -> Option<(char, &str)>
```

*Defined in [`semver-1.0.27/src/parse.rs:186-196`](../../../.source_1765633015/semver-1.0.27/src/parse.rs#L186-L196)*

### `dot`

```rust
fn dot(input: &str, pos: crate::error::Position) -> Result<&str, Error>
```

*Defined in [`semver-1.0.27/src/parse.rs:198-206`](../../../.source_1765633015/semver-1.0.27/src/parse.rs#L198-L206)*

### `prerelease_identifier`

```rust
fn prerelease_identifier(input: &str) -> Result<(crate::Prerelease, &str), Error>
```

*Defined in [`semver-1.0.27/src/parse.rs:208-212`](../../../.source_1765633015/semver-1.0.27/src/parse.rs#L208-L212)*

### `build_identifier`

```rust
fn build_identifier(input: &str) -> Result<(crate::BuildMetadata, &str), Error>
```

*Defined in [`semver-1.0.27/src/parse.rs:214-218`](../../../.source_1765633015/semver-1.0.27/src/parse.rs#L214-L218)*

### `identifier`

```rust
fn identifier(input: &str, pos: crate::error::Position) -> Result<(&str, &str), Error>
```

*Defined in [`semver-1.0.27/src/parse.rs:220-260`](../../../.source_1765633015/semver-1.0.27/src/parse.rs#L220-L260)*

### `op`

```rust
fn op(input: &str) -> (crate::Op, &str)
```

*Defined in [`semver-1.0.27/src/parse.rs:262-285`](../../../.source_1765633015/semver-1.0.27/src/parse.rs#L262-L285)*

### `comparator`

```rust
fn comparator(input: &str) -> Result<(crate::Comparator, crate::error::Position, &str), Error>
```

*Defined in [`semver-1.0.27/src/parse.rs:287-364`](../../../.source_1765633015/semver-1.0.27/src/parse.rs#L287-L364)*

### `version_req`

```rust
fn version_req(input: &str, out: &mut alloc::vec::Vec<crate::Comparator>, depth: usize) -> Result<usize, Error>
```

*Defined in [`semver-1.0.27/src/parse.rs:366-404`](../../../.source_1765633015/semver-1.0.27/src/parse.rs#L366-L404)*

