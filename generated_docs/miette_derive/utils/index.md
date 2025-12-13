*[miette_derive](../index.md) / [utils](index.md)*

---

# Module `utils`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MemberOrString`](#memberorstring) | enum |  |
| [`gen_all_variants_with`](#gen-all-variants-with) | fn |  |
| [`gen_unused_pat`](#gen-unused-pat) | fn |  |
| [`gen_fields_pat`](#gen-fields-pat) | fn | Goes in the slot `let Self #pat = self;` or `match self { Self #pat => ... |
| [`display_pat_members`](#display-pat-members) | fn | The returned tokens go in the slot `let Self #pat = self;` or `match self { Self #pat => ... |

## Enums

### `MemberOrString`

```rust
enum MemberOrString {
    Member(syn::Member),
    String(syn::LitStr),
}
```

*Defined in [`miette-derive-7.6.0/src/utils.rs:8-11`](../../../.source_1765521767/miette-derive-7.6.0/src/utils.rs#L8-L11)*

#### Trait Implementations

##### `impl Any for MemberOrString`

- <span id="memberorstring-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MemberOrString`

- <span id="memberorstring-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MemberOrString`

- <span id="memberorstring-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for MemberOrString`

- <span id="memberorstring-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MemberOrString`

- <span id="memberorstring-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for MemberOrString`

- <span id="memberorstring-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

##### `impl Spanned for MemberOrString`

- <span id="memberorstring-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for MemberOrString`

- <span id="memberorstring-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for MemberOrString`

- <span id="memberorstring-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="memberorstring-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MemberOrString`

- <span id="memberorstring-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="memberorstring-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `gen_all_variants_with`

```rust
fn gen_all_variants_with(variants: &[crate::diagnostic::DiagnosticDef], which_fn: crate::forward::WhichFn, f: impl FnMut(&syn::Ident, &syn::Fields, &crate::diagnostic::DiagnosticConcreteArgs) -> Option<proc_macro2::TokenStream>) -> Option<proc_macro2::TokenStream>
```

*Defined in [`miette-derive-7.6.0/src/utils.rs:44-72`](../../../.source_1765521767/miette-derive-7.6.0/src/utils.rs#L44-L72)*

### `gen_unused_pat`

```rust
fn gen_unused_pat(fields: &syn::Fields) -> proc_macro2::TokenStream
```

*Defined in [`miette-derive-7.6.0/src/utils.rs:77-83`](../../../.source_1765521767/miette-derive-7.6.0/src/utils.rs#L77-L83)*

### `gen_fields_pat`

```rust
fn gen_fields_pat(fields: &syn::Fields) -> proc_macro2::TokenStream
```

*Defined in [`miette-derive-7.6.0/src/utils.rs:87-104`](../../../.source_1765521767/miette-derive-7.6.0/src/utils.rs#L87-L104)*

Goes in the slot `let Self #pat = self;` or `match self { Self #pat => ...
}`.

### `display_pat_members`

```rust
fn display_pat_members(fields: &syn::Fields) -> (proc_macro2::TokenStream, std::collections::HashSet<syn::Member>)
```

*Defined in [`miette-derive-7.6.0/src/utils.rs:109-126`](../../../.source_1765521767/miette-derive-7.6.0/src/utils.rs#L109-L126)*

The returned tokens go in the slot `let Self #pat = self;` or `match self {
Self #pat => ... }`. The members can be passed to
`Display::expand_shorthand[_cloned]`.

