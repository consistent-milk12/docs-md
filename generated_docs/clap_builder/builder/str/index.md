*[clap_builder](../../index.md) / [builder](../index.md) / [str](index.md)*

---

# Module `str`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`inner`](#inner) | mod |  |
| [`Str`](#str) | struct | A UTF-8-encoded fixed string |

## Modules

- [`inner`](inner/index.md) - 

## Structs

### `Str`

```rust
struct Str {
    name: inner::Inner,
}
```

A UTF-8-encoded fixed string

<div class="warning">

**NOTE:** To support dynamic values (i.e. `String`), enable the `string`
feature

</div>

#### Implementations

- <span id="str-from-static-ref"></span>`fn from_static_ref(name: &'static str) -> Self`

- <span id="str-into-inner"></span>`fn into_inner(self) -> Inner` — [`Inner`](inner/index.md)

- <span id="str-as-str"></span>`fn as_str(&self) -> &str`

#### Trait Implementations

##### `impl AsRef for Str`

- <span id="str-as-ref"></span>`fn as_ref(&self) -> &[u8]`

##### `impl Clone for Str`

- <span id="str-clone"></span>`fn clone(&self) -> Str` — [`Str`](../index.md)

##### `impl Debug for Str`

- <span id="str-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default for Str`

- <span id="str-default"></span>`fn default() -> Str` — [`Str`](../index.md)

##### `impl Deref for Str`

- <span id="str-target"></span>`type Target = str`

- <span id="str-deref"></span>`fn deref(&self) -> &str`

##### `impl Display for Str`

- <span id="str-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for Str`

##### `impl Hash for Str`

- <span id="str-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<I> IntoResettable for Str`

- <span id="str-into-resettable"></span>`fn into_resettable(self) -> Resettable<Str>` — [`Resettable`](../index.md), [`Str`](../index.md)

##### `impl Ord for Str`

- <span id="str-cmp"></span>`fn cmp(&self, other: &Str) -> cmp::Ordering` — [`Str`](../index.md)

##### `impl PartialEq for Str`

- <span id="str-eq"></span>`fn eq(&self, other: &String) -> bool`

##### `impl PartialOrd for Str`

- <span id="str-partial-cmp"></span>`fn partial_cmp(&self, other: &Str) -> option::Option<cmp::Ordering>` — [`Str`](../index.md)

##### `impl<P, T> Receiver for Str`

- <span id="str-target"></span>`type Target = T`

##### `impl StructuralPartialEq for Str`

##### `impl<T> ToString for Str`

- <span id="str-to-string"></span>`fn to_string(&self) -> String`

