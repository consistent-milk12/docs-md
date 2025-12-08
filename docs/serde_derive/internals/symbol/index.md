*[serde_derive](../../index.md) / [internals](../index.md) / [symbol](index.md)*

---

# Module `symbol`

## Structs

### `Symbol`

```rust
struct Symbol(&'static str);
```

#### Trait Implementations

##### `impl Clone for Symbol`

- `fn clone(self: &Self) -> Symbol` — [`Symbol`](#symbol)

##### `impl Copy for Symbol`

##### `impl Display for Symbol`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for Symbol`

- `fn to_string(self: &Self) -> String`

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

