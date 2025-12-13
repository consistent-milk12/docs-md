*[clap_builder](../../index.md) / [util](../index.md) / [id](index.md)*

---

# Module `id`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Id`](#id) | struct | [`Arg`][crate::Arg] or [`ArgGroup`][crate::ArgGroup] identifier |

## Structs

### `Id`

```rust
struct Id(crate::builder::Str);
```

*Defined in [`clap_builder-4.5.53/src/util/id.rs:11`](../../../../.source_1765521767/clap_builder-4.5.53/src/util/id.rs#L11)*

`Arg` or `ArgGroup` identifier

This is used for accessing the value in `ArgMatches` or defining
relationships between `Arg`s and `ArgGroup`s with functions like
`Arg::conflicts_with`.

#### Implementations

- <span id="id-const-help"></span>`const HELP: &'static str`

- <span id="id-const-version"></span>`const VERSION: &'static str`

- <span id="id-const-external"></span>`const EXTERNAL: &'static str`

- <span id="id-from-static-ref"></span>`fn from_static_ref(name: &'static str) -> Self`

- <span id="id-as-str"></span>`fn as_str(&self) -> &str`

  Get the raw string of the `Id`

- <span id="id-as-internal-str"></span>`fn as_internal_str(&self) -> &Str` — [`Str`](../../builder/str/index.md#str)

#### Trait Implementations

##### `impl Any for Id`

- <span id="id-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl AsRef for Id`

- <span id="id-asref-as-ref"></span>`fn as_ref(&self) -> &str`

##### `impl<T> Borrow for Id`

- <span id="id-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Id`

- <span id="id-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Id`

- <span id="id-clone"></span>`fn clone(&self) -> Id` — [`Id`](#id)

##### `impl CloneToUninit for Id`

- <span id="id-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Id`

- <span id="id-debug-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default for Id`

- <span id="id-default"></span>`fn default() -> Id` — [`Id`](#id)

##### `impl Display for Id`

- <span id="id-display-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for Id`

##### `impl<T> From for Id`

- <span id="id-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Id`

- <span id="id-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Index for Command`

- <span id="command-index-type-output"></span>`type Output = Arg`

- <span id="command-index"></span>`fn index(&self, key: &Id) -> &<Self as >::Output` — [`Id`](#id)

##### `impl<U> Into for Id`

- <span id="id-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for Str`

- <span id="str-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<Id>` — [`Resettable`](../../builder/resettable/index.md#resettable), [`Id`](#id)

##### `impl Ord for Id`

- <span id="id-ord-cmp"></span>`fn cmp(&self, other: &Id) -> cmp::Ordering` — [`Id`](#id)

##### `impl PartialEq for Id`

- <span id="id-partialeq-eq"></span>`fn eq(&self, other: &Id) -> bool` — [`Id`](#id)

##### `impl PartialOrd for Id`

- <span id="id-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Id) -> option::Option<cmp::Ordering>` — [`Id`](#id)

##### `impl StructuralPartialEq for Id`

##### `impl ToOwned for Id`

- <span id="id-toowned-type-owned"></span>`type Owned = T`

- <span id="id-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="id-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Id`

- <span id="id-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Id`

- <span id="id-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="id-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Id`

- <span id="id-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="id-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

