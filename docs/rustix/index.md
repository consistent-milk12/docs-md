# Crate `rustix`

`rustix` provides efficient memory-safe and [I/O-safe] wrappers to
POSIX-like, Unix-like, Linux, and Winsock syscall-like APIs, with
configurable backends.

With rustix, you can write code like this:

```
# #[cfg(feature = "net")]
# fn read(sock: std::net::TcpStream, buf: &mut [u8](#u8)
) -> std::io::Result<()> {
# use rustix::net::RecvFlags;
let (nread, _received) = rustix::net::recv(&sock, buf, RecvFlags::PEEK)?;
# let _ = nread;
# Ok(())
# }
```

instead of like this:

```
# #[cfg(feature = "net")]
# fn read(sock: std::net::TcpStream, buf: &mut [u8](#u8)
) -> std::io::Result<()> {
# #[cfg(unix)]
# use std::os::unix::io::AsRawFd;
# #[cfg(target_os = "wasi")]
# use std::os::wasi::io::AsRawFd;
# #[cfg(windows)]
# use windows_sys::Win32::Networking::WinSock as libc;
# #[cfg(windows)]
# use std::os::windows::io::AsRawSocket;
# const MSG_PEEK: i32 = libc::MSG_PEEK;
let nread = unsafe {
    #[cfg(any(unix, target_os = "wasi"))]
    let raw = sock.as_raw_fd();
    #[cfg(windows)]
    let raw = sock.as_raw_socket();
    match libc::recv(
        raw as _,
        buf.as_mut_ptr().cast(),
        buf.len().try_into().unwrap_or(i32::MAX as _),
        MSG_PEEK,
    ) {
        -1 => return Err(std::io::Error::last_os_error()),
        nread => nread as usize,
    }
};
# let _ = nread;
# Ok(())
# }
```

rustix's APIs perform the following tasks:
 - Error values are translated to [`Result`](#result)s.
 - Buffers are passed as Rust slices.
 - Out-parameters are presented as return values.
 - Path arguments use [`Arg`](#arg), so they accept any string type.
 - File descriptors are passed and returned via [`AsFd`](#asfd) and [`OwnedFd`](#ownedfd)
   instead of bare integers, ensuring I/O safety.
 - Constants use `enum`s and [`bitflags`](#bitflags) types, and enable [support for
   externally defined flags].
 - Multiplexed functions (eg. `fcntl`, `ioctl`, etc.) are de-multiplexed.
 - Variadic functions (eg. `openat`, etc.) are presented as non-variadic.
 - Functions that return strings automatically allocate sufficient memory
   and retry the syscall as needed to determine the needed length.
 - Functions and types which need `l` prefixes or `64` suffixes to enable
   large-file support (LFS) are used automatically. File sizes and offsets
   are always presented as `u64` and `i64`.
 - Behaviors that depend on the sizes of C types like `long` are hidden.
 - In some places, more human-friendly and less historical-accident names
   are used (and documentation aliases are used so that the original names
   can still be searched for).
 - Provide y2038 compatibility, on platforms which support this.
 - Correct selected platform bugs, such as behavioral differences when
   running under seccomp.
 - Use `timespec` for timestamps and timeouts instead of `timeval` and
   `c_int` milliseconds.

Things they don't do include:
 - Detecting whether functions are supported at runtime, except in specific
   cases where new interfaces need to be detected to support y2038 and LFS.
 - Hiding significant differences between platforms.
 - Restricting ambient authorities.
 - Imposing sandboxing features such as filesystem path or network address
   sandboxing.

See [`cap-std`](#cap-std), [`system-interface`](#system-interface), and [`io-streams`](#io-streams) for libraries
which do hide significant differences between platforms, and [`cap-std`](#cap-std)
which does perform sandboxing and restricts ambient authorities.






[I/O-safe]: https://github.com/rust-lang/rfcs/blob/master/text/3128-io-safety.md

[support for externally defined flags]: bitflags#externally-defined-flags

## Modules

- [`buffer`](buffer/index.md) - Utilities for functions that return data via buffers.
- [`fd`](fd/index.md) - Export the `*Fd` types and traits that are used in rustix's public API.
- [`ffi`](ffi/index.md) - Utilities related to FFI bindings.
- [`io`](io/index.md) - I/O operations.
- [`ioctl`](ioctl/index.md) - Unsafe `ioctl` API.
- [`termios`](termios/index.md) - Terminal I/O stream operations.
- [`not_implemented`](not_implemented/index.md) - Documentation about unimplemented functions.

## Macros

### `cstr!`

A macro for [`CStr`](ffi/index.md) literals.

This can make passing string literals to rustix APIs more efficient, since
most underlying system calls with string arguments expect NUL-terminated
strings, and passing strings to rustix as `CStr`s means that rustix doesn't
need to copy them into a separate buffer to NUL-terminate them.

In Rust ≥ 1.77, users can use [C-string literals] instead of this macro.

[C-string literals]: https://blog.rust-lang.org/2024/03/21/Rust-1.77.0.html#c-string-literals

# Examples

```
# #[cfg(feature = "fs")]
# fn main() -> rustix::io::Result<()> {
use rustix::cstr;
use rustix::fs::{statat, AtFlags, CWD};

let metadata = statat(CWD, cstr!("Cargo.toml"), AtFlags::empty())?;
# Ok(())
# }
# #[cfg(not(feature = "fs"))]
# fn main() {}
```

