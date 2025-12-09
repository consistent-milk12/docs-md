*[serde_derive](../../index.md) / [internals](../index.md) / [name](index.md)*

---

# Module `name`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MultiName`](#multiname) | struct |  |
| [`Name`](#name) | struct |  |

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

*Defined in [`serde_derive-1.0.228/src/internals/name.rs:9-15`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/name.rs#L9-L15)*

#### Implementations

- <span id="multiname-from-attrs"></span>`fn from_attrs(source_name: Name, ser_name: Attr<'_, Name>, de_name: Attr<'_, Name>, de_aliases: Option<VecAttr<'_, Name>>) -> Self` — [`Name`](#name), [`Attr`](../attr/index.md), [`VecAttr`](../attr/index.md)

- <span id="multiname-serialize-name"></span>`fn serialize_name(&self) -> &Name` — [`Name`](#name)

- <span id="multiname-deserialize-name"></span>`fn deserialize_name(&self) -> &Name` — [`Name`](#name)

- <span id="multiname-deserialize-aliases"></span>`fn deserialize_aliases(&self) -> &BTreeSet<Name>` — [`Name`](#name)

### `Name`

```rust
struct Name {
    pub value: String,
    pub span: proc_macro2::Span,
}
```

*Defined in [`serde_derive-1.0.228/src/internals/name.rs:60-63`](../../../../.source_1765210505/serde_derive-1.0.228/src/internals/name.rs#L60-L63)*

#### Trait Implementations

##### `impl Clone for Name`

- <span id="name-clone"></span>`fn clone(&self) -> Name` — [`Name`](#name)

##### `impl Display for Name`

- <span id="name-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Name`

##### `impl Ord for Name`

- <span id="name-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl PartialEq for Name`

- <span id="name-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for Name`

- <span id="name-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl Spanned for Name`

- <span id="name-span"></span>`fn span(&self) -> Span`

##### `impl ToString for Name`

- <span id="name-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for Name`

- <span id="name-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

