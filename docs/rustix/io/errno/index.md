*[rustix](../../index.md) / [io](../index.md) / [errno](index.md)*

---

# Module `errno`

The `Errno` type, which is a minimal wrapper around an error code.

We define the error constants as individual `const`s instead of an enum
because we may not know about all of the host's error values and we don't
want unrecognized values to create undefined behavior.

## Structs

### `Errno`

```rust
struct Errno(u16);
```

`errno`—An error code.

The error type for `rustix` APIs. This is similar to [`std::io::Error`](../../../docs_md/error/index.md),
but only holds an OS error code, and no extra error value.

# References
 - [POSIX]
 - [Linux]
 - [Winsock]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]
 - [DragonFly BSD]
 - [illumos]
 - [`glibc`](../../../libc/new/glibc/index.md)










#### Implementations

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

##### `impl Clone for Errno`

- `fn clone(self: &Self) -> Errno` — [`Errno`](../index.md)

##### `impl Copy for Errno`

##### `impl Debug for Errno`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Errno`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Errno`

##### `impl Error for Errno`

##### `impl Hash for Errno`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for Errno`

- `fn eq(self: &Self, other: &Errno) -> bool` — [`Errno`](../index.md)

##### `impl StructuralPartialEq for Errno`

##### `impl<T> ToString for Errno`

- `fn to_string(self: &Self) -> String`

## Functions

### `retry_on_intr`

```rust
fn retry_on_intr<T, F: FnMut() -> Result<T>>(f: F) -> Result<T>
```

Call `f` until it either succeeds or fails other than `Errno::INTR`.

## Type Aliases

### `Result<T>`

```rust
type Result<T> = result::Result<T, Errno>;
```

A specialized [`Result`](../index.md) type for `rustix` APIs.

