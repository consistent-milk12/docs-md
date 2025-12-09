*[syn](../../index.md) / [derive](../index.md) / [parsing](index.md)*

---

# Module `parsing`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`data_struct`](#data_struct) | fn |  |
| [`data_enum`](#data_enum) | fn |  |
| [`data_union`](#data_union) | fn |  |

## Functions

### `data_struct`

```rust
fn data_struct(input: crate::parse::ParseStream<'_>) -> crate::error::Result<(Option<crate::generics::WhereClause>, crate::data::Fields, Option<token::Semi>)>
```

*Defined in [`syn-2.0.111/src/derive.rs:148-182`](../../../../.source_1765210505/syn-2.0.111/src/derive.rs#L148-L182)*

### `data_enum`

```rust
fn data_enum(input: crate::parse::ParseStream<'_>) -> crate::error::Result<(Option<crate::generics::WhereClause>, token::Brace, crate::punctuated::Punctuated<crate::data::Variant, token::Comma>)>
```

*Defined in [`syn-2.0.111/src/derive.rs:184-198`](../../../../.source_1765210505/syn-2.0.111/src/derive.rs#L184-L198)*

### `data_union`

```rust
fn data_union(input: crate::parse::ParseStream<'_>) -> crate::error::Result<(Option<crate::generics::WhereClause>, crate::data::FieldsNamed)>
```

*Defined in [`syn-2.0.111/src/derive.rs:200-204`](../../../../.source_1765210505/syn-2.0.111/src/derive.rs#L200-L204)*

