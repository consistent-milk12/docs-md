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
  - [`DENY_UNKNOWN_FIELDS`](#deny_unknown_fields)
  - [`DESERIALIZE`](#deserialize)
  - [`DESERIALIZE_WITH`](#deserialize_with)
  - [`EXPECTING`](#expecting)
  - [`FIELD_IDENTIFIER`](#field_identifier)
  - [`FLATTEN`](#flatten)
  - [`FROM`](#from)
  - [`GETTER`](#getter)
  - [`INTO`](#into)
  - [`NON_EXHAUSTIVE`](#non_exhaustive)
  - [`OTHER`](#other)
  - [`REMOTE`](#remote)
  - [`RENAME`](#rename)
  - [`RENAME_ALL`](#rename_all)
  - [`RENAME_ALL_FIELDS`](#rename_all_fields)
  - [`REPR`](#repr)
  - [`SERDE`](#serde)
  - [`SERIALIZE`](#serialize)
  - [`SERIALIZE_WITH`](#serialize_with)
  - [`SKIP`](#skip)
  - [`SKIP_DESERIALIZING`](#skip_deserializing)
  - [`SKIP_SERIALIZING`](#skip_serializing)
  - [`SKIP_SERIALIZING_IF`](#skip_serializing_if)
  - [`TAG`](#tag)
  - [`TRANSPARENT`](#transparent)
  - [`TRY_FROM`](#try_from)
  - [`UNTAGGED`](#untagged)
  - [`VARIANT_IDENTIFIER`](#variant_identifier)
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
| [`DENY_UNKNOWN_FIELDS`](#deny_unknown_fields) | const |  |
| [`DESERIALIZE`](#deserialize) | const |  |
| [`DESERIALIZE_WITH`](#deserialize_with) | const |  |
| [`EXPECTING`](#expecting) | const |  |
| [`FIELD_IDENTIFIER`](#field_identifier) | const |  |
| [`FLATTEN`](#flatten) | const |  |
| [`FROM`](#from) | const |  |
| [`GETTER`](#getter) | const |  |
| [`INTO`](#into) | const |  |
| [`NON_EXHAUSTIVE`](#non_exhaustive) | const |  |
| [`OTHER`](#other) | const |  |
| [`REMOTE`](#remote) | const |  |
| [`RENAME`](#rename) | const |  |
| [`RENAME_ALL`](#rename_all) | const |  |
| [`RENAME_ALL_FIELDS`](#rename_all_fields) | const |  |
| [`REPR`](#repr) | const |  |
| [`SERDE`](#serde) | const |  |
| [`SERIALIZE`](#serialize) | const |  |
| [`SERIALIZE_WITH`](#serialize_with) | const |  |
| [`SKIP`](#skip) | const |  |
| [`SKIP_DESERIALIZING`](#skip_deserializing) | const |  |
| [`SKIP_SERIALIZING`](#skip_serializing) | const |  |
| [`SKIP_SERIALIZING_IF`](#skip_serializing_if) | const |  |
| [`TAG`](#tag) | const |  |
| [`TRANSPARENT`](#transparent) | const |  |
| [`TRY_FROM`](#try_from) | const |  |
| [`UNTAGGED`](#untagged) | const |  |
| [`VARIANT_IDENTIFIER`](#variant_identifier) | const |  |
| [`WITH`](#with) | const |  |

## Structs

### `Symbol`

```rust
struct Symbol(&'static str);
```

#### Trait Implementations

##### `impl Clone for Symbol`

- <span id="symbol-clone"></span>`fn clone(&self) -> Symbol` — [`Symbol`](#symbol)

##### `impl Copy for Symbol`

##### `impl Display for Symbol`

- <span id="symbol-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for Symbol`

- <span id="symbol-to-string"></span>`fn to_string(&self) -> String`

## Constants

### `ALIAS`

```rust
const ALIAS: Symbol;
```

### `BORROW`

```rust
const BORROW: Symbol;
```

### `BOUND`

```rust
const BOUND: Symbol;
```

### `CONTENT`

```rust
const CONTENT: Symbol;
```

### `CRATE`

```rust
const CRATE: Symbol;
```

### `DEFAULT`

```rust
const DEFAULT: Symbol;
```

### `DENY_UNKNOWN_FIELDS`

```rust
const DENY_UNKNOWN_FIELDS: Symbol;
```

### `DESERIALIZE`

```rust
const DESERIALIZE: Symbol;
```

### `DESERIALIZE_WITH`

```rust
const DESERIALIZE_WITH: Symbol;
```

### `EXPECTING`

```rust
const EXPECTING: Symbol;
```

### `FIELD_IDENTIFIER`

```rust
const FIELD_IDENTIFIER: Symbol;
```

### `FLATTEN`

```rust
const FLATTEN: Symbol;
```

### `FROM`

```rust
const FROM: Symbol;
```

### `GETTER`

```rust
const GETTER: Symbol;
```

### `INTO`

```rust
const INTO: Symbol;
```

### `NON_EXHAUSTIVE`

```rust
const NON_EXHAUSTIVE: Symbol;
```

### `OTHER`

```rust
const OTHER: Symbol;
```

### `REMOTE`

```rust
const REMOTE: Symbol;
```

### `RENAME`

```rust
const RENAME: Symbol;
```

### `RENAME_ALL`

```rust
const RENAME_ALL: Symbol;
```

### `RENAME_ALL_FIELDS`

```rust
const RENAME_ALL_FIELDS: Symbol;
```

### `REPR`

```rust
const REPR: Symbol;
```

### `SERDE`

```rust
const SERDE: Symbol;
```

### `SERIALIZE`

```rust
const SERIALIZE: Symbol;
```

### `SERIALIZE_WITH`

```rust
const SERIALIZE_WITH: Symbol;
```

### `SKIP`

```rust
const SKIP: Symbol;
```

### `SKIP_DESERIALIZING`

```rust
const SKIP_DESERIALIZING: Symbol;
```

### `SKIP_SERIALIZING`

```rust
const SKIP_SERIALIZING: Symbol;
```

### `SKIP_SERIALIZING_IF`

```rust
const SKIP_SERIALIZING_IF: Symbol;
```

### `TAG`

```rust
const TAG: Symbol;
```

### `TRANSPARENT`

```rust
const TRANSPARENT: Symbol;
```

### `TRY_FROM`

```rust
const TRY_FROM: Symbol;
```

### `UNTAGGED`

```rust
const UNTAGGED: Symbol;
```

### `VARIANT_IDENTIFIER`

```rust
const VARIANT_IDENTIFIER: Symbol;
```

### `WITH`

```rust
const WITH: Symbol;
```

