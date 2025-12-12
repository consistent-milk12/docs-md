*[regex_syntax](../../index.md) / [unicode_tables](../index.md) / [sentence_break](index.md)*

---

# Module `sentence_break`

## Contents

- [Constants](#constants)
  - [`BY_NAME`](#by-name)
  - [`ATERM`](#aterm)
  - [`CR`](#cr)
  - [`CLOSE`](#close)
  - [`EXTEND`](#extend)
  - [`FORMAT`](#format)
  - [`LF`](#lf)
  - [`LOWER`](#lower)
  - [`NUMERIC`](#numeric)
  - [`OLETTER`](#oletter)
  - [`SCONTINUE`](#scontinue)
  - [`STERM`](#sterm)
  - [`SEP`](#sep)
  - [`SP`](#sp)
  - [`UPPER`](#upper)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BY_NAME`](#by-name) | const |  |
| [`ATERM`](#aterm) | const |  |
| [`CR`](#cr) | const |  |
| [`CLOSE`](#close) | const |  |
| [`EXTEND`](#extend) | const |  |
| [`FORMAT`](#format) | const |  |
| [`LF`](#lf) | const |  |
| [`LOWER`](#lower) | const |  |
| [`NUMERIC`](#numeric) | const |  |
| [`OLETTER`](#oletter) | const |  |
| [`SCONTINUE`](#scontinue) | const |  |
| [`STERM`](#sterm) | const |  |
| [`SEP`](#sep) | const |  |
| [`SP`](#sp) | const |  |
| [`UPPER`](#upper) | const |  |

## Constants

### `BY_NAME`
```rust
const BY_NAME: &'static [(&'static str, &'static [(char, char)])];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs:9-24`](../../../../.source_1765521767/regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs#L9-L24)*

### `ATERM`
```rust
const ATERM: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs:26-27`](../../../../.source_1765521767/regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs#L26-L27)*

### `CR`
```rust
const CR: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs:29`](../../../../.source_1765521767/regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs#L29)*

### `CLOSE`
```rust
const CLOSE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs:31-77`](../../../../.source_1765521767/regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs#L31-L77)*

### `EXTEND`
```rust
const EXTEND: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs:79-404`](../../../../.source_1765521767/regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs#L79-L404)*

### `FORMAT`
```rust
const FORMAT: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs:406-422`](../../../../.source_1765521767/regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs#L406-L422)*

### `LF`
```rust
const LF: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs:424`](../../../../.source_1765521767/regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs#L424)*

### `LOWER`
```rust
const LOWER: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs:426-1100`](../../../../.source_1765521767/regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs#L426-L1100)*

### `NUMERIC`
```rust
const NUMERIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs:1102-1181`](../../../../.source_1765521767/regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs#L1102-L1181)*

### `OLETTER`
```rust
const OLETTER: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs:1183-1745`](../../../../.source_1765521767/regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs#L1183-L1745)*

### `SCONTINUE`
```rust
const SCONTINUE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs:1747-1768`](../../../../.source_1765521767/regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs#L1747-L1768)*

### `STERM`
```rust
const STERM: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs:1770-1855`](../../../../.source_1765521767/regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs#L1770-L1855)*

### `SEP`
```rust
const SEP: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs:1857-1858`](../../../../.source_1765521767/regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs#L1857-L1858)*

### `SP`
```rust
const SP: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs:1860-1870`](../../../../.source_1765521767/regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs#L1860-L1870)*

### `UPPER`
```rust
const UPPER: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs:1872-2530`](../../../../.source_1765521767/regex-syntax-0.8.8/src/unicode_tables/sentence_break.rs#L1872-L2530)*

