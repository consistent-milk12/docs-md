*[rustix](../index.md) / [io](index.md)*

---

# Module `io`

I/O operations.

If you're looking for [`SeekFrom`](#seekfrom), it's in the [`fs`](../../fs_err/fs_err/os/unix/fs/index.md) module.



## Structs

### `Errno`

```rust
struct Errno();
```

`errno`—An error code.

The error type for `rustix` APIs. This is similar to [`std::io::Error`](#error),
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
[illumos](#illumos)
: https://illumos.org/man/3C/errno
[glibc](#glibc)
: https://sourceware.org/glibc/manual/latest/html_node/Error-Codes.html

#### Implementations

- `fn from_io_error(io_err: &std::io::Error) -> Option<Self>`
  Extract an `Errno` value from a `std::io::Error`.

- `const fn raw_os_error(self: Self) -> i32`
  Extract the raw OS error number from this error.

- `const fn from_raw_os_error(raw: i32) -> Self`
  Construct an `Errno` from a raw OS error number.

- `fn kind(self: Self) -> std::io::ErrorKind`
  Shorthand for `std::io::Error::from(self).kind()`.

- `const ACCESS: Self`

- `const ADDRINUSE: Self`

- `const ADDRNOTAVAIL: Self`

- `const ADV: Self`

- `const AFNOSUPPORT: Self`

- `const AGAIN: Self`

- `const ALREADY: Self`

- `const BADE: Self`

- `const BADF: Self`

- `const BADFD: Self`

- `const BADMSG: Self`

- `const BADR: Self`

- `const BADRQC: Self`

- `const BADSLT: Self`

- `const BFONT: Self`

- `const BUSY: Self`

- `const CANCELED: Self`

- `const CHILD: Self`

- `const CHRNG: Self`

- `const COMM: Self`

- `const CONNABORTED: Self`

- `const CONNREFUSED: Self`

- `const CONNRESET: Self`

- `const DEADLK: Self`

- `const DEADLOCK: Self`

- `const DESTADDRREQ: Self`

- `const DOM: Self`

- `const DOTDOT: Self`

- `const DQUOT: Self`

- `const EXIST: Self`

- `const FAULT: Self`

- `const FBIG: Self`

- `const HOSTDOWN: Self`

- `const HOSTUNREACH: Self`

- `const HWPOISON: Self`

- `const IDRM: Self`

- `const ILSEQ: Self`

- `const INPROGRESS: Self`

- `const INTR: Self`

- `const INVAL: Self`

- `const IO: Self`

- `const ISCONN: Self`

- `const ISDIR: Self`

- `const ISNAM: Self`

- `const KEYEXPIRED: Self`

- `const KEYREJECTED: Self`

- `const KEYREVOKED: Self`

- `const L2HLT: Self`

- `const L2NSYNC: Self`

- `const L3HLT: Self`

- `const L3RST: Self`

- `const LIBACC: Self`

- `const LIBBAD: Self`

- `const LIBEXEC: Self`

- `const LIBMAX: Self`

- `const LIBSCN: Self`

- `const LNRNG: Self`

- `const LOOP: Self`

- `const MEDIUMTYPE: Self`

- `const MFILE: Self`

- `const MLINK: Self`

- `const MSGSIZE: Self`

- `const MULTIHOP: Self`

- `const NAMETOOLONG: Self`

- `const NAVAIL: Self`

- `const NETDOWN: Self`

- `const NETRESET: Self`

- `const NETUNREACH: Self`

- `const NFILE: Self`

- `const NOANO: Self`

- `const NOBUFS: Self`

- `const NOCSI: Self`

- `const NODATA: Self`

- `const NODEV: Self`

- `const NOENT: Self`

- `const NOEXEC: Self`

- `const NOKEY: Self`

- `const NOLCK: Self`

- `const NOLINK: Self`

- `const NOMEDIUM: Self`

- `const NOMEM: Self`

- `const NOMSG: Self`

- `const NONET: Self`

- `const NOPKG: Self`

- `const NOPROTOOPT: Self`

- `const NOSPC: Self`

- `const NOSR: Self`

- `const NOSTR: Self`

- `const NOSYS: Self`

- `const NOTBLK: Self`

- `const NOTCONN: Self`

- `const NOTDIR: Self`

- `const NOTEMPTY: Self`

- `const NOTNAM: Self`

- `const NOTRECOVERABLE: Self`

- `const NOTSOCK: Self`

- `const NOTSUP: Self`

- `const NOTTY: Self`

- `const NOTUNIQ: Self`

- `const NXIO: Self`

- `const OPNOTSUPP: Self`

- `const OVERFLOW: Self`

- `const OWNERDEAD: Self`

- `const PERM: Self`

- `const PFNOSUPPORT: Self`

- `const PIPE: Self`

- `const PROTO: Self`

- `const PROTONOSUPPORT: Self`

- `const PROTOTYPE: Self`

- `const RANGE: Self`

- `const REMCHG: Self`

- `const REMOTE: Self`

- `const REMOTEIO: Self`

- `const RESTART: Self`

- `const RFKILL: Self`

- `const ROFS: Self`

- `const SHUTDOWN: Self`

- `const SOCKTNOSUPPORT: Self`

- `const SPIPE: Self`

- `const SRCH: Self`

- `const SRMNT: Self`

- `const STALE: Self`

- `const STRPIPE: Self`

- `const TIME: Self`

- `const TIMEDOUT: Self`

- `const TOOBIG: Self`

- `const TOOMANYREFS: Self`

- `const TXTBSY: Self`

- `const UCLEAN: Self`

- `const UNATCH: Self`

- `const USERS: Self`

- `const WOULDBLOCK: Self`

- `const XDEV: Self`

- `const XFULL: Self`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Errno`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Error`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Errno) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

## Type Aliases

