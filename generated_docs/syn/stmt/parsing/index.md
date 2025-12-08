*[syn](../../index.md) / [stmt](../index.md) / [parsing](index.md)*

---

# Module `parsing`

## Structs

### `AllowNoSemi`

```rust
struct AllowNoSemi(bool);
```

## Functions

### `parse_stmt`

```rust
fn parse_stmt(input: crate::parse::ParseStream<'_>, allow_nosemi: AllowNoSemi) -> crate::error::Result<crate::stmt::Stmt>
```

### `stmt_mac`

```rust
fn stmt_mac(input: crate::parse::ParseStream<'_>, attrs: Vec<crate::attr::Attribute>, path: crate::path::Path) -> crate::error::Result<crate::stmt::StmtMacro>
```

### `stmt_local`

```rust
fn stmt_local(input: crate::parse::ParseStream<'_>, attrs: Vec<crate::attr::Attribute>) -> crate::error::Result<crate::stmt::Local>
```

### `stmt_expr`

```rust
fn stmt_expr(input: crate::parse::ParseStream<'_>, allow_nosemi: AllowNoSemi, attrs: Vec<crate::attr::Attribute>) -> crate::error::Result<crate::stmt::Stmt>
```

