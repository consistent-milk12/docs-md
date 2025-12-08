*[clap_builder](../../../index.md) / [builder](../../index.md) / [os_str](../index.md) / [inner](index.md)*

---

# Module `inner`

## Structs

### `Inner`

```rust
struct Inner(&'static std::ffi::OsStr);
```

#### Implementations

- `fn from_static_ref(name: &'static std::ffi::OsStr) -> Self`

- `fn as_os_str(self: &Self) -> &std::ffi::OsStr`

- `fn into_os_string(self: Self) -> std::ffi::OsString`

#### Trait Implementations

##### `impl Clone for Inner`

- `fn clone(self: &Self) -> Inner` — [`Inner`](#inner)

##### `impl Default for inner::Inner`

- `fn default() -> Self`

##### `impl Eq for inner::Inner`

##### `impl Hash for inner::Inner`

- `fn hash<H: std::hash::Hasher>(self: &Self, state: &mut H)`

##### `impl Ord for inner::Inner`

- `fn cmp(self: &Self, other: &Inner) -> std::cmp::Ordering` — [`Inner`](#inner)

##### `impl PartialEq for inner::Inner`

- `fn eq(self: &Self, other: &Inner) -> bool` — [`Inner`](#inner)

##### `impl PartialOrd for inner::Inner`

- `fn partial_cmp(self: &Self, other: &Self) -> Option<std::cmp::Ordering>`

