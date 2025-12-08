*[fs_err](../index.md) / [open_options](index.md)*

---

# Module `open_options`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`unix`](#unix) | mod |  |
| [`OpenOptions`](#openoptions) | struct | Wrapper around [`std::fs::OpenOptions`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html) |

## Modules

- [`unix`](unix/index.md) - 

## Structs

### `OpenOptions`

```rust
struct OpenOptions(fs::OpenOptions);
```

Wrapper around [`std::fs::OpenOptions`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html)

#### Implementations

- <span id="openoptions-from-options"></span>`fn from_options(options: fs::OpenOptions) -> Self`

- <span id="openoptions-options"></span>`fn options(&self) -> &fs::OpenOptions`

- <span id="openoptions-options-mut"></span>`fn options_mut(&mut self) -> &mut fs::OpenOptions`

#### Trait Implementations

##### `impl Clone for OpenOptions`

- <span id="openoptions-clone"></span>`fn clone(&self) -> OpenOptions` — [`OpenOptions`](../index.md)

##### `impl Debug for OpenOptions`

- <span id="openoptions-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OpenOptionsExt for crate::OpenOptions`

- <span id="crateopenoptions-mode"></span>`fn mode(&mut self, mode: u32) -> &mut Self`

- <span id="crateopenoptions-custom-flags"></span>`fn custom_flags(&mut self, flags: i32) -> &mut Self`

##### `impl Sealed for crate::OpenOptions`

