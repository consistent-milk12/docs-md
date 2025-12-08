*[serde_derive](../../index.md) / [internals](../index.md) / [name](index.md)*

---

# Module `name`

## Structs

### `MultiName`

```rust
struct MultiName {
    serialize: Name,
    serialize_renamed: bool,
    deserialize: Name,
    deserialize_renamed: bool,
    deserialize_aliases: std::collections::BTreeSet<Name>,
}
```

#### Implementations

- `fn from_attrs(source_name: Name, ser_name: Attr<'_, Name>, de_name: Attr<'_, Name>, de_aliases: Option<VecAttr<'_, Name>>) -> Self` — [`Name`](#name), [`Attr`](../attr/index.md), [`VecAttr`](../attr/index.md)

- `fn serialize_name(self: &Self) -> &Name` — [`Name`](#name)

- `fn deserialize_name(self: &Self) -> &Name` — [`Name`](#name)

- `fn deserialize_aliases(self: &Self) -> &BTreeSet<Name>` — [`Name`](#name)

### `Name`

```rust
struct Name {
    pub value: String,
    pub span: proc_macro2::Span,
}
```

#### Trait Implementations

##### `impl Clone for Name`

- `fn clone(self: &Self) -> Name` — [`Name`](#name)

##### `impl Display for Name`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Name`

##### `impl Ord for Name`

- `fn cmp(self: &Self, other: &Self) -> Ordering`

##### `impl PartialEq for Name`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl PartialOrd for Name`

- `fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering>`

##### `impl<T> Spanned for Name`

- `fn span(self: &Self) -> Span`

##### `impl<T> ToString for Name`

- `fn to_string(self: &Self) -> String`

##### `impl ToTokens for Name`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

