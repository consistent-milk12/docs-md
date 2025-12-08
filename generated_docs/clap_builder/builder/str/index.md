*[clap_builder](../../index.md) / [builder](../index.md) / [str](index.md)*

---

# Module `str`

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

- `fn from_static_ref(name: &'static str) -> Self`

- `fn into_inner(self: Self) -> Inner` — [`Inner`](inner/index.md)

- `fn as_str(self: &Self) -> &str`

#### Trait Implementations

##### `impl AsRef for Str`

- `fn as_ref(self: &Self) -> &[u8]`

##### `impl Clone for Str`

- `fn clone(self: &Self) -> Str` — [`Str`](../index.md)

##### `impl Debug for Str`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default for Str`

- `fn default() -> Str` — [`Str`](../index.md)

##### `impl Deref for Str`

- `type Target = str`

- `fn deref(self: &Self) -> &str`

##### `impl Display for Str`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for Str`

##### `impl Hash for Str`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<I> IntoResettable for Str`

- `fn into_resettable(self: Self) -> Resettable<Str>` — [`Resettable`](../index.md), [`Str`](../index.md)

##### `impl Ord for Str`

- `fn cmp(self: &Self, other: &Str) -> $crate::cmp::Ordering` — [`Str`](../index.md)

##### `impl PartialEq for Str`

- `fn eq(self: &Self, other: &str) -> bool`

##### `impl PartialOrd for Str`

- `fn partial_cmp(self: &Self, other: &Str) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Str`](../index.md)

##### `impl<P, T> Receiver for Str`

- `type Target = T`

##### `impl StructuralPartialEq for Str`

##### `impl<T> ToString for Str`

- `fn to_string(self: &Self) -> String`

