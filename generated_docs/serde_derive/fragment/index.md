*[serde_derive](../index.md) / [fragment](index.md)*

---

# Module `fragment`

## Structs

### `Expr`

```rust
struct Expr(Fragment);
```

Interpolate a fragment in place of an expression. This involves surrounding
Block fragments in curly braces.

#### Trait Implementations

##### `impl<T> Spanned for Expr`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Expr`

- `fn to_tokens(self: &Self, out: &mut TokenStream)`

### `Stmts`

```rust
struct Stmts(Fragment);
```

Interpolate a fragment as the statements of a block.

#### Trait Implementations

##### `impl<T> Spanned for Stmts`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Stmts`

- `fn to_tokens(self: &Self, out: &mut TokenStream)`

### `Match`

```rust
struct Match(Fragment);
```

Interpolate a fragment as the value part of a `match` expression. This
involves putting a comma after expressions and curly braces around blocks.

#### Trait Implementations

##### `impl<T> Spanned for Match`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Match`

- `fn to_tokens(self: &Self, out: &mut TokenStream)`

## Enums

### `Fragment`

```rust
enum Fragment {
    Expr(proc_macro2::TokenStream),
    Block(proc_macro2::TokenStream),
}
```

#### Variants

- **`Expr`**

  Tokens that can be used as an expression.

- **`Block`**

  Tokens that can be used inside a block. The surrounding curly braces are
  not part of these tokens.

#### Trait Implementations

##### `impl AsRef for Fragment`

- `fn as_ref(self: &Self) -> &TokenStream`

## Macros

### `quote_expr!`

### `quote_block!`

