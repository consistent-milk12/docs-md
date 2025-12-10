*[clap_builder](../../index.md) / [builder](../index.md) / [os_str](index.md)*

---

# Module `os_str`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`inner`](#inner) | mod |  |
| [`OsStr`](#osstr) | struct | A UTF-8-encoded fixed string |

## Modules

- [`inner`](inner/index.md)

## Structs

### `OsStr`

```rust
struct OsStr {
    name: inner::Inner,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/os_str.rs:14-16`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/os_str.rs#L14-L16)*

A UTF-8-encoded fixed string

<div class="warning">

**NOTE:** To support dynamic values (i.e. `OsString`), enable the `string`
feature

</div>

#### Implementations

- <span id="osstr-from-static-ref"></span>`fn from_static_ref(name: &'static std::ffi::OsStr) -> Self`

- <span id="osstr-as-os-str"></span>`fn as_os_str(&self) -> &std::ffi::OsStr`

- <span id="osstr-to-os-string"></span>`fn to_os_string(&self) -> std::ffi::OsString`

#### Trait Implementations

##### `impl AsRef for OsStr`

- <span id="osstr-as-ref"></span>`fn as_ref(&self) -> &std::ffi::OsStr`

##### `impl Clone for OsStr`

- <span id="osstr-clone"></span>`fn clone(&self) -> OsStr` — [`OsStr`](#osstr)

##### `impl Debug for OsStr`

- <span id="osstr-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default for OsStr`

- <span id="osstr-default"></span>`fn default() -> OsStr` — [`OsStr`](#osstr)

##### `impl Deref for OsStr`

- <span id="osstr-type-target"></span>`type Target = OsStr`

- <span id="osstr-deref"></span>`fn deref(&self) -> &std::ffi::OsStr`

##### `impl Eq for OsStr`

##### `impl Hash for OsStr`

- <span id="osstr-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl IntoResettable for OsStr`

- <span id="osstr-into-resettable"></span>`fn into_resettable(self) -> Resettable<OsStr>` — [`Resettable`](../resettable/index.md#resettable), [`OsStr`](#osstr)

##### `impl Ord for OsStr`

- <span id="osstr-cmp"></span>`fn cmp(&self, other: &OsStr) -> cmp::Ordering` — [`OsStr`](#osstr)

##### `impl PartialEq for OsStr`

- <span id="osstr-eq"></span>`fn eq(&self, other: &OsStr) -> bool` — [`OsStr`](#osstr)

##### `impl PartialOrd for OsStr`

- <span id="osstr-partial-cmp"></span>`fn partial_cmp(&self, other: &OsStr) -> option::Option<cmp::Ordering>` — [`OsStr`](#osstr)

##### `impl Receiver for OsStr`

- <span id="osstr-type-target"></span>`type Target = T`

##### `impl StructuralPartialEq for OsStr`

