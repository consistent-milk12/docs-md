*[rustix](../../../index.md) / [backend](../../index.md) / [io](../index.md) / [errno](index.md)*

---

# Module `errno`

The `rustix` `Errno` type.

This type holds an OS error code, which conceptually corresponds to an
`errno` value.

# Safety

Linux uses error codes in `-4095..0`; we use rustc attributes to describe
this restricted range of values.

## Structs

### `Errno`

```rust
struct Errno(u16);
```

`errno`—An error code.

The error type for `rustix` APIs. This is similar to [`std::io::Error`](../../../../addr2line/index.md),
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
 - [`glibc`](../../../../libc/new/glibc/index.md)










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

- `fn clone(self: &Self) -> Errno` — [`Errno`](#errno)

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

- `fn eq(self: &Self, other: &Errno) -> bool` — [`Errno`](#errno)

##### `impl StructuralPartialEq for Errno`

##### `impl<T> ToString for Errno`

- `fn to_string(self: &Self) -> String`

## Functions

### `try_decode_c_int`

```rust
fn try_decode_c_int<Num: RetNumber>(raw: crate::backend::reg::RetReg<Num>) -> io::Result<c::c_int>
```

Check for an error from the result of a syscall which encodes a
`c::c_int` on success.

### `try_decode_c_uint`

```rust
fn try_decode_c_uint<Num: RetNumber>(raw: crate::backend::reg::RetReg<Num>) -> io::Result<c::c_uint>
```

Check for an error from the result of a syscall which encodes a
`c::c_uint` on success.

### `try_decode_usize`

```rust
fn try_decode_usize<Num: RetNumber>(raw: crate::backend::reg::RetReg<Num>) -> io::Result<usize>
```

Check for an error from the result of a syscall which encodes a `usize` on
success.

### `try_decode_void_star`

```rust
fn try_decode_void_star<Num: RetNumber>(raw: crate::backend::reg::RetReg<Num>) -> io::Result<*mut c::c_void>
```

Check for an error from the result of a syscall which encodes a
`*mut c_void` on success.

### `try_decode_u64`

```rust
fn try_decode_u64<Num: RetNumber>(raw: crate::backend::reg::RetReg<Num>) -> io::Result<u64>
```

Check for an error from the result of a syscall which encodes a
`u64` on success.

### `try_decode_raw_fd`

```rust
unsafe fn try_decode_raw_fd<Num: RetNumber>(raw: crate::backend::reg::RetReg<Num>) -> io::Result<crate::backend::fd::RawFd>
```

Check for an error from the result of a syscall which encodes a file
descriptor on success.

# Safety

This must only be used with syscalls which return file descriptors on
success.

### `try_decode_void`

```rust
unsafe fn try_decode_void<Num: RetNumber>(raw: crate::backend::reg::RetReg<Num>) -> io::Result<()>
```

Check for an error from the result of a syscall which encodes no value on
success. On success, return the unconsumed `raw` value.

# Safety

This must only be used with syscalls which return no value on success.

