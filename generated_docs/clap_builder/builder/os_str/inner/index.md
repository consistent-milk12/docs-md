*[clap_builder](../../../index.md) / [builder](../../index.md) / [os_str](../index.md) / [inner](index.md)*

---

# Module `inner`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Inner`](#inner) | struct |  |

## Structs

### `Inner`

```rust
struct Inner(&'static std::ffi::OsStr);
```

*Defined in [`clap_builder-4.5.53/src/builder/os_str.rs:292`](../../../../../.source_1765521767/clap_builder-4.5.53/src/builder/os_str.rs#L292)*

#### Implementations

- <span id="inner-from-static-ref"></span>`fn from_static_ref(name: &'static std::ffi::OsStr) -> Self`

- <span id="inner-as-os-str"></span>`fn as_os_str(&self) -> &std::ffi::OsStr`

- <span id="inner-into-os-string"></span>`fn into_os_string(self) -> std::ffi::OsString`

#### Trait Implementations

##### `impl Clone for Inner`

- <span id="inner-clone"></span>`fn clone(&self) -> Inner` — [`Inner`](#inner)

##### `impl Default for inner::Inner`

- <span id="innerinner-default"></span>`fn default() -> Self`

##### `impl Eq for inner::Inner`

##### `impl Hash for inner::Inner`

- <span id="innerinner-hash"></span>`fn hash<H: std::hash::Hasher>(&self, state: &mut H)`

##### `impl Ord for inner::Inner`

- <span id="innerinner-cmp"></span>`fn cmp(&self, other: &Inner) -> std::cmp::Ordering` — [`Inner`](#inner)

##### `impl PartialEq for inner::Inner`

- <span id="innerinner-eq"></span>`fn eq(&self, other: &Inner) -> bool` — [`Inner`](#inner)

##### `impl PartialOrd for inner::Inner`

- <span id="innerinner-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>`

