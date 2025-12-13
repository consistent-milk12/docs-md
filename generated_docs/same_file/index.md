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

Additionally, this crate provides a [`Handle`](unix/index.md) type that permits a more efficient
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
| [`is_same_file`](#is-same-file) | fn | Returns true if the two file paths may correspond to the same file. |

## Modules

- [`unix`](unix/index.md)

## Structs

### `Handle`

```rust
struct Handle(imp::Handle);
```

*Defined in [`same-file-1.0.6/src/lib.rs:109`](../../.source_1765521767/same-file-1.0.6/src/lib.rs#L109)*

A handle to a file that can be tested for equality with other handles.

If two files are the same, then any two handles of those files will compare
equal. If two files are not the same, then any two handles of those files
will compare not-equal.

A handle consumes an open file resource as long as it exists.

Equality is determined by comparing inode numbers on Unix and a combination
of identifier, volume serial, and file size on Windows. Note that it's
possible for comparing two handles to produce a false positive on some
platforms. Namely, two handles can compare equal even if the two handles
*don't* point to the same file. Check the [`source`](../cargo_docs_md/source/index.md) for specific
implementation details.


#### Implementations

- <span id="handle-from-path"></span>`fn from_path<P: AsRef<Path>>(p: P) -> io::Result<Handle>` — [`Handle`](#handle)

  Construct a handle from a path.

  

  Note that the underlying [`File`](#file) is opened in read-only mode on all

  platforms.

  

  # Errors

  This method will return an `io::Error` if the path cannot

  be opened, or the file's metadata cannot be obtained.

  The most common reasons for this are: the path does not

  exist, or there were not enough permissions.

  

  # Examples

  Check that two paths are not the same file:

  

  ```rust,no_run

  use std::error::Error;

  use same_file::Handle;

  

  fn try_main() -> Result<(), Box<Error>> {

  let source = Handle::from_path("./source")?;

  let target = Handle::from_path("./target")?;

  assert_ne!(source, target, "The files are the same.");

  Ok(())

  }

  

  fn main() {

      try_main().unwrap();

  }

  ```

- <span id="handle-from-file"></span>`fn from_file(file: File) -> io::Result<Handle>` — [`Handle`](#handle)

  Construct a handle from a file.

  

  # Errors

  This method will return an `io::Error` if the metadata for

  the given [`File`](#file) cannot be obtained.

  

  

  # Examples

  Check that two files are not in fact the same file:

  

  ```rust,no_run

  use std::error::Error;

  use std::fs::File;

  use same_file::Handle;

  

  fn try_main() -> Result<(), Box<Error>> {

  let source = File::open("./source")?;

  let target = File::open("./target")?;

  

  assert_ne!(

      Handle::from_file(source)?,

      Handle::from_file(target)?,

      "The files are the same."

  );

      Ok(())

  }

  

  fn main() {

      try_main().unwrap();

  }

  ```

- <span id="handle-stdin"></span>`fn stdin() -> io::Result<Handle>` — [`Handle`](#handle)

  Construct a handle from stdin.

  

  # Errors

  This method will return an `io::Error` if stdin cannot

  be opened due to any I/O-related reason.

  

  # Examples

  

  ```rust

  use std::error::Error;

  use same_file::Handle;

  

  fn try_main() -> Result<(), Box<Error>> {

  let stdin = Handle::stdin()?;

  let stdout = Handle::stdout()?;

  let stderr = Handle::stderr()?;

  

  if stdin == stdout {

      println!("stdin == stdout");

  }

  if stdin == stderr {

      println!("stdin == stderr");

  }

  if stdout == stderr {

      println!("stdout == stderr");

  }

  

      Ok(())

  }

  

  fn main() {

      try_main().unwrap();

  }

  ```

  

  The output differs depending on the platform.

  

  On Linux:

  

  ```text

  $ ./example

  stdin == stdout

  stdin == stderr

  stdout == stderr

  $ ./example > result

  $ cat result

  stdin == stderr

  $ ./example > result 2>&1

  $ cat result

  stdout == stderr

  ```

  

  Windows:

  

  ```text

  > example

  > example > result 2>&1

  > type result

  stdout == stderr

  ```

- <span id="handle-stdout"></span>`fn stdout() -> io::Result<Handle>` — [`Handle`](#handle)

  Construct a handle from stdout.

  

  # Errors

  This method will return an `io::Error` if stdout cannot

  be opened due to any I/O-related reason.

  

  # Examples

  See the example for `stdin()`.

- <span id="handle-stderr"></span>`fn stderr() -> io::Result<Handle>` — [`Handle`](#handle)

  Construct a handle from stderr.

  

  # Errors

  This method will return an `io::Error` if stderr cannot

  be opened due to any I/O-related reason.

  

  # Examples

  See the example for `stdin()`.

- <span id="handle-as-file"></span>`fn as_file(&self) -> &File`

  Return a reference to the underlying file.

  

  # Examples

  Ensure that the target file is not the same as the source one,

  and copy the data to it:

  

  ```rust,no_run

  use std::error::Error;

  use std::io::prelude::*;

  use std::io::Write;

  use std::fs::File;

  use same_file::Handle;

  

  fn try_main() -> Result<(), Box<Error>> {

  let source = File::open("source")?;

  let target = File::create("target")?;

  

  let source_handle = Handle::from_file(source)?;

  let mut target_handle = Handle::from_file(target)?;

  assert_ne!(source_handle, target_handle, "The files are the same.");

  

  let mut source = source_handle.as_file();

  let target = target_handle.as_file_mut();

  

  let mut buffer = Vec::new();

  // data copy is simplified for the purposes of the example

  source.read_to_end(&mut buffer)?;

  target.write_all(&buffer)?;

  

     Ok(())

  }

  

  fn main() {

     try_main().unwrap();

  }

  ```

- <span id="handle-as-file-mut"></span>`fn as_file_mut(&mut self) -> &mut File`

  Return a mutable reference to the underlying file.

  

  # Examples

  See the example for `as_file()`.

- <span id="handle-dev"></span>`fn dev(&self) -> u64`

  Return the underlying device number of this handle.

  

  Note that this only works on unix platforms.

- <span id="handle-ino"></span>`fn ino(&self) -> u64`

  Return the underlying inode number of this handle.

  

  Note that this only works on unix platforms.

#### Trait Implementations

##### `impl Any for Handle`

- <span id="handle-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl AsRawFd for crate::Handle`

- <span id="cratehandle-asrawfd-as-raw-fd"></span>`fn as_raw_fd(&self) -> RawFd`

##### `impl<T> Borrow for Handle`

- <span id="handle-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Handle`

- <span id="handle-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Handle`

- <span id="handle-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Handle`

##### `impl<T> From for Handle`

- <span id="handle-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Handle`

- <span id="handle-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Handle`

- <span id="handle-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoRawFd for crate::Handle`

- <span id="cratehandle-intorawfd-into-raw-fd"></span>`fn into_raw_fd(self) -> RawFd`

##### `impl PartialEq for Handle`

- <span id="handle-partialeq-eq"></span>`fn eq(&self, other: &Handle) -> bool` — [`Handle`](#handle)

##### `impl StructuralPartialEq for Handle`

##### `impl<U> TryFrom for Handle`

- <span id="handle-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="handle-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Handle`

- <span id="handle-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="handle-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `is_same_file`

```rust
fn is_same_file<P, Q>(path1: P, path2: Q) -> io::Result<bool>
where
    P: AsRef<std::path::Path>,
    Q: AsRef<std::path::Path>
```

*Defined in [`same-file-1.0.6/src/lib.rs:370-376`](../../.source_1765521767/same-file-1.0.6/src/lib.rs#L370-L376)*

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

