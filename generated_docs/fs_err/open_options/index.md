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

*Defined in [`fs-err-3.2.0/src/open_options.rs:7`](../../../.source_1765633015/fs-err-3.2.0/src/open_options.rs#L7)*

Wrapper around [`std::fs::OpenOptions`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html)

#### Implementations

- <span id="openoptions-new"></span>`fn new() -> Self`

  Creates a blank new set of options ready for configuration.

  

  Wrapper for [`std::fs::OpenOptions::new`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.new)

- <span id="openoptions-read"></span>`fn read(&mut self, read: bool) -> &mut Self`

  Sets the option for read access.

  

  Wrapper for [`std::fs::OpenOptions::read`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.read)

- <span id="openoptions-write"></span>`fn write(&mut self, write: bool) -> &mut Self`

  Sets the option for write access.

  

  Wrapper for [`std::fs::OpenOptions::write`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.write)

- <span id="openoptions-append"></span>`fn append(&mut self, append: bool) -> &mut Self`

  Sets the option for the append mode.

  

  Wrapper for [`std::fs::OpenOptions::append`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.append)

- <span id="openoptions-truncate"></span>`fn truncate(&mut self, truncate: bool) -> &mut Self`

  Sets the option for truncating a previous file.

  

  Wrapper for [`std::fs::OpenOptions::truncate`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.truncate)

- <span id="openoptions-create"></span>`fn create(&mut self, create: bool) -> &mut Self`

  Sets the option to create a new file, or open it if it already exists.

  

  Wrapper for [`std::fs::OpenOptions::create`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.create)

- <span id="openoptions-create-new"></span>`fn create_new(&mut self, create_new: bool) -> &mut Self`

  Sets the option to create a new file, failing if it already exists.

  

  Wrapper for [`std::fs::OpenOptions::create_new`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.create_new)

- <span id="openoptions-open"></span>`fn open<P>(&self, path: P) -> io::Result<crate::File>` — [`File`](../index.md#file)

  Opens a file at `path` with the options specified by `self`.

  

  Wrapper for [`std::fs::OpenOptions::open`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.open)

#### Trait Implementations

##### `impl Any for OpenOptions`

- <span id="openoptions-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OpenOptions`

- <span id="openoptions-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OpenOptions`

- <span id="openoptions-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for OpenOptions`

- <span id="openoptions-clone"></span>`fn clone(&self) -> OpenOptions` — [`OpenOptions`](#openoptions)

##### `impl CloneToUninit for OpenOptions`

- <span id="openoptions-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for OpenOptions`

- <span id="openoptions-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for OpenOptions`

- <span id="openoptions-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for OpenOptions`

- <span id="openoptions-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OpenOptionsExt for crate::OpenOptions`

- <span id="crateopenoptions-openoptionsext-mode"></span>`fn mode(&mut self, mode: u32) -> &mut Self`

- <span id="crateopenoptions-openoptionsext-custom-flags"></span>`fn custom_flags(&mut self, flags: i32) -> &mut Self`

##### `impl Sealed for crate::OpenOptions`

##### `impl ToOwned for OpenOptions`

- <span id="openoptions-toowned-type-owned"></span>`type Owned = T`

- <span id="openoptions-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="openoptions-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for OpenOptions`

- <span id="openoptions-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="openoptions-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OpenOptions`

- <span id="openoptions-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="openoptions-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

