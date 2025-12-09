*[linux_raw_sys](../index.md) / [errno](index.md)*

---

# Module `errno`

## Contents

- [Constants](#constants)
  - [`EPERM`](#eperm)
  - [`ENOENT`](#enoent)
  - [`ESRCH`](#esrch)
  - [`EINTR`](#eintr)
  - [`EIO`](#eio)
  - [`ENXIO`](#enxio)
  - [`E2BIG`](#e2big)
  - [`ENOEXEC`](#enoexec)
  - [`EBADF`](#ebadf)
  - [`ECHILD`](#echild)
  - [`EAGAIN`](#eagain)
  - [`ENOMEM`](#enomem)
  - [`EACCES`](#eacces)
  - [`EFAULT`](#efault)
  - [`ENOTBLK`](#enotblk)
  - [`EBUSY`](#ebusy)
  - [`EEXIST`](#eexist)
  - [`EXDEV`](#exdev)
  - [`ENODEV`](#enodev)
  - [`ENOTDIR`](#enotdir)
  - [`EISDIR`](#eisdir)
  - [`EINVAL`](#einval)
  - [`ENFILE`](#enfile)
  - [`EMFILE`](#emfile)
  - [`ENOTTY`](#enotty)
  - [`ETXTBSY`](#etxtbsy)
  - [`EFBIG`](#efbig)
  - [`ENOSPC`](#enospc)
  - [`ESPIPE`](#espipe)
  - [`EROFS`](#erofs)
  - [`EMLINK`](#emlink)
  - [`EPIPE`](#epipe)
  - [`EDOM`](#edom)
  - [`ERANGE`](#erange)
  - [`EDEADLK`](#edeadlk)
  - [`ENAMETOOLONG`](#enametoolong)
  - [`ENOLCK`](#enolck)
  - [`ENOSYS`](#enosys)
  - [`ENOTEMPTY`](#enotempty)
  - [`ELOOP`](#eloop)
  - [`EWOULDBLOCK`](#ewouldblock)
  - [`ENOMSG`](#enomsg)
  - [`EIDRM`](#eidrm)
  - [`ECHRNG`](#echrng)
  - [`EL2NSYNC`](#el2nsync)
  - [`EL3HLT`](#el3hlt)
  - [`EL3RST`](#el3rst)
  - [`ELNRNG`](#elnrng)
  - [`EUNATCH`](#eunatch)
  - [`ENOCSI`](#enocsi)
  - [`EL2HLT`](#el2hlt)
  - [`EBADE`](#ebade)
  - [`EBADR`](#ebadr)
  - [`EXFULL`](#exfull)
  - [`ENOANO`](#enoano)
  - [`EBADRQC`](#ebadrqc)
  - [`EBADSLT`](#ebadslt)
  - [`EDEADLOCK`](#edeadlock)
  - [`EBFONT`](#ebfont)
  - [`ENOSTR`](#enostr)
  - [`ENODATA`](#enodata)
  - [`ETIME`](#etime)
  - [`ENOSR`](#enosr)
  - [`ENONET`](#enonet)
  - [`ENOPKG`](#enopkg)
  - [`EREMOTE`](#eremote)
  - [`ENOLINK`](#enolink)
  - [`EADV`](#eadv)
  - [`ESRMNT`](#esrmnt)
  - [`ECOMM`](#ecomm)
  - [`EPROTO`](#eproto)
  - [`EMULTIHOP`](#emultihop)
  - [`EDOTDOT`](#edotdot)
  - [`EBADMSG`](#ebadmsg)
  - [`EOVERFLOW`](#eoverflow)
  - [`ENOTUNIQ`](#enotuniq)
  - [`EBADFD`](#ebadfd)
  - [`EREMCHG`](#eremchg)
  - [`ELIBACC`](#elibacc)
  - [`ELIBBAD`](#elibbad)
  - [`ELIBSCN`](#elibscn)
  - [`ELIBMAX`](#elibmax)
  - [`ELIBEXEC`](#elibexec)
  - [`EILSEQ`](#eilseq)
  - [`ERESTART`](#erestart)
  - [`ESTRPIPE`](#estrpipe)
  - [`EUSERS`](#eusers)
  - [`ENOTSOCK`](#enotsock)
  - [`EDESTADDRREQ`](#edestaddrreq)
  - [`EMSGSIZE`](#emsgsize)
  - [`EPROTOTYPE`](#eprototype)
  - [`ENOPROTOOPT`](#enoprotoopt)
  - [`EPROTONOSUPPORT`](#eprotonosupport)
  - [`ESOCKTNOSUPPORT`](#esocktnosupport)
  - [`EOPNOTSUPP`](#eopnotsupp)
  - [`EPFNOSUPPORT`](#epfnosupport)
  - [`EAFNOSUPPORT`](#eafnosupport)
  - [`EADDRINUSE`](#eaddrinuse)
  - [`EADDRNOTAVAIL`](#eaddrnotavail)
  - [`ENETDOWN`](#enetdown)
  - [`ENETUNREACH`](#enetunreach)
  - [`ENETRESET`](#enetreset)
  - [`ECONNABORTED`](#econnaborted)
  - [`ECONNRESET`](#econnreset)
  - [`ENOBUFS`](#enobufs)
  - [`EISCONN`](#eisconn)
  - [`ENOTCONN`](#enotconn)
  - [`ESHUTDOWN`](#eshutdown)
  - [`ETOOMANYREFS`](#etoomanyrefs)
  - [`ETIMEDOUT`](#etimedout)
  - [`ECONNREFUSED`](#econnrefused)
  - [`EHOSTDOWN`](#ehostdown)
  - [`EHOSTUNREACH`](#ehostunreach)
  - [`EALREADY`](#ealready)
  - [`EINPROGRESS`](#einprogress)
  - [`ESTALE`](#estale)
  - [`EUCLEAN`](#euclean)
  - [`ENOTNAM`](#enotnam)
  - [`ENAVAIL`](#enavail)
  - [`EISNAM`](#eisnam)
  - [`EREMOTEIO`](#eremoteio)
  - [`EDQUOT`](#edquot)
  - [`ENOMEDIUM`](#enomedium)
  - [`EMEDIUMTYPE`](#emediumtype)
  - [`ECANCELED`](#ecanceled)
  - [`ENOKEY`](#enokey)
  - [`EKEYEXPIRED`](#ekeyexpired)
  - [`EKEYREVOKED`](#ekeyrevoked)
  - [`EKEYREJECTED`](#ekeyrejected)
  - [`EOWNERDEAD`](#eownerdead)
  - [`ENOTRECOVERABLE`](#enotrecoverable)
  - [`ERFKILL`](#erfkill)
  - [`EHWPOISON`](#ehwpoison)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`EPERM`](#eperm) | const |  |
| [`ENOENT`](#enoent) | const |  |
| [`ESRCH`](#esrch) | const |  |
| [`EINTR`](#eintr) | const |  |
| [`EIO`](#eio) | const |  |
| [`ENXIO`](#enxio) | const |  |
| [`E2BIG`](#e2big) | const |  |
| [`ENOEXEC`](#enoexec) | const |  |
| [`EBADF`](#ebadf) | const |  |
| [`ECHILD`](#echild) | const |  |
| [`EAGAIN`](#eagain) | const |  |
| [`ENOMEM`](#enomem) | const |  |
| [`EACCES`](#eacces) | const |  |
| [`EFAULT`](#efault) | const |  |
| [`ENOTBLK`](#enotblk) | const |  |
| [`EBUSY`](#ebusy) | const |  |
| [`EEXIST`](#eexist) | const |  |
| [`EXDEV`](#exdev) | const |  |
| [`ENODEV`](#enodev) | const |  |
| [`ENOTDIR`](#enotdir) | const |  |
| [`EISDIR`](#eisdir) | const |  |
| [`EINVAL`](#einval) | const |  |
| [`ENFILE`](#enfile) | const |  |
| [`EMFILE`](#emfile) | const |  |
| [`ENOTTY`](#enotty) | const |  |
| [`ETXTBSY`](#etxtbsy) | const |  |
| [`EFBIG`](#efbig) | const |  |
| [`ENOSPC`](#enospc) | const |  |
| [`ESPIPE`](#espipe) | const |  |
| [`EROFS`](#erofs) | const |  |
| [`EMLINK`](#emlink) | const |  |
| [`EPIPE`](#epipe) | const |  |
| [`EDOM`](#edom) | const |  |
| [`ERANGE`](#erange) | const |  |
| [`EDEADLK`](#edeadlk) | const |  |
| [`ENAMETOOLONG`](#enametoolong) | const |  |
| [`ENOLCK`](#enolck) | const |  |
| [`ENOSYS`](#enosys) | const |  |
| [`ENOTEMPTY`](#enotempty) | const |  |
| [`ELOOP`](#eloop) | const |  |
| [`EWOULDBLOCK`](#ewouldblock) | const |  |
| [`ENOMSG`](#enomsg) | const |  |
| [`EIDRM`](#eidrm) | const |  |
| [`ECHRNG`](#echrng) | const |  |
| [`EL2NSYNC`](#el2nsync) | const |  |
| [`EL3HLT`](#el3hlt) | const |  |
| [`EL3RST`](#el3rst) | const |  |
| [`ELNRNG`](#elnrng) | const |  |
| [`EUNATCH`](#eunatch) | const |  |
| [`ENOCSI`](#enocsi) | const |  |
| [`EL2HLT`](#el2hlt) | const |  |
| [`EBADE`](#ebade) | const |  |
| [`EBADR`](#ebadr) | const |  |
| [`EXFULL`](#exfull) | const |  |
| [`ENOANO`](#enoano) | const |  |
| [`EBADRQC`](#ebadrqc) | const |  |
| [`EBADSLT`](#ebadslt) | const |  |
| [`EDEADLOCK`](#edeadlock) | const |  |
| [`EBFONT`](#ebfont) | const |  |
| [`ENOSTR`](#enostr) | const |  |
| [`ENODATA`](#enodata) | const |  |
| [`ETIME`](#etime) | const |  |
| [`ENOSR`](#enosr) | const |  |
| [`ENONET`](#enonet) | const |  |
| [`ENOPKG`](#enopkg) | const |  |
| [`EREMOTE`](#eremote) | const |  |
| [`ENOLINK`](#enolink) | const |  |
| [`EADV`](#eadv) | const |  |
| [`ESRMNT`](#esrmnt) | const |  |
| [`ECOMM`](#ecomm) | const |  |
| [`EPROTO`](#eproto) | const |  |
| [`EMULTIHOP`](#emultihop) | const |  |
| [`EDOTDOT`](#edotdot) | const |  |
| [`EBADMSG`](#ebadmsg) | const |  |
| [`EOVERFLOW`](#eoverflow) | const |  |
| [`ENOTUNIQ`](#enotuniq) | const |  |
| [`EBADFD`](#ebadfd) | const |  |
| [`EREMCHG`](#eremchg) | const |  |
| [`ELIBACC`](#elibacc) | const |  |
| [`ELIBBAD`](#elibbad) | const |  |
| [`ELIBSCN`](#elibscn) | const |  |
| [`ELIBMAX`](#elibmax) | const |  |
| [`ELIBEXEC`](#elibexec) | const |  |
| [`EILSEQ`](#eilseq) | const |  |
| [`ERESTART`](#erestart) | const |  |
| [`ESTRPIPE`](#estrpipe) | const |  |
| [`EUSERS`](#eusers) | const |  |
| [`ENOTSOCK`](#enotsock) | const |  |
| [`EDESTADDRREQ`](#edestaddrreq) | const |  |
| [`EMSGSIZE`](#emsgsize) | const |  |
| [`EPROTOTYPE`](#eprototype) | const |  |
| [`ENOPROTOOPT`](#enoprotoopt) | const |  |
| [`EPROTONOSUPPORT`](#eprotonosupport) | const |  |
| [`ESOCKTNOSUPPORT`](#esocktnosupport) | const |  |
| [`EOPNOTSUPP`](#eopnotsupp) | const |  |
| [`EPFNOSUPPORT`](#epfnosupport) | const |  |
| [`EAFNOSUPPORT`](#eafnosupport) | const |  |
| [`EADDRINUSE`](#eaddrinuse) | const |  |
| [`EADDRNOTAVAIL`](#eaddrnotavail) | const |  |
| [`ENETDOWN`](#enetdown) | const |  |
| [`ENETUNREACH`](#enetunreach) | const |  |
| [`ENETRESET`](#enetreset) | const |  |
| [`ECONNABORTED`](#econnaborted) | const |  |
| [`ECONNRESET`](#econnreset) | const |  |
| [`ENOBUFS`](#enobufs) | const |  |
| [`EISCONN`](#eisconn) | const |  |
| [`ENOTCONN`](#enotconn) | const |  |
| [`ESHUTDOWN`](#eshutdown) | const |  |
| [`ETOOMANYREFS`](#etoomanyrefs) | const |  |
| [`ETIMEDOUT`](#etimedout) | const |  |
| [`ECONNREFUSED`](#econnrefused) | const |  |
| [`EHOSTDOWN`](#ehostdown) | const |  |
| [`EHOSTUNREACH`](#ehostunreach) | const |  |
| [`EALREADY`](#ealready) | const |  |
| [`EINPROGRESS`](#einprogress) | const |  |
| [`ESTALE`](#estale) | const |  |
| [`EUCLEAN`](#euclean) | const |  |
| [`ENOTNAM`](#enotnam) | const |  |
| [`ENAVAIL`](#enavail) | const |  |
| [`EISNAM`](#eisnam) | const |  |
| [`EREMOTEIO`](#eremoteio) | const |  |
| [`EDQUOT`](#edquot) | const |  |
| [`ENOMEDIUM`](#enomedium) | const |  |
| [`EMEDIUMTYPE`](#emediumtype) | const |  |
| [`ECANCELED`](#ecanceled) | const |  |
| [`ENOKEY`](#enokey) | const |  |
| [`EKEYEXPIRED`](#ekeyexpired) | const |  |
| [`EKEYREVOKED`](#ekeyrevoked) | const |  |
| [`EKEYREJECTED`](#ekeyrejected) | const |  |
| [`EOWNERDEAD`](#eownerdead) | const |  |
| [`ENOTRECOVERABLE`](#enotrecoverable) | const |  |
| [`ERFKILL`](#erfkill) | const |  |
| [`EHWPOISON`](#ehwpoison) | const |  |

## Constants

### `EPERM`
```rust
const EPERM: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:3`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L3)*

### `ENOENT`
```rust
const ENOENT: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:4`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L4)*

### `ESRCH`
```rust
const ESRCH: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:5`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L5)*

### `EINTR`
```rust
const EINTR: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:6`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L6)*

### `EIO`
```rust
const EIO: u32 = 5u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:7`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L7)*

### `ENXIO`
```rust
const ENXIO: u32 = 6u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:8`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L8)*

### `E2BIG`
```rust
const E2BIG: u32 = 7u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:9`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L9)*

### `ENOEXEC`
```rust
const ENOEXEC: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:10`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L10)*

### `EBADF`
```rust
const EBADF: u32 = 9u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:11`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L11)*

### `ECHILD`
```rust
const ECHILD: u32 = 10u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:12`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L12)*

### `EAGAIN`
```rust
const EAGAIN: u32 = 11u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:13`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L13)*

### `ENOMEM`
```rust
const ENOMEM: u32 = 12u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:14`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L14)*

### `EACCES`
```rust
const EACCES: u32 = 13u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:15`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L15)*

### `EFAULT`
```rust
const EFAULT: u32 = 14u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:16`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L16)*

### `ENOTBLK`
```rust
const ENOTBLK: u32 = 15u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:17`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L17)*

### `EBUSY`
```rust
const EBUSY: u32 = 16u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:18`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L18)*

### `EEXIST`
```rust
const EEXIST: u32 = 17u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:19`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L19)*

### `EXDEV`
```rust
const EXDEV: u32 = 18u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:20`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L20)*

### `ENODEV`
```rust
const ENODEV: u32 = 19u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:21`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L21)*

### `ENOTDIR`
```rust
const ENOTDIR: u32 = 20u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:22`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L22)*

### `EISDIR`
```rust
const EISDIR: u32 = 21u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:23`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L23)*

### `EINVAL`
```rust
const EINVAL: u32 = 22u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:24`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L24)*

### `ENFILE`
```rust
const ENFILE: u32 = 23u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:25`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L25)*

### `EMFILE`
```rust
const EMFILE: u32 = 24u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:26`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L26)*

### `ENOTTY`
```rust
const ENOTTY: u32 = 25u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:27`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L27)*

### `ETXTBSY`
```rust
const ETXTBSY: u32 = 26u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:28`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L28)*

### `EFBIG`
```rust
const EFBIG: u32 = 27u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:29`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L29)*

### `ENOSPC`
```rust
const ENOSPC: u32 = 28u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:30`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L30)*

### `ESPIPE`
```rust
const ESPIPE: u32 = 29u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:31`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L31)*

### `EROFS`
```rust
const EROFS: u32 = 30u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:32`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L32)*

### `EMLINK`
```rust
const EMLINK: u32 = 31u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:33`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L33)*

### `EPIPE`
```rust
const EPIPE: u32 = 32u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:34`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L34)*

### `EDOM`
```rust
const EDOM: u32 = 33u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:35`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L35)*

### `ERANGE`
```rust
const ERANGE: u32 = 34u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:36`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L36)*

### `EDEADLK`
```rust
const EDEADLK: u32 = 35u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:37`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L37)*

### `ENAMETOOLONG`
```rust
const ENAMETOOLONG: u32 = 36u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:38`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L38)*

### `ENOLCK`
```rust
const ENOLCK: u32 = 37u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:39`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L39)*

### `ENOSYS`
```rust
const ENOSYS: u32 = 38u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:40`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L40)*

### `ENOTEMPTY`
```rust
const ENOTEMPTY: u32 = 39u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:41`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L41)*

### `ELOOP`
```rust
const ELOOP: u32 = 40u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:42`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L42)*

### `EWOULDBLOCK`
```rust
const EWOULDBLOCK: u32 = 11u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:43`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L43)*

### `ENOMSG`
```rust
const ENOMSG: u32 = 42u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:44`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L44)*

### `EIDRM`
```rust
const EIDRM: u32 = 43u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:45`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L45)*

### `ECHRNG`
```rust
const ECHRNG: u32 = 44u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:46`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L46)*

### `EL2NSYNC`
```rust
const EL2NSYNC: u32 = 45u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:47`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L47)*

### `EL3HLT`
```rust
const EL3HLT: u32 = 46u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:48`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L48)*

### `EL3RST`
```rust
const EL3RST: u32 = 47u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:49`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L49)*

### `ELNRNG`
```rust
const ELNRNG: u32 = 48u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:50`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L50)*

### `EUNATCH`
```rust
const EUNATCH: u32 = 49u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:51`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L51)*

### `ENOCSI`
```rust
const ENOCSI: u32 = 50u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:52`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L52)*

### `EL2HLT`
```rust
const EL2HLT: u32 = 51u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:53`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L53)*

### `EBADE`
```rust
const EBADE: u32 = 52u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:54`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L54)*

### `EBADR`
```rust
const EBADR: u32 = 53u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:55`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L55)*

### `EXFULL`
```rust
const EXFULL: u32 = 54u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:56`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L56)*

### `ENOANO`
```rust
const ENOANO: u32 = 55u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:57`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L57)*

### `EBADRQC`
```rust
const EBADRQC: u32 = 56u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:58`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L58)*

### `EBADSLT`
```rust
const EBADSLT: u32 = 57u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:59`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L59)*

### `EDEADLOCK`
```rust
const EDEADLOCK: u32 = 35u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:60`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L60)*

### `EBFONT`
```rust
const EBFONT: u32 = 59u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:61`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L61)*

### `ENOSTR`
```rust
const ENOSTR: u32 = 60u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:62`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L62)*

### `ENODATA`
```rust
const ENODATA: u32 = 61u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:63`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L63)*

### `ETIME`
```rust
const ETIME: u32 = 62u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:64`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L64)*

### `ENOSR`
```rust
const ENOSR: u32 = 63u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:65`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L65)*

### `ENONET`
```rust
const ENONET: u32 = 64u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:66`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L66)*

### `ENOPKG`
```rust
const ENOPKG: u32 = 65u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:67`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L67)*

### `EREMOTE`
```rust
const EREMOTE: u32 = 66u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:68`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L68)*

### `ENOLINK`
```rust
const ENOLINK: u32 = 67u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:69`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L69)*

### `EADV`
```rust
const EADV: u32 = 68u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:70`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L70)*

### `ESRMNT`
```rust
const ESRMNT: u32 = 69u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:71`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L71)*

### `ECOMM`
```rust
const ECOMM: u32 = 70u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:72`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L72)*

### `EPROTO`
```rust
const EPROTO: u32 = 71u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:73`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L73)*

### `EMULTIHOP`
```rust
const EMULTIHOP: u32 = 72u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:74`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L74)*

### `EDOTDOT`
```rust
const EDOTDOT: u32 = 73u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:75`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L75)*

### `EBADMSG`
```rust
const EBADMSG: u32 = 74u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:76`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L76)*

### `EOVERFLOW`
```rust
const EOVERFLOW: u32 = 75u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:77`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L77)*

### `ENOTUNIQ`
```rust
const ENOTUNIQ: u32 = 76u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:78`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L78)*

### `EBADFD`
```rust
const EBADFD: u32 = 77u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:79`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L79)*

### `EREMCHG`
```rust
const EREMCHG: u32 = 78u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:80`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L80)*

### `ELIBACC`
```rust
const ELIBACC: u32 = 79u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:81`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L81)*

### `ELIBBAD`
```rust
const ELIBBAD: u32 = 80u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:82`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L82)*

### `ELIBSCN`
```rust
const ELIBSCN: u32 = 81u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:83`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L83)*

### `ELIBMAX`
```rust
const ELIBMAX: u32 = 82u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:84`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L84)*

### `ELIBEXEC`
```rust
const ELIBEXEC: u32 = 83u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:85`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L85)*

### `EILSEQ`
```rust
const EILSEQ: u32 = 84u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:86`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L86)*

### `ERESTART`
```rust
const ERESTART: u32 = 85u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:87`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L87)*

### `ESTRPIPE`
```rust
const ESTRPIPE: u32 = 86u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:88`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L88)*

### `EUSERS`
```rust
const EUSERS: u32 = 87u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:89`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L89)*

### `ENOTSOCK`
```rust
const ENOTSOCK: u32 = 88u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:90`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L90)*

### `EDESTADDRREQ`
```rust
const EDESTADDRREQ: u32 = 89u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:91`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L91)*

### `EMSGSIZE`
```rust
const EMSGSIZE: u32 = 90u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:92`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L92)*

### `EPROTOTYPE`
```rust
const EPROTOTYPE: u32 = 91u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:93`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L93)*

### `ENOPROTOOPT`
```rust
const ENOPROTOOPT: u32 = 92u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:94`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L94)*

### `EPROTONOSUPPORT`
```rust
const EPROTONOSUPPORT: u32 = 93u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:95`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L95)*

### `ESOCKTNOSUPPORT`
```rust
const ESOCKTNOSUPPORT: u32 = 94u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:96`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L96)*

### `EOPNOTSUPP`
```rust
const EOPNOTSUPP: u32 = 95u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:97`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L97)*

### `EPFNOSUPPORT`
```rust
const EPFNOSUPPORT: u32 = 96u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:98`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L98)*

### `EAFNOSUPPORT`
```rust
const EAFNOSUPPORT: u32 = 97u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:99`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L99)*

### `EADDRINUSE`
```rust
const EADDRINUSE: u32 = 98u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:100`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L100)*

### `EADDRNOTAVAIL`
```rust
const EADDRNOTAVAIL: u32 = 99u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:101`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L101)*

### `ENETDOWN`
```rust
const ENETDOWN: u32 = 100u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:102`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L102)*

### `ENETUNREACH`
```rust
const ENETUNREACH: u32 = 101u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:103`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L103)*

### `ENETRESET`
```rust
const ENETRESET: u32 = 102u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:104`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L104)*

### `ECONNABORTED`
```rust
const ECONNABORTED: u32 = 103u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:105`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L105)*

### `ECONNRESET`
```rust
const ECONNRESET: u32 = 104u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:106`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L106)*

### `ENOBUFS`
```rust
const ENOBUFS: u32 = 105u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:107`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L107)*

### `EISCONN`
```rust
const EISCONN: u32 = 106u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:108`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L108)*

### `ENOTCONN`
```rust
const ENOTCONN: u32 = 107u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:109`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L109)*

### `ESHUTDOWN`
```rust
const ESHUTDOWN: u32 = 108u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:110`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L110)*

### `ETOOMANYREFS`
```rust
const ETOOMANYREFS: u32 = 109u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:111`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L111)*

### `ETIMEDOUT`
```rust
const ETIMEDOUT: u32 = 110u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:112`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L112)*

### `ECONNREFUSED`
```rust
const ECONNREFUSED: u32 = 111u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:113`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L113)*

### `EHOSTDOWN`
```rust
const EHOSTDOWN: u32 = 112u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:114`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L114)*

### `EHOSTUNREACH`
```rust
const EHOSTUNREACH: u32 = 113u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:115`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L115)*

### `EALREADY`
```rust
const EALREADY: u32 = 114u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:116`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L116)*

### `EINPROGRESS`
```rust
const EINPROGRESS: u32 = 115u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:117`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L117)*

### `ESTALE`
```rust
const ESTALE: u32 = 116u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:118`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L118)*

### `EUCLEAN`
```rust
const EUCLEAN: u32 = 117u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:119`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L119)*

### `ENOTNAM`
```rust
const ENOTNAM: u32 = 118u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:120`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L120)*

### `ENAVAIL`
```rust
const ENAVAIL: u32 = 119u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:121`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L121)*

### `EISNAM`
```rust
const EISNAM: u32 = 120u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:122`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L122)*

### `EREMOTEIO`
```rust
const EREMOTEIO: u32 = 121u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:123`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L123)*

### `EDQUOT`
```rust
const EDQUOT: u32 = 122u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:124`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L124)*

### `ENOMEDIUM`
```rust
const ENOMEDIUM: u32 = 123u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:125`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L125)*

### `EMEDIUMTYPE`
```rust
const EMEDIUMTYPE: u32 = 124u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:126`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L126)*

### `ECANCELED`
```rust
const ECANCELED: u32 = 125u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:127`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L127)*

### `ENOKEY`
```rust
const ENOKEY: u32 = 126u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:128`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L128)*

### `EKEYEXPIRED`
```rust
const EKEYEXPIRED: u32 = 127u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:129`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L129)*

### `EKEYREVOKED`
```rust
const EKEYREVOKED: u32 = 128u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:130`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L130)*

### `EKEYREJECTED`
```rust
const EKEYREJECTED: u32 = 129u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:131`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L131)*

### `EOWNERDEAD`
```rust
const EOWNERDEAD: u32 = 130u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:132`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L132)*

### `ENOTRECOVERABLE`
```rust
const ENOTRECOVERABLE: u32 = 131u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:133`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L133)*

### `ERFKILL`
```rust
const ERFKILL: u32 = 132u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:134`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L134)*

### `EHWPOISON`
```rust
const EHWPOISON: u32 = 133u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/x86_64/errno.rs:135`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/x86_64/errno.rs#L135)*

