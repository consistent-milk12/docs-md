*[serde_derive](../index.md) / [fragment](index.md)*

---

# Module `fragment`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Expr`](#expr) | struct | Interpolate a fragment in place of an expression. |
| [`Stmts`](#stmts) | struct | Interpolate a fragment as the statements of a block. |
| [`Match`](#match) | struct | Interpolate a fragment as the value part of a `match` expression. |
| [`Fragment`](#fragment) | enum |  |
| [`quote_expr!`](#quote-expr) | macro |  |
| [`quote_block!`](#quote-block) | macro |  |

## Structs

### `Expr`

```rust
struct Expr(Fragment);
```

*Defined in [`serde_derive-1.0.228/src/fragment.rs:27`](../../../.source_1765521767/serde_derive-1.0.228/src/fragment.rs#L27)*

Interpolate a fragment in place of an expression. This involves surrounding
Block fragments in curly braces.

#### Trait Implementations

##### `impl Spanned for Expr`

- <span id="expr-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Expr`

- <span id="expr-to-tokens"></span>`fn to_tokens(&self, out: &mut TokenStream)`

### `Stmts`

```rust
struct Stmts(Fragment);
```

*Defined in [`serde_derive-1.0.228/src/fragment.rs:40`](../../../.source_1765521767/serde_derive-1.0.228/src/fragment.rs#L40)*

Interpolate a fragment as the statements of a block.

#### Trait Implementations

##### `impl Spanned for Stmts`

- <span id="stmts-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Stmts`

- <span id="stmts-to-tokens"></span>`fn to_tokens(&self, out: &mut TokenStream)`

### `Match`

```rust
struct Match(Fragment);
```

*Defined in [`serde_derive-1.0.228/src/fragment.rs:52`](../../../.source_1765521767/serde_derive-1.0.228/src/fragment.rs#L52)*

Interpolate a fragment as the value part of a `match` expression. This
involves putting a comma after expressions and curly braces around blocks.

#### Trait Implementations

##### `impl Spanned for Match`

- <span id="match-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Match`

- <span id="match-to-tokens"></span>`fn to_tokens(&self, out: &mut TokenStream)`

## Enums

### `Fragment`

```rust
enum Fragment {
    Expr(proc_macro2::TokenStream),
    Block(proc_macro2::TokenStream),
}
```

*Defined in [`serde_derive-1.0.228/src/fragment.rs:5-11`](../../../.source_1765521767/serde_derive-1.0.228/src/fragment.rs#L5-L11)*

#### Variants

- **`Expr`**

  Tokens that can be used as an expression.

- **`Block`**

  Tokens that can be used inside a block. The surrounding curly braces are
  not part of these tokens.

#### Trait Implementations

##### `impl AsRef for Fragment`

- <span id="fragment-as-ref"></span>`fn as_ref(&self) -> &TokenStream`

## Macros

### `quote_expr!`

*Defined in [`serde_derive-1.0.228/src/fragment.rs:13-17`](../../../.source_1765521767/serde_derive-1.0.228/src/fragment.rs#L13-L17)*

### `quote_block!`

*Defined in [`serde_derive-1.0.228/src/fragment.rs:19-23`](../../../.source_1765521767/serde_derive-1.0.228/src/fragment.rs#L19-L23)*

