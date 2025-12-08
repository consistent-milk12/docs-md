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

### `ENOENT`

```rust
const ENOENT: u32 = 2u32;
```

### `ESRCH`

```rust
const ESRCH: u32 = 3u32;
```

### `EINTR`

```rust
const EINTR: u32 = 4u32;
```

### `EIO`

```rust
const EIO: u32 = 5u32;
```

### `ENXIO`

```rust
const ENXIO: u32 = 6u32;
```

### `E2BIG`

```rust
const E2BIG: u32 = 7u32;
```

### `ENOEXEC`

```rust
const ENOEXEC: u32 = 8u32;
```

### `EBADF`

```rust
const EBADF: u32 = 9u32;
```

### `ECHILD`

```rust
const ECHILD: u32 = 10u32;
```

### `EAGAIN`

```rust
const EAGAIN: u32 = 11u32;
```

### `ENOMEM`

```rust
const ENOMEM: u32 = 12u32;
```

### `EACCES`

```rust
const EACCES: u32 = 13u32;
```

### `EFAULT`

```rust
const EFAULT: u32 = 14u32;
```

### `ENOTBLK`

```rust
const ENOTBLK: u32 = 15u32;
```

### `EBUSY`

```rust
const EBUSY: u32 = 16u32;
```

### `EEXIST`

```rust
const EEXIST: u32 = 17u32;
```

### `EXDEV`

```rust
const EXDEV: u32 = 18u32;
```

### `ENODEV`

```rust
const ENODEV: u32 = 19u32;
```

### `ENOTDIR`

```rust
const ENOTDIR: u32 = 20u32;
```

### `EISDIR`

```rust
const EISDIR: u32 = 21u32;
```

### `EINVAL`

```rust
const EINVAL: u32 = 22u32;
```

### `ENFILE`

```rust
const ENFILE: u32 = 23u32;
```

### `EMFILE`

```rust
const EMFILE: u32 = 24u32;
```

### `ENOTTY`

```rust
const ENOTTY: u32 = 25u32;
```

### `ETXTBSY`

```rust
const ETXTBSY: u32 = 26u32;
```

### `EFBIG`

```rust
const EFBIG: u32 = 27u32;
```

### `ENOSPC`

```rust
const ENOSPC: u32 = 28u32;
```

### `ESPIPE`

```rust
const ESPIPE: u32 = 29u32;
```

### `EROFS`

```rust
const EROFS: u32 = 30u32;
```

### `EMLINK`

```rust
const EMLINK: u32 = 31u32;
```

### `EPIPE`

```rust
const EPIPE: u32 = 32u32;
```

### `EDOM`

```rust
const EDOM: u32 = 33u32;
```

### `ERANGE`

```rust
const ERANGE: u32 = 34u32;
```

### `EDEADLK`

```rust
const EDEADLK: u32 = 35u32;
```

### `ENAMETOOLONG`

```rust
const ENAMETOOLONG: u32 = 36u32;
```

### `ENOLCK`

```rust
const ENOLCK: u32 = 37u32;
```

### `ENOSYS`

```rust
const ENOSYS: u32 = 38u32;
```

### `ENOTEMPTY`

```rust
const ENOTEMPTY: u32 = 39u32;
```

### `ELOOP`

```rust
const ELOOP: u32 = 40u32;
```

### `EWOULDBLOCK`

```rust
const EWOULDBLOCK: u32 = 11u32;
```

### `ENOMSG`

```rust
const ENOMSG: u32 = 42u32;
```

### `EIDRM`

```rust
const EIDRM: u32 = 43u32;
```

### `ECHRNG`

```rust
const ECHRNG: u32 = 44u32;
```

### `EL2NSYNC`

```rust
const EL2NSYNC: u32 = 45u32;
```

### `EL3HLT`

```rust
const EL3HLT: u32 = 46u32;
```

### `EL3RST`

```rust
const EL3RST: u32 = 47u32;
```

### `ELNRNG`

