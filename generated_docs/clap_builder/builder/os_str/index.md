*[clap_builder](../../index.md) / [builder](../index.md) / [os_str](index.md)*

---

# Module `os_str`

## Modules

- [`inner`](inner/index.md) - 

## Structs

### `OsStr`

```rust
struct OsStr {
    name: inner::Inner,
}
```

A UTF-8-encoded fixed string

<div class="warning">

**NOTE:** To support dynamic values (i.e. `OsString`), enable the `string`
feature

</div>

#### Implementations

- `fn from_static_ref(name: &'static std::ffi::OsStr) -> Self`

- `fn as_os_str(self: &Self) -> &std::ffi::OsStr`

- `fn to_os_string(self: &Self) -> std::ffi::OsString`

#### Trait Implementations

##### `impl AsRef for OsStr`

- `fn as_ref(self: &Self) -> &std::path::Path`

##### `impl Clone for OsStr`

- `fn clone(self: &Self) -> OsStr` — [`OsStr`](../index.md)

##### `impl Debug for OsStr`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default for OsStr`

- `fn default() -> OsStr` — [`OsStr`](../index.md)

##### `impl Deref for OsStr`

- `type Target = OsStr`

- `fn deref(self: &Self) -> &std::ffi::OsStr`

##### `impl Eq for OsStr`

##### `impl Hash for OsStr`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<I> IntoResettable for OsStr`

- `fn into_resettable(self: Self) -> Resettable<OsStr>` — [`Resettable`](../index.md), [`OsStr`](../index.md)

##### `impl Ord for OsStr`

- `fn cmp(self: &Self, other: &OsStr) -> $crate::cmp::Ordering` — [`OsStr`](../index.md)

##### `impl PartialEq for OsStr`

- `fn eq(self: &Self, other: &std::ffi::OsString) -> bool`

##### `impl PartialOrd for OsStr`

- `fn partial_cmp(self: &Self, other: &OsStr) -> $crate::option::Option<$crate::cmp::Ordering>` — [`OsStr`](../index.md)

##### `impl<P, T> Receiver for OsStr`

- `type Target = T`

##### `impl StructuralPartialEq for OsStr`

