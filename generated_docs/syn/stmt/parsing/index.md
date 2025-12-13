*[syn](../../index.md) / [stmt](../index.md) / [parsing](index.md)*

---

# Module `parsing`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AllowNoSemi`](#allownosemi) | struct |  |
| [`parse_stmt`](#parse-stmt) | fn |  |
| [`stmt_mac`](#stmt-mac) | fn |  |
| [`stmt_local`](#stmt-local) | fn |  |
| [`stmt_expr`](#stmt-expr) | fn |  |

## Structs

### `AllowNoSemi`

```rust
struct AllowNoSemi(bool);
```

*Defined in [`syn-2.0.111/src/stmt.rs:98`](../../../../.source_1765521767/syn-2.0.111/src/stmt.rs#L98)*

#### Trait Implementations

##### `impl Any for AllowNoSemi`

- <span id="allownosemi-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AllowNoSemi`

- <span id="allownosemi-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AllowNoSemi`

- <span id="allownosemi-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for AllowNoSemi`

- <span id="allownosemi-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AllowNoSemi`

- <span id="allownosemi-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for AllowNoSemi`

- <span id="allownosemi-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="allownosemi-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AllowNoSemi`

- <span id="allownosemi-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="allownosemi-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `parse_stmt`

```rust
fn parse_stmt(input: crate::parse::ParseStream<'_>, allow_nosemi: AllowNoSemi) -> crate::error::Result<crate::stmt::Stmt>
```

*Defined in [`syn-2.0.111/src/stmt.rs:198-264`](../../../../.source_1765521767/syn-2.0.111/src/stmt.rs#L198-L264)*

### `stmt_mac`

```rust
fn stmt_mac(input: crate::parse::ParseStream<'_>, attrs: Vec<crate::attr::Attribute>, path: crate::path::Path) -> crate::error::Result<crate::stmt::StmtMacro>
```

*Defined in [`syn-2.0.111/src/stmt.rs:266-281`](../../../../.source_1765521767/syn-2.0.111/src/stmt.rs#L266-L281)*

### `stmt_local`

```rust
fn stmt_local(input: crate::parse::ParseStream<'_>, attrs: Vec<crate::attr::Attribute>) -> crate::error::Result<crate::stmt::Local>
```

*Defined in [`syn-2.0.111/src/stmt.rs:283-332`](../../../../.source_1765521767/syn-2.0.111/src/stmt.rs#L283-L332)*

### `stmt_expr`

```rust
fn stmt_expr(input: crate::parse::ParseStream<'_>, allow_nosemi: AllowNoSemi, attrs: Vec<crate::attr::Attribute>) -> crate::error::Result<crate::stmt::Stmt>
```

*Defined in [`syn-2.0.111/src/stmt.rs:334-411`](../../../../.source_1765521767/syn-2.0.111/src/stmt.rs#L334-L411)*

