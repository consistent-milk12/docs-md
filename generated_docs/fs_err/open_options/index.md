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

- `fn new() -> Self`

- `fn read(self: &mut Self, read: bool) -> &mut Self`

- `fn write(self: &mut Self, write: bool) -> &mut Self`

- `fn append(self: &mut Self, append: bool) -> &mut Self`

- `fn truncate(self: &mut Self, truncate: bool) -> &mut Self`

- `fn create(self: &mut Self, create: bool) -> &mut Self`

- `fn create_new(self: &mut Self, create_new: bool) -> &mut Self`

- `fn open<P>(self: &Self, path: P) -> io::Result<crate::File>` — [`File`](../index.md)

#### Trait Implementations

##### `impl Clone for OpenOptions`

- `fn clone(self: &Self) -> OpenOptions` — [`OpenOptions`](../index.md)

##### `impl Debug for OpenOptions`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl OpenOptionsExt for crate::OpenOptions`

- `fn mode(self: &mut Self, mode: u32) -> &mut Self`

- `fn custom_flags(self: &mut Self, flags: i32) -> &mut Self`

##### `impl Sealed for crate::OpenOptions`

