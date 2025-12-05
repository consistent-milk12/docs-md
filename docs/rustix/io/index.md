*[rustix](../index.md) / [io](index.md)*

---

# Module `io`

I/O operations.

If you're looking for [`SeekFrom`](#seekfrom), it's in the [`fs`](#fs) module.



## Structs

### `Errno`

```rust
struct Errno(u16);
```

`errno`—An error code.

The error type for `rustix` APIs. This is similar to `std::io::Error`,
but only holds an OS error code, and no extra error value.

# References
 - [POSIX]
 - [Linux]
 - [Winsock]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]
 - [DragonFly BSD]
 - [illumos](#illumos)
 - [glibc](#glibc)

[POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/errno.html
[Linux]: https://man7.org/linux/man-pages/man3/errno.3.html
[Winsock]: https://learn.microsoft.com/en-us/windows/win32/winsock/windows-sockets-error-codes-2
[FreeBSD]: https://man.freebsd.org/cgi/man.cgi?errno
[NetBSD]: https://man.netbsd.org/errno.2
[OpenBSD]: https://man.openbsd.org/errno.2
[DragonFly BSD]: https://man.dragonflybsd.org/?command=errno&section=2
[illumos](#illumos): https://illumos.org/man/3C/errno
[glibc](#glibc): https://sourceware.org/glibc/manual/latest/html_node/Error-Codes.html

#### Implementations

- `fn from_io_error(io_err: &std::io::Error) -> Option<Self>`

- `const fn raw_os_error(self: Self) -> i32`

- `const fn from_raw_os_error(raw: i32) -> Self`

- `const fn from_errno(raw: u32) -> Self`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Errno` — [`Errno`](../../backend/io/errno/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Error`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Errno) -> bool` — [`Errno`](../../backend/io/errno/index.md)

##### `impl StructuralPartialEq`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

## Functions

## Type Aliases