```rust
const ELNRNG: u32 = 48u32;
```

### `EUNATCH`

```rust
const EUNATCH: u32 = 49u32;
```

### `ENOCSI`

```rust
const ENOCSI: u32 = 50u32;
```

### `EL2HLT`

```rust
const EL2HLT: u32 = 51u32;
```

### `EBADE`

```rust
const EBADE: u32 = 52u32;
```

### `EBADR`

```rust
const EBADR: u32 = 53u32;
```

### `EXFULL`

```rust
const EXFULL: u32 = 54u32;
```

### `ENOANO`

```rust
const ENOANO: u32 = 55u32;
```

### `EBADRQC`

```rust
const EBADRQC: u32 = 56u32;
```

### `EBADSLT`

```rust
const EBADSLT: u32 = 57u32;
```

### `EDEADLOCK`

```rust
const EDEADLOCK: u32 = 35u32;
```

### `EBFONT`

```rust
const EBFONT: u32 = 59u32;
```

### `ENOSTR`

```rust
const ENOSTR: u32 = 60u32;
```

### `ENODATA`

```rust
const ENODATA: u32 = 61u32;
```

### `ETIME`

```rust
const ETIME: u32 = 62u32;
```

### `ENOSR`

```rust
const ENOSR: u32 = 63u32;
```

### `ENONET`

```rust
const ENONET: u32 = 64u32;
```

### `ENOPKG`

```rust
const ENOPKG: u32 = 65u32;
```

### `EREMOTE`

```rust
const EREMOTE: u32 = 66u32;
```

### `ENOLINK`

```rust
const ENOLINK: u32 = 67u32;
```

### `EADV`

```rust
const EADV: u32 = 68u32;
```

### `ESRMNT`

```rust
const ESRMNT: u32 = 69u32;
```

### `ECOMM`

```rust
const ECOMM: u32 = 70u32;
```

### `EPROTO`

```rust
const EPROTO: u32 = 71u32;
```

### `EMULTIHOP`

```rust
const EMULTIHOP: u32 = 72u32;
```

### `EDOTDOT`

```rust
const EDOTDOT: u32 = 73u32;
```

### `EBADMSG`

```rust
const EBADMSG: u32 = 74u32;
```

### `EOVERFLOW`

```rust
const EOVERFLOW: u32 = 75u32;
```

### `ENOTUNIQ`

```rust
const ENOTUNIQ: u32 = 76u32;
```

### `EBADFD`

```rust
const EBADFD: u32 = 77u32;
```

### `EREMCHG`

```rust
const EREMCHG: u32 = 78u32;
```

### `ELIBACC`

```rust
const ELIBACC: u32 = 79u32;
```

### `ELIBBAD`

```rust
const ELIBBAD: u32 = 80u32;
```

### `ELIBSCN`

```rust
const ELIBSCN: u32 = 81u32;
```

### `ELIBMAX`

```rust
const ELIBMAX: u32 = 82u32;
```

### `ELIBEXEC`

```rust
const ELIBEXEC: u32 = 83u32;
```

### `EILSEQ`

```rust
const EILSEQ: u32 = 84u32;
```

### `ERESTART`

```rust
const ERESTART: u32 = 85u32;
```

### `ESTRPIPE`

```rust
const ESTRPIPE: u32 = 86u32;
```

### `EUSERS`

```rust
const EUSERS: u32 = 87u32;
```

### `ENOTSOCK`

```rust
const ENOTSOCK: u32 = 88u32;
```

### `EDESTADDRREQ`

```rust
const EDESTADDRREQ: u32 = 89u32;
```

### `EMSGSIZE`

```rust
const EMSGSIZE: u32 = 90u32;
```

### `EPROTOTYPE`

```rust
const EPROTOTYPE: u32 = 91u32;
```

### `ENOPROTOOPT`

```rust
const ENOPROTOOPT: u32 = 92u32;
```

### `EPROTONOSUPPORT`

