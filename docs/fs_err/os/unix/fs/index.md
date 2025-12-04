*[fs_err](../../../index.md) / [os](../../index.md) / [unix](../index.md) / [fs](index.md)*

---

# Module `fs`

Unix-specific extensions to wrappers in `fs_err` for `std::fs` types.

## Traits

### `FileExt`

```rust
trait FileExt: crate::Sealed { ... }
```

Wrapper for [`std::os::unix::fs::FileExt`](https://doc.rust-lang.org/std/os/unix/fs/trait.FileExt.html).

The std traits might be extended in the future (See issue [#49961](https://github.com/rust-lang/rust/issues/49961#issuecomment-382751777)).
This trait is sealed and can not be implemented by other crates.

#### Required Methods

- `fn read_at(self: &Self, buf: &mut [u8], offset: u64) -> io::Result<usize>`

  Wrapper for [`FileExt::read_at`](https://doc.rust-lang.org/std/os/unix/fs/trait.FileExt.html#tymethod.read_at)

- `fn write_at(self: &Self, buf: &[u8], offset: u64) -> io::Result<usize>`

  Wrapper for [`FileExt::write_at`](https://doc.rust-lang.org/std/os/unix/fs/trait.FileExt.html#tymethod.write_at)

### `OpenOptionsExt`

```rust
trait OpenOptionsExt: crate::Sealed { ... }
```

Wrapper for [`std::os::unix::fs::OpenOptionsExt`](https://doc.rust-lang.org/std/os/unix/fs/trait.OpenOptionsExt.html)

The std traits might be extended in the future (See issue [#49961](https://github.com/rust-lang/rust/issues/49961#issuecomment-382751777)).
This trait is sealed and can not be implemented by other crates.

#### Required Methods

- `fn mode(self: &mut Self, mode: u32) -> &mut Self`

  Wrapper for [`OpenOptionsExt::mode`](https://doc.rust-lang.org/std/os/unix/fs/trait.OpenOptionsExt.html#tymethod.mode)

- `fn custom_flags(self: &mut Self, flags: i32) -> &mut Self`

  Wrapper for [`OpenOptionsExt::custom_flags`](https://doc.rust-lang.org/std/os/unix/fs/trait.OpenOptionsExt.html#tymethod.custom_flags)

## Functions

### `symlink`

```rust
fn symlink<P: AsRef<std::path::Path>, Q: AsRef<std::path::Path>>(src: P, dst: Q) -> io::Result<()>
```

Creates a new symbolic link on the filesystem.

Wrapper for [`std::os::unix::fs::symlink`](https://doc.rust-lang.org/std/os/unix/fs/fn.symlink.html)

