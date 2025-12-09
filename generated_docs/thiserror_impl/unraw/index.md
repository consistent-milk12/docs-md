*[thiserror_impl](../index.md) / [unraw](index.md)*

---

# Module `unraw`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IdentUnraw`](#identunraw) | struct |  |
| [`MemberUnraw`](#memberunraw) | enum |  |

## Structs

### `IdentUnraw`

```rust
struct IdentUnraw(proc_macro2::Ident);
```

#### Implementations

- <span id="identunraw-new"></span>`fn new(ident: Ident) -> Self`

- <span id="identunraw-to-local"></span>`fn to_local(&self) -> Ident`

- <span id="identunraw-set-span"></span>`fn set_span(&mut self, span: Span)`

#### Trait Implementations

##### `impl Clone for IdentUnraw`

- <span id="identunraw-clone"></span>`fn clone(&self) -> IdentUnraw` — [`IdentUnraw`](#identunraw)

##### `impl Display for IdentUnraw`

- <span id="identunraw-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for IdentUnraw`

##### `impl Ord for IdentUnraw`

- <span id="identunraw-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl Parse for IdentUnraw`

- <span id="identunraw-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl PartialEq for IdentUnraw`

- <span id="identunraw-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for IdentUnraw`

- <span id="identunraw-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<T> Spanned for IdentUnraw`

- <span id="identunraw-span"></span>`fn span(&self) -> Span`

##### `impl<T> ToString for IdentUnraw`

- <span id="identunraw-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for IdentUnraw`

- <span id="identunraw-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

## Enums

### `MemberUnraw`

```rust
enum MemberUnraw {
    Named(IdentUnraw),
    Unnamed(syn::Index),
}
```

#### Implementations

- <span id="memberunraw-span"></span>`fn span(&self) -> Span`

#### Trait Implementations

##### `impl Clone for MemberUnraw`

- <span id="memberunraw-clone"></span>`fn clone(&self) -> MemberUnraw` — [`MemberUnraw`](#memberunraw)

##### `impl Display for MemberUnraw`

- <span id="memberunraw-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for MemberUnraw`

##### `impl Hash for MemberUnraw`

- <span id="memberunraw-hash"></span>`fn hash<H: Hasher>(&self, hasher: &mut H)`

##### `impl PartialEq for MemberUnraw`

- <span id="memberunraw-eq"></span>`fn eq(&self, other: &str) -> bool`

##### `impl<T> Spanned for MemberUnraw`

- <span id="memberunraw-span"></span>`fn span(&self) -> Span`

##### `impl<T> ToString for MemberUnraw`

- <span id="memberunraw-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for MemberUnraw`

- <span id="memberunraw-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

