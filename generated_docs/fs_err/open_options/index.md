*[fs_err](../index.md) / [open_options](index.md)*

---

# Module `open_options`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`unix`](#unix) | mod |  |
| [`OpenOptions`](#openoptions) | struct | Wrapper around [`std::fs::OpenOptions`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html) |

## Modules

- [`unix`](unix/index.md)

## Structs

### `OpenOptions`

```rust
struct OpenOptions(fs::OpenOptions);
```

*Defined in [`fs-err-3.2.0/src/open_options.rs:7`](../../../.source_1765521767/fs-err-3.2.0/src/open_options.rs#L7)*

Wrapper around [`std::fs::OpenOptions`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html)

#### Implementations

- <span id="openoptions-new"></span>`fn new() -> Self`

- <span id="openoptions-read"></span>`fn read(&mut self, read: bool) -> &mut Self`

- <span id="openoptions-write"></span>`fn write(&mut self, write: bool) -> &mut Self`

- <span id="openoptions-append"></span>`fn append(&mut self, append: bool) -> &mut Self`

- <span id="openoptions-truncate"></span>`fn truncate(&mut self, truncate: bool) -> &mut Self`

- <span id="openoptions-create"></span>`fn create(&mut self, create: bool) -> &mut Self`

- <span id="openoptions-create-new"></span>`fn create_new(&mut self, create_new: bool) -> &mut Self`

- <span id="openoptions-open"></span>`fn open<P>(&self, path: P) -> io::Result<crate::File>` — [`File`](../index.md#file)

#### Trait Implementations

##### `impl Clone for OpenOptions`

- <span id="openoptions-clone"></span>`fn clone(&self) -> OpenOptions` — [`OpenOptions`](#openoptions)

##### `impl Debug for OpenOptions`

- <span id="openoptions-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OpenOptionsExt for crate::OpenOptions`

- <span id="crateopenoptions-mode"></span>`fn mode(&mut self, mode: u32) -> &mut Self`

- <span id="crateopenoptions-custom-flags"></span>`fn custom_flags(&mut self, flags: i32) -> &mut Self`

##### `impl Sealed for crate::OpenOptions`

