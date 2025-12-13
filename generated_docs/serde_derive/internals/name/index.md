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

*Defined in [`serde_derive-1.0.228/src/internals/name.rs:9-15`](../../../../.source_1765633015/serde_derive-1.0.228/src/internals/name.rs#L9-L15)*

#### Implementations

- <span id="multiname-from-attrs"></span>`fn from_attrs(source_name: Name, ser_name: Attr<'_, Name>, de_name: Attr<'_, Name>, de_aliases: Option<VecAttr<'_, Name>>) -> Self` — [`Name`](#name), [`Attr`](../attr/index.md#attr), [`VecAttr`](../attr/index.md#vecattr)

- <span id="multiname-serialize-name"></span>`fn serialize_name(&self) -> &Name` — [`Name`](#name)

  Return the container name for the container when serializing.

- <span id="multiname-deserialize-name"></span>`fn deserialize_name(&self) -> &Name` — [`Name`](#name)

  Return the container name for the container when deserializing.

- <span id="multiname-deserialize-aliases"></span>`fn deserialize_aliases(&self) -> &BTreeSet<Name>` — [`Name`](#name)

#### Trait Implementations

##### `impl Any for MultiName`

- <span id="multiname-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MultiName`

- <span id="multiname-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MultiName`

- <span id="multiname-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for MultiName`

- <span id="multiname-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MultiName`

- <span id="multiname-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for MultiName`

- <span id="multiname-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="multiname-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MultiName`

- <span id="multiname-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="multiname-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Name`

```rust
struct Name {
    pub value: String,
    pub span: proc_macro2::Span,
}
```

*Defined in [`serde_derive-1.0.228/src/internals/name.rs:60-63`](../../../../.source_1765633015/serde_derive-1.0.228/src/internals/name.rs#L60-L63)*

#### Trait Implementations

##### `impl Any for Name`

- <span id="name-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Name`

- <span id="name-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Name`

- <span id="name-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Name`

- <span id="name-clone"></span>`fn clone(&self) -> Name` — [`Name`](#name)

##### `impl CloneToUninit for Name`

- <span id="name-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Display for Name`

- <span id="name-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Name`

##### `impl<T> From for Name`

- <span id="name-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Name`

- <span id="name-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Name`

- <span id="name-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl PartialEq for Name`

- <span id="name-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for Name`

- <span id="name-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl Spanned for Name`

- <span id="name-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Name`

- <span id="name-toowned-type-owned"></span>`type Owned = T`

- <span id="name-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="name-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Name`

- <span id="name-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for Name`

- <span id="name-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Name`

- <span id="name-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="name-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Name`

- <span id="name-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="name-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

