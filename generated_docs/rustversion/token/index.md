*[rustversion](../index.md) / [token](index.md)*

---

# Module `token`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parse_punct`](#parse-punct) | fn |  |
| [`parse_optional_punct`](#parse-optional-punct) | fn |  |
| [`parse_optional_keyword`](#parse-optional-keyword) | fn |  |
| [`parse_literal`](#parse-literal) | fn |  |
| [`parse_paren`](#parse-paren) | fn |  |
| [`parse_optional_paren`](#parse-optional-paren) | fn |  |
| [`parse_end`](#parse-end) | fn |  |

## Functions

### `parse_punct`

```rust
fn parse_punct(iter: &'_ mut IterImpl, ch: char) -> std::result::Result<(), Error>
```

*Defined in [`rustversion-1.0.22/src/token.rs:5-15`](../../../.source_1765210505/rustversion-1.0.22/src/token.rs#L5-L15)*

### `parse_optional_punct`

```rust
fn parse_optional_punct(iter: &'_ mut IterImpl, ch: char) -> Option<()>
```

*Defined in [`rustversion-1.0.22/src/token.rs:17-22`](../../../.source_1765210505/rustversion-1.0.22/src/token.rs#L17-L22)*

### `parse_optional_keyword`

```rust
fn parse_optional_keyword(iter: &'_ mut IterImpl, keyword: &str) -> Option<proc_macro::Span>
```

*Defined in [`rustversion-1.0.22/src/token.rs:24-31`](../../../.source_1765210505/rustversion-1.0.22/src/token.rs#L24-L31)*

### `parse_literal`

```rust
fn parse_literal(iter: &'_ mut IterImpl) -> std::result::Result<proc_macro::Literal, Error>
```

*Defined in [`rustversion-1.0.22/src/token.rs:33-43`](../../../.source_1765210505/rustversion-1.0.22/src/token.rs#L33-L43)*

### `parse_paren`

```rust
fn parse_paren(introducer: &proc_macro::Ident, iter: &'_ mut IterImpl) -> std::result::Result<proc_macro::Group, Error>
```

*Defined in [`rustversion-1.0.22/src/token.rs:45-59`](../../../.source_1765210505/rustversion-1.0.22/src/token.rs#L45-L59)*

### `parse_optional_paren`

```rust
fn parse_optional_paren(iter: &'_ mut IterImpl) -> Option<proc_macro::Group>
```

*Defined in [`rustversion-1.0.22/src/token.rs:61-71`](../../../.source_1765210505/rustversion-1.0.22/src/token.rs#L61-L71)*

### `parse_end`

```rust
fn parse_end(iter: &'_ mut IterImpl) -> std::result::Result<(), Error>
```

*Defined in [`rustversion-1.0.22/src/token.rs:73-78`](../../../.source_1765210505/rustversion-1.0.22/src/token.rs#L73-L78)*

