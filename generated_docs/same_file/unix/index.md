*[same_file](../index.md) / [unix](index.md)*

---

# Module `unix`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Handle`](#handle) | struct |  |

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

*Defined in [`same-file-1.0.6/src/unix.rs:9-16`](../../../.source_1765210505/same-file-1.0.6/src/unix.rs#L9-L16)*

#### Implementations

- <span id="handle-from-path"></span>`fn from_path<P: AsRef<Path>>(p: P) -> io::Result<Handle>` — [`Handle`](#handle)

- <span id="handle-from-file"></span>`fn from_file(file: File) -> io::Result<Handle>` — [`Handle`](#handle)

- <span id="handle-from-std"></span>`fn from_std(file: File) -> io::Result<Handle>` — [`Handle`](#handle)

- <span id="handle-stdin"></span>`fn stdin() -> io::Result<Handle>` — [`Handle`](#handle)

- <span id="handle-stdout"></span>`fn stdout() -> io::Result<Handle>` — [`Handle`](#handle)

- <span id="handle-stderr"></span>`fn stderr() -> io::Result<Handle>` — [`Handle`](#handle)

- <span id="handle-as-file"></span>`fn as_file(&self) -> &File`

- <span id="handle-as-file-mut"></span>`fn as_file_mut(&mut self) -> &mut File`

- <span id="handle-dev"></span>`fn dev(&self) -> u64`

- <span id="handle-ino"></span>`fn ino(&self) -> u64`

#### Trait Implementations

##### `impl Debug for Handle`

- <span id="handle-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for Handle`

- <span id="handle-drop"></span>`fn drop(&mut self)`

##### `impl Eq for Handle`

##### `impl Hash for Handle`

- <span id="handle-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl PartialEq for Handle`

- <span id="handle-eq"></span>`fn eq(&self, other: &Handle) -> bool` — [`Handle`](#handle)

