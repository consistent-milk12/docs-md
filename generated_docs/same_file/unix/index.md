*[same_file](../index.md) / [unix](index.md)*

---

# Module `unix`

## Structs

### `Handle`

```rust
struct Handle {
    file: Option<std::fs::File>,
    is_std: bool,
    dev: u64,
    ino: u64,
}
```

#### Implementations

- `fn from_path<P: AsRef<Path>>(p: P) -> io::Result<Handle>` — [`Handle`](#handle)

- `fn from_file(file: File) -> io::Result<Handle>` — [`Handle`](#handle)

- `fn from_std(file: File) -> io::Result<Handle>` — [`Handle`](#handle)

- `fn stdin() -> io::Result<Handle>` — [`Handle`](#handle)

- `fn stdout() -> io::Result<Handle>` — [`Handle`](#handle)

- `fn stderr() -> io::Result<Handle>` — [`Handle`](#handle)

- `fn as_file(self: &Self) -> &File`

- `fn as_file_mut(self: &mut Self) -> &mut File`

- `fn dev(self: &Self) -> u64`

- `fn ino(self: &Self) -> u64`

#### Trait Implementations

##### `impl Debug for Handle`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Drop for Handle`

- `fn drop(self: &mut Self)`

##### `impl Eq for Handle`

##### `impl Hash for Handle`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl PartialEq for Handle`

- `fn eq(self: &Self, other: &Handle) -> bool` — [`Handle`](#handle)

