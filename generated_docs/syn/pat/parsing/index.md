*[syn](../../index.md) / [pat](../index.md) / [parsing](index.md)*

---

# Module `parsing`

## Contents

- [Enums](#enums)
  - [`PatRangeBound`](#patrangebound)
- [Functions](#functions)
  - [`multi_pat_impl`](#multi_pat_impl)
  - [`pat_path_or_macro_or_struct_or_range`](#pat_path_or_macro_or_struct_or_range)
  - [`pat_wild`](#pat_wild)
  - [`pat_box`](#pat_box)
  - [`pat_ident`](#pat_ident)
  - [`pat_tuple_struct`](#pat_tuple_struct)
  - [`pat_struct`](#pat_struct)
  - [`field_pat`](#field_pat)
  - [`pat_range`](#pat_range)
  - [`pat_range_half_open`](#pat_range_half_open)
  - [`pat_paren_or_tuple`](#pat_paren_or_tuple)
  - [`pat_reference`](#pat_reference)
  - [`pat_lit_or_range`](#pat_lit_or_range)
  - [`pat_range_bound`](#pat_range_bound)
  - [`pat_slice`](#pat_slice)
  - [`pat_const`](#pat_const)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PatRangeBound`](#patrangebound) | enum |  |
| [`multi_pat_impl`](#multi_pat_impl) | fn |  |
| [`pat_path_or_macro_or_struct_or_range`](#pat_path_or_macro_or_struct_or_range) | fn |  |
| [`pat_wild`](#pat_wild) | fn |  |
| [`pat_box`](#pat_box) | fn |  |
| [`pat_ident`](#pat_ident) | fn |  |
| [`pat_tuple_struct`](#pat_tuple_struct) | fn |  |
| [`pat_struct`](#pat_struct) | fn |  |
| [`field_pat`](#field_pat) | fn |  |
| [`pat_range`](#pat_range) | fn |  |
| [`pat_range_half_open`](#pat_range_half_open) | fn |  |
| [`pat_paren_or_tuple`](#pat_paren_or_tuple) | fn |  |
| [`pat_reference`](#pat_reference) | fn |  |
| [`pat_lit_or_range`](#pat_lit_or_range) | fn |  |
| [`pat_range_bound`](#pat_range_bound) | fn |  |
| [`pat_slice`](#pat_slice) | fn |  |
| [`pat_const`](#pat_const) | fn |  |

## Enums

### `PatRangeBound`

```rust
enum PatRangeBound {
    Const(crate::expr::ExprConst),
    Lit(crate::expr::ExprLit),
    Path(crate::expr::ExprPath),
}
```

*Defined in [`syn-2.0.111/src/pat.rs:702-706`](../../../../.source_1765210505/syn-2.0.111/src/pat.rs#L702-L706)*

#### Implementations

- <span id="patrangebound-into-expr"></span>`fn into_expr(self) -> Box<Expr>` — [`Expr`](../../expr/index.md#expr)

- <span id="patrangebound-into-pat"></span>`fn into_pat(self) -> Pat` — [`Pat`](../index.md#pat)

## Functions

### `multi_pat_impl`

```rust
fn multi_pat_impl(input: crate::parse::ParseStream<'_>, leading_vert: Option<token::Or>) -> crate::error::Result<crate::pat::Pat>
```

*Defined in [`syn-2.0.111/src/pat.rs:397-417`](../../../../.source_1765210505/syn-2.0.111/src/pat.rs#L397-L417)*

### `pat_path_or_macro_or_struct_or_range`

```rust
fn pat_path_or_macro_or_struct_or_range(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::pat::Pat>
```

*Defined in [`syn-2.0.111/src/pat.rs:419-454`](../../../../.source_1765210505/syn-2.0.111/src/pat.rs#L419-L454)*

### `pat_wild`

```rust
fn pat_wild(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::pat::PatWild>
```

*Defined in [`syn-2.0.111/src/pat.rs:456-461`](../../../../.source_1765210505/syn-2.0.111/src/pat.rs#L456-L461)*

### `pat_box`

```rust
fn pat_box(begin: crate::parse::ParseBuffer<'_>, input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::pat::Pat>
```

*Defined in [`syn-2.0.111/src/pat.rs:463-467`](../../../../.source_1765210505/syn-2.0.111/src/pat.rs#L463-L467)*

### `pat_ident`

```rust
fn pat_ident(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::pat::PatIdent>
```

*Defined in [`syn-2.0.111/src/pat.rs:469-491`](../../../../.source_1765210505/syn-2.0.111/src/pat.rs#L469-L491)*

### `pat_tuple_struct`

```rust
fn pat_tuple_struct(input: crate::parse::ParseStream<'_>, qself: Option<crate::path::QSelf>, path: crate::path::Path) -> crate::error::Result<crate::pat::PatTupleStruct>
```

*Defined in [`syn-2.0.111/src/pat.rs:493-519`](../../../../.source_1765210505/syn-2.0.111/src/pat.rs#L493-L519)*

### `pat_struct`

```rust
fn pat_struct(input: crate::parse::ParseStream<'_>, qself: Option<crate::path::QSelf>, path: crate::path::Path) -> crate::error::Result<crate::pat::PatStruct>
```

*Defined in [`syn-2.0.111/src/pat.rs:521-554`](../../../../.source_1765210505/syn-2.0.111/src/pat.rs#L521-L554)*

### `field_pat`

```rust
fn field_pat(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::pat::FieldPat>
```

*Defined in [`syn-2.0.111/src/pat.rs:556-602`](../../../../.source_1765210505/syn-2.0.111/src/pat.rs#L556-L602)*

### `pat_range`

```rust
fn pat_range(input: crate::parse::ParseStream<'_>, qself: Option<crate::path::QSelf>, path: crate::path::Path) -> crate::error::Result<crate::pat::Pat>
```

*Defined in [`syn-2.0.111/src/pat.rs:604-620`](../../../../.source_1765210505/syn-2.0.111/src/pat.rs#L604-L620)*

### `pat_range_half_open`

```rust
fn pat_range_half_open(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::pat::Pat>
```

*Defined in [`syn-2.0.111/src/pat.rs:622-641`](../../../../.source_1765210505/syn-2.0.111/src/pat.rs#L622-L641)*

### `pat_paren_or_tuple`

```rust
fn pat_paren_or_tuple(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::pat::Pat>
```

*Defined in [`syn-2.0.111/src/pat.rs:643-671`](../../../../.source_1765210505/syn-2.0.111/src/pat.rs#L643-L671)*

### `pat_reference`

```rust
fn pat_reference(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::pat::PatReference>
```

*Defined in [`syn-2.0.111/src/pat.rs:673-680`](../../../../.source_1765210505/syn-2.0.111/src/pat.rs#L673-L680)*

### `pat_lit_or_range`

```rust
fn pat_lit_or_range(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::pat::Pat>
```

*Defined in [`syn-2.0.111/src/pat.rs:682-699`](../../../../.source_1765210505/syn-2.0.111/src/pat.rs#L682-L699)*

### `pat_range_bound`

```rust
fn pat_range_bound(input: crate::parse::ParseStream<'_>) -> crate::error::Result<Option<PatRangeBound>>
```

*Defined in [`syn-2.0.111/src/pat.rs:726-757`](../../../../.source_1765210505/syn-2.0.111/src/pat.rs#L726-L757)*

### `pat_slice`

```rust
fn pat_slice(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::pat::PatSlice>
```

*Defined in [`syn-2.0.111/src/pat.rs:759-792`](../../../../.source_1765210505/syn-2.0.111/src/pat.rs#L759-L792)*

### `pat_const`

```rust
fn pat_const(input: crate::parse::ParseStream<'_>) -> crate::error::Result<proc_macro2::TokenStream>
```

*Defined in [`syn-2.0.111/src/pat.rs:794-804`](../../../../.source_1765210505/syn-2.0.111/src/pat.rs#L794-L804)*

