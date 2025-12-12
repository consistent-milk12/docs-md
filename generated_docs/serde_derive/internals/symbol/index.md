*[serde_derive](../../index.md) / [internals](../index.md) / [symbol](index.md)*

---

# Module `symbol`

## Contents

- [Structs](#structs)
  - [`Symbol`](#symbol)
- [Constants](#constants)
  - [`ALIAS`](#alias)
  - [`BORROW`](#borrow)
  - [`BOUND`](#bound)
  - [`CONTENT`](#content)
  - [`CRATE`](#crate)
  - [`DEFAULT`](#default)
  - [`DENY_UNKNOWN_FIELDS`](#deny-unknown-fields)
  - [`DESERIALIZE`](#deserialize)
  - [`DESERIALIZE_WITH`](#deserialize-with)
  - [`EXPECTING`](#expecting)
  - [`FIELD_IDENTIFIER`](#field-identifier)
  - [`FLATTEN`](#flatten)
  - [`FROM`](#from)
  - [`GETTER`](#getter)
  - [`INTO`](#into)
  - [`NON_EXHAUSTIVE`](#non-exhaustive)
  - [`OTHER`](#other)
  - [`REMOTE`](#remote)
  - [`RENAME`](#rename)
  - [`RENAME_ALL`](#rename-all)
  - [`RENAME_ALL_FIELDS`](#rename-all-fields)
  - [`REPR`](#repr)
  - [`SERDE`](#serde)
  - [`SERIALIZE`](#serialize)
  - [`SERIALIZE_WITH`](#serialize-with)
  - [`SKIP`](#skip)
  - [`SKIP_DESERIALIZING`](#skip-deserializing)
  - [`SKIP_SERIALIZING`](#skip-serializing)
  - [`SKIP_SERIALIZING_IF`](#skip-serializing-if)
  - [`TAG`](#tag)
  - [`TRANSPARENT`](#transparent)
  - [`TRY_FROM`](#try-from)
  - [`UNTAGGED`](#untagged)
  - [`VARIANT_IDENTIFIER`](#variant-identifier)
  - [`WITH`](#with)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Symbol`](#symbol) | struct |  |
| [`ALIAS`](#alias) | const |  |
| [`BORROW`](#borrow) | const |  |
| [`BOUND`](#bound) | const |  |
| [`CONTENT`](#content) | const |  |
| [`CRATE`](#crate) | const |  |
| [`DEFAULT`](#default) | const |  |
| [`DENY_UNKNOWN_FIELDS`](#deny-unknown-fields) | const |  |
| [`DESERIALIZE`](#deserialize) | const |  |
| [`DESERIALIZE_WITH`](#deserialize-with) | const |  |
| [`EXPECTING`](#expecting) | const |  |
| [`FIELD_IDENTIFIER`](#field-identifier) | const |  |
| [`FLATTEN`](#flatten) | const |  |
| [`FROM`](#from) | const |  |
| [`GETTER`](#getter) | const |  |
| [`INTO`](#into) | const |  |
| [`NON_EXHAUSTIVE`](#non-exhaustive) | const |  |
| [`OTHER`](#other) | const |  |
| [`REMOTE`](#remote) | const |  |
| [`RENAME`](#rename) | const |  |
| [`RENAME_ALL`](#rename-all) | const |  |
| [`RENAME_ALL_FIELDS`](#rename-all-fields) | const |  |
| [`REPR`](#repr) | const |  |
| [`SERDE`](#serde) | const |  |
| [`SERIALIZE`](#serialize) | const |  |
| [`SERIALIZE_WITH`](#serialize-with) | const |  |
| [`SKIP`](#skip) | const |  |
| [`SKIP_DESERIALIZING`](#skip-deserializing) | const |  |
| [`SKIP_SERIALIZING`](#skip-serializing) | const |  |
| [`SKIP_SERIALIZING_IF`](#skip-serializing-if) | const |  |
| [`TAG`](#tag) | const |  |
| [`TRANSPARENT`](#transparent) | const |  |
| [`TRY_FROM`](#try-from) | const |  |
| [`UNTAGGED`](#untagged) | const |  |
| [`VARIANT_IDENTIFIER`](#variant-identifier) | const |  |
| [`WITH`](#with) | const |  |

## Structs

### `Symbol`

```rust
struct Symbol(&'static str);
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:5`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L5)*

#### Trait Implementations

##### `impl Clone for Symbol`

- <span id="symbol-clone"></span>`fn clone(&self) -> Symbol` — [`Symbol`](#symbol)

##### `impl Copy for Symbol`

##### `impl Display for Symbol`

- <span id="symbol-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for syn::Ident`

- <span id="synident-eq"></span>`fn eq(&self, word: &Symbol) -> bool` — [`Symbol`](#symbol)

##### `impl ToString for Symbol`

- <span id="symbol-to-string"></span>`fn to_string(&self) -> String`

## Constants

### `ALIAS`
```rust
const ALIAS: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:7`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L7)*

### `BORROW`
```rust
const BORROW: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:8`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L8)*

### `BOUND`
```rust
const BOUND: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:9`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L9)*

### `CONTENT`
```rust
const CONTENT: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:10`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L10)*

### `CRATE`
```rust
const CRATE: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:11`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L11)*

### `DEFAULT`
```rust
const DEFAULT: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:12`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L12)*

### `DENY_UNKNOWN_FIELDS`
```rust
const DENY_UNKNOWN_FIELDS: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:13`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L13)*

### `DESERIALIZE`
```rust
const DESERIALIZE: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:14`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L14)*

### `DESERIALIZE_WITH`
```rust
const DESERIALIZE_WITH: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:15`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L15)*

### `EXPECTING`
```rust
const EXPECTING: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:16`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L16)*

### `FIELD_IDENTIFIER`
```rust
const FIELD_IDENTIFIER: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:17`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L17)*

### `FLATTEN`
```rust
const FLATTEN: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:18`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L18)*

### `FROM`
```rust
const FROM: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:19`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L19)*

### `GETTER`
```rust
const GETTER: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:20`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L20)*

### `INTO`
```rust
const INTO: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:21`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L21)*

### `NON_EXHAUSTIVE`
```rust
const NON_EXHAUSTIVE: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:22`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L22)*

### `OTHER`
```rust
const OTHER: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:23`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L23)*

### `REMOTE`
```rust
const REMOTE: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:24`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L24)*

### `RENAME`
```rust
const RENAME: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:25`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L25)*

### `RENAME_ALL`
```rust
const RENAME_ALL: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:26`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L26)*

### `RENAME_ALL_FIELDS`
```rust
const RENAME_ALL_FIELDS: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:27`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L27)*

### `REPR`
```rust
const REPR: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:28`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L28)*

### `SERDE`
```rust
const SERDE: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:29`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L29)*

### `SERIALIZE`
```rust
const SERIALIZE: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:30`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L30)*

### `SERIALIZE_WITH`
```rust
const SERIALIZE_WITH: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:31`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L31)*

### `SKIP`
```rust
const SKIP: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:32`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L32)*

### `SKIP_DESERIALIZING`
```rust
const SKIP_DESERIALIZING: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:33`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L33)*

### `SKIP_SERIALIZING`
```rust
const SKIP_SERIALIZING: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:34`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L34)*

### `SKIP_SERIALIZING_IF`
```rust
const SKIP_SERIALIZING_IF: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:35`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L35)*

### `TAG`
```rust
const TAG: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:36`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L36)*

### `TRANSPARENT`
```rust
const TRANSPARENT: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:37`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L37)*

### `TRY_FROM`
```rust
const TRY_FROM: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:38`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L38)*

### `UNTAGGED`
```rust
const UNTAGGED: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:39`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L39)*

### `VARIANT_IDENTIFIER`
```rust
const VARIANT_IDENTIFIER: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:40`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L40)*

### `WITH`
```rust
const WITH: Symbol;
```

*Defined in [`serde_derive-1.0.228/src/internals/symbol.rs:41`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/symbol.rs#L41)*

