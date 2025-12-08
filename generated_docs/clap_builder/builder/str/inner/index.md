*[clap_builder](../../../index.md) / [builder](../../index.md) / [str](../index.md) / [inner](index.md)*

---

# Module `inner`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Inner`](#inner) | struct |  |

## Structs

### `Inner`

```rust
struct Inner(&'static str);
```

#### Implementations

- <span id="inner-from-static-ref"></span>`fn from_static_ref(name: &'static str) -> Self`

- <span id="inner-as-str"></span>`fn as_str(&self) -> &str`

- <span id="inner-into-string"></span>`fn into_string(self) -> String`

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

