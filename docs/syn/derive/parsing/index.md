*[syn](../../index.md) / [derive](../index.md) / [parsing](index.md)*

---

# Module `parsing`

## Functions

### `data_struct`

```rust
fn data_struct(input: crate::parse::ParseStream<'_>) -> crate::error::Result<(Option<crate::generics::WhereClause>, crate::data::Fields, Option<$crate::token::Semi>)>
```

### `data_enum`

```rust
fn data_enum(input: crate::parse::ParseStream<'_>) -> crate::error::Result<(Option<crate::generics::WhereClause>, token::Brace, crate::punctuated::Punctuated<crate::data::Variant, $crate::token::Comma>)>
```

### `data_union`

```rust
fn data_union(input: crate::parse::ParseStream<'_>) -> crate::error::Result<(Option<crate::generics::WhereClause>, crate::data::FieldsNamed)>
```

