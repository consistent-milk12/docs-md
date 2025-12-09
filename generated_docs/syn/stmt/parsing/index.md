*[syn](../../index.md) / [stmt](../index.md) / [parsing](index.md)*

---

# Module `parsing`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AllowNoSemi`](#allownosemi) | struct |  |
| [`parse_stmt`](#parse_stmt) | fn |  |
| [`stmt_mac`](#stmt_mac) | fn |  |
| [`stmt_local`](#stmt_local) | fn |  |
| [`stmt_expr`](#stmt_expr) | fn |  |

## Structs

### `AllowNoSemi`

```rust
struct AllowNoSemi(bool);
```

*Defined in [`syn-2.0.111/src/stmt.rs:98`](../../../../.source_1765210505/syn-2.0.111/src/stmt.rs#L98)*

## Functions

### `parse_stmt`

```rust
fn parse_stmt(input: crate::parse::ParseStream<'_>, allow_nosemi: AllowNoSemi) -> crate::error::Result<crate::stmt::Stmt>
```

*Defined in [`syn-2.0.111/src/stmt.rs:198-264`](../../../../.source_1765210505/syn-2.0.111/src/stmt.rs#L198-L264)*

### `stmt_mac`

```rust
fn stmt_mac(input: crate::parse::ParseStream<'_>, attrs: Vec<crate::attr::Attribute>, path: crate::path::Path) -> crate::error::Result<crate::stmt::StmtMacro>
```

*Defined in [`syn-2.0.111/src/stmt.rs:266-281`](../../../../.source_1765210505/syn-2.0.111/src/stmt.rs#L266-L281)*

### `stmt_local`

```rust
fn stmt_local(input: crate::parse::ParseStream<'_>, attrs: Vec<crate::attr::Attribute>) -> crate::error::Result<crate::stmt::Local>
```

*Defined in [`syn-2.0.111/src/stmt.rs:283-332`](../../../../.source_1765210505/syn-2.0.111/src/stmt.rs#L283-L332)*

### `stmt_expr`

```rust
fn stmt_expr(input: crate::parse::ParseStream<'_>, allow_nosemi: AllowNoSemi, attrs: Vec<crate::attr::Attribute>) -> crate::error::Result<crate::stmt::Stmt>
```

*Defined in [`syn-2.0.111/src/stmt.rs:334-411`](../../../../.source_1765210505/syn-2.0.111/src/stmt.rs#L334-L411)*