```rust
const EPROTONOSUPPORT: u32 = 93u32;
```

### `ESOCKTNOSUPPORT`

```rust
const ESOCKTNOSUPPORT: u32 = 94u32;
```

### `EOPNOTSUPP`

```rust
const EOPNOTSUPP: u32 = 95u32;
```

### `EPFNOSUPPORT`

```rust
const EPFNOSUPPORT: u32 = 96u32;
```

### `EAFNOSUPPORT`

```rust
const EAFNOSUPPORT: u32 = 97u32;
```

### `EADDRINUSE`

```rust
const EADDRINUSE: u32 = 98u32;
```

### `EADDRNOTAVAIL`

```rust
const EADDRNOTAVAIL: u32 = 99u32;
```

### `ENETDOWN`

```rust
const ENETDOWN: u32 = 100u32;
```

### `ENETUNREACH`

```rust
const ENETUNREACH: u32 = 101u32;
```

### `ENETRESET`

```rust
const ENETRESET: u32 = 102u32;
```

### `ECONNABORTED`

```rust
const ECONNABORTED: u32 = 103u32;
```

### `ECONNRESET`

```rust
const ECONNRESET: u32 = 104u32;
```

### `ENOBUFS`

```rust
const ENOBUFS: u32 = 105u32;
```

### `EISCONN`

```rust
const EISCONN: u32 = 106u32;
```

### `ENOTCONN`

```rust
const ENOTCONN: u32 = 107u32;
```

### `ESHUTDOWN`

```rust
const ESHUTDOWN: u32 = 108u32;
```

### `ETOOMANYREFS`

```rust
const ETOOMANYREFS: u32 = 109u32;
```

### `ETIMEDOUT`

```rust
const ETIMEDOUT: u32 = 110u32;
```

### `ECONNREFUSED`

```rust
const ECONNREFUSED: u32 = 111u32;
```

### `EHOSTDOWN`

```rust
const EHOSTDOWN: u32 = 112u32;
```

### `EHOSTUNREACH`

```rust
const EHOSTUNREACH: u32 = 113u32;
```

### `EALREADY`

```rust
const EALREADY: u32 = 114u32;
```

### `EINPROGRESS`

```rust
const EINPROGRESS: u32 = 115u32;
```

### `ESTALE`

```rust
const ESTALE: u32 = 116u32;
```

### `EUCLEAN`

```rust
const EUCLEAN: u32 = 117u32;
```

### `ENOTNAM`

```rust
const ENOTNAM: u32 = 118u32;
```

### `ENAVAIL`

```rust
const ENAVAIL: u32 = 119u32;
```

### `EISNAM`

```rust
const EISNAM: u32 = 120u32;
```

### `EREMOTEIO`

```rust
const EREMOTEIO: u32 = 121u32;
```

### `EDQUOT`

```rust
const EDQUOT: u32 = 122u32;
```

### `ENOMEDIUM`

```rust
const ENOMEDIUM: u32 = 123u32;
```

### `EMEDIUMTYPE`

```rust
const EMEDIUMTYPE: u32 = 124u32;
```

### `ECANCELED`

```rust
const ECANCELED: u32 = 125u32;
```

### `ENOKEY`

```rust
const ENOKEY: u32 = 126u32;
```

### `EKEYEXPIRED`

```rust
const EKEYEXPIRED: u32 = 127u32;
```

### `EKEYREVOKED`

```rust
const EKEYREVOKED: u32 = 128u32;
```

### `EKEYREJECTED`

```rust
const EKEYREJECTED: u32 = 129u32;
```

### `EOWNERDEAD`

```rust
const EOWNERDEAD: u32 = 130u32;
```

### `ENOTRECOVERABLE`

```rust
const ENOTRECOVERABLE: u32 = 131u32;
```

### `ERFKILL`

```rust
const ERFKILL: u32 = 132u32;
```

### `EHWPOISON`

```rust
const EHWPOISON: u32 = 133u32;
```

