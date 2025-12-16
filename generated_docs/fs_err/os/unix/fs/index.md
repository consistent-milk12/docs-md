*[fs_err](../../../index.md) / [os](../../index.md) / [unix](../index.md) / [fs](index.md)*

---

# Module `fs`

Unix-specific extensions to wrappers in `fs_err` for `std::fs` types.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FileExt`](#fileext) | trait | Wrapper for [`std::os::unix::fs::FileExt`](https://doc.rust-lang.org/std/os/unix/fs/trait.FileExt.html). |
| [`OpenOptionsExt`](#openoptionsext) | trait | Wrapper for [`std::os::unix::fs::OpenOptionsExt`](https://doc.rust-lang.org/std/os/unix/fs/trait.OpenOptionsExt.html) |
| [`symlink`](#symlink) | fn | Creates a new symbolic link on the filesystem. |

## Traits

### `FileExt`

```rust
trait FileExt: crate::Sealed { ... }
```

*Defined in [`fs-err-3.2.1/src/os/unix.rs:26-31`](../../../../../.source_1765900590/fs-err-3.2.1/src/os/unix.rs#L26-L31)*

Wrapper for [`std::os::unix::fs::FileExt`](https://doc.rust-lang.org/std/os/unix/fs/trait.FileExt.html).

The std traits might be extended in the future (See issue [#49961](https://github.com/rust-lang/rust/issues/49961#issuecomment-382751777)).
This trait is sealed and can not be implemented by other crates.

#### Required Methods

- `fn FileExt::read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize>`

  Wrapper for [`FileExt::read_at`](https://doc.rust-lang.org/std/os/unix/fs/trait.FileExt.html#tymethod.read_at)

- `fn FileExt::write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize>`

  Wrapper for [`FileExt::write_at`](https://doc.rust-lang.org/std/os/unix/fs/trait.FileExt.html#tymethod.write_at)

#### Implementors

- [`File`](../../../index.md#file)

### `OpenOptionsExt`

```rust
trait OpenOptionsExt: crate::Sealed { ... }
```

*Defined in [`fs-err-3.2.1/src/os/unix.rs:37-42`](../../../../../.source_1765900590/fs-err-3.2.1/src/os/unix.rs#L37-L42)*

Wrapper for [`std::os::unix::fs::OpenOptionsExt`](https://doc.rust-lang.org/std/os/unix/fs/trait.OpenOptionsExt.html)

The std traits might be extended in the future (See issue [#49961](https://github.com/rust-lang/rust/issues/49961#issuecomment-382751777)).
This trait is sealed and can not be implemented by other crates.

#### Required Methods

- `fn OpenOptionsExt::mode(&mut self, mode: u32) -> &mut Self`

  Wrapper for [`OpenOptionsExt::mode`](https://doc.rust-lang.org/std/os/unix/fs/trait.OpenOptionsExt.html#tymethod.mode)

- `fn OpenOptionsExt::custom_flags(&mut self, flags: i32) -> &mut Self`

  Wrapper for [`OpenOptionsExt::custom_flags`](https://doc.rust-lang.org/std/os/unix/fs/trait.OpenOptionsExt.html#tymethod.custom_flags)

#### Implementors

- [`OpenOptions`](../../../open_options/index.md#openoptions)

## Functions

### `symlink`

```rust
fn symlink<P: AsRef<std::path::Path>, Q: AsRef<std::path::Path>>(original: P, link: Q) -> io::Result<()>
```

*Defined in [`fs-err-3.2.1/src/os/unix.rs:14-20`](../../../../../.source_1765900590/fs-err-3.2.1/src/os/unix.rs#L14-L20)*

Creates a new symbolic link on the filesystem.

The `link` path will be a symbolic link pointing to the `original` path.

Wrapper for [`std::os::unix::fs::symlink`](https://doc.rust-lang.org/std/os/unix/fs/fn.symlink.html)

