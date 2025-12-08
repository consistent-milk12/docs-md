*[fs_err](../index.md) / [open_options](index.md)*

---

# Module `open_options`

## Modules

- [`unix`](unix/index.md) - 

## Structs

### `OpenOptions`

```rust
struct OpenOptions(fs::OpenOptions);
```

Wrapper around [`std::fs::OpenOptions`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html)

#### Implementations

- `fn from_options(options: fs::OpenOptions) -> Self`

- `fn options(self: &Self) -> &fs::OpenOptions`

- `fn options_mut(self: &mut Self) -> &mut fs::OpenOptions`

#### Trait Implementations

##### `impl Clone for OpenOptions`

- `fn clone(self: &Self) -> OpenOptions` — [`OpenOptions`](../index.md)

##### `impl Debug for OpenOptions`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl OpenOptionsExt for crate::OpenOptions`

- `fn mode(self: &mut Self, mode: u32) -> &mut Self`

- `fn custom_flags(self: &mut Self, flags: i32) -> &mut Self`

##### `impl Sealed for crate::OpenOptions`

