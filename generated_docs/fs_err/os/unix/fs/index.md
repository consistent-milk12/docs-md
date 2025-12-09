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

*Defined in [`fs-err-3.2.0/src/os/unix.rs:23-28`](../../../../../.source_1765210505/fs-err-3.2.0/src/os/unix.rs#L23-L28)*

Wrapper for [`std::os::unix::fs::FileExt`](https://doc.rust-lang.org/std/os/unix/fs/trait.FileExt.html).

The std traits might be extended in the future (See issue [#49961](https://github.com/rust-lang/rust/issues/49961#issuecomment-382751777)).
This trait is sealed and can not be implemented by other crates.

#### Required Methods

- `fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize>`

  Wrapper for [`FileExt::read_at`](https://doc.rust-lang.org/std/os/unix/fs/trait.FileExt.html#tymethod.read_at)

- `fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize>`

  Wrapper for [`FileExt::write_at`](https://doc.rust-lang.org/std/os/unix/fs/trait.FileExt.html#tymethod.write_at)

#### Implementors

- [`File`](../../../index.md)

### `OpenOptionsExt`

```rust
trait OpenOptionsExt: crate::Sealed { ... }
```

*Defined in [`fs-err-3.2.0/src/os/unix.rs:34-39`](../../../../../.source_1765210505/fs-err-3.2.0/src/os/unix.rs#L34-L39)*

Wrapper for [`std::os::unix::fs::OpenOptionsExt`](https://doc.rust-lang.org/std/os/unix/fs/trait.OpenOptionsExt.html)

The std traits might be extended in the future (See issue [#49961](https://github.com/rust-lang/rust/issues/49961#issuecomment-382751777)).
This trait is sealed and can not be implemented by other crates.

#### Required Methods

- `fn mode(&mut self, mode: u32) -> &mut Self`

  Wrapper for [`OpenOptionsExt::mode`](https://doc.rust-lang.org/std/os/unix/fs/trait.OpenOptionsExt.html#tymethod.mode)

- `fn custom_flags(&mut self, flags: i32) -> &mut Self`

  Wrapper for [`OpenOptionsExt::custom_flags`](https://doc.rust-lang.org/std/os/unix/fs/trait.OpenOptionsExt.html#tymethod.custom_flags)

#### Implementors

- [`OpenOptions`](../../../open_options/index.md)

## Functions

### `symlink`

```rust
fn symlink<P: AsRef<std::path::Path>, Q: AsRef<std::path::Path>>(src: P, dst: Q) -> io::Result<()>
```

*Defined in [`fs-err-3.2.0/src/os/unix.rs:12-17`](../../../../../.source_1765210505/fs-err-3.2.0/src/os/unix.rs#L12-L17)*

Creates a new symbolic link on the filesystem.

Wrapper for [`std::os::unix::fs::symlink`](https://doc.rust-lang.org/std/os/unix/fs/fn.symlink.html)

