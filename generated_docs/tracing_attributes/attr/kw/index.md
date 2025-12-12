*[tracing_attributes](../../index.md) / [attr](../index.md) / [kw](index.md)*

---

# Module `kw`

## Contents

- [Structs](#structs)
  - [`fields`](#fields)
  - [`skip`](#skip)
  - [`skip_all`](#skip-all)
  - [`level`](#level)
  - [`target`](#target)
  - [`parent`](#parent)
  - [`follows_from`](#follows-from)
  - [`name`](#name)
  - [`err`](#err)
  - [`ret`](#ret)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`fields`](#fields) | struct |  |
| [`skip`](#skip) | struct |  |
| [`skip_all`](#skip-all) | struct |  |
| [`level`](#level) | struct |  |
| [`target`](#target) | struct |  |
| [`parent`](#parent) | struct |  |
| [`follows_from`](#follows-from) | struct |  |
| [`name`](#name) | struct |  |
| [`err`](#err) | struct |  |
| [`ret`](#ret) | struct |  |

## Structs

### `fields`

```rust
struct fields {
    pub span: __private::Span,
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:497`](../../../../.source_1765210505/tracing-attributes-0.1.31/src/attr.rs#L497)*

#### Trait Implementations

##### `impl Clone for fields`

- <span id="fields-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for fields`

##### `impl Debug for fields`

- <span id="fields-fmt"></span>`fn fmt(&self, f: &mut __private::Formatter<'_>) -> __private::FmtResult`

##### `impl Default for fields`

- <span id="fields-default"></span>`fn default() -> Self`

##### `impl Eq for fields`

##### `impl Hash for fields`

- <span id="fields-hash"></span>`fn hash<__H: __private::Hasher>(&self, _state: &mut __H)`

##### `impl Parse for fields`

- <span id="fields-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<fields>` — [`fields`](#fields)

##### `impl PartialEq for fields`

- <span id="fields-eq"></span>`fn eq(&self, _other: &Self) -> __private::bool`

##### `impl Spanned for fields`

- <span id="fields-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for fields`

- <span id="fields-to-tokens"></span>`fn to_tokens(&self, tokens: &mut __private::TokenStream2)`

##### `impl Token for fields`

- <span id="fields-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool`

- <span id="fields-display"></span>`fn display() -> &'static str`

### `skip`

```rust
struct skip {
    pub span: __private::Span,
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:498`](../../../../.source_1765210505/tracing-attributes-0.1.31/src/attr.rs#L498)*

#### Trait Implementations

##### `impl Clone for skip`

- <span id="skip-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for skip`

##### `impl Debug for skip`

- <span id="skip-fmt"></span>`fn fmt(&self, f: &mut __private::Formatter<'_>) -> __private::FmtResult`

##### `impl Default for skip`

- <span id="skip-default"></span>`fn default() -> Self`

##### `impl Eq for skip`

##### `impl Hash for skip`

- <span id="skip-hash"></span>`fn hash<__H: __private::Hasher>(&self, _state: &mut __H)`

##### `impl Parse for skip`

- <span id="skip-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<skip>` — [`skip`](#skip)

##### `impl PartialEq for skip`

- <span id="skip-eq"></span>`fn eq(&self, _other: &Self) -> __private::bool`

##### `impl Spanned for skip`

- <span id="skip-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for skip`

- <span id="skip-to-tokens"></span>`fn to_tokens(&self, tokens: &mut __private::TokenStream2)`

##### `impl Token for skip`

- <span id="skip-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool`

- <span id="skip-display"></span>`fn display() -> &'static str`

### `skip_all`

```rust
struct skip_all {
    pub span: __private::Span,
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:499`](../../../../.source_1765210505/tracing-attributes-0.1.31/src/attr.rs#L499)*

#### Trait Implementations

##### `impl Clone for skip_all`

- <span id="skip-all-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for skip_all`

##### `impl Debug for skip_all`

- <span id="skip-all-fmt"></span>`fn fmt(&self, f: &mut __private::Formatter<'_>) -> __private::FmtResult`

##### `impl Default for skip_all`

- <span id="skip-all-default"></span>`fn default() -> Self`

##### `impl Eq for skip_all`

##### `impl Hash for skip_all`

- <span id="skip-all-hash"></span>`fn hash<__H: __private::Hasher>(&self, _state: &mut __H)`

##### `impl Parse for skip_all`

- <span id="skip-all-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<skip_all>` — [`skip_all`](#skip-all)

##### `impl PartialEq for skip_all`

- <span id="skip-all-eq"></span>`fn eq(&self, _other: &Self) -> __private::bool`

##### `impl Spanned for skip_all`

- <span id="skip-all-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for skip_all`

- <span id="skip-all-to-tokens"></span>`fn to_tokens(&self, tokens: &mut __private::TokenStream2)`

##### `impl Token for skip_all`

- <span id="skip-all-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool`

- <span id="skip-all-display"></span>`fn display() -> &'static str`

### `level`

```rust
struct level {
    pub span: __private::Span,
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:500`](../../../../.source_1765210505/tracing-attributes-0.1.31/src/attr.rs#L500)*

#### Trait Implementations

##### `impl Clone for level`

- <span id="level-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for level`

##### `impl Debug for level`

- <span id="level-fmt"></span>`fn fmt(&self, f: &mut __private::Formatter<'_>) -> __private::FmtResult`

##### `impl Default for level`

- <span id="level-default"></span>`fn default() -> Self`

##### `impl Eq for level`

##### `impl Hash for level`

- <span id="level-hash"></span>`fn hash<__H: __private::Hasher>(&self, _state: &mut __H)`

##### `impl Parse for level`

- <span id="level-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<level>` — [`level`](#level)

##### `impl PartialEq for level`

- <span id="level-eq"></span>`fn eq(&self, _other: &Self) -> __private::bool`

##### `impl Spanned for level`

- <span id="level-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for level`

- <span id="level-to-tokens"></span>`fn to_tokens(&self, tokens: &mut __private::TokenStream2)`

##### `impl Token for level`

- <span id="level-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool`

- <span id="level-display"></span>`fn display() -> &'static str`

### `target`

```rust
struct target {
    pub span: __private::Span,
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:501`](../../../../.source_1765210505/tracing-attributes-0.1.31/src/attr.rs#L501)*

#### Trait Implementations

##### `impl Clone for target`

- <span id="target-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for target`

##### `impl Debug for target`

- <span id="target-fmt"></span>`fn fmt(&self, f: &mut __private::Formatter<'_>) -> __private::FmtResult`

##### `impl Default for target`

- <span id="target-default"></span>`fn default() -> Self`

##### `impl Eq for target`

##### `impl Hash for target`

- <span id="target-hash"></span>`fn hash<__H: __private::Hasher>(&self, _state: &mut __H)`

##### `impl Parse for target`

- <span id="target-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<target>` — [`target`](#target)

##### `impl PartialEq for target`

- <span id="target-eq"></span>`fn eq(&self, _other: &Self) -> __private::bool`

##### `impl Spanned for target`

- <span id="target-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for target`

- <span id="target-to-tokens"></span>`fn to_tokens(&self, tokens: &mut __private::TokenStream2)`

##### `impl Token for target`

- <span id="target-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool`

- <span id="target-display"></span>`fn display() -> &'static str`

### `parent`

```rust
struct parent {
    pub span: __private::Span,
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:502`](../../../../.source_1765210505/tracing-attributes-0.1.31/src/attr.rs#L502)*

#### Trait Implementations

##### `impl Clone for parent`

- <span id="parent-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for parent`

##### `impl Debug for parent`

- <span id="parent-fmt"></span>`fn fmt(&self, f: &mut __private::Formatter<'_>) -> __private::FmtResult`

##### `impl Default for parent`

- <span id="parent-default"></span>`fn default() -> Self`

##### `impl Eq for parent`

##### `impl Hash for parent`

- <span id="parent-hash"></span>`fn hash<__H: __private::Hasher>(&self, _state: &mut __H)`

##### `impl Parse for parent`

- <span id="parent-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<parent>` — [`parent`](#parent)

##### `impl PartialEq for parent`

- <span id="parent-eq"></span>`fn eq(&self, _other: &Self) -> __private::bool`

##### `impl Spanned for parent`

- <span id="parent-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for parent`

- <span id="parent-to-tokens"></span>`fn to_tokens(&self, tokens: &mut __private::TokenStream2)`

##### `impl Token for parent`

- <span id="parent-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool`

- <span id="parent-display"></span>`fn display() -> &'static str`

### `follows_from`

```rust
struct follows_from {
    pub span: __private::Span,
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:503`](../../../../.source_1765210505/tracing-attributes-0.1.31/src/attr.rs#L503)*

#### Trait Implementations

##### `impl Clone for follows_from`

- <span id="follows-from-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for follows_from`

##### `impl Debug for follows_from`

- <span id="follows-from-fmt"></span>`fn fmt(&self, f: &mut __private::Formatter<'_>) -> __private::FmtResult`

##### `impl Default for follows_from`

- <span id="follows-from-default"></span>`fn default() -> Self`

##### `impl Eq for follows_from`

##### `impl Hash for follows_from`

- <span id="follows-from-hash"></span>`fn hash<__H: __private::Hasher>(&self, _state: &mut __H)`

##### `impl Parse for follows_from`

- <span id="follows-from-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<follows_from>` — [`follows_from`](#follows-from)

##### `impl PartialEq for follows_from`

- <span id="follows-from-eq"></span>`fn eq(&self, _other: &Self) -> __private::bool`

##### `impl Spanned for follows_from`

- <span id="follows-from-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for follows_from`

- <span id="follows-from-to-tokens"></span>`fn to_tokens(&self, tokens: &mut __private::TokenStream2)`

##### `impl Token for follows_from`

- <span id="follows-from-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool`

- <span id="follows-from-display"></span>`fn display() -> &'static str`

### `name`

```rust
struct name {
    pub span: __private::Span,
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:504`](../../../../.source_1765210505/tracing-attributes-0.1.31/src/attr.rs#L504)*

#### Trait Implementations

##### `impl Clone for name`

- <span id="name-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for name`

##### `impl Debug for name`

- <span id="name-fmt"></span>`fn fmt(&self, f: &mut __private::Formatter<'_>) -> __private::FmtResult`

##### `impl Default for name`

- <span id="name-default"></span>`fn default() -> Self`

##### `impl Eq for name`

##### `impl Hash for name`

- <span id="name-hash"></span>`fn hash<__H: __private::Hasher>(&self, _state: &mut __H)`

##### `impl Parse for name`

- <span id="name-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<name>` — [`name`](#name)

##### `impl PartialEq for name`

- <span id="name-eq"></span>`fn eq(&self, _other: &Self) -> __private::bool`

##### `impl Spanned for name`

- <span id="name-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for name`

- <span id="name-to-tokens"></span>`fn to_tokens(&self, tokens: &mut __private::TokenStream2)`

##### `impl Token for name`

- <span id="name-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool`

- <span id="name-display"></span>`fn display() -> &'static str`

### `err`

```rust
struct err {
    pub span: __private::Span,
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:505`](../../../../.source_1765210505/tracing-attributes-0.1.31/src/attr.rs#L505)*

#### Trait Implementations

##### `impl Clone for err`

- <span id="err-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for err`

##### `impl Debug for err`

- <span id="err-fmt"></span>`fn fmt(&self, f: &mut __private::Formatter<'_>) -> __private::FmtResult`

##### `impl Default for err`

- <span id="err-default"></span>`fn default() -> Self`

##### `impl Eq for err`

##### `impl Hash for err`

- <span id="err-hash"></span>`fn hash<__H: __private::Hasher>(&self, _state: &mut __H)`

##### `impl Parse for err`

- <span id="err-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<err>` — [`err`](#err)

##### `impl PartialEq for err`

- <span id="err-eq"></span>`fn eq(&self, _other: &Self) -> __private::bool`

##### `impl Spanned for err`

- <span id="err-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for err`

- <span id="err-to-tokens"></span>`fn to_tokens(&self, tokens: &mut __private::TokenStream2)`

##### `impl Token for err`

- <span id="err-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool`

- <span id="err-display"></span>`fn display() -> &'static str`

### `ret`

```rust
struct ret {
    pub span: __private::Span,
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:506`](../../../../.source_1765210505/tracing-attributes-0.1.31/src/attr.rs#L506)*

#### Trait Implementations

##### `impl Clone for ret`

- <span id="ret-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for ret`

##### `impl Debug for ret`

- <span id="ret-fmt"></span>`fn fmt(&self, f: &mut __private::Formatter<'_>) -> __private::FmtResult`

##### `impl Default for ret`

- <span id="ret-default"></span>`fn default() -> Self`

##### `impl Eq for ret`

##### `impl Hash for ret`

- <span id="ret-hash"></span>`fn hash<__H: __private::Hasher>(&self, _state: &mut __H)`

##### `impl Parse for ret`

- <span id="ret-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<ret>` — [`ret`](#ret)

##### `impl PartialEq for ret`

- <span id="ret-eq"></span>`fn eq(&self, _other: &Self) -> __private::bool`

##### `impl Spanned for ret`

- <span id="ret-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for ret`

- <span id="ret-to-tokens"></span>`fn to_tokens(&self, tokens: &mut __private::TokenStream2)`

##### `impl Token for ret`

- <span id="ret-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool`

- <span id="ret-display"></span>`fn display() -> &'static str`

