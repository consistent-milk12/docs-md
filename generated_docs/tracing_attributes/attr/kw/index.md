*[tracing_attributes](../../index.md) / [attr](../index.md) / [kw](index.md)*

---

# Module `kw`

## Structs

### `fields`

```rust
struct fields {
    pub span: $crate::__private::Span,
}
```

#### Trait Implementations

##### `impl Clone for fields`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for fields`

##### `impl Debug for fields`

- `fn fmt(self: &Self, f: &mut $crate::__private::Formatter<'_>) -> $crate::__private::FmtResult`

##### `impl Default for fields`

- `fn default() -> Self`

##### `impl Eq for fields`

##### `impl Hash for fields`

- `fn hash<__H: $crate::__private::Hasher>(self: &Self, _state: &mut __H)`

##### `impl Parse for fields`

- `fn parse(input: $crate::parse::ParseStream<'_>) -> $crate::parse::Result<fields>` — [`fields`](#fields)

##### `impl PartialEq for fields`

- `fn eq(self: &Self, _other: &Self) -> $crate::__private::bool`

##### `impl<T> Spanned for fields`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for fields`

- `fn to_tokens(self: &Self, tokens: &mut $crate::__private::TokenStream2)`

##### `impl<T> Token for fields`

- `fn peek(cursor: Cursor<'_>) -> bool`

- `fn display() -> &'static str`

### `skip`

```rust
struct skip {
    pub span: $crate::__private::Span,
}
```

#### Trait Implementations

##### `impl Clone for skip`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for skip`

##### `impl Debug for skip`

- `fn fmt(self: &Self, f: &mut $crate::__private::Formatter<'_>) -> $crate::__private::FmtResult`

##### `impl Default for skip`

- `fn default() -> Self`

##### `impl Eq for skip`

##### `impl Hash for skip`

- `fn hash<__H: $crate::__private::Hasher>(self: &Self, _state: &mut __H)`

##### `impl Parse for skip`

- `fn parse(input: $crate::parse::ParseStream<'_>) -> $crate::parse::Result<skip>` — [`skip`](#skip)

##### `impl PartialEq for skip`

- `fn eq(self: &Self, _other: &Self) -> $crate::__private::bool`

##### `impl<T> Spanned for skip`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for skip`

- `fn to_tokens(self: &Self, tokens: &mut $crate::__private::TokenStream2)`

##### `impl<T> Token for skip`

- `fn peek(cursor: Cursor<'_>) -> bool`

- `fn display() -> &'static str`

### `skip_all`

```rust
struct skip_all {
    pub span: $crate::__private::Span,
}
```

#### Trait Implementations

##### `impl Clone for skip_all`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for skip_all`

##### `impl Debug for skip_all`

- `fn fmt(self: &Self, f: &mut $crate::__private::Formatter<'_>) -> $crate::__private::FmtResult`

##### `impl Default for skip_all`

- `fn default() -> Self`

##### `impl Eq for skip_all`

##### `impl Hash for skip_all`

- `fn hash<__H: $crate::__private::Hasher>(self: &Self, _state: &mut __H)`

##### `impl Parse for skip_all`

- `fn parse(input: $crate::parse::ParseStream<'_>) -> $crate::parse::Result<skip_all>` — [`skip_all`](#skip-all)

##### `impl PartialEq for skip_all`

- `fn eq(self: &Self, _other: &Self) -> $crate::__private::bool`

##### `impl<T> Spanned for skip_all`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for skip_all`

- `fn to_tokens(self: &Self, tokens: &mut $crate::__private::TokenStream2)`

##### `impl<T> Token for skip_all`

- `fn peek(cursor: Cursor<'_>) -> bool`

- `fn display() -> &'static str`

### `level`

```rust
struct level {
    pub span: $crate::__private::Span,
}
```

#### Trait Implementations

##### `impl Clone for level`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for level`

##### `impl Debug for level`

- `fn fmt(self: &Self, f: &mut $crate::__private::Formatter<'_>) -> $crate::__private::FmtResult`

##### `impl Default for level`

- `fn default() -> Self`

##### `impl Eq for level`

##### `impl Hash for level`

- `fn hash<__H: $crate::__private::Hasher>(self: &Self, _state: &mut __H)`

##### `impl Parse for level`

- `fn parse(input: $crate::parse::ParseStream<'_>) -> $crate::parse::Result<level>` — [`level`](#level)

##### `impl PartialEq for level`

- `fn eq(self: &Self, _other: &Self) -> $crate::__private::bool`

##### `impl<T> Spanned for level`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for level`

- `fn to_tokens(self: &Self, tokens: &mut $crate::__private::TokenStream2)`

##### `impl<T> Token for level`

- `fn peek(cursor: Cursor<'_>) -> bool`

- `fn display() -> &'static str`

### `target`

```rust
struct target {
    pub span: $crate::__private::Span,
}
```

#### Trait Implementations

##### `impl Clone for target`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for target`

##### `impl Debug for target`

- `fn fmt(self: &Self, f: &mut $crate::__private::Formatter<'_>) -> $crate::__private::FmtResult`

##### `impl Default for target`

- `fn default() -> Self`

##### `impl Eq for target`

##### `impl Hash for target`

- `fn hash<__H: $crate::__private::Hasher>(self: &Self, _state: &mut __H)`

##### `impl Parse for target`

- `fn parse(input: $crate::parse::ParseStream<'_>) -> $crate::parse::Result<target>` — [`target`](#target)

##### `impl PartialEq for target`

- `fn eq(self: &Self, _other: &Self) -> $crate::__private::bool`

##### `impl<T> Spanned for target`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for target`

- `fn to_tokens(self: &Self, tokens: &mut $crate::__private::TokenStream2)`

##### `impl<T> Token for target`

- `fn peek(cursor: Cursor<'_>) -> bool`

- `fn display() -> &'static str`

### `parent`

```rust
struct parent {
    pub span: $crate::__private::Span,
}
```

#### Trait Implementations

##### `impl Clone for parent`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for parent`

##### `impl Debug for parent`

- `fn fmt(self: &Self, f: &mut $crate::__private::Formatter<'_>) -> $crate::__private::FmtResult`

##### `impl Default for parent`

- `fn default() -> Self`

##### `impl Eq for parent`

##### `impl Hash for parent`

- `fn hash<__H: $crate::__private::Hasher>(self: &Self, _state: &mut __H)`

##### `impl Parse for parent`

- `fn parse(input: $crate::parse::ParseStream<'_>) -> $crate::parse::Result<parent>` — [`parent`](#parent)

##### `impl PartialEq for parent`

- `fn eq(self: &Self, _other: &Self) -> $crate::__private::bool`

##### `impl<T> Spanned for parent`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for parent`

- `fn to_tokens(self: &Self, tokens: &mut $crate::__private::TokenStream2)`

##### `impl<T> Token for parent`

- `fn peek(cursor: Cursor<'_>) -> bool`

- `fn display() -> &'static str`

### `follows_from`

```rust
struct follows_from {
    pub span: $crate::__private::Span,
}
```

#### Trait Implementations

##### `impl Clone for follows_from`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for follows_from`

##### `impl Debug for follows_from`

- `fn fmt(self: &Self, f: &mut $crate::__private::Formatter<'_>) -> $crate::__private::FmtResult`

##### `impl Default for follows_from`

- `fn default() -> Self`

##### `impl Eq for follows_from`

##### `impl Hash for follows_from`

- `fn hash<__H: $crate::__private::Hasher>(self: &Self, _state: &mut __H)`

##### `impl Parse for follows_from`

- `fn parse(input: $crate::parse::ParseStream<'_>) -> $crate::parse::Result<follows_from>` — [`follows_from`](#follows-from)

##### `impl PartialEq for follows_from`

- `fn eq(self: &Self, _other: &Self) -> $crate::__private::bool`

##### `impl<T> Spanned for follows_from`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for follows_from`

- `fn to_tokens(self: &Self, tokens: &mut $crate::__private::TokenStream2)`

##### `impl<T> Token for follows_from`

- `fn peek(cursor: Cursor<'_>) -> bool`

- `fn display() -> &'static str`

### `name`

```rust
struct name {
    pub span: $crate::__private::Span,
}
```

#### Trait Implementations

##### `impl Clone for name`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for name`

##### `impl Debug for name`

- `fn fmt(self: &Self, f: &mut $crate::__private::Formatter<'_>) -> $crate::__private::FmtResult`

##### `impl Default for name`

- `fn default() -> Self`

##### `impl Eq for name`

##### `impl Hash for name`

- `fn hash<__H: $crate::__private::Hasher>(self: &Self, _state: &mut __H)`

##### `impl Parse for name`

- `fn parse(input: $crate::parse::ParseStream<'_>) -> $crate::parse::Result<name>` — [`name`](#name)

##### `impl PartialEq for name`

- `fn eq(self: &Self, _other: &Self) -> $crate::__private::bool`

##### `impl<T> Spanned for name`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for name`

- `fn to_tokens(self: &Self, tokens: &mut $crate::__private::TokenStream2)`

##### `impl<T> Token for name`

- `fn peek(cursor: Cursor<'_>) -> bool`

- `fn display() -> &'static str`

### `err`

```rust
struct err {
    pub span: $crate::__private::Span,
}
```

#### Trait Implementations

##### `impl Clone for err`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for err`

##### `impl Debug for err`

- `fn fmt(self: &Self, f: &mut $crate::__private::Formatter<'_>) -> $crate::__private::FmtResult`

##### `impl Default for err`

- `fn default() -> Self`

##### `impl Eq for err`

##### `impl Hash for err`

- `fn hash<__H: $crate::__private::Hasher>(self: &Self, _state: &mut __H)`

##### `impl Parse for err`

- `fn parse(input: $crate::parse::ParseStream<'_>) -> $crate::parse::Result<err>` — [`err`](#err)

##### `impl PartialEq for err`

- `fn eq(self: &Self, _other: &Self) -> $crate::__private::bool`

##### `impl<T> Spanned for err`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for err`

- `fn to_tokens(self: &Self, tokens: &mut $crate::__private::TokenStream2)`

##### `impl<T> Token for err`

- `fn peek(cursor: Cursor<'_>) -> bool`

- `fn display() -> &'static str`

### `ret`

```rust
struct ret {
    pub span: $crate::__private::Span,
}
```

#### Trait Implementations

##### `impl Clone for ret`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for ret`

##### `impl Debug for ret`

- `fn fmt(self: &Self, f: &mut $crate::__private::Formatter<'_>) -> $crate::__private::FmtResult`

##### `impl Default for ret`

- `fn default() -> Self`

##### `impl Eq for ret`

##### `impl Hash for ret`

- `fn hash<__H: $crate::__private::Hasher>(self: &Self, _state: &mut __H)`

##### `impl Parse for ret`

- `fn parse(input: $crate::parse::ParseStream<'_>) -> $crate::parse::Result<ret>` — [`ret`](#ret)

##### `impl PartialEq for ret`

- `fn eq(self: &Self, _other: &Self) -> $crate::__private::bool`

##### `impl<T> Spanned for ret`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for ret`

- `fn to_tokens(self: &Self, tokens: &mut $crate::__private::TokenStream2)`

##### `impl<T> Token for ret`

- `fn peek(cursor: Cursor<'_>) -> bool`

- `fn display() -> &'static str`

