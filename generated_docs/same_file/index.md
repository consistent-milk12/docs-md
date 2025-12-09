# Crate `same_file`

This crate provides a safe and simple **cross platform** way to determine
whether two file paths refer to the same file or directory.

Most uses of this crate should be limited to the top-level [`is_same_file`](#is-same-file)
function, which takes two file paths and returns true if they refer to the
same file or directory:

```rust,no_run
use std::error::Error;
use same_file::is_same_file;

fn try_main() -> Result<(), Box<Error>> {
assert!(is_same_file("/bin/sh", "/usr/bin/sh")?);
   Ok(())
}

fn main() {
   try_main().unwrap();
}
```

Additionally, this crate provides a [`Handle`](#handle) type that permits a more efficient
equality check depending on your access pattern. For example, if one wanted to
check whether any path in a list of paths corresponded to the process' stdout
handle, then one could build a handle once for stdout. The equality check for
each file in the list then only requires one stat call instead of two. The code
might look like this:

```rust,no_run
use std::error::Error;
use same_file::Handle;

fn try_main() -> Result<(), Box<Error>> {
let candidates = &[
    "examples/is_same_file.rs",
    "examples/is_stderr.rs",
    "examples/stderr",
];
let stdout_handle = Handle::stdout()?;
for candidate in candidates {
    let handle = Handle::from_path(candidate)?;
    if stdout_handle == handle {
        println!("{:?} is stdout!", candidate);
    } else {
        println!("{:?} is NOT stdout!", candidate);
    }
}
   Ok(())
}

fn main() {
    try_main().unwrap();
}
```

See `examples/is_stderr.rs` for a runnable example and compare the output of:

- `cargo run --example is_stderr 2> examples/stderr` and
- `cargo run --example is_stderr`.




## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`unix`](#unix) | mod |  |
| [`Handle`](#handle) | struct | A handle to a file that can be tested for equality with other handles. |
| [`is_same_file`](#is_same_file) | fn | Returns true if the two file paths may correspond to the same file. |

## Modules

- [`unix`](unix/index.md)

## Structs

### `Handle`

```rust
struct Handle(imp::Handle);
```

A handle to a file that can be tested for equality with other handles.

If two files are the same, then any two handles of those files will compare
equal. If two files are not the same, then any two handles of those files
will compare not-equal.

A handle consumes an open file resource as long as it exists.

Equality is determined by comparing inode numbers on Unix and a combination
of identifier, volume serial, and file size on Windows. Note that it's
possible for comparing two handles to produce a false positive on some
platforms. Namely, two handles can compare equal even if the two handles
*don't* point to the same file. Check the [source] for specific
implementation details.


#### Implementations

- <span id="handle-from-path"></span>`fn from_path<P: AsRef<Path>>(p: P) -> io::Result<Handle>` — [`Handle`](#handle)

- <span id="handle-from-file"></span>`fn from_file(file: File) -> io::Result<Handle>` — [`Handle`](#handle)

- <span id="handle-stdin"></span>`fn stdin() -> io::Result<Handle>` — [`Handle`](#handle)

- <span id="handle-stdout"></span>`fn stdout() -> io::Result<Handle>` — [`Handle`](#handle)

- <span id="handle-stderr"></span>`fn stderr() -> io::Result<Handle>` — [`Handle`](#handle)

- <span id="handle-as-file"></span>`fn as_file(&self) -> &File`

- <span id="handle-as-file-mut"></span>`fn as_file_mut(&mut self) -> &mut File`

- <span id="handle-dev"></span>`fn dev(&self) -> u64`

- <span id="handle-ino"></span>`fn ino(&self) -> u64`

#### Trait Implementations

##### `impl AsRawFd for crate::Handle`

- <span id="cratehandle-as-raw-fd"></span>`fn as_raw_fd(&self) -> RawFd`

##### `impl Debug for Handle`

- <span id="handle-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Handle`

##### `impl Hash for Handle`

- <span id="handle-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl IntoRawFd for crate::Handle`

- <span id="cratehandle-into-raw-fd"></span>`fn into_raw_fd(self) -> RawFd`

##### `impl PartialEq for Handle`

- <span id="handle-eq"></span>`fn eq(&self, other: &Handle) -> bool` — [`Handle`](#handle)

##### `impl StructuralPartialEq for Handle`

## Functions

### `is_same_file`

```rust
fn is_same_file<P, Q>(path1: P, path2: Q) -> io::Result<bool>
where
    P: AsRef<std::path::Path>,
    Q: AsRef<std::path::Path>
```

Returns true if the two file paths may correspond to the same file.

Note that it's possible for this to produce a false positive on some
platforms. Namely, this can return true even if the two file paths *don't*
resolve to the same file.
# Errors
This function will return an `io::Error` if any of the two paths cannot
be opened. The most common reasons for this are: the path does not exist,
or there were not enough permissions.

# Example

```rust,no_run
use same_file::is_same_file;

assert!(is_same_file("./foo", "././foo").unwrap_or(false));
```

