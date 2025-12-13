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

*Defined in [`same-file-1.0.6/src/unix.rs:9-16`](../../../.source_1765633015/same-file-1.0.6/src/unix.rs#L9-L16)*

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

##### `impl Any for Handle`

- <span id="handle-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Handle`

- <span id="handle-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Handle`

- <span id="handle-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Handle`

- <span id="handle-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for Handle`

- <span id="handle-drop"></span>`fn drop(&mut self)`

##### `impl Eq for Handle`

##### `impl<T> From for Handle`

- <span id="handle-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Handle`

- <span id="handle-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<U> Into for Handle`

- <span id="handle-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Handle`

- <span id="handle-partialeq-eq"></span>`fn eq(&self, other: &Handle) -> bool` — [`Handle`](#handle)

##### `impl<U> TryFrom for Handle`

- <span id="handle-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="handle-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Handle`

- <span id="handle-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="handle-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

