*[regex_syntax](../../index.md) / [unicode_tables](../index.md) / [word_break](index.md)*

---

# Module `word_break`

## Contents

- [Constants](#constants)
  - [`BY_NAME`](#by-name)
  - [`ALETTER`](#aletter)
  - [`CR`](#cr)
  - [`DOUBLE_QUOTE`](#double-quote)
  - [`EXTEND`](#extend)
  - [`EXTENDNUMLET`](#extendnumlet)
  - [`FORMAT`](#format)
  - [`HEBREW_LETTER`](#hebrew-letter)
  - [`KATAKANA`](#katakana)
  - [`LF`](#lf)
  - [`MIDLETTER`](#midletter)
  - [`MIDNUM`](#midnum)
  - [`MIDNUMLET`](#midnumlet)
  - [`NEWLINE`](#newline)
  - [`NUMERIC`](#numeric)
  - [`REGIONAL_INDICATOR`](#regional-indicator)
  - [`SINGLE_QUOTE`](#single-quote)
  - [`WSEGSPACE`](#wsegspace)
  - [`ZWJ`](#zwj)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BY_NAME`](#by-name) | const |  |
| [`ALETTER`](#aletter) | const |  |
| [`CR`](#cr) | const |  |
| [`DOUBLE_QUOTE`](#double-quote) | const |  |
| [`EXTEND`](#extend) | const |  |
| [`EXTENDNUMLET`](#extendnumlet) | const |  |
| [`FORMAT`](#format) | const |  |
| [`HEBREW_LETTER`](#hebrew-letter) | const |  |
| [`KATAKANA`](#katakana) | const |  |
| [`LF`](#lf) | const |  |
| [`MIDLETTER`](#midletter) | const |  |
| [`MIDNUM`](#midnum) | const |  |
| [`MIDNUMLET`](#midnumlet) | const |  |
| [`NEWLINE`](#newline) | const |  |
| [`NUMERIC`](#numeric) | const |  |
| [`REGIONAL_INDICATOR`](#regional-indicator) | const |  |
| [`SINGLE_QUOTE`](#single-quote) | const |  |
| [`WSEGSPACE`](#wsegspace) | const |  |
| [`ZWJ`](#zwj) | const |  |

## Constants

### `BY_NAME`
```rust
const BY_NAME: &'static [(&'static str, &'static [(char, char)])];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/word_break.rs:9-28`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/word_break.rs#L9-L28)*

### `ALETTER`
```rust
const ALETTER: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/word_break.rs:30-626`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/word_break.rs#L30-L626)*

### `CR`
```rust
const CR: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/word_break.rs:628`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/word_break.rs#L628)*

### `DOUBLE_QUOTE`
```rust
const DOUBLE_QUOTE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/word_break.rs:630`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/word_break.rs#L630)*

### `EXTEND`
```rust
const EXTEND: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/word_break.rs:632-958`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/word_break.rs#L632-L958)*

### `EXTENDNUMLET`
```rust
const EXTENDNUMLET: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/word_break.rs:960-968`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/word_break.rs#L960-L968)*

### `FORMAT`
```rust
const FORMAT: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/word_break.rs:970-984`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/word_break.rs#L970-L984)*

### `HEBREW_LETTER`
```rust
const HEBREW_LETTER: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/word_break.rs:986-997`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/word_break.rs#L986-L997)*

### `KATAKANA`
```rust
const KATAKANA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/word_break.rs:999-1015`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/word_break.rs#L999-L1015)*

### `LF`
```rust
const LF: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/word_break.rs:1017`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/word_break.rs#L1017)*

### `MIDLETTER`
```rust
const MIDLETTER: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/word_break.rs:1019-1029`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/word_break.rs#L1019-L1029)*

### `MIDNUM`
```rust
const MIDNUM: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/word_break.rs:1031-1044`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/word_break.rs#L1031-L1044)*

### `MIDNUMLET`
```rust
const MIDNUMLET: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/word_break.rs:1046-1053`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/word_break.rs#L1046-L1053)*

### `NEWLINE`
```rust
const NEWLINE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/word_break.rs:1055-1056`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/word_break.rs#L1055-L1056)*

### `NUMERIC`
```rust
const NUMERIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/word_break.rs:1058-1137`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/word_break.rs#L1058-L1137)*

### `REGIONAL_INDICATOR`
```rust
const REGIONAL_INDICATOR: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/word_break.rs:1139`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/word_break.rs#L1139)*

### `SINGLE_QUOTE`
```rust
const SINGLE_QUOTE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/word_break.rs:1141`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/word_break.rs#L1141)*

### `WSEGSPACE`
```rust
const WSEGSPACE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/word_break.rs:1143-1150`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/word_break.rs#L1143-L1150)*

### `ZWJ`
```rust
const ZWJ: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/word_break.rs:1152`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/word_break.rs#L1152)*

